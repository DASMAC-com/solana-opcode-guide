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

// --- Test case definitions ---

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
            Self::UserDataLen => "User has non-zero data length",
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

// --- Test runners that return CU consumption ---

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

    // Make tree account a duplicate of user.
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

// --- Individual tests for each case/implementation ---

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

// --- Compute unit comparison table ---

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
