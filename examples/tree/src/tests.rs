use interface::error_codes;
use mollusk_svm::program;
use mollusk_svm::result::Check;
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use std::vec;
use test_utils::{setup_test, ProgramLanguage};

enum AccountIndex {
    User = 0,
    Tree = 1,
    SystemProgram = 2,
}

// ============================================================================
// Entrypoint branching
// ============================================================================

#[derive(Clone, Copy)]
enum EntrypointCase {
    NoAccounts,
    OneAccount,
    FourAccounts,
}

impl EntrypointCase {
    const CASES: &'static [Self] = &[
        Self::NoAccounts,
        Self::OneAccount,
        Self::FourAccounts,
    ];

    const fn name(&self) -> &'static str {
        match self {
            Self::NoAccounts => "No accounts",
            Self::OneAccount => "One account",
            Self::FourAccounts => "Four accounts",
        }
    }

    const fn n_accounts(&self) -> usize {
        match self {
            Self::NoAccounts => 0,
            Self::OneAccount => 1,
            Self::FourAccounts => 4,
        }
    }

    fn run(&self, lang: ProgramLanguage) -> u64 {
        run_entrypoint(lang, self.n_accounts())
    }
}

fn run_entrypoint(lang: ProgramLanguage, n_accounts: usize) -> u64 {
    let setup = setup_test(lang);

    let account_metas: Vec<AccountMeta> = (0..n_accounts)
        .map(|_| AccountMeta::new(Pubkey::new_unique(), false))
        .collect();
    let accounts: Vec<(Pubkey, Account)> = account_metas
        .iter()
        .map(|meta| (meta.pubkey, Account::default()))
        .collect();

    let instruction = Instruction::new_with_bytes(setup.program_id, &[], account_metas);

    setup
        .mollusk
        .process_and_validate_instruction(
            &instruction,
            &accounts,
            &[Check::err(ProgramError::Custom(
                error_codes::error::N_ACCOUNTS.into(),
            ))],
        )
        .compute_units_consumed
}

#[test]
fn test_asm_no_accounts() {
    EntrypointCase::NoAccounts.run(ProgramLanguage::Assembly);
}

#[test]
fn test_asm_one_account() {
    EntrypointCase::OneAccount.run(ProgramLanguage::Assembly);
}

#[test]
fn test_asm_four_accounts() {
    EntrypointCase::FourAccounts.run(ProgramLanguage::Assembly);
}

#[test]
fn test_rs_no_accounts() {
    EntrypointCase::NoAccounts.run(ProgramLanguage::Rust);
}

#[test]
fn test_rs_one_account() {
    EntrypointCase::OneAccount.run(ProgramLanguage::Rust);
}

#[test]
fn test_rs_four_accounts() {
    EntrypointCase::FourAccounts.run(ProgramLanguage::Rust);
}

#[test]
fn test_entrypoint_branching() {
    println!("| Case | ASM (CUs) | Rust (CUs) | Overhead | Overhead % |");
    println!("|------|-----------|------------|----------|------------|");

    for case in EntrypointCase::CASES {
        let asm_cu = case.run(ProgramLanguage::Assembly);
        let rs_cu = case.run(ProgramLanguage::Rust);
        let overhead = rs_cu as i64 - asm_cu as i64;
        let overhead_pct = if asm_cu > 0 {
            (overhead as f64 / asm_cu as f64) * 100.0
        } else {
            0.0
        };
        println!(
            "| {} | {} | {} | {:+} | {:+.1}% |",
            case.name(),
            asm_cu,
            rs_cu,
            overhead,
            overhead_pct
        );
    }
}

// ============================================================================
// Initialize input checks
// ============================================================================

fn init_setup(
    program_language: ProgramLanguage,
) -> (test_utils::TestSetup, Instruction, Vec<(Pubkey, Account)>) {
    let setup = setup_test(program_language);
    let (system_program_pubkey, system_program_account) =
        program::keyed_account_for_system_program();

    let user_pubkey = Pubkey::new_unique();
    let tree_pubkey = Pubkey::new_unique();

    let instruction = Instruction::new_with_bytes(
        setup.program_id,
        &[],
        vec![
            AccountMeta::new(user_pubkey, true),
            AccountMeta::new(tree_pubkey, false),
            AccountMeta::new_readonly(system_program_pubkey, false),
        ],
    );

    let accounts = vec![
        (
            user_pubkey,
            Account::new(1_000_000, 0, &system_program_pubkey),
        ),
        (tree_pubkey, Account::new(0, 0, &system_program_pubkey)),
        (system_program_pubkey, system_program_account),
    ];

    (setup, instruction, accounts)
}

#[derive(Clone, Copy)]
enum InitCase {
    UserDataLen,
    TreeDuplicate,
    TreeDataLen,
    SystemProgramDuplicate,
    SystemProgramDataLen,
    InstructionData,
}

impl InitCase {
    const CASES: &'static [Self] = &[
        Self::UserDataLen,
        Self::TreeDuplicate,
        Self::TreeDataLen,
        Self::SystemProgramDuplicate,
        Self::SystemProgramDataLen,
        Self::InstructionData,
    ];

    const fn name(&self) -> &'static str {
        match self {
            Self::UserDataLen => "User has nonzero data length",
            Self::TreeDuplicate => "Tree account is duplicate",
            Self::TreeDataLen => "Tree has nonzero data length",
            Self::SystemProgramDuplicate => "System program is duplicate",
            Self::SystemProgramDataLen => "System program wrong data length",
            Self::InstructionData => "Non-empty instruction data",
        }
    }

    fn run(&self, lang: ProgramLanguage) -> u64 {
        match self {
            Self::UserDataLen => run_init_user_data_len(lang),
            Self::TreeDuplicate => run_init_tree_duplicate(lang),
            Self::TreeDataLen => run_init_tree_data_len(lang),
            Self::SystemProgramDuplicate => run_init_system_program_duplicate(lang),
            Self::SystemProgramDataLen => run_init_system_program_data_len(lang),
            Self::InstructionData => run_init_instruction_data(lang),
        }
    }
}

