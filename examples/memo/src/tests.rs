use mollusk_svm::{result::Check, Mollusk};
use solana_sdk::account::AccountSharedData;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signer::Signer;

#[test]
fn test_asm_fail() {
    let keypair =
        read_keypair_file("deploy/memo-keypair.json").expect("Failed to read keypair file");
    let program_id = keypair.pubkey();
    let mollusk = Mollusk::new(&program_id, "deploy/memo");

    let mock_account_pubkey = Pubkey::new_unique();
    let mock_account_data = AccountSharedData::default();
    let accounts = vec![AccountMeta::new(mock_account_pubkey, false)];
    let n_accounts = accounts.len() as u32;
    let instruction = Instruction::new_with_bytes(program_id, b"Whoops", accounts);

    let result = mollusk.process_and_validate_instruction(
        &instruction,
        &[(mock_account_pubkey, mock_account_data.into())],
        &[Check::err(ProgramError::Custom(n_accounts))],
    );
    assert!(result.program_result.is_err());
}

#[test]
fn test_asm_pass() {
    let keypair =
        read_keypair_file("deploy/memo-keypair.json").expect("Failed to read keypair file");
    let program_id = keypair.pubkey();
    let mollusk = Mollusk::new(&program_id, "deploy/memo");

    let instruction = Instruction::new_with_bytes(program_id, b"Hello again, DASMAC!", vec![]);

    let result = mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
    assert!(!result.program_result.is_err());
}

#[test]
fn test_rs() {
    let keypair =
        read_keypair_file("deploy/memo-keypair.json").expect("Failed to read keypair file");
    let program_id = keypair.pubkey();
    let mollusk = Mollusk::new(&program_id, "../target/deploy/memo");

    let instruction = Instruction::new_with_bytes(program_id, b"Hello again, DASMAC!", vec![]);

    let result = mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
    assert!(!result.program_result.is_err());
}
