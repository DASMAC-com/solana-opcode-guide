use mollusk_svm::result::Check;
use solana_sdk::account::AccountSharedData;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use test_utils::{setup_test, ProgramLanguage};

#[test]
fn test_asm_fail() {
    let setup = setup_test(ProgramLanguage::Assembly);

    // Create a mock account that will trigger an error when passed.
    let mock_account_pubkey = Pubkey::new_unique();
    let accounts = vec![AccountMeta::new(mock_account_pubkey, false)];
    let instruction = Instruction::new_with_bytes(setup.program_id, b"Whoops", accounts.clone());
    let mock_account_data = AccountSharedData::default();

    // Verify that the instruction fails with the expected error code.
    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &[(mock_account_pubkey, mock_account_data.into())],
        &[Check::err(ProgramError::Custom(accounts.len() as u32))],
    );
}

#[test]
fn test_asm_pass() {
    happy_path(ProgramLanguage::Assembly);
}

#[test]
fn test_rs() {
    happy_path(ProgramLanguage::Rust);
}

fn happy_path(program_language: ProgramLanguage) {
    let setup = setup_test(program_language);

    // Create an instruction with a simple memo message.
    let instruction =
        Instruction::new_with_bytes(setup.program_id, b"Hello again, DASMAC!", vec![]);

    // Verify the instruction completes successfully.
    setup
        .mollusk
        .process_and_validate_instruction(&instruction, &[], &[Check::success()]);
}
