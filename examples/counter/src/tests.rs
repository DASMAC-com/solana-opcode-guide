use crate::constants::constants;
use mollusk_svm::program;
use mollusk_svm::result::Check;
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use std::fs;
use test_utils::{setup_test, ProgramLanguage};

#[test]
fn test_asm_file_constants() {
    const GLOBAL_ENTRYPOINT: &str = ".global entrypoint";

    // Parse assembly file.
    let asm_path = setup_test(ProgramLanguage::Assembly)
        .asm_source_path
        .expect("Assembly source file not found");
    let content = fs::read_to_string(&asm_path).expect("Failed to read assembly file");
    let global_pos = content
        .find(GLOBAL_ENTRYPOINT)
        .expect("Could not find '.global entrypoint' in assembly file");

    // Overwrite assembly file with updated constants, asserting nothing changed.
    let after_global = &content[global_pos..];
    let new_content = format!("{}\n{}", constants().to_asm(), after_global);
    let changed = new_content != content;
    fs::write(&asm_path, new_content).expect("Failed to write assembly file");
    assert!(
        !changed,
        "Assembly file constants were out of date and have been updated. Please re-run the test."
    );
}

const USER_STARTING_LAMPORTS: u64 = 10_000;

enum Operation {
    Initialize,
    Increment,
}

enum AccountIndex {
    User = 0,
    Pda = 1,
    SystemProgram = 2,
}
fn happy_path_setup(
    program_language: ProgramLanguage,
    operation: Operation,
) -> (
    test_utils::TestSetup,
    Instruction,
    Vec<(Pubkey, Account)>,
    Vec<Check<'static>>,
) {
    let setup = setup_test(program_language);
    let (system_program, system_account) = program::keyed_account_for_system_program();

    let user_pubkey = Pubkey::new_unique();
    let (pda_pubkey, _bump) =
        Pubkey::find_program_address(&[user_pubkey.as_ref()], &setup.program_id);

    let mut instruction = Instruction::new_with_bytes(
        setup.program_id,
        &[],
        vec![
            AccountMeta::new(user_pubkey, true),
            AccountMeta::new(pda_pubkey, false),
        ],
    );

    let mut accounts = vec![
        (
            instruction.accounts[AccountIndex::User as usize].pubkey,
            Account::new(USER_STARTING_LAMPORTS, 0, &system_program),
        ),
        (
            instruction.accounts[AccountIndex::Pda as usize].pubkey,
            Account::new(0, 0, &setup.program_id),
        ),
    ];

    let checks = vec![Check::success()];

    match operation {
        Operation::Initialize => {
            instruction
                .accounts
                .push(AccountMeta::new_readonly(system_program, false));
            accounts.push((system_program, system_account));
        }
        Operation::Increment => {}
    }
    (setup, instruction, accounts, checks)
}

#[test]
fn test_asm_no_accounts() {
    let (setup, mut instruction, mut accounts, _checks) =
        happy_path_setup(ProgramLanguage::Assembly, Operation::Initialize);

    instruction.accounts.clear();
    accounts.clear();

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            constants().get("E_N_ACCOUNTS") as u32,
        ))],
    );
}

#[test]
fn test_asm_too_many_accounts() {
    let (setup, mut instruction, mut accounts, _checks) =
        happy_path_setup(ProgramLanguage::Assembly, Operation::Initialize);

    instruction
        .accounts
        .push(AccountMeta::new_readonly(Pubkey::new_unique(), false));
    accounts.push((
        instruction.accounts.last().unwrap().pubkey,
        Account::default(),
    ));

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            constants().get("E_N_ACCOUNTS") as u32,
        ))],
    );
}

#[test]
fn test_asm_initialize_user_data_len() {
    let (setup, instruction, mut accounts, _checks) =
        happy_path_setup(ProgramLanguage::Assembly, Operation::Initialize);

    accounts[AccountIndex::User as usize].1.data = vec![1u8; 1];

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            constants().get("E_USER_DATA_LEN") as u32,
        ))],
    );
}

#[test]
fn test_asm_initialize_pda_data_len() {
    let (setup, instruction, mut accounts, _checks) =
        happy_path_setup(ProgramLanguage::Assembly, Operation::Initialize);

    accounts[AccountIndex::Pda as usize].1.data = vec![1u8; 1];

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            constants().get("E_PDA_DATA_LEN") as u32,
        ))],
    );
}

#[test]
fn test_asm_initialize_system_program_data_len() {
    let (setup, instruction, mut accounts, _checks) =
        happy_path_setup(ProgramLanguage::Assembly, Operation::Initialize);

    accounts[AccountIndex::SystemProgram as usize].1.data = vec![];

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            constants().get("E_SYSTEM_PROGRAM_DATA_LEN") as u32,
        ))],
    );
}

#[test]
fn test_asm_initialize_pda_duplicate() {
    let (setup, mut instruction, mut accounts, _checks) =
        happy_path_setup(ProgramLanguage::Assembly, Operation::Initialize);

    instruction.accounts[AccountIndex::Pda as usize] =
        instruction.accounts[AccountIndex::User as usize].clone();
    accounts[AccountIndex::Pda as usize] = accounts[AccountIndex::User as usize].clone();

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            constants().get("E_PDA_DUPLICATE") as u32,
        ))],
    );
}

#[test]
fn test_asm_initialize_system_program_duplicate() {
    let (setup, mut instruction, mut accounts, _checks) =
        happy_path_setup(ProgramLanguage::Assembly, Operation::Initialize);

    instruction.accounts[AccountIndex::SystemProgram as usize] =
        instruction.accounts[AccountIndex::User as usize].clone();
    accounts[AccountIndex::SystemProgram as usize] = accounts[AccountIndex::User as usize].clone();

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            constants().get("E_SYSTEM_PROGRAM_DUPLICATE") as u32,
        ))],
    );
}

#[test]
fn test_asm_initialize_pda_mismatch() {
    let (setup, mut instruction, mut accounts, _checks) =
        happy_path_setup(ProgramLanguage::Assembly, Operation::Initialize);

    instruction.accounts[AccountIndex::Pda as usize].pubkey = Pubkey::new_unique();
    accounts[AccountIndex::Pda as usize].0 =
        instruction.accounts[AccountIndex::Pda as usize].pubkey;

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            constants().get("E_PDA_MISMATCH") as u32,
        ))],
    );
}

#[test]
fn test_asm_initialize_happy_path() {
    let (setup, instruction, accounts, checks) =
        happy_path_setup(ProgramLanguage::Assembly, Operation::Initialize);

    setup
        .mollusk
        .process_and_validate_instruction(&instruction, &accounts, &checks);
}
