use super::*;
use tree_interface::{
    input_buffer, tree, Color, InsertInstruction, Instruction as TreeInstruction,
    InstructionHeader, StackNode, TreeHeader, TreeNode,
};

// ---------------------------------------------------------------------------
// Helpers: allocation setup
// ---------------------------------------------------------------------------

fn insert_instruction_data() -> InsertInstruction {
    InsertInstruction {
        header: InstructionHeader {
            discriminator: TreeInstruction::Insert as u8,
        },
        key: 42,
        value: 1,
    }
}

fn insert_setup(
    program_language: ProgramLanguage,
) -> (TestSetup, Instruction, Vec<(Pubkey, Account)>) {
    let mut setup = setup_test_with_rent(program_language);
    let (system_program_pubkey, system_program_account) =
        program::keyed_account_for_system_program();
    let (rent_sysvar_pubkey, rent_sysvar_account) =
        setup.mollusk.sysvars.keyed_account_for_rent_sysvar();

    let user_pubkey = Pubkey::new_unique();
    let tree_pubkey = Pubkey::new_unique();

    let insn_data = insert_instruction_data();
    let instruction = Instruction::new_with_bytes(
        setup.program_id,
        unsafe { as_bytes(&insn_data) },
        vec![
            AccountMeta::new(user_pubkey, true),
            AccountMeta::new(tree_pubkey, false),
            AccountMeta::new_readonly(system_program_pubkey, false),
            AccountMeta::new_readonly(rent_sysvar_pubkey, false),
        ],
    );

    // Tree starts with TREE_DATA_LEN (header only), top = null to trigger allocation.
    let rent = Rent::from_bytes(&rent_sysvar_account.data).unwrap();
    let tree_lamports = rent.try_minimum_balance(cpi::TREE_DATA_LEN).unwrap();
    let mut tree_account = Account::new(tree_lamports, cpi::TREE_DATA_LEN, &setup.program_id);
    // top is null (zeroed) — triggers allocation path.
    // next pointer must point to the first allocation slot (right after header).
    let next_ptr =
        MM_INPUT_START + input_buffer::TREE_DATA_OFF as u64 + size_of::<TreeHeader>() as u64;
    let next_off = tree::HEADER_NEXT_OFF as usize;
    tree_account.data[next_off..next_off + size_of::<*mut TreeNode>()]
        .copy_from_slice(&next_ptr.to_le_bytes());

    let accounts = vec![
        (
            user_pubkey,
            Account::new(USER_LAMPORTS, 0, &system_program_pubkey),
        ),
        (tree_pubkey, tree_account),
        (system_program_pubkey, system_program_account),
        (rent_sysvar_pubkey, rent_sysvar_account),
    ];

    (setup, instruction, accounts)
}

fn insert_skip_alloc_setup(
    program_language: ProgramLanguage,
) -> (TestSetup, Instruction, Vec<(Pubkey, Account)>) {
    let setup = setup_test(program_language);
    let (system_program_pubkey, _) = program::keyed_account_for_system_program();

    let user_pubkey = Pubkey::new_unique();
    let tree_pubkey = Pubkey::new_unique();

    let insn_data = insert_instruction_data();
    let instruction = Instruction::new_with_bytes(
        setup.program_id,
        unsafe { as_bytes(&insn_data) },
        vec![
            AccountMeta::new(user_pubkey, true),
            AccountMeta::new(tree_pubkey, false),
        ],
    );

    // Initialize tree account with a free node on the stack so insert pops instead of allocating.
    let tree_data_len = cpi::TREE_DATA_LEN + size_of::<TreeNode>();
    let mut tree_data = vec![0u8; tree_data_len];
    // top points to the free node (right after header in memory map).
    let top_ptr =
        MM_INPUT_START + input_buffer::TREE_DATA_OFF as u64 + size_of::<TreeHeader>() as u64;
    let top_off = tree::HEADER_TOP_OFF as usize;
    tree_data[top_off..top_off + size_of::<*mut StackNode>()]
        .copy_from_slice(&top_ptr.to_le_bytes());
    // Free node's next is null (zeroed) — only one free node on the stack.
    let mut tree_account = Account::new(0, tree_data_len, &setup.program_id);
    tree_account.data = tree_data;

    let accounts = vec![
        (
            user_pubkey,
            Account::new(USER_LAMPORTS, 0, &system_program_pubkey),
        ),
        (tree_pubkey, tree_account),
    ];

    (setup, instruction, accounts)
}

fn insert_alloc_address_mismatch(
    lang: ProgramLanguage,
    account_index: usize,
    chunk_index: usize,
    expected_error: error_codes::error,
) -> CaseResult {
    let (setup, mut instruction, mut accounts) = insert_setup(lang);
    flip_account_address(
        &mut instruction,
        &mut accounts,
        account_index,
        chunk_index,
        size_of::<u64>(),
    );
    check_error(&setup, &instruction, &accounts, expected_error)
}

/// Set up an insert where the tree account is already at
/// `MAX_PERMITTED_DATA_LENGTH` so that allocating one more `TreeNode`
/// exceeds the absolute account size limit.
fn insert_max_data_setup(
    program_language: ProgramLanguage,
) -> (TestSetup, Instruction, Vec<(Pubkey, Account)>) {
    const MAX_PERMITTED_DATA_LENGTH: usize = 10 * 1024 * 1024;

    let mut setup = setup_test_with_rent(program_language);
    let (system_program_pubkey, system_program_account) =
        program::keyed_account_for_system_program();
    let (rent_sysvar_pubkey, rent_sysvar_account) =
        setup.mollusk.sysvars.keyed_account_for_rent_sysvar();

    let user_pubkey = Pubkey::new_unique();
    let tree_pubkey = Pubkey::new_unique();

    let insn_data = insert_instruction_data();
    let instruction = Instruction::new_with_bytes(
        setup.program_id,
        unsafe { as_bytes(&insn_data) },
        vec![
            AccountMeta::new(user_pubkey, true),
            AccountMeta::new(tree_pubkey, false),
            AccountMeta::new_readonly(system_program_pubkey, false),
            AccountMeta::new_readonly(rent_sysvar_pubkey, false),
        ],
    );

    // Tree account already at MAX_PERMITTED_DATA_LENGTH.
    let rent = Rent::from_bytes(&rent_sysvar_account.data).unwrap();
    let tree_lamports = rent.try_minimum_balance(MAX_PERMITTED_DATA_LENGTH).unwrap();
    let mut tree_account =
        Account::new(tree_lamports, MAX_PERMITTED_DATA_LENGTH, &setup.program_id);
    // top = null → forces allocation path (no free nodes to recycle).
    // next → points right after current data (where the new node would go).
    let next_ptr =
        MM_INPUT_START + input_buffer::TREE_DATA_OFF as u64 + MAX_PERMITTED_DATA_LENGTH as u64;
    let next_off = tree::HEADER_NEXT_OFF as usize;
    tree_account.data[next_off..next_off + size_of::<*mut TreeNode>()]
        .copy_from_slice(&next_ptr.to_le_bytes());

    let accounts = vec![
        (
            user_pubkey,
            Account::new(USER_LAMPORTS, 0, &system_program_pubkey),
        ),
        (tree_pubkey, tree_account),
        (system_program_pubkey, system_program_account),
        (rent_sysvar_pubkey, rent_sysvar_account),
    ];

    (setup, instruction, accounts)
}

