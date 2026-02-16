use core::ptr::{addr_of, addr_of_mut, read_unaligned};
use pinocchio::{
    account::RuntimeAccount,
    entrypoint::NON_DUP_MARKER,
    hint::{likely, unlikely},
    no_allocator, nostd_panic_handler,
    sysvars::rent::RENT_ID,
    Address, SUCCESS,
};
use tree_interface::{
    cpi, data, error_codes::error, input_buffer, instruction, tree, CreateAccountInstructionData,
    Direction, InitializeInstruction, InsertInstruction, SolAccountInfo, SolAccountMeta,
    SolInstruction, SolSignerSeed, SolSignerSeeds, TransferInstructionData, TreeHeader, TreeNode,
};
#[cfg(target_os = "solana")]
use {
    core::mem::MaybeUninit,
    pinocchio::syscalls::{
        sol_invoke_signed_c, sol_log_compute_units_, sol_try_find_program_address,
    },
};

#[inline(always)]
unsafe fn account_at(input: *mut u8, offset: i16) -> *mut RuntimeAccount {
    input.add(offset as usize).cast()
}

#[inline(always)]
unsafe fn ldxb(ptr: *const u8, offset: i16) -> u8 {
    read_unaligned(ptr.add(offset as usize))
}

#[inline(always)]
unsafe fn ldxdw(ptr: *const u8, offset: i16) -> u64 {
    read_unaligned(ptr.add(offset as usize).cast())
}

/// Checks if the account is a duplicate by checking if it's borrowed, since this is equivalent
/// via the underlying API due to the borrow state implementation.
#[inline(always)]
unsafe fn is_duplicate(account: *const RuntimeAccount) -> bool {
    (*account).borrow_state != NON_DUP_MARKER
}

/// Compares two addresses by pointer, avoiding references in calling code while harnessing
/// the underlying `address_eq` implementation which is assembly-optimal.
#[inline(always)]
unsafe fn address_eq(a: *const Address, b: *const Address) -> bool {
    use pinocchio::address::address_eq as eq;
    eq(&*a, &*b)
}

/// Insert a syscall to log CUs, useful for sectioning off disassembled program.
#[allow(dead_code)]
unsafe fn log_cus() {
    #[cfg(target_os = "solana")]
    sol_log_compute_units_();
}

macro_rules! if_err {
    ($condition:expr, $error:expr) => {
        if unlikely($condition) {
            return $error.into();
        }
    };
}

macro_rules! check_instruction_data_len {
    ($instruction_data_len:expr, $type:ty) => {
        if_err!(
            $instruction_data_len != size_of::<$type>() as u64,
            error::INSTRUCTION_DATA_LEN
        );
    };
}

macro_rules! user_account {
    ($input:expr) => {{
        let user = account_at($input, input_buffer::USER_ACCOUNT_OFF);
        if_err!(
            (*user).data_len != data::DATA_LEN_ZERO,
            error::USER_DATA_LEN
        );
        user
    }};
}

macro_rules! check_data_len {
    ($account:expr, $expected:expr, $error:expr) => {
        if_err!((*$account).data_len != $expected, $error);
    };
}

macro_rules! account_non_dup {
    ($input:expr, $offset:expr, $error:expr) => {{
        let account = account_at($input, $offset);
        if_err!(is_duplicate(account), $error);
        account
    }};
}

/// Checks the System Program and Rent sysvar accounts relative to a given input buffer pointer.
/// In `initialize`, this is the base `input`; in `insert`, it is `shifted_input` which accounts
/// for the tree's existing data length.
macro_rules! check_cpi_accounts {
    ($input:expr) => {
        let system_program = account_non_dup!(
            $input,
            input_buffer::SYSTEM_PROGRAM_ACCOUNT_OFF,
            error::SYSTEM_PROGRAM_DUPLICATE
        );
        check_data_len!(
            system_program,
            input_buffer::SYSTEM_PROGRAM_DATA_LEN as u64,
            error::SYSTEM_PROGRAM_DATA_LEN
        );
        let rent_sysvar = account_non_dup!(
            $input,
            input_buffer::RENT_ACCOUNT_OFF,
            error::RENT_DUPLICATE
        );
        let rent_id = RENT_ID;
        if_err!(
            !address_eq(addr_of!((*rent_sysvar).address), addr_of!(rent_id)),
            error::RENT_ADDRESS
        );
    };
}

// ANCHOR: entrypoint-branching
no_allocator!();
nostd_panic_handler!();

