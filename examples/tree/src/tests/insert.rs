use super::*;
use mollusk_svm::program;
use mollusk_svm::result::{Check, Config};
use pinocchio::sysvars::rent::Rent;
use solana_sdk::instruction::AccountMeta;
use tree_interface::{
    cpi, input_buffer, tree, InstructionHeader, Instruction as TreeInstruction, InsertInstruction,
    StackNode, TreeHeader, TreeNode,
};

const TEST_KEY: u16 = 42;
const TEST_VALUE: u16 = 1;

fn insert_instruction_data() -> InsertInstruction {
    InsertInstruction {
        header: InstructionHeader {
            discriminator: TreeInstruction::Insert as u8,
        },
        key: TEST_KEY,
        value: TEST_VALUE,
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

#[derive(Clone, Copy)]
pub(super) enum InsertCase {
    InstructionDataLenShort,
    InstructionDataLenLong,
    InsertSkipAlloc,
    InsertAllocHappyPath,
}

impl InsertCase {
    pub(super) const CASES: &'static [Self] = &[
        Self::InstructionDataLenShort,
        Self::InstructionDataLenLong,
        Self::InsertSkipAlloc,
        Self::InsertAllocHappyPath,
    ];
}

impl TestCase for InsertCase {
    fn name(&self) -> &'static str {
        match self {
            Self::InstructionDataLenShort => "Instruction data too short",
            Self::InstructionDataLenLong => "Instruction data too long",
            Self::InsertSkipAlloc => "Insert skip alloc",
            Self::InsertAllocHappyPath => "Insert alloc happy path",
        }
    }

    fn fixed_costs(&self) -> u64 {
        match self {
            Self::InsertAllocHappyPath => fixed_costs::CPI_BASE + fixed_costs::SYSTEM_PROGRAM,
            _ => 0,
        }
    }

    fn run(&self, lang: ProgramLanguage) -> CaseResult {
        match self {
            Self::InstructionDataLenShort => {
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
            Self::InstructionDataLenLong => {
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
            Self::InsertSkipAlloc => {
                let (setup, instruction, accounts) = insert_skip_alloc_setup(lang);
                let result = setup.mollusk.process_instruction(&instruction, &accounts);
                match &result.program_result {
                    MolluskResult::Success => CaseResult {
                        cu: result.compute_units_consumed,
                        error: None,
                    },
                    other => CaseResult {
                        cu: result.compute_units_consumed,
                        error: Some(format!("expected Success, got {:?}", other)),
                    },
                }
            }
            Self::InsertAllocHappyPath => {
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
                        if key != TEST_KEY {
                            errors.push(format!("key: expected {}, got {}", TEST_KEY, key));
                        }
                        if value != TEST_VALUE {
                            errors.push(format!("value: expected {}, got {}", TEST_VALUE, value));
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
        }
    }
}