// ---------------------------------------------------------------------------
// Helpers: tree description types
// ---------------------------------------------------------------------------

struct NodeSpec {
    key: u16,
    value: u16,
    color: u8,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

impl NodeSpec {
    fn val(mut self, v: u16) -> Self {
        self.value = v;
        self
    }
}

struct TreeSpec<'a> {
    root: Option<usize>,
    top: Option<usize>,
    nodes: &'a [NodeSpec],
}

/// Compute the virtual address of node slot `i` in the tree account.
fn node_vaddr(i: usize) -> u64 {
    MM_INPUT_START
        + input_buffer::TREE_DATA_OFF as u64
        + size_of::<TreeHeader>() as u64
        + (i as u64) * (size_of::<TreeNode>() as u64)
}

/// Convert an optional node index to a virtual address (0 for None).
fn opt_vaddr(idx: Option<usize>) -> u64 {
    match idx {
        Some(i) => node_vaddr(i),
        None => 0,
    }
}

// ---------------------------------------------------------------------------
// Helper: build tree account data
// ---------------------------------------------------------------------------

/// Build tree account data with pre-existing nodes and one free StackNode.
///
/// Memory layout: TreeHeader | node[0] | node[1] | ... | node[N-1] | free_slot
///
/// - `header.root` → virtual address of `nodes[root]`, or null.
/// - `header.top`  → virtual address of the free slot (index = nodes.len()).
/// - `header.next` → 0 (unused in skip-alloc path).
fn build_tree_account(desc: &TreeSpec, program_id: &Pubkey) -> (Pubkey, Account) {
    let n = desc.nodes.len();
    // N existing nodes + 1 free slot.
    let data_len = size_of::<TreeHeader>() + (n + 1) * size_of::<TreeNode>();
    let mut data = vec![0u8; data_len];

    // Write header.
    let header = data.as_mut_ptr() as *mut TreeHeader;
    unsafe {
        (*header).root = opt_vaddr(desc.root) as *mut TreeNode;
        (*header).top = node_vaddr(n) as *mut StackNode;
        (*header).next = core::ptr::null_mut();
    }

    // Write existing nodes.
    for (i, node) in desc.nodes.iter().enumerate() {
        let offset = size_of::<TreeHeader>() + i * size_of::<TreeNode>();
        let ptr = unsafe { data.as_mut_ptr().add(offset) as *mut TreeNode };
        unsafe {
            (*ptr).parent = opt_vaddr(node.parent) as *mut TreeNode;
            (*ptr).child[tree::DIR_L] = opt_vaddr(node.left) as *mut TreeNode;
            (*ptr).child[tree::DIR_R] = opt_vaddr(node.right) as *mut TreeNode;
            (*ptr).key = node.key;
            (*ptr).value = node.value;
            (*ptr).color = core::mem::transmute(node.color);
        }
    }

    // Free slot is already zeroed (StackNode.next = null).

    let pubkey = Pubkey::new_unique();
    let mut account = Account::new(0, data_len, program_id);
    account.data = data;
    (pubkey, account)
}

// ---------------------------------------------------------------------------
// Helper: assert tree account (full state)
// ---------------------------------------------------------------------------

