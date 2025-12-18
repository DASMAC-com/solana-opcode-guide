use mollusk_svm::result::Check;
use solana_sdk::account::AccountSharedData;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use test_utils::{setup_test, ProgramLanguage};

#[test]
fn test_asm_pass() {
    happy_path(ProgramLanguage::Assembly);

    // Create a mock account that will trigger an error when passed.
    let mock_account_pubkey = Pubkey::new_unique();
    let mock_account_data = AccountSharedData::default();
    let accounts = vec![AccountMeta::new(mock_account_pubkey, false)];
    let n_accounts = accounts.len() as u32;
    let instruction = Instruction::new_with_bytes(setup.program_id, b"Whoops", accounts);

    // Verify that the instruction fails with the expected error code.
    let result = setup.mollusk.process_and_validate_instruction(
        &instruction,
        &[(mock_account_pubkey, mock_account_data.into())],
        &[Check::err(ProgramError::Custom(n_accounts))],
    );
    assert!(result.program_result.is_err());
}
