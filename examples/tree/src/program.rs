use core::mem::transmute;
use core::ptr::read_unaligned;
use pinocchio::{
    address::address_eq,
    hint::{likely, unlikely},
    no_allocator, nostd_panic_handler,
    sysvars::rent::RENT_ID,
    AccountView, Address, SUCCESS,
};
use tree_interface::{
    cpi, data, error_codes::error, input_buffer, instruction, tree, CreateAccountInstructionData,
    Direction, InsertInstruction, Instruction, SolAccountInfo, SolAccountMeta, SolInstruction,
    SolSignerSeed, SolSignerSeeds, TreeHeader, TreeNode,
};
#[cfg(target_os = "solana")]
use {
    core::mem::MaybeUninit,
    pinocchio::syscalls::{
        sol_invoke_signed_c, sol_log_compute_units_, sol_try_find_program_address,
    },
};

#[inline(always)]
unsafe fn account_at(input_buffer_ptr: *mut u8, offset: i16) -> AccountView {
    AccountView::new_unchecked(input_buffer_ptr.add(offset as usize).cast())
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
fn is_duplicate(account: &AccountView) -> bool {
    account.is_borrowed()
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

// ANCHOR: entrypoint-branching
no_allocator!();
nostd_panic_handler!();

#[no_mangle]
pub unsafe extern "C" fn entrypoint(
    input_buffer_ptr: *mut u8,
    instruction_data_ptr: *mut u8,
) -> u64 {
    let n_accounts = ldxdw(input_buffer_ptr, input_buffer::N_ACCOUNTS_OFF);
    if likely(n_accounts == input_buffer::N_ACCOUNTS_GENERAL) {
        general(input_buffer_ptr, instruction_data_ptr)
    } else if likely(n_accounts == input_buffer::N_ACCOUNTS_INIT) {
        initialize(input_buffer_ptr, instruction_data_ptr)
    } else {
        error::N_ACCOUNTS.into()
    }
}
// ANCHOR_END: entrypoint-branching

// ANCHOR: general-branching
#[inline(always)]
unsafe fn general(input_buffer_ptr: *mut u8, instruction_data_ptr: *mut u8) -> u64 {
    // Error if user has data.
    let user = account_at(input_buffer_ptr, input_buffer::USER_ACCOUNT_OFF);
    if_err!(!user.is_data_empty(), error::USER_DATA_LEN);

    // Error if tree is duplicate.
    let tree = account_at(input_buffer_ptr, input_buffer::TREE_ACCOUNT_OFF);
    if_err!(is_duplicate(&tree), error::TREE_DUPLICATE);

    // Get instruction data length and discriminator, branch to instruction.
    let instruction_data_len = ldxdw(instruction_data_ptr, -(size_of::<u64>() as i16));
    if likely(
        ldxb(instruction_data_ptr, instruction::DISCRIMINATOR_OFF) == Instruction::Insert as u8,
    ) {
        insert(input_buffer_ptr, instruction_data_ptr, instruction_data_len)
    } else {
        error::INSTRUCTION_DISCRIMINATOR.into()
    }
}
// ANCHOR_END: general-branching

// ANCHOR: insert
#[inline(always)]
unsafe fn insert(
    input_buffer_ptr: *mut u8,
    instruction_data_ptr: *mut u8,
    instruction_data_len: u64,
) -> u64 {
    if_err!(
        instruction_data_len != size_of::<InsertInstruction>() as u64,
        error::INSTRUCTION_DATA_LEN
    );

    let tree_header: *mut TreeHeader =
        transmute(input_buffer_ptr.add(input_buffer::TREE_DATA_OFF as usize));

    if (*tree_header).top.is_null() { // If stack is empty, need to allocate a node.
    }

    SUCCESS
}
// ANCHOR_END: insert

// ANCHOR: initialize-input-checks
#[inline(always)]
unsafe fn initialize(input_buffer_ptr: *mut u8, instruction_data_ptr: *mut u8) -> u64 {
    // Error if user has data.
    let user = account_at(input_buffer_ptr, input_buffer::USER_ACCOUNT_OFF);
    if_err!(!user.is_data_empty(), error::USER_DATA_LEN);

    // Error if tree is duplicate or has data.
    let tree = account_at(input_buffer_ptr, input_buffer::TREE_ACCOUNT_OFF);
    if_err!(is_duplicate(&tree), error::TREE_DUPLICATE);
    if_err!(!tree.is_data_empty(), error::TREE_DATA_LEN);

    // Error if System Program is duplicate or has invalid data length.
    let system_program = account_at(input_buffer_ptr, input_buffer::SYSTEM_PROGRAM_ACCOUNT_OFF);
    if_err!(
        is_duplicate(&system_program),
        error::SYSTEM_PROGRAM_DUPLICATE
    );
    if_err!(
        system_program.data_len() != input_buffer::SYSTEM_PROGRAM_DATA_LEN,
        error::SYSTEM_PROGRAM_DATA_LEN
    );

    // Error if Rent account is duplicate or has incorrect address.
    let rent_sysvar = account_at(input_buffer_ptr, input_buffer::RENT_ACCOUNT_OFF);
    if_err!(is_duplicate(&rent_sysvar), error::RENT_DUPLICATE);
    if_err!(
        !address_eq(rent_sysvar.address(), &RENT_ID),
        error::RENT_ADDRESS
    );

    // Error if instruction data provided.
    let instruction_data_len = ldxdw(instruction_data_ptr, -(size_of::<u64>() as i16));
    if_err!(
        instruction_data_len != data::DATA_LEN_ZERO,
        error::INSTRUCTION_DATA
    );
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
            input_buffer_ptr,
            cpi::N_SEEDS_TRY_FIND_PDA,
            input_buffer_ptr.add(input_buffer::INIT_PROGRAM_ID_OFF_IMM as usize),
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
            &pda,
            #[allow(clippy::transmute_ptr_to_ref, clippy::missing_transmute_annotations)]
            transmute(input_buffer_ptr.add(input_buffer::TREE_ADDRESS_OFF_0 as usize))
        ),
        error::PDA_MISMATCH
    );
    // ANCHOR_END: initialize-pda-checks

    // ANCHOR: initialize-create-account
    // Pack CreateAccount instruction data.
    let instruction_data = CreateAccountInstructionData {
        discriminator: cpi::CREATE_ACCOUNT_DISCRIMINATOR,
        lamports: (cpi::ACCOUNT_DATA_SCALAR as u64)
            * ldxdw(input_buffer_ptr, input_buffer::RENT_DATA_OFF),
        space: cpi::TREE_DATA_LEN as u64,
        owner: read_unaligned(
            input_buffer_ptr
                .add(input_buffer::INIT_PROGRAM_ID_OFF_IMM as usize)
                .cast(),
        ),
    };

    // Pack account metas and infos.
    let user_key = input_buffer_ptr
        .add(input_buffer::USER_ADDRESS_OFF as usize)
        .cast();
    let tree_key = input_buffer_ptr
        .add(input_buffer::TREE_ADDRESS_OFF as usize)
        .cast();
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
            owner: input_buffer_ptr
                .add(input_buffer::USER_OWNER_OFF as usize)
                .cast(),
            lamports: input_buffer_ptr
                .add(input_buffer::USER_LAMPORTS_OFF as usize)
                .cast(),
            data: input_buffer_ptr.add(input_buffer::USER_DATA_OFF as usize),
            data_len: data::DATA_LEN_ZERO,
            rent_epoch: cpi::RENT_EPOCH_NULL,
            is_signer: true,
            is_writable: true,
            executable: false,
        },
        SolAccountInfo {
            key: tree_key,
            owner: input_buffer_ptr
                .add(input_buffer::TREE_OWNER_OFF as usize)
                .cast(),
            lamports: input_buffer_ptr
                .add(input_buffer::TREE_LAMPORTS_OFF as usize)
                .cast(),
            data: input_buffer_ptr.add(input_buffer::TREE_DATA_OFF as usize),
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
        #[allow(clippy::useless_transmute, clippy::missing_transmute_annotations)]
        program_id: transmute(&system_program_address),
        accounts: sol_account_metas.as_ptr() as *mut SolAccountMeta,
        account_len: sol_account_metas.len() as u64,
        #[allow(clippy::useless_transmute, clippy::missing_transmute_annotations)]
        data: transmute(&instruction_data),
        data_len: cpi::INSN_DATA_LEN as u64,
    };

    // Initialize signer seed for PDA bump.
    let bump_seed = SolSignerSeed {
        #[allow(clippy::useless_transmute, clippy::missing_transmute_annotations)]
        addr: transmute(&bump),
        len: size_of::<u8>() as u64,
    };

    // Initialize signer seeds for PDA.
    let signers_seeds = SolSignerSeeds {
        #[allow(clippy::useless_transmute, clippy::missing_transmute_annotations)]
        addr: transmute(&bump_seed),
        len: cpi::N_SEEDS as u64,
    };

    #[cfg(target_os = "solana")]
    sol_invoke_signed_c(
        transmute(&sol_instruction),
        transmute(&sol_account_infos),
        cpi::N_ACCOUNTS as u64,
        transmute(&signers_seeds),
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
    let next = tree.data_ptr().add(size_of::<TreeHeader>()).cast();
    (*tree.data_ptr().cast::<TreeHeader>()).next = next;
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
        (*parent).child
            [(subtree as *const TreeNode == (*parent).child[tree::DIR_R]) as usize] =
            new_root;
    } else {
        (*tree).root = new_root;
    }

    new_root
}