/// Assert every field of the tree account data against expected state.
/// Returns Ok(()) on match, Err(description) on mismatch.
fn assert_tree_account(data: &[u8], expected: &TreeSpec) -> Result<(), String> {
    let mut errors = Vec::new();
    let n = expected.nodes.len();

    // Check data length (at least enough for the expected nodes).
    let min_len = size_of::<TreeHeader>() + n * size_of::<TreeNode>();
    if data.len() < min_len {
        errors.push(format!(
            "data len: expected at least {}, got {}",
            min_len,
            data.len()
        ));
    }

    // Check header.
    let header = data.as_ptr() as *const TreeHeader;
    unsafe {
        let root_addr = (*header).root as u64;
        let expected_root = opt_vaddr(expected.root);
        if root_addr != expected_root {
            errors.push(format!(
                "header.root: expected {:#x}, got {:#x}",
                expected_root, root_addr
            ));
        }

        let top_addr = (*header).top as u64;
        let expected_top = opt_vaddr(expected.top);
        if top_addr != expected_top {
            errors.push(format!(
                "header.top: expected {:#x}, got {:#x}",
                expected_top, top_addr
            ));
        }

        let next_addr = (*header).next as u64;
        if next_addr != 0 {
            errors.push(format!("header.next: expected 0x0, got {:#x}", next_addr));
        }
    }

    // Check each node.
    for i in 0..n {
        let offset = size_of::<TreeHeader>() + i * size_of::<TreeNode>();
        if offset + size_of::<TreeNode>() > data.len() {
            errors.push(format!("N{}: out of bounds", i));
            continue;
        }
        let ptr = unsafe { data.as_ptr().add(offset) as *const TreeNode };
        let exp = &expected.nodes[i];
        let label = format!("N{}", i);

        unsafe {
            let parent_addr = core::ptr::read_unaligned(core::ptr::addr_of!((*ptr).parent)) as u64;
            let expected_parent = opt_vaddr(exp.parent);
            if parent_addr != expected_parent {
                errors.push(format!(
                    "{}.parent: expected {:#x}, got {:#x}",
                    label, expected_parent, parent_addr
                ));
            }

            let left_addr =
                core::ptr::read_unaligned(core::ptr::addr_of!((*ptr).child[tree::DIR_L])) as u64;
            let expected_left = opt_vaddr(exp.left);
            if left_addr != expected_left {
                errors.push(format!(
                    "{}.L: expected {:#x}, got {:#x}",
                    label, expected_left, left_addr
                ));
            }

            let right_addr =
                core::ptr::read_unaligned(core::ptr::addr_of!((*ptr).child[tree::DIR_R])) as u64;
            let expected_right = opt_vaddr(exp.right);
            if right_addr != expected_right {
                errors.push(format!(
                    "{}.R: expected {:#x}, got {:#x}",
                    label, expected_right, right_addr
                ));
            }

            let key = core::ptr::read_unaligned(core::ptr::addr_of!((*ptr).key));
            if key != exp.key {
                errors.push(format!("{}.key: expected {}, got {}", label, exp.key, key));
            }

            let value = core::ptr::read_unaligned(core::ptr::addr_of!((*ptr).value));
            if value != exp.value {
                errors.push(format!(
                    "{}.value: expected {}, got {}",
                    label, exp.value, value
                ));
            }

            let color = core::ptr::read_unaligned(core::ptr::addr_of!((*ptr).color)) as u8;
            if color != exp.color {
                let color_name = |c: u8| if c == 0 { "B" } else { "R" };
                errors.push(format!(
                    "{}.color: expected {}, got {}",
                    label,
                    color_name(exp.color),
                    color_name(color)
                ));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("; "))
    }
}

// ---------------------------------------------------------------------------
// Helpers: tree test setup and runners
// ---------------------------------------------------------------------------

fn insert_tree_setup(
    lang: ProgramLanguage,
    desc: &TreeSpec,
    insert_key: u16,
) -> (TestSetup, Instruction, Vec<(Pubkey, Account)>) {
    let setup = setup_test(lang);
    let (system_program_pubkey, _) = mollusk_svm::program::keyed_account_for_system_program();

    let user_pubkey = Pubkey::new_unique();
    let (tree_pubkey, tree_account) = build_tree_account(desc, &setup.program_id);

    let insn_data = InsertInstruction {
        header: InstructionHeader {
            discriminator: TreeInstruction::Insert as u8,
        },
        key: insert_key,
        value: 1,
    };

    let instruction = Instruction::new_with_bytes(
        setup.program_id,
        unsafe { as_bytes(&insn_data) },
        vec![
            AccountMeta::new(user_pubkey, true),
            AccountMeta::new(tree_pubkey, false),
        ],
    );

    let accounts = vec![
        (
            user_pubkey,
            Account::new(USER_LAMPORTS, 0, &system_program_pubkey),
        ),
        (tree_pubkey, tree_account),
    ];

    (setup, instruction, accounts)
}

/// Run an insert and assert success with full tree state check.
fn run_success(
    lang: ProgramLanguage,
    desc: &TreeSpec,
    insert_key: u16,
    expected: &TreeSpec,
) -> CaseResult {
    let (setup, instruction, accounts) = insert_tree_setup(lang, desc, insert_key);
    let result = setup.mollusk.process_instruction(&instruction, &accounts);
    match &result.program_result {
        MolluskResult::Success => {
            let tree_data = &result.resulting_accounts[AccountIndex::Tree as usize]
                .1
                .data;
            match assert_tree_account(tree_data, expected) {
                Ok(()) => CaseResult {
                    cu: result.compute_units_consumed,
                    error: None,
                },
                Err(e) => CaseResult {
                    cu: result.compute_units_consumed,
                    error: Some(e),
                },
            }
        }
        other => CaseResult {
            cu: result.compute_units_consumed,
            error: Some(format!("expected Success, got {:?}", other)),
        },
    }
}

/// Run an insert and check for KEY_EXISTS error.
fn run_dup_error(lang: ProgramLanguage, desc: &TreeSpec, insert_key: u16) -> CaseResult {
    let (setup, instruction, accounts) = insert_tree_setup(lang, desc, insert_key);
    check_error(
        &setup,
        &instruction,
        &accounts,
        error_codes::error::KEY_EXISTS,
    )
}

// ---------------------------------------------------------------------------
// Shorthand constructors
// ---------------------------------------------------------------------------

const B: u8 = Color::Black as u8;
const R: u8 = Color::Red as u8;

fn node(
    key: u16,
    color: u8,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
) -> NodeSpec {
    NodeSpec {
        key,
        value: key,
        color,
        parent,
        left,
        right,
    }
}

// ---------------------------------------------------------------------------
// Test case enum
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub(super) enum InsertCase {
    // Input validation.
    InputDataShort,
    InputDataLong,
    InputNAccounts,
    InputUserDataLen,
    InputTreeDuplicate,
    // Allocation checks.
    AllocNAccounts,
    AllocSysprogDuplicate,
    AllocSysprogDataLen,
    AllocRentDuplicate,
    AllocRentAddrChunk0,
    AllocRentAddrChunk1,
    AllocRentAddrChunk2,
    AllocRentAddrChunk3,
    // Allocation happy path (CPI overhead).
    AllocHappyPath,
    // Allocation exceeds max permitted data length.
    AllocMaxDataLen,
    // Search — expect KEY_EXISTS error.
    SearchDupRoot,
    SearchDupLeft,
    SearchDupRight,
    // Insert to empty tree.
    EmptyTree,
    // Case 1: parent is black.
    Case1Left,
    Case1Right,
    // Case 4: parent is root and red.
    Case4Left,
    Case4Right,
    // Case 2+3: red uncle, propagate to root.
    Case23LeftLeft,
    Case23LeftRight,
    Case23RightLeft,
    Case23RightRight,
    // Case 2+1: red uncle, propagate to black ancestor.
    Case21Left,
    Case21Right,
    // Case 6: single rotation (outer child).
    Case6LeftNull,
    Case6RightNull,
    Case6LeftBlack,
    Case6RightBlack,
    // Case 5+6: double rotation (inner child).
    Case56LeftNull,
    Case56RightNull,
    Case56LeftBlack,
    Case56RightBlack,
    // Case 6: non-null great-grandparent.
    Case6GgpLeftLeft,
    Case6GgpLeftRight,
    Case6GgpRightRight,
    Case6GgpRightLeft,
    // Case 2+6: non-null new_child in rotation.
    Case26Left,
    Case26Right,
    // Case 2+5+6: non-null new_child in rotations.
    Case256Left,
    Case256Right,
}

impl InsertCase {
    pub(super) const INPUT_CASES: &'static [Self] = &[
        Self::InputDataShort,
        Self::InputDataLong,
        Self::InputNAccounts,
        Self::InputUserDataLen,
        Self::InputTreeDuplicate,
    ];

    pub(super) const ALLOC_CHECK_CASES: &'static [Self] = &[
        Self::AllocNAccounts,
        Self::AllocSysprogDuplicate,
        Self::AllocSysprogDataLen,
        Self::AllocRentDuplicate,
        Self::AllocRentAddrChunk0,
        Self::AllocRentAddrChunk1,
        Self::AllocRentAddrChunk2,
        Self::AllocRentAddrChunk3,
    ];

    pub(super) const ALLOC_CASES: &'static [Self] = &[Self::AllocHappyPath, Self::AllocMaxDataLen];

    pub(super) const SEARCH_CASES: &'static [Self] = &[
        Self::SearchDupRoot,
        Self::SearchDupLeft,
        Self::SearchDupRight,
    ];

    pub(super) const TREE_CASES: &'static [Self] = &[
        Self::EmptyTree,
        Self::Case1Left,
        Self::Case1Right,
        Self::Case4Left,
        Self::Case4Right,
        Self::Case23LeftLeft,
        Self::Case23LeftRight,
        Self::Case23RightLeft,
        Self::Case23RightRight,
        Self::Case21Left,
        Self::Case21Right,
        Self::Case6LeftNull,
        Self::Case6RightNull,
        Self::Case6LeftBlack,
        Self::Case6RightBlack,
        Self::Case56LeftNull,
        Self::Case56RightNull,
        Self::Case56LeftBlack,
        Self::Case56RightBlack,
        Self::Case6GgpLeftLeft,
        Self::Case6GgpLeftRight,
        Self::Case6GgpRightRight,
        Self::Case6GgpRightLeft,
        Self::Case26Left,
        Self::Case26Right,
        Self::Case256Left,
        Self::Case256Right,
    ];
}