#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8, instruction_data: *mut u8) -> u64 {
    let instruction_data_len = ldxdw(instruction_data, -(size_of::<u64>() as i16));
    let n_accounts = ldxdw(input, input_buffer::N_ACCOUNTS_OFF);
    let instruction_discriminator = ldxb(instruction_data, instruction::DISCRIMINATOR_OFF);
    if likely(instruction_discriminator == instruction::DISCRIMINATOR_INSERT) {
        insert(input, instruction_data, instruction_data_len, n_accounts)
    } else if likely(instruction_discriminator == instruction::DISCRIMINATOR_INITIALIZE) {
        initialize(input, instruction_data, instruction_data_len, n_accounts)
    } else {
        error::INSTRUCTION_DISCRIMINATOR.into()
    }
}
// ANCHOR_END: entrypoint-branching

// ANCHOR: insert-input-checks
#[inline(always)]
unsafe fn insert(
    input: *mut u8,
    instruction_data: *mut u8,
    instruction_data_len: u64,
    n_accounts: u64,
) -> u64 {
    check_instruction_data_len!(instruction_data_len, InsertInstruction);

    // Error if too few accounts.
    if_err!(
        n_accounts < input_buffer::N_ACCOUNTS_GENERAL,
        error::N_ACCOUNTS
    );

    // Error if user has data.
    let user = user_account!(input);

    // Error if tree is duplicate.
    let tree = account_non_dup!(input, input_buffer::TREE_ACCOUNT_OFF, error::TREE_DUPLICATE);
    // ANCHOR_END: insert-input-checks

    // ANCHOR: insert-allocate
    // Allocate or recycle a node.
    let tree_header: *mut TreeHeader = input.add(input_buffer::TREE_DATA_OFF as usize).cast();
    let node: *mut TreeNode = if (*tree_header).top.is_null() {
        // Error if wrong number of accounts passed, since need extra accounts to allocate space.
        if_err!(
            n_accounts != input_buffer::N_ACCOUNTS_INIT,
            error::N_ACCOUNTS_INSERT_ALLOCATION
        );

        // Get shifted input buffer pointer based on tree data length.
        let tree_data_len: *mut u64 = addr_of_mut!((*tree).data_len);
        let shifted_input =
            input.add((*tree_data_len).next_multiple_of(data::BPF_ALIGN_OF_U128 as u64) as usize);

        // Check system program and rent sysvar accounts using shifted input buffer pointer.
        check_cpi_accounts!(shifted_input);

        // Calculate additional lamports for rent exemption of one TreeNode.
        let lamports_per_byte = ldxdw(shifted_input, input_buffer::RENT_DATA_OFF);
        let transfer_lamports = size_of::<TreeNode>() as u64 * lamports_per_byte;

        // Pack Transfer instruction data.
        let transfer_instruction_data = TransferInstructionData {
            discriminator: cpi::TRANSFER_DISCRIMINATOR,
            lamports: transfer_lamports,
        };

        // Pack account metas and infos.
        let user_key = input.add(input_buffer::USER_ADDRESS_OFF as usize).cast();
        let tree_key = input.add(input_buffer::TREE_ADDRESS_OFF as usize).cast();
        let sol_account_metas = [
            SolAccountMeta {
                pubkey: user_key,
                is_writable: true,
                is_signer: true,
            },
            SolAccountMeta {
                pubkey: tree_key,
                is_writable: true,
                is_signer: false,
            },
        ];
        let sol_account_infos = [
            SolAccountInfo {
                key: user_key,
                owner: input.add(input_buffer::USER_OWNER_OFF as usize).cast(),
                lamports: input.add(input_buffer::USER_LAMPORTS_OFF as usize).cast(),
                data: input.add(input_buffer::USER_DATA_OFF as usize),
                data_len: data::DATA_LEN_ZERO,
                rent_epoch: cpi::RENT_EPOCH_NULL,
                is_signer: true,
                is_writable: true,
                executable: false,
            },
            SolAccountInfo {
                key: tree_key,
                owner: input.add(input_buffer::TREE_OWNER_OFF as usize).cast(),
                lamports: input.add(input_buffer::TREE_LAMPORTS_OFF as usize).cast(),
                data: input.add(input_buffer::TREE_DATA_OFF as usize),
                data_len: *tree_data_len,
                rent_epoch: cpi::RENT_EPOCH_NULL,
                is_signer: false,
                is_writable: true,
                executable: false,
            },
        ];

        // Pack instruction.
        let system_program_address = Address::default();
        let sol_instruction = SolInstruction {
            program_id: addr_of!(system_program_address).cast_mut().cast(),
            accounts: sol_account_metas.as_ptr().cast_mut().cast(),
            account_len: sol_account_metas.len() as u64,
            data: addr_of!(transfer_instruction_data).cast_mut().cast(),
            data_len: cpi::TRANSFER_INSN_DATA_LEN as u64,
        };

        // No signers needed, since user is already a signer on the transaction.
        let empty_signers = SolSignerSeeds {
            addr: core::ptr::null(),
            len: 0,
        };

        #[cfg(target_os = "solana")]
        sol_invoke_signed_c(
            addr_of!(sol_instruction).cast(),
            addr_of!(sol_account_infos).cast(),
            cpi::N_ACCOUNTS as u64,
            addr_of!(empty_signers).cast(),
            cpi::N_PDA_SIGNERS_TRANSFER,
        );
        #[cfg(not(target_os = "solana"))]
        #[allow(path_statements)]
        {
            empty_signers;
            sol_account_infos;
            sol_instruction;
        }

        // Increase tree data length by size of one TreeNode.
        *tree_data_len += size_of::<TreeNode>() as u64;

        // Advance next pointer by one TreeNode.
        let node = (*tree_header).next;
        (*tree_header).next = (*tree_header).next.add(1);
        node
    } else {
        // Pop node from free stack.
        let top = (*tree_header).top;
        (*tree_header).top = (*top).next;
        top.cast()
    };
    // ANCHOR_END: insert-allocate

    // Initialize node as root of tree.
    (*tree_header).root = node;

    // Set key and value together as a single word.
    *addr_of_mut!((*node).key).cast::<u32>() = read_unaligned(
        instruction_data
            .add(instruction::INSERT_KEY_OFF as usize)
            .cast(),
    );

    SUCCESS
}

