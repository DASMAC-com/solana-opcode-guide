use mollusk_svm::program;
use mollusk_svm::result::Check;
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use std::mem::size_of;
use test_utils::{setup_test, ProgramLanguage};

const E_N_ACCOUNTS: u32 = 1;
const E_DUPLICATE_ACCOUNT_RECIPIENT: u32 = 2;
const E_DUPLICATE_ACCOUNT_SYSTEM_PROGRAM: u32 = 3;

#[test]
fn test_asm() {
    let setup = setup_test(ProgramLanguage::Assembly);

    // Set up accounts.
    let (system_program, system_account) = program::keyed_account_for_system_program();
    let system_meta = AccountMeta::new_readonly(system_program, false);
    let sender_pubkey = Pubkey::new_unique();
    let sender_meta = AccountMeta::new(sender_pubkey, true);
    let sender_account = Account::new(0, 0, &system_program);
    let recipient_pubkey = Pubkey::new_unique();
    let recipient_meta = AccountMeta::new(recipient_pubkey, false);
    let recipient_account = Account::new(0, 0, &system_program);

    // Check no accounts passed.
    let mut instruction = Instruction::new_with_bytes(setup.program_id, &[], vec![]);
    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &[],
        &[Check::err(ProgramError::Custom(E_N_ACCOUNTS))],
    );

    // Check duplicate recipient account.
    instruction.accounts = vec![
        sender_meta.clone(),
        sender_meta.clone(),
        system_meta.clone(),
    ];
    let mut accounts = vec![
        (sender_pubkey, sender_account.clone()),
        (sender_pubkey, sender_account.clone()),
        (system_program, system_account.clone()),
    ];
    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            E_DUPLICATE_ACCOUNT_RECIPIENT,
        ))],
    );

    // Check duplicate system program account.
    instruction.accounts[1] = recipient_meta.clone();
    instruction.accounts[2] = sender_meta.clone();
    accounts[1] = (recipient_pubkey, recipient_account.clone());
    accounts[2] = (sender_pubkey, sender_account.clone());
    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            E_DUPLICATE_ACCOUNT_SYSTEM_PROGRAM,
        ))],
    );
}

#[test]
fn test_offsets() {
    const SENDER_OFFSET: usize = 8;
    const MAX_PERMITTED_DATA_INCREASE: usize = 10240;

    struct StandardAccount {
        non_dup_marker: u8,
        is_signer: u8,
        is_writable: u8,
        is_executable: u8,
        padding: [u8; 4],
        pubkey: [u8; 32],
        owner: [u8; 32],
        lamports: u64,
        data_length: u64,
        data_padded: [u8; MAX_PERMITTED_DATA_INCREASE],
        rent_epoch: u64,
    }

    struct SystemProgramAccount {
        non_dup_marker: u8,
        is_signer: u8,
        is_writable: u8,
        is_executable: u8,
        padding: [u8; 4],
        pubkey: [u8; 32],
        owner: [u8; 32],
        lamports: u64,
        data_length: u64,
        data_padded: [u8; MAX_PERMITTED_DATA_INCREASE + 16],
        rent_epoch: u64,
    }

    const RECIPIENT_OFFSET: usize = 10344;

    const SYSTEM_PROGRAM_OFFSET: usize = 20680;

    const INSTRUCTION_DATA_LENGTH_OFFSET: usize = 31032;

    assert_eq!(
        RECIPIENT_OFFSET,
        SENDER_OFFSET + size_of::<StandardAccount>()
    );
    assert_eq!(
        SYSTEM_PROGRAM_OFFSET,
        RECIPIENT_OFFSET + size_of::<StandardAccount>()
    );
    assert_eq!(
        INSTRUCTION_DATA_LENGTH_OFFSET,
        SYSTEM_PROGRAM_OFFSET + size_of::<SystemProgramAccount>()
    );
}