impl TestCase for InsertCase {
    fn name(&self) -> &'static str {
        match self {
            Self::InputDataShort => "Instruction data too short",
            Self::InputDataLong => "Instruction data too long",
            Self::InputNAccounts => "Too few accounts",
            Self::InputUserDataLen => "User has nonzero data length",
            Self::InputTreeDuplicate => "Tree account is duplicate",
            Self::AllocNAccounts => "Wrong N accounts for allocation",
            Self::AllocSysprogDuplicate => "System program is duplicate",
            Self::AllocSysprogDataLen => "System program wrong data length",
            Self::AllocRentDuplicate => "Rent sysvar is duplicate",
            Self::AllocRentAddrChunk0 => "Rent address mismatch chunk 0",
            Self::AllocRentAddrChunk1 => "Rent address mismatch chunk 1",
            Self::AllocRentAddrChunk2 => "Rent address mismatch chunk 2",
            Self::AllocRentAddrChunk3 => "Rent address mismatch chunk 3",
            Self::AllocHappyPath => "Insert alloc happy path",
            Self::AllocMaxDataLen => "Alloc exceeds max data length",
            Self::SearchDupRoot => "Dup at root",
            Self::SearchDupLeft => "Dup in left",
            Self::SearchDupRight => "Dup in right",
            Self::EmptyTree => "Empty tree",
            Self::Case1Left => "Case 1: left child",
            Self::Case1Right => "Case 1: right child",
            Self::Case4Left => "Case 4: left child",
            Self::Case4Right => "Case 4: right child",
            Self::Case23LeftLeft => "Case 2+3: left-left",
            Self::Case23LeftRight => "Case 2+3: left-right",
            Self::Case23RightLeft => "Case 2+3: right-left",
            Self::Case23RightRight => "Case 2+3: right-right",
            Self::Case21Left => "Case 2+1: left",
            Self::Case21Right => "Case 2+1: right",
            Self::Case6LeftNull => "Case 6: left-left null uncle",
            Self::Case6RightNull => "Case 6: right-right null uncle",
            Self::Case6LeftBlack => "Case 6: left-left black uncle",
            Self::Case6RightBlack => "Case 6: right-right black uncle",
            Self::Case56LeftNull => "Case 5+6: left-right null uncle",
            Self::Case56RightNull => "Case 5+6: right-left null uncle",
            Self::Case56LeftBlack => "Case 5+6: left-right black uncle",
            Self::Case56RightBlack => "Case 5+6: right-left black uncle",
            Self::Case6GgpLeftLeft => "Case 6: GGP non-null, LL GP-left",
            Self::Case6GgpLeftRight => "Case 6: GGP non-null, LL GP-right",
            Self::Case6GgpRightRight => "Case 6: GGP non-null, RR GP-right",
            Self::Case6GgpRightLeft => "Case 6: GGP non-null, RR GP-left",
            Self::Case26Left => "Case 2+6: non-null new_child dir_l",
            Self::Case26Right => "Case 2+6: non-null new_child dir_r",
            Self::Case256Left => "Case 2+5+6: non-null new_child dir_l",
            Self::Case256Right => "Case 2+5+6: non-null new_child dir_r",
        }
    }

    fn fixed_costs(&self) -> u64 {
        match self {
            Self::AllocHappyPath | Self::AllocMaxDataLen => {
                fixed_costs::CPI_BASE + fixed_costs::SYSTEM_PROGRAM
            }
            _ => 0,
        }
    }

    fn run(&self, lang: ProgramLanguage) -> CaseResult {
        match self {
            // ----- Input validation -----
            Self::InputDataShort => {
                let (setup, mut instruction, accounts) = insert_skip_alloc_setup(lang);
                // Correct discriminator but wrong length (1 byte instead of 5).
                instruction.data = vec![TreeInstruction::Insert as u8];
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::INSTRUCTION_DATA_LEN,
                )
            }
            Self::InputDataLong => {
                let (setup, mut instruction, accounts) = insert_skip_alloc_setup(lang);
                // Correct discriminator but wrong length (6 bytes instead of 5).
                instruction.data = vec![TreeInstruction::Insert as u8, 0, 0, 0, 0, 0];
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::INSTRUCTION_DATA_LEN,
                )
            }
            Self::InputNAccounts => {
                let (setup, mut instruction, mut accounts) = insert_skip_alloc_setup(lang);
                // Remove tree account (1 account instead of 2).
                instruction.accounts.pop();
                accounts.pop();
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::N_ACCOUNTS,
                )
            }
            Self::InputUserDataLen => {
                let (setup, instruction, mut accounts) = insert_skip_alloc_setup(lang);
                accounts[AccountIndex::User as usize].1.data = vec![1u8; 1];
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::USER_DATA_LEN,
                )
            }
            Self::InputTreeDuplicate => {
                let (setup, mut instruction, mut accounts) = insert_skip_alloc_setup(lang);
                instruction.accounts[AccountIndex::Tree as usize] =
                    instruction.accounts[AccountIndex::User as usize].clone();
                accounts[AccountIndex::Tree as usize] =
                    accounts[AccountIndex::User as usize].clone();
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::TREE_DUPLICATE,
                )
            }

            // ----- Allocation checks -----
            Self::AllocNAccounts => {
                // Use insert_setup (top=null triggers allocation) but strip CPI accounts.
                let (setup, mut instruction, mut accounts) = insert_setup(lang);
                instruction.accounts.truncate(2);
                accounts.truncate(2);
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::N_ACCOUNTS_INSERT_ALLOCATION,
                )
            }
            Self::AllocSysprogDuplicate => {
                let (setup, mut instruction, mut accounts) = insert_setup(lang);
                instruction.accounts[AccountIndex::SystemProgram as usize] =
                    instruction.accounts[AccountIndex::User as usize].clone();
                accounts[AccountIndex::SystemProgram as usize] =
                    accounts[AccountIndex::User as usize].clone();
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::SYSTEM_PROGRAM_DUPLICATE,
                )
            }
            Self::AllocSysprogDataLen => {
                let (setup, instruction, mut accounts) = insert_setup(lang);
                accounts[AccountIndex::SystemProgram as usize].1.data = vec![];
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::SYSTEM_PROGRAM_DATA_LEN,
                )
            }
            Self::AllocRentDuplicate => {
                let (setup, mut instruction, mut accounts) = insert_setup(lang);
                instruction.accounts[AccountIndex::RentSysvar as usize] =
                    instruction.accounts[AccountIndex::User as usize].clone();
                accounts[AccountIndex::RentSysvar as usize] =
                    accounts[AccountIndex::User as usize].clone();
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::RENT_DUPLICATE,
                )
            }
            Self::AllocRentAddrChunk0 => insert_alloc_address_mismatch(
                lang,
                AccountIndex::RentSysvar as usize,
                0,
                error_codes::error::RENT_ADDRESS,
            ),
            Self::AllocRentAddrChunk1 => insert_alloc_address_mismatch(
                lang,
                AccountIndex::RentSysvar as usize,
                1,
                error_codes::error::RENT_ADDRESS,
            ),
            Self::AllocRentAddrChunk2 => insert_alloc_address_mismatch(
                lang,
                AccountIndex::RentSysvar as usize,
                2,
                error_codes::error::RENT_ADDRESS,
            ),
            Self::AllocRentAddrChunk3 => insert_alloc_address_mismatch(
                lang,
                AccountIndex::RentSysvar as usize,
                3,
                error_codes::error::RENT_ADDRESS,
            ),

            // ----- Allocation happy path -----
            Self::AllocHappyPath => {
                let (setup, instruction, accounts) = insert_setup(lang);
                let result = setup.mollusk.process_instruction(&instruction, &accounts);
                match &result.program_result {
                    MolluskResult::Success => {
                        let tree = &result.resulting_accounts[AccountIndex::Tree as usize].1;
                        let rent_data = &accounts[AccountIndex::RentSysvar as usize].1.data;
                        let rent = Rent::from_bytes(rent_data).unwrap();
                        let expected_data_len = cpi::TREE_DATA_LEN + size_of::<TreeNode>();
                        let expected_lamports =
                            rent.try_minimum_balance(expected_data_len).unwrap();
                        let mut errors = Vec::new();
                        if tree.data.len() != expected_data_len {
                            errors.push(format!(
                                "data len: expected {}, got {}",
                                expected_data_len,
                                tree.data.len()
                            ));
                        }
                        if tree.lamports != expected_lamports {
                            errors.push(format!(
                                "lamports: expected {}, got {}",
                                expected_lamports, tree.lamports
                            ));
                        }
                        // Verify header pointers.
                        let header = unsafe { &*(tree.data.as_ptr() as *const TreeHeader) };
                        let node_addr = MM_INPUT_START
                            + input_buffer::TREE_DATA_OFF as u64
                            + size_of::<TreeHeader>() as u64;
                        let expected_next = node_addr + size_of::<TreeNode>() as u64;
                        if header.next as u64 != expected_next {
                            errors.push(format!(
                                "next: expected {:#x}, got {:#x}",
                                expected_next, header.next as u64
                            ));
                        }
                        if header.root as u64 != node_addr {
                            errors.push(format!(
                                "root: expected {:#x}, got {:#x}",
                                node_addr, header.root as u64
                            ));
                        }
                        // Verify node key and value.
                        let node = unsafe {
                            &*(tree.data.as_ptr().add(size_of::<TreeHeader>()) as *const TreeNode)
                        };
                        let key = node.key;
                        let value = node.value;
                        if key != 42 {
                            errors.push(format!("key: expected 42, got {}", key));
                        }
                        if value != 1 {
                            errors.push(format!("value: expected 1, got {}", value));
                        }
                        let config = Config {
                            panic: false,
                            verbose: false,
                        };
                        if !result.run_checks(&[Check::all_rent_exempt()], &config, &setup.mollusk)
                        {
                            errors.push("not all accounts are rent exempt".to_string());
                        }
                        CaseResult {
                            cu: result.compute_units_consumed,
                            error: if errors.is_empty() {
                                None
                            } else {
                                Some(errors.join("; "))
                            },
                        }
                    }
                    other => CaseResult {
                        cu: result.compute_units_consumed,
                        error: Some(format!("expected Success, got {:?}", other)),
                    },
                }
            }

            // ----- Allocation: max data length -----
            Self::AllocMaxDataLen => {
                let (setup, instruction, accounts) = insert_max_data_setup(lang);
                check_result(
                    &setup,
                    &instruction,
                    &accounts,
                    ProgramError::InvalidRealloc,
                )
            }

            // ----- Search: duplicate key errors -----

            // Root with key 10, insert 10.
            Self::SearchDupRoot => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[node(10, B, None, None, None)],
                };
                run_dup_error(lang, &desc, 10)
            }
            // Root 10, left child 5, insert 5.
            Self::SearchDupLeft => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), None),
                        node(5, R, Some(0), None, None),
                    ],
                };
                run_dup_error(lang, &desc, 5)
            }
            // Root 10, right child 15, insert 15.
            Self::SearchDupRight => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, None, Some(1)),
                        node(15, R, Some(0), None, None),
                    ],
                };
                run_dup_error(lang, &desc, 15)
            }

            // ----- Insert to empty tree -----
            Self::EmptyTree => {
                let desc = TreeSpec {
                    root: None,
                    top: None,
                    nodes: &[],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[node(42, R, None, None, None).val(1)],
                };
                run_success(lang, &desc, 42, &exp)
            }

            // ----- Case 1: parent is black -----

            // B(10) root, insert 5 → left child.
            Self::Case1Left => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[node(10, B, None, None, None)],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), None),
                        node(5, R, Some(0), None, None).val(1),
                    ],
                };
                run_success(lang, &desc, 5, &exp)
            }
            // B(10) root, insert 15 → right child.
            Self::Case1Right => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[node(10, B, None, None, None)],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, None, Some(1)),
                        node(15, R, Some(0), None, None).val(1),
                    ],
                };
                run_success(lang, &desc, 15, &exp)
            }

            // ----- Case 4: parent is root and red -----

            // R(10) root, insert 5 → left child, parent recolored B.
            Self::Case4Left => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[node(10, R, None, None, None)],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), None),
                        node(5, R, Some(0), None, None).val(1),
                    ],
                };
                run_success(lang, &desc, 5, &exp)
            }
            // R(10) root, insert 15 → right child, parent recolored B.
            Self::Case4Right => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[node(10, R, None, None, None)],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, None, Some(1)),
                        node(15, R, Some(0), None, None).val(1),
                    ],
                };
                run_success(lang, &desc, 15, &exp)
            }

            // ----- Case 2+3: red uncle, propagate to root -----
            // Before: B(10) root, R(5) left, R(15) right.
            // After recolor: R(10), B(5), B(15), inserted node red.
            Self::Case23LeftLeft => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, R, Some(0), None, None),
                        node(15, R, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, R, None, Some(1), Some(2)),
                        node(5, B, Some(0), Some(3), None),
                        node(15, B, Some(0), None, None),
                        node(1, R, Some(1), None, None),
                    ],
                };
                run_success(lang, &desc, 1, &exp)
            }
            Self::Case23LeftRight => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, R, Some(0), None, None),
                        node(15, R, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, R, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, Some(3)),
                        node(15, B, Some(0), None, None),
                        node(7, R, Some(1), None, None).val(1),
                    ],
                };
                run_success(lang, &desc, 7, &exp)
            }
            Self::Case23RightLeft => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, R, Some(0), None, None),
                        node(15, R, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, R, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(15, B, Some(0), Some(3), None),
                        node(12, R, Some(2), None, None).val(1),
                    ],
                };
                run_success(lang, &desc, 12, &exp)
            }
            Self::Case23RightRight => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, R, Some(0), None, None),
                        node(15, R, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, R, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(15, B, Some(0), None, Some(3)),
                        node(20, R, Some(2), None, None).val(1),
                    ],
                };
                run_success(lang, &desc, 20, &exp)
            }

            // ----- Case 2+1: red uncle, propagate to black ancestor -----

            // Before: B(20) root, B(10) left of root, R(5) left of 10, R(15) right of 10.
            // After: B(20), R(10), B(5), B(15), R(1) inserted.
            Self::Case21Left => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(20, B, None, Some(1), None),
                        node(10, B, Some(0), Some(2), Some(3)),
                        node(5, R, Some(1), None, None),
                        node(15, R, Some(1), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(20, B, None, Some(1), None),
                        node(10, R, Some(0), Some(2), Some(3)),
                        node(5, B, Some(1), Some(4), None),
                        node(15, B, Some(1), None, None),
                        node(1, R, Some(2), None, None),
                    ],
                };
                run_success(lang, &desc, 1, &exp)
            }
            // Mirror: B(2) root, B(10) right of root, R(5) left of 10, R(15) right of 10.
            Self::Case21Right => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(2, B, None, None, Some(1)),
                        node(10, B, Some(0), Some(2), Some(3)),
                        node(5, R, Some(1), None, None),
                        node(15, R, Some(1), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(2, B, None, None, Some(1)),
                        node(10, R, Some(0), Some(2), Some(3)),
                        node(5, B, Some(1), None, None),
                        node(15, B, Some(1), None, Some(4)),
                        node(20, R, Some(3), None, None).val(1),
                    ],
                };
                run_success(lang, &desc, 20, &exp)
            }

            // ----- Case 6: single rotation (outer child) -----

            // Left-left, null uncle: B(10) root, R(5) left, insert 1.
            // After: B(5) new root, R(1) left, R(10) right.
            Self::Case6LeftNull => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), None),
                        node(5, R, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(1),
                    top: None,
                    nodes: &[
                        node(10, R, Some(1), None, None),
                        node(5, B, None, Some(2), Some(0)),
                        node(1, R, Some(1), None, None),
                    ],
                };
                run_success(lang, &desc, 1, &exp)
            }
            // Right-right, null uncle: B(10) root, R(15) right, insert 20.
            // After: B(15) new root, R(10) left, R(20) right.
            Self::Case6RightNull => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, None, Some(1)),
                        node(15, R, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(1),
                    top: None,
                    nodes: &[
                        node(10, R, Some(1), None, None),
                        node(15, B, None, Some(0), Some(2)),
                        node(20, R, Some(1), None, None).val(1),
                    ],
                };
                run_success(lang, &desc, 20, &exp)
            }
            // Left-left, black uncle: B(10) root, R(5) left, B(15) right, insert 1.
            // After: B(5) new root, R(1) left, R(10) right with B(15) as 10's right.
            Self::Case6LeftBlack => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, R, Some(0), None, None),
                        node(15, B, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(1),
                    top: None,
                    nodes: &[
                        node(10, R, Some(1), None, Some(2)),
                        node(5, B, None, Some(3), Some(0)),
                        node(15, B, Some(0), None, None),
                        node(1, R, Some(1), None, None),
                    ],
                };
                run_success(lang, &desc, 1, &exp)
            }
            // Right-right, black uncle: B(10) root, B(5) left, R(15) right, insert 20.
            // After: B(15) new root, R(10) left with B(5) as 10's left, R(20) right.
            Self::Case6RightBlack => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(15, R, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(2),
                    top: None,
                    nodes: &[
                        node(10, R, Some(2), Some(1), None),
                        node(5, B, Some(0), None, None),
                        node(15, B, None, Some(0), Some(3)),
                        node(20, R, Some(2), None, None).val(1),
                    ],
                };
                run_success(lang, &desc, 20, &exp)
            }

            // ----- Case 5+6: double rotation (inner child) -----

            // Left-right, null uncle: B(10) root, R(5) left, insert 7.
            // After: B(7) new root, R(5) left, R(10) right.
            Self::Case56LeftNull => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), None),
                        node(5, R, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(2),
                    top: None,
                    nodes: &[
                        node(10, R, Some(2), None, None),
                        node(5, R, Some(2), None, None),
                        node(7, B, None, Some(1), Some(0)).val(1),
                    ],
                };
                run_success(lang, &desc, 7, &exp)
            }
            // Right-left, null uncle: B(10) root, R(15) right, insert 12.
            // After: B(12) new root, R(10) left, R(15) right.
            Self::Case56RightNull => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, None, Some(1)),
                        node(15, R, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(2),
                    top: None,
                    nodes: &[
                        node(10, R, Some(2), None, None),
                        node(15, R, Some(2), None, None),
                        node(12, B, None, Some(0), Some(1)).val(1),
                    ],
                };
                run_success(lang, &desc, 12, &exp)
            }
            // Left-right, black uncle: B(10) root, R(5) left, B(15) right, insert 7.
            // After: B(7) new root, R(5) left, R(10) right with B(15) as 10's right.
            Self::Case56LeftBlack => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, R, Some(0), None, None),
                        node(15, B, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(3),
                    top: None,
                    nodes: &[
                        node(10, R, Some(3), None, Some(2)),
                        node(5, R, Some(3), None, None),
                        node(15, B, Some(0), None, None),
                        node(7, B, None, Some(1), Some(0)).val(1),
                    ],
                };
                run_success(lang, &desc, 7, &exp)
            }
            // Right-left, black uncle: B(10) root, B(5) left, R(15) right, insert 12.
            // After: B(12) new root, R(10) left with B(5) as 10's left, R(15) right.
            Self::Case56RightBlack => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(15, R, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(3),
                    top: None,
                    nodes: &[
                        node(10, R, Some(3), Some(1), None),
                        node(5, B, Some(0), None, None),
                        node(15, R, Some(3), None, None),
                        node(12, B, None, Some(0), Some(2)).val(1),
                    ],
                };
                run_success(lang, &desc, 12, &exp)
            }

            // ----- Case 6: non-null great-grandparent -----

            // LL, GP is left child of GGP. Insert 1.
            // B(20) root, B(10) left with R(5) left, B(25) right.
            // Case 6 dir_l rotates GP=B(10) right under GGP=B(20).
            // GGP.child[L] = parent (GP was left child).
            Self::Case6GgpLeftLeft => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(20, B, None, Some(1), Some(3)),
                        node(10, B, Some(0), Some(2), None),
                        node(5, R, Some(1), None, None),
                        node(25, B, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(20, B, None, Some(2), Some(3)),
                        node(10, R, Some(2), None, None),
                        node(5, B, Some(0), Some(4), Some(1)),
                        node(25, B, Some(0), None, None),
                        node(1, R, Some(2), None, None),
                    ],
                };
                run_success(lang, &desc, 1, &exp)
            }
            // LL, GP is right child of GGP. Insert 10.
            // B(5) root, B(3) left, B(20) right with R(15) left.
            // Case 6 dir_l rotates GP=B(20) right under GGP=B(5).
            // GGP.child[R] = parent (GP was right child).
            Self::Case6GgpLeftRight => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(5, B, None, Some(1), Some(2)),
                        node(3, B, Some(0), None, None),
                        node(20, B, Some(0), Some(3), None),
                        node(15, R, Some(2), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(5, B, None, Some(1), Some(3)),
                        node(3, B, Some(0), None, None),
                        node(20, R, Some(3), None, None),
                        node(15, B, Some(0), Some(4), Some(2)),
                        node(10, R, Some(3), None, None).val(1),
                    ],
                };
                run_success(lang, &desc, 10, &exp)
            }
            // RR, GP is right child of GGP. Insert 25.
            // B(5) root, B(3) left, B(15) right with R(20) right.
            // Case 6 dir_r rotates GP=B(15) left under GGP=B(5).
            // GGP.child[R] = parent (GP was right child).
            Self::Case6GgpRightRight => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(5, B, None, Some(1), Some(2)),
                        node(3, B, Some(0), None, None),
                        node(15, B, Some(0), None, Some(3)),
                        node(20, R, Some(2), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(5, B, None, Some(1), Some(3)),
                        node(3, B, Some(0), None, None),
                        node(15, R, Some(3), None, None),
                        node(20, B, Some(0), Some(2), Some(4)),
                        node(25, R, Some(3), None, None).val(1),
                    ],
                };
                run_success(lang, &desc, 25, &exp)
            }
            // RR, GP is left child of GGP. Insert 17.
            // B(20) root, B(10) left with R(15) right, B(25) right.
            // Case 6 dir_r rotates GP=B(10) left under GGP=B(20).
            // GGP.child[L] = parent (GP was left child).
            Self::Case6GgpRightLeft => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(20, B, None, Some(1), Some(3)),
                        node(10, B, Some(0), None, Some(2)),
                        node(15, R, Some(1), None, None),
                        node(25, B, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(20, B, None, Some(2), Some(3)),
                        node(10, R, Some(2), None, None),
                        node(15, B, Some(0), Some(1), Some(4)),
                        node(25, B, Some(0), None, None),
                        node(17, R, Some(2), None, None).val(1),
                    ],
                };
                run_success(lang, &desc, 17, &exp)
            }

            // ----- Case 2+6: non-null new_child in rotation -----

            // Dir_l: insert 1 into 7-node tree.
            // Case 2 recolors at bottom, then case 6 dir_l rotates with
            // new_child = B(15) non-null.
            Self::Case26Left => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(20, B, None, Some(1), Some(3)),
                        node(10, R, Some(0), Some(2), Some(6)),
                        node(5, B, Some(1), Some(4), Some(5)),
                        node(25, B, Some(0), None, None),
                        node(3, R, Some(2), None, None),
                        node(7, R, Some(2), None, None),
                        node(15, B, Some(1), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(1),
                    top: None,
                    nodes: &[
                        node(20, R, Some(1), Some(6), Some(3)),
                        node(10, B, None, Some(2), Some(0)),
                        node(5, R, Some(1), Some(4), Some(5)),
                        node(25, B, Some(0), None, None),
                        node(3, B, Some(2), Some(7), None),
                        node(7, B, Some(2), None, None),
                        node(15, B, Some(0), None, None),
                        node(1, R, Some(4), None, None),
                    ],
                };
                run_success(lang, &desc, 1, &exp)
            }
            // Dir_r: insert 30 into 7-node tree.
            // Case 2 recolors at bottom, then case 6 dir_r rotates with
            // new_child = B(10) non-null.
            Self::Case26Right => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(5, B, None, Some(1), Some(2)),
                        node(3, B, Some(0), None, None),
                        node(15, R, Some(0), Some(3), Some(4)),
                        node(10, B, Some(2), None, None),
                        node(20, B, Some(2), Some(5), Some(6)),
                        node(17, R, Some(4), None, None),
                        node(25, R, Some(4), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(2),
                    top: None,
                    nodes: &[
                        node(5, R, Some(2), Some(1), Some(3)),
                        node(3, B, Some(0), None, None),
                        node(15, B, None, Some(0), Some(4)),
                        node(10, B, Some(0), None, None),
                        node(20, R, Some(2), Some(5), Some(6)),
                        node(17, B, Some(4), None, None),
                        node(25, B, Some(4), None, Some(7)),
                        node(30, R, Some(6), None, None).val(1),
                    ],
                };
                run_success(lang, &desc, 30, &exp)
            }

            // ----- Case 2+5+6: non-null new_child in rotations -----

            // Dir_l: insert 11 into 7-node tree.
            // Case 2 recolors at bottom, then case 5 dir_l rotates with
            // new_child = B(12) non-null, then case 6 dir_l rotates with
            // new_child = B(17) non-null.
            Self::Case256Left => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(20, B, None, Some(1), Some(4)),
                        node(10, R, Some(0), Some(2), Some(3)),
                        node(5, B, Some(1), None, None),
                        node(15, B, Some(1), Some(5), Some(6)),
                        node(25, B, Some(0), None, None),
                        node(12, R, Some(3), None, None),
                        node(17, R, Some(3), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(3),
                    top: None,
                    nodes: &[
                        node(20, R, Some(3), Some(6), Some(4)),
                        node(10, R, Some(3), Some(2), Some(5)),
                        node(5, B, Some(1), None, None),
                        node(15, B, None, Some(1), Some(0)),
                        node(25, B, Some(0), None, None),
                        node(12, B, Some(1), Some(7), None),
                        node(17, B, Some(0), None, None),
                        node(11, R, Some(5), None, None).val(1),
                    ],
                };
                run_success(lang, &desc, 11, &exp)
            }
            // Dir_r: insert 18 into 7-node tree.
            // Case 2 recolors at bottom, then case 5 dir_r rotates with
            // new_child = B(17) non-null, then case 6 dir_r rotates with
            // new_child = B(12) non-null.
            Self::Case256Right => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(20, R, Some(0), Some(3), Some(4)),
                        node(15, B, Some(2), Some(5), Some(6)),
                        node(25, B, Some(2), None, None),
                        node(12, R, Some(3), None, None),
                        node(17, R, Some(3), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(3),
                    top: None,
                    nodes: &[
                        node(10, R, Some(3), Some(1), Some(5)),
                        node(5, B, Some(0), None, None),
                        node(20, R, Some(3), Some(6), Some(4)),
                        node(15, B, None, Some(0), Some(2)),
                        node(25, B, Some(2), None, None),
                        node(12, B, Some(0), None, None),
                        node(17, B, Some(2), None, Some(7)),
                        node(18, R, Some(6), None, None).val(1),
                    ],
                };
                run_success(lang, &desc, 18, &exp)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Multi-insert integration tests
