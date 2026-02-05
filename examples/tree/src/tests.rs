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
}

fn happy_path_setup(
    program_language: ProgramLanguage,
) -> (test_utils::TestSetup, Instruction, Vec<(Pubkey, Account)>) {
    let setup = setup_test(program_language);
    let (system_program, _) = program::keyed_account_for_system_program();

    let user_pubkey = Pubkey::new_unique();
    let tree_pubkey = Pubkey::new_unique();

    let instruction = Instruction::new_with_bytes(
        setup.program_id,
        &[],
        vec![
            AccountMeta::new(user_pubkey, true),
            AccountMeta::new(tree_pubkey, false),
        ],
    );

    let accounts = vec![
        (user_pubkey, Account::new(1_000_000, 0, &system_program)),
        (tree_pubkey, Account::new(0, 0, &system_program)),
    ];

    (setup, instruction, accounts)
}

#[derive(Clone, Copy)]
enum Case {
    NoAccounts,
    TooManyAccounts,
    UserDataLen,
    TreeDuplicate,
}

impl Case {
    const PARSING_CASES: &'static [Case] = &[
        Case::NoAccounts,
        Case::TooManyAccounts,
        Case::UserDataLen,
        Case::TreeDuplicate,
    ];

    const fn name(&self) -> &'static str {
        match self {
            Self::NoAccounts => "No accounts passed",
            Self::TooManyAccounts => "Too many accounts passed",
            Self::UserDataLen => "User has nonzero data length",
            Self::TreeDuplicate => "Tree account is duplicate",
        }
    }

    fn run(&self, lang: ProgramLanguage) -> u64 {
        match self {
            Self::NoAccounts => run_no_accounts(lang),
            Self::TooManyAccounts => run_too_many_accounts(lang),
            Self::UserDataLen => run_user_data_len(lang),
            Self::TreeDuplicate => run_tree_duplicate(lang),
        }
    }
}

fn run_no_accounts(lang: ProgramLanguage) -> u64 {
    let (setup, mut instruction, mut accounts) = happy_path_setup(lang);

    instruction.accounts.clear();
    accounts.clear();

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

fn run_too_many_accounts(lang: ProgramLanguage) -> u64 {
    let (setup, mut instruction, mut accounts) = happy_path_setup(lang);

    instruction
        .accounts
        .push(AccountMeta::new_readonly(Pubkey::new_unique(), false));
    accounts.push((
        instruction.accounts.last().unwrap().pubkey,
        Account::default(),
    ));

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

fn run_user_data_len(lang: ProgramLanguage) -> u64 {
    let (setup, instruction, mut accounts) = happy_path_setup(lang);

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

fn run_tree_duplicate(lang: ProgramLanguage) -> u64 {
    let (setup, mut instruction, mut accounts) = happy_path_setup(lang);

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

#[test]
fn test_asm_no_accounts() {
    run_no_accounts(ProgramLanguage::Assembly);
}

#[test]
fn test_asm_too_many_accounts() {
    run_too_many_accounts(ProgramLanguage::Assembly);
}

#[test]
fn test_asm_user_data_len() {
    run_user_data_len(ProgramLanguage::Assembly);
}

#[test]
fn test_asm_tree_duplicate() {
    run_tree_duplicate(ProgramLanguage::Assembly);
}

#[test]
fn test_rs_no_accounts() {
    run_no_accounts(ProgramLanguage::Rust);
}

#[test]
fn test_rs_too_many_accounts() {
    run_too_many_accounts(ProgramLanguage::Rust);
}

#[test]
fn test_rs_user_data_len() {
    run_user_data_len(ProgramLanguage::Rust);
}

#[test]
fn test_rs_tree_duplicate() {
    run_tree_duplicate(ProgramLanguage::Rust);
}

#[test]
fn test_fast_fails() {
    println!("| Case | ASM (CUs) | Rust (CUs) | Overhead | Overhead % |");
    println!("|------|-----------|------------|----------|------------|");

    for case in Case::PARSING_CASES {
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
// Initialize operation tests
// ============================================================================

/// Setup for initialize instruction (empty instruction data, correct PDA).
fn init_setup(
    program_language: ProgramLanguage,
) -> (test_utils::TestSetup, Instruction, Vec<(Pubkey, Account)>) {
    let setup = setup_test(program_language);
    let (system_program, _) = program::keyed_account_for_system_program();

    let user_pubkey = Pubkey::new_unique();
    // Derive the correct PDA for the tree account.
    let (tree_pda, _bump) = Pubkey::find_program_address(&[], &setup.program_id);

    let instruction = Instruction::new_with_bytes(
        setup.program_id,
        &[], // Empty instruction data triggers initialize path
        vec![
            AccountMeta::new(user_pubkey, true),
            AccountMeta::new(tree_pda, false),
        ],
    );

    let accounts = vec![
        (user_pubkey, Account::new(1_000_000, 0, &system_program)),
        (tree_pda, Account::new(0, 0, &system_program)),
    ];

    (setup, instruction, accounts)
}

#[derive(Clone, Copy)]
enum InitCase {
    PdaMismatch,
}

impl InitCase {
    const CASES: &'static [InitCase] = &[InitCase::PdaMismatch];

    const fn name(&self) -> &'static str {
        match self {
            Self::PdaMismatch => "PDA mismatch",
        }
    }

    fn run(&self, lang: ProgramLanguage) -> u64 {
        match self {
            Self::PdaMismatch => run_init_pda_mismatch(lang),
        }
    }
}

fn run_init_pda_mismatch(lang: ProgramLanguage) -> u64 {
    let (setup, mut instruction, mut accounts) = init_setup(lang);
    let (system_program, _) = program::keyed_account_for_system_program();

    // Replace tree PDA with a random pubkey (not the correct PDA).
    let wrong_tree_pubkey = Pubkey::new_unique();
    instruction.accounts[AccountIndex::Tree as usize] = AccountMeta::new(wrong_tree_pubkey, false);
    accounts[AccountIndex::Tree as usize] =
        (wrong_tree_pubkey, Account::new(0, 0, &system_program));

    setup
        .mollusk
        .process_and_validate_instruction(
            &instruction,
            &accounts,
            &[Check::err(ProgramError::Custom(
                error_codes::error::PDA_MISMATCH.into(),
            ))],
        )
        .compute_units_consumed
}

#[test]
fn test_rs_init_pda_mismatch() {
    run_init_pda_mismatch(ProgramLanguage::Rust);
}

#[test]
#[should_panic] // ASM doesn't have PDA check yet
fn test_asm_init_pda_mismatch() {
    run_init_pda_mismatch(ProgramLanguage::Assembly);
}

#[test]
fn test_init_fails() {
    println!("\n| Init Case | ASM (CUs) | Rust (CUs) | Overhead | Overhead % |");
    println!("|-----------|-----------|------------|----------|------------|");

    for case in InitCase::CASES {
        // Only run Rust for now since ASM doesn't have init logic yet.
        let rs_cu = case.run(ProgramLanguage::Rust);
        println!("| {} | N/A | {} | N/A | N/A |", case.name(), rs_cu);
    }
}