fn run_init_user_data_len(lang: ProgramLanguage) -> u64 {
    let (setup, instruction, mut accounts) = init_setup(lang);
    accounts[AccountIndex::User as usize].1.data = vec![1u8; 1];

    setup
        .mollusk
        .process_and_validate_instruction(
            &instruction,
            &accounts,
            &[Check::err(ProgramError::Custom(
                error_codes::error::USER_DATA_LEN.into(),
            ))],
        )
        .compute_units_consumed
}

fn run_init_tree_duplicate(lang: ProgramLanguage) -> u64 {
    let (setup, mut instruction, mut accounts) = init_setup(lang);
    instruction.accounts[AccountIndex::Tree as usize] =
        instruction.accounts[AccountIndex::User as usize].clone();
    accounts[AccountIndex::Tree as usize] = accounts[AccountIndex::User as usize].clone();

    setup
        .mollusk
        .process_and_validate_instruction(
            &instruction,
            &accounts,
            &[Check::err(ProgramError::Custom(
                error_codes::error::TREE_DUPLICATE.into(),
            ))],
        )
        .compute_units_consumed
}

fn run_init_tree_data_len(lang: ProgramLanguage) -> u64 {
    let (setup, instruction, mut accounts) = init_setup(lang);
    accounts[AccountIndex::Tree as usize].1.data = vec![1u8; 1];

    setup
        .mollusk
        .process_and_validate_instruction(
            &instruction,
            &accounts,
            &[Check::err(ProgramError::Custom(
                error_codes::error::TREE_DATA_LEN.into(),
            ))],
        )
        .compute_units_consumed
}

fn run_init_system_program_duplicate(lang: ProgramLanguage) -> u64 {
    let (setup, mut instruction, mut accounts) = init_setup(lang);
    instruction.accounts[AccountIndex::SystemProgram as usize] =
        instruction.accounts[AccountIndex::User as usize].clone();
    accounts[AccountIndex::SystemProgram as usize] =
        accounts[AccountIndex::User as usize].clone();

    setup
        .mollusk
        .process_and_validate_instruction(
            &instruction,
            &accounts,
            &[Check::err(ProgramError::Custom(
                error_codes::error::SYSTEM_PROGRAM_DUPLICATE.into(),
            ))],
        )
        .compute_units_consumed
}

fn run_init_system_program_data_len(lang: ProgramLanguage) -> u64 {
    let (setup, instruction, mut accounts) = init_setup(lang);
    accounts[AccountIndex::SystemProgram as usize].1.data = vec![];

    setup
        .mollusk
        .process_and_validate_instruction(
            &instruction,
            &accounts,
            &[Check::err(ProgramError::Custom(
                error_codes::error::SYSTEM_PROGRAM_DATA_LEN.into(),
            ))],
        )
        .compute_units_consumed
}

fn run_init_instruction_data(lang: ProgramLanguage) -> u64 {
    let (setup, mut instruction, accounts) = init_setup(lang);
    instruction.data = vec![1u8; 1];

    setup
        .mollusk
        .process_and_validate_instruction(
            &instruction,
            &accounts,
            &[Check::err(ProgramError::Custom(
                error_codes::error::INSTRUCTION_DATA.into(),
            ))],
        )
        .compute_units_consumed
}

#[test]
fn test_rs_init_user_data_len() {
    run_init_user_data_len(ProgramLanguage::Rust);
}

#[test]
fn test_rs_init_tree_duplicate() {
    run_init_tree_duplicate(ProgramLanguage::Rust);
}

#[test]
fn test_rs_init_tree_data_len() {
    run_init_tree_data_len(ProgramLanguage::Rust);
}

#[test]
fn test_rs_init_system_program_duplicate() {
    run_init_system_program_duplicate(ProgramLanguage::Rust);
}

#[test]
fn test_rs_init_system_program_data_len() {
    run_init_system_program_data_len(ProgramLanguage::Rust);
}

#[test]
fn test_rs_init_instruction_data() {
    run_init_instruction_data(ProgramLanguage::Rust);
}

#[test]
fn test_asm_init_user_data_len() {
    run_init_user_data_len(ProgramLanguage::Assembly);
}

#[test]
fn test_asm_init_tree_duplicate() {
    run_init_tree_duplicate(ProgramLanguage::Assembly);
}

#[test]
fn test_asm_init_tree_data_len() {
    run_init_tree_data_len(ProgramLanguage::Assembly);
}

#[test]
fn test_asm_init_system_program_duplicate() {
    run_init_system_program_duplicate(ProgramLanguage::Assembly);
}

#[test]
fn test_asm_init_system_program_data_len() {
    run_init_system_program_data_len(ProgramLanguage::Assembly);
}

#[test]
fn test_asm_init_instruction_data() {
    run_init_instruction_data(ProgramLanguage::Assembly);
}

#[test]
fn test_initialize_input_checks() {
    println!("\n| Init Case | Rust (CUs) |");
    println!("|-----------|------------|");

    for case in InitCase::CASES {
        let rs_cu = case.run(ProgramLanguage::Rust);
        println!("| {} | {} |", case.name(), rs_cu);
    }
}