// ---------------------------------------------------------------------------

/// Build an empty tree account with `n` pre-allocated free slots.
fn build_empty_tree(n: usize, program_id: &Pubkey) -> (Pubkey, Account) {
    let data_len = size_of::<TreeHeader>() + n * size_of::<TreeNode>();
    let mut data = vec![0u8; data_len];

    let header = data.as_mut_ptr() as *mut TreeHeader;
    unsafe {
        (*header).root = core::ptr::null_mut();
        (*header).top = if n > 0 {
            node_vaddr(0) as *mut StackNode
        } else {
            core::ptr::null_mut()
        };
        (*header).next = core::ptr::null_mut();
    }

    // Link free slots into a singly-linked list.
    for i in 0..n {
        let offset = size_of::<TreeHeader>() + i * size_of::<TreeNode>();
        let slot = unsafe { data.as_mut_ptr().add(offset) as *mut StackNode };
        unsafe {
            (*slot).next = if i + 1 < n {
                node_vaddr(i + 1) as *mut StackNode
            } else {
                core::ptr::null_mut()
            };
        }
    }

    let pubkey = Pubkey::new_unique();
    let mut account = Account::new(0, data_len, program_id);
    account.data = data;
    (pubkey, account)
}

struct MultiInsertStep<'a> {
    key: u16,
    expected: TreeSpec<'a>,
}

