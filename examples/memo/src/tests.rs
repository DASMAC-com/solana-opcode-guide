use mollusk_svm::{result::Check, Mollusk};
use solana_sdk::account::AccountSharedData;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signer::Signer;

#[test]
fn asm_happy() {
    let keypair =
        read_keypair_file("deploy/memo-keypair.json").expect("Failed to read keypair file");
    let program_id = keypair.pubkey();
    let mollusk = Mollusk::new(&program_id, "deploy/memo");

    let instruction = Instruction::new_with_bytes(program_id, b"Hello, again DASMAC!", vec![]);

    let result = mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
    assert!(!result.program_result.is_err());
}

#[test]
fn asm_with_account_fails() {
    let keypair =
        read_keypair_file("deploy/memo-keypair.json").expect("Failed to read keypair file");
    let program_id = keypair.pubkey();
    let mollusk = Mollusk::new(&program_id, "deploy/memo");

    let dummy_account_key = Pubkey::new_unique();
    let dummy_account_data = AccountSharedData::default();
    let accounts = vec![AccountMeta::new(dummy_account_key, false)];
    let instruction = Instruction::new_with_bytes(program_id, b"This should fail", accounts);

    let result = mollusk.process_and_validate_instruction(
        &instruction,
        &[(dummy_account_key, dummy_account_data.into())],
        &[Check::err(ProgramError::Custom(1))],
    );
    assert!(result.program_result.is_err());
}