// ANCHOR: initialize-input-checks
#[inline(always)]
unsafe fn initialize(
    input: *mut u8,
    instruction_data: *mut u8,
    instruction_data_len: u64,
    n_accounts: u64,
) -> u64 {
    check_instruction_data_len!(instruction_data_len, InitializeInstruction);

    // Error if incorrect number of accounts.
    if_err!(
        n_accounts != input_buffer::N_ACCOUNTS_INIT,
        error::N_ACCOUNTS
    );

    // Error if user has data.
    let user = user_account!(input);

    // Error if tree is duplicate or has data.
    let tree = account_non_dup!(input, input_buffer::TREE_ACCOUNT_OFF, error::TREE_DUPLICATE);
    check_data_len!(tree, data::DATA_LEN_ZERO, error::TREE_DATA_LEN);

    check_cpi_accounts!(input);
    // ANCHOR_END: initialize-input-checks

    // ANCHOR: initialize-pda-checks
    #[cfg(target_os = "solana")]
    // Invoke syscall.
    let (pda, bump) = {
        let mut pda = MaybeUninit::<Address>::uninit();
        let mut bump = MaybeUninit::<u8>::uninit();
        // Get input buffer footer pointer.
        sol_try_find_program_address(
            // Pass a declared pointer instead of null to prevent unnecessary register assignment.
            input,
            cpi::N_SEEDS_TRY_FIND_PDA,
            input.add(input_buffer::INIT_PROGRAM_ID_OFF_IMM as usize),
            pda.as_mut_ptr().cast(),
            bump.as_mut_ptr(),
        );
        (pda.assume_init(), bump.assume_init())
    };
    #[cfg(not(target_os = "solana"))]
    let (pda, bump) = (Address::default(), 0);

    // Compare result with passed PDA.
    if_err!(
        !address_eq(
            addr_of!(pda),
            input.add(input_buffer::TREE_ADDRESS_OFF_0 as usize).cast()
        ),
        error::PDA_MISMATCH
    );
    // ANCHOR_END: initialize-pda-checks

    // ANCHOR: initialize-create-account
    // Pack CreateAccount instruction data.
    let create_account_instruction_data = CreateAccountInstructionData {
        discriminator: cpi::CREATE_ACCOUNT_DISCRIMINATOR,
        lamports: (cpi::ACCOUNT_DATA_SCALAR as u64) * ldxdw(input, input_buffer::RENT_DATA_OFF),
        space: cpi::TREE_DATA_LEN as u64,
        owner: read_unaligned(
            input
                .add(input_buffer::INIT_PROGRAM_ID_OFF_IMM as usize)
                .cast(),
        ),
    };

    // Pack account metas and infos.
    let user_key = input.add(input_buffer::USER_ADDRESS_OFF as usize).cast();
    let tree_key = input.add(input_buffer::TREE_ADDRESS_OFF as usize).cast();
    let sol_account_metas = [
        SolAccountMeta {
            pubkey: user_key,
            is_writable: true,
            is_signer: true,
        },
        SolAccountMeta {
            pubkey: tree_key,
            is_writable: true,
            is_signer: true,
        },
    ];
    let sol_account_infos = [
        SolAccountInfo {
            key: user_key,
            owner: input.add(input_buffer::USER_OWNER_OFF as usize).cast(),
            lamports: input.add(input_buffer::USER_LAMPORTS_OFF as usize).cast(),
            data: input.add(input_buffer::USER_DATA_OFF as usize),
            data_len: data::DATA_LEN_ZERO,
            rent_epoch: cpi::RENT_EPOCH_NULL,
            is_signer: true,
            is_writable: true,
            executable: false,
        },
        SolAccountInfo {
            key: tree_key,
            owner: input.add(input_buffer::TREE_OWNER_OFF as usize).cast(),
            lamports: input.add(input_buffer::TREE_LAMPORTS_OFF as usize).cast(),
            data: input.add(input_buffer::TREE_DATA_OFF as usize),
            data_len: data::DATA_LEN_ZERO,
            rent_epoch: cpi::RENT_EPOCH_NULL,
            is_signer: true,
            is_writable: true,
            executable: false,
        },
    ];

    // Pack instruction.
    let system_program_address = Address::default();
    let sol_instruction = SolInstruction {
        program_id: addr_of!(system_program_address).cast_mut().cast(),
        accounts: sol_account_metas.as_ptr().cast_mut().cast(),
        account_len: sol_account_metas.len() as u64,
        data: addr_of!(create_account_instruction_data).cast_mut().cast(),
        data_len: cpi::CREATE_ACCOUNT_INSN_DATA_LEN as u64,
    };

    // Initialize signer seed for PDA bump.
    let bump_seed = SolSignerSeed {
        addr: addr_of!(bump).cast(),
        len: size_of::<u8>() as u64,
    };

    // Initialize signer seeds for PDA.
    let signers_seeds = SolSignerSeeds {
        addr: addr_of!(bump_seed).cast(),
        len: cpi::N_SEEDS_CREATE_ACCOUNT as u64,
    };

    #[cfg(target_os = "solana")]
    sol_invoke_signed_c(
        addr_of!(sol_instruction).cast(),
        addr_of!(sol_account_infos).cast(),
        cpi::N_ACCOUNTS as u64,
        addr_of!(signers_seeds).cast(),
        cpi::N_PDA_SIGNERS as u64,
    );
    #[cfg(not(target_os = "solana"))]
    #[allow(path_statements)]
    {
        signers_seeds;
        sol_account_infos;
        sol_instruction;
    }

    // Store next pointer in tree header.
    let tree_data: *mut TreeHeader = input.add(input_buffer::TREE_DATA_OFF as usize).cast();
    (*tree_data).next = tree_data.add(1).cast();
    // ANCHOR_END: initialize-create-account

    SUCCESS
}

/// Return the direction of the node with respect to its parent.
#[inline(always)]
unsafe fn direction(node: *const TreeNode) -> Direction {
    if node == (*(*node).parent).child[tree::DIR_R] {
        Direction::Right
    } else {
        Direction::Left
    }
}

#[inline(always)]
const fn opposite(direction: usize) -> usize {
    1 - direction
}

/// Rotate the subtree rooted at `subtree` in the given direction, returning new root of subtree.
#[inline(always)]
unsafe fn rotate_subtree(
    tree: *mut TreeHeader,
    subtree: *mut TreeNode,
    direction: usize,
) -> *mut TreeNode {
    let parent = (*subtree).parent;
    let new_root = (*subtree).child[opposite(direction)];
    let new_child = (*new_root).child[direction];

    (*subtree).child[opposite(direction)] = new_child;

    if !new_child.is_null() {
        (*new_child).parent = subtree;
    }

    (*new_root).child[direction] = subtree;
    (*new_root).parent = parent;
    (*subtree).parent = new_root;

    if !parent.is_null() {
        (*parent).child[(subtree == (*parent).child[tree::DIR_R]) as usize] = new_root;
    } else {
        (*tree).root = new_root;
    }

    new_root
}