fn run_multi_insert(lang: ProgramLanguage, steps: &[MultiInsertStep]) {
    let setup = setup_test(lang);
    let (system_program_pubkey, _) = program::keyed_account_for_system_program();

    let user_pubkey = Pubkey::new_unique();
    let (tree_pubkey, mut tree_account) =
        build_empty_tree(steps.len(), &setup.program_id);

    for (i, step) in steps.iter().enumerate() {
        let insn_data = InsertInstruction {
            header: InstructionHeader {
                discriminator: TreeInstruction::Insert as u8,
            },
            key: step.key,
            value: 1,
        };

        let instruction = Instruction::new_with_bytes(
            setup.program_id,
            unsafe { as_bytes(&insn_data) },
            vec![
                AccountMeta::new(user_pubkey, true),
                AccountMeta::new(tree_pubkey, false),
            ],
        );

        let accounts = vec![
            (
                user_pubkey,
                Account::new(USER_LAMPORTS, 0, &system_program_pubkey),
            ),
            (tree_pubkey, tree_account.clone()),
        ];

        let result = setup.mollusk.process_instruction(&instruction, &accounts);
        match &result.program_result {
            MolluskResult::Success => {
                tree_account =
                    result.resulting_accounts[AccountIndex::Tree as usize].1.clone();
                if let Err(e) = assert_tree_account(&tree_account.data, &step.expected) {
                    panic!(
                        "step {} (key={}): {}",
                        i, step.key, e
                    );
                }
            }
            other => panic!(
                "step {} (key={}): expected Success, got {:?}",
                i, step.key, other
            ),
        }
    }
}

