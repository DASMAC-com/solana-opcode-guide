use mollusk_svm::program;
use mollusk_svm::result::Check;
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use std::mem::{offset_of, size_of};
use test_utils::{setup_test, ProgramLanguage};

const E_N_ACCOUNTS: u32 = 1;
const E_DATA_LENGTH_NONZERO_SENDER: u32 = 2;
const E_DUPLICATE_ACCOUNT_RECIPIENT: u32 = 3;
const E_DATA_LENGTH_NONZERO_RECIPIENT: u32 = 4;
const E_DUPLICATE_ACCOUNT_SYSTEM_PROGRAM: u32 = 5;
const E_INVALID_INSTRUCTION_DATA_LENGTH: u32 = 6;
const E_INSUFFICIENT_LAMPORTS: u32 = 7;

enum AccountPosition {
    Sender = 0,
    Recipient = 1,
    SystemProgram = 2,
}

const TRANSFER_AMOUNT: u64 = 10;

#[test]
fn test_asm() {
    let setup = setup_test(ProgramLanguage::Assembly);

    // Set up accounts.
    let (system_program, system_account) = program::keyed_account_for_system_program();
    let system_meta = AccountMeta::new_readonly(system_program, false);
    let sender_pubkey = Pubkey::new_unique();
    let sender_meta = AccountMeta::new(sender_pubkey, true);
    let recipient_pubkey = Pubkey::new_unique();
    let recipient_meta = AccountMeta::new(recipient_pubkey, false);
    let mut recipient_account = Account::new(0, 0, &system_program);

    // Check no accounts passed.
    let mut instruction = Instruction::new_with_bytes(setup.program_id, &[], vec![]);
    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &[],
        &[Check::err(ProgramError::Custom(E_N_ACCOUNTS))],
    );

    // Check nonzero sender data length.
    let mut sender_account = Account::new(0, 1, &system_program);
    instruction.accounts = vec![
        sender_meta.clone(),
        recipient_meta.clone(),
        system_meta.clone(),
    ];
    let mut accounts = vec![
        (sender_pubkey, sender_account.clone()),
        (recipient_pubkey, recipient_account.clone()),
        (system_program, system_account.clone()),
    ];
    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            E_DATA_LENGTH_NONZERO_SENDER,
        ))],
    );
    sender_account.data = vec![];
    accounts[AccountPosition::Sender as usize].1 = sender_account.clone();

    // Check duplicate recipient account.
    instruction.accounts[AccountPosition::Recipient as usize] = sender_meta.clone();
    accounts[AccountPosition::Recipient as usize] = (sender_pubkey, sender_account.clone());
    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            E_DUPLICATE_ACCOUNT_RECIPIENT,
        ))],
    );
    recipient_account.data = vec![0];
    instruction.accounts[AccountPosition::Recipient as usize] = recipient_meta.clone();

    // Check nonzero recipient data length.
    accounts[AccountPosition::Recipient as usize] = (recipient_pubkey, recipient_account.clone());
    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            E_DATA_LENGTH_NONZERO_RECIPIENT,
        ))],
    );
    recipient_account.data = vec![];
    accounts[AccountPosition::Recipient as usize].1 = recipient_account.clone();

    // Check duplicate system program account.
    instruction.accounts[AccountPosition::SystemProgram as usize] = recipient_meta.clone();
    accounts[AccountPosition::SystemProgram as usize] =
        (recipient_pubkey, recipient_account.clone());
    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            E_DUPLICATE_ACCOUNT_SYSTEM_PROGRAM,
        ))],
    );
    instruction.accounts[AccountPosition::SystemProgram as usize] = system_meta.clone();
    accounts[AccountPosition::SystemProgram as usize] = (system_program, system_account.clone());

    // Check invalid instruction data length.
    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            E_INVALID_INSTRUCTION_DATA_LENGTH,
        ))],
    );

    // Check insufficient lamports.
    instruction.data = TRANSFER_AMOUNT.to_le_bytes().to_vec();
    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(E_INSUFFICIENT_LAMPORTS))],
    );
}

#[test]
fn test_offsets() {
    const MAX_PERMITTED_DATA_INCREASE: usize = 10240;

    #[allow(dead_code)]
    #[repr(C)]
    struct AccountLayout<const PADDED_DATA_SIZE: usize> {
        non_dup_marker: u8,
        is_signer: u8,
        is_writable: u8,
        is_executable: u8,
        padding: [u8; 4],
        pubkey: [u8; 32],
        owner: [u8; 32],
        lamports: u64,
        data_length: u64,
        data_padded: [u8; PADDED_DATA_SIZE],
        rent_epoch: u64,
    }

    type StandardAccount = AccountLayout<MAX_PERMITTED_DATA_INCREASE>;
    type SystemProgramAccount = AccountLayout<{ MAX_PERMITTED_DATA_INCREASE + 16 }>;

    // Sender.
    const SENDER_OFFSET: usize = 8;
    const SENDER_LAMPORTS_OFFSET: usize = 80;
    const SENDER_DATA_LENGTH_OFFSET: usize = 88;

    // Recipient.
    const RECIPIENT_OFFSET: usize = 10344;
    const RECIPIENT_DATA_LENGTH_OFFSET: usize = 10424;

    // System program.
    const SYSTEM_PROGRAM_OFFSET: usize = 20680;

    // Instruction data.
    const INSTRUCTION_DATA_LENGTH_OFFSET: usize = 31032;
    const INSTRUCTION_DATA_OFFSET: usize = 31040;

    assert_eq!(
        SENDER_LAMPORTS_OFFSET,
        SENDER_OFFSET + offset_of!(StandardAccount, lamports),
    );
    assert_eq!(
        SENDER_DATA_LENGTH_OFFSET,
        SENDER_OFFSET + offset_of!(StandardAccount, data_length),
    );
    assert_eq!(
        RECIPIENT_OFFSET,
        SENDER_OFFSET + size_of::<StandardAccount>()
    );
    assert_eq!(
        RECIPIENT_DATA_LENGTH_OFFSET,
        RECIPIENT_OFFSET + offset_of!(StandardAccount, data_length),
    );
    assert_eq!(
        SYSTEM_PROGRAM_OFFSET,
        RECIPIENT_OFFSET + size_of::<StandardAccount>()
    );
    assert_eq!(
        INSTRUCTION_DATA_LENGTH_OFFSET,
        SYSTEM_PROGRAM_OFFSET + size_of::<SystemProgramAccount>()
    );
    assert_eq!(
        INSTRUCTION_DATA_OFFSET,
        INSTRUCTION_DATA_LENGTH_OFFSET + size_of::<u64>(),
    );
}