pub(super) fn test_multi_insert(lang: ProgramLanguage) {
    // 3-node balanced: 10, 5, 15.
    run_multi_insert(lang, &[
        MultiInsertStep {
            key: 10,
            expected: TreeSpec {
                root: Some(0),
                top: Some(1),
                nodes: &[node(10, R, None, None, None).val(1)],
            },
        },
        MultiInsertStep {
            key: 5,
            expected: TreeSpec {
                root: Some(0),
                top: Some(2),
                nodes: &[
                    node(10, B, None, Some(1), None).val(1),
                    node(5, R, Some(0), None, None).val(1),
                ],
            },
        },
        MultiInsertStep {
            key: 15,
            expected: TreeSpec {
                root: Some(0),
                top: None,
                nodes: &[
                    node(10, B, None, Some(1), Some(2)).val(1),
                    node(5, R, Some(0), None, None).val(1),
                    node(15, R, Some(0), None, None).val(1),
                ],
            },
        },
    ]);

    // Left-skew: 10, 5, 1 → right rotation.
    run_multi_insert(lang, &[
        MultiInsertStep {
            key: 10,
            expected: TreeSpec {
                root: Some(0),
                top: Some(1),
                nodes: &[node(10, R, None, None, None).val(1)],
            },
        },
        MultiInsertStep {
            key: 5,
            expected: TreeSpec {
                root: Some(0),
                top: Some(2),
                nodes: &[
                    node(10, B, None, Some(1), None).val(1),
                    node(5, R, Some(0), None, None).val(1),
                ],
            },
        },
        MultiInsertStep {
            key: 1,
            expected: TreeSpec {
                root: Some(1),
                top: None,
                nodes: &[
                    node(10, R, Some(1), None, None).val(1),
                    node(5, B, None, Some(2), Some(0)).val(1),
                    node(1, R, Some(1), None, None).val(1),
                ],
            },
        },
    ]);

    // Right-skew: 10, 15, 20 → left rotation.
    run_multi_insert(lang, &[
        MultiInsertStep {
            key: 10,
            expected: TreeSpec {
                root: Some(0),
                top: Some(1),
                nodes: &[node(10, R, None, None, None).val(1)],
            },
        },
        MultiInsertStep {
            key: 15,
            expected: TreeSpec {
                root: Some(0),
                top: Some(2),
                nodes: &[
                    node(10, B, None, None, Some(1)).val(1),
                    node(15, R, Some(0), None, None).val(1),
                ],
            },
        },
        MultiInsertStep {
            key: 20,
            expected: TreeSpec {
                root: Some(1),
                top: None,
                nodes: &[
                    node(10, R, Some(1), None, None).val(1),
                    node(15, B, None, Some(0), Some(2)).val(1),
                    node(20, R, Some(1), None, None).val(1),
                ],
            },
        },
    ]);

    // Zigzag: 10, 5, 7 → double rotation.
    run_multi_insert(lang, &[
        MultiInsertStep {
            key: 10,
            expected: TreeSpec {
                root: Some(0),
                top: Some(1),
                nodes: &[node(10, R, None, None, None).val(1)],
            },
        },
        MultiInsertStep {
            key: 5,
            expected: TreeSpec {
                root: Some(0),
                top: Some(2),
                nodes: &[
                    node(10, B, None, Some(1), None).val(1),
                    node(5, R, Some(0), None, None).val(1),
                ],
            },
        },
        MultiInsertStep {
            key: 7,
            expected: TreeSpec {
                root: Some(2),
                top: None,
                nodes: &[
                    node(10, R, Some(2), None, None).val(1),
                    node(5, R, Some(2), None, None).val(1),
                    node(7, B, None, Some(1), Some(0)).val(1),
                ],
            },
        },
    ]);

    // 7-node full: 10, 5, 15, 3, 7, 12, 20.
    run_multi_insert(lang, &[
        MultiInsertStep {
            key: 10,
            expected: TreeSpec {
                root: Some(0),
                top: Some(1),
                nodes: &[node(10, R, None, None, None).val(1)],
            },
        },
        MultiInsertStep {
            key: 5,
            expected: TreeSpec {
                root: Some(0),
                top: Some(2),
                nodes: &[
                    node(10, B, None, Some(1), None).val(1),
                    node(5, R, Some(0), None, None).val(1),
                ],
            },
        },
        MultiInsertStep {
            key: 15,
            expected: TreeSpec {
                root: Some(0),
                top: Some(3),
                nodes: &[
                    node(10, B, None, Some(1), Some(2)).val(1),
                    node(5, R, Some(0), None, None).val(1),
                    node(15, R, Some(0), None, None).val(1),
                ],
            },
        },
        MultiInsertStep {
            key: 3,
            expected: TreeSpec {
                root: Some(0),
                top: Some(4),
                nodes: &[
                    node(10, R, None, Some(1), Some(2)).val(1),
                    node(5, B, Some(0), Some(3), None).val(1),
                    node(15, B, Some(0), None, None).val(1),
                    node(3, R, Some(1), None, None).val(1),
                ],
            },
        },
        MultiInsertStep {
            key: 7,
            expected: TreeSpec {
                root: Some(0),
                top: Some(5),
                nodes: &[
                    node(10, R, None, Some(1), Some(2)).val(1),
                    node(5, B, Some(0), Some(3), Some(4)).val(1),
                    node(15, B, Some(0), None, None).val(1),
                    node(3, R, Some(1), None, None).val(1),
                    node(7, R, Some(1), None, None).val(1),
                ],
            },
        },
        MultiInsertStep {
            key: 12,
            expected: TreeSpec {
                root: Some(0),
                top: Some(6),
                nodes: &[
                    node(10, R, None, Some(1), Some(2)).val(1),
                    node(5, B, Some(0), Some(3), Some(4)).val(1),
                    node(15, B, Some(0), Some(5), None).val(1),
                    node(3, R, Some(1), None, None).val(1),
                    node(7, R, Some(1), None, None).val(1),
                    node(12, R, Some(2), None, None).val(1),
                ],
            },
        },
        MultiInsertStep {
            key: 20,
            expected: TreeSpec {
                root: Some(0),
                top: None,
                nodes: &[
                    node(10, R, None, Some(1), Some(2)).val(1),
                    node(5, B, Some(0), Some(3), Some(4)).val(1),
                    node(15, B, Some(0), Some(5), Some(6)).val(1),
                    node(3, R, Some(1), None, None).val(1),
                    node(7, R, Some(1), None, None).val(1),
                    node(12, R, Some(2), None, None).val(1),
                    node(20, R, Some(2), None, None).val(1),
                ],
            },
        },
    ]);
}
