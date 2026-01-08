use mollusk_svm::program;
use mollusk_svm::result::Check;
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use std::mem::{offset_of, size_of};
use test_utils::{setup_test, ProgramLanguage};

const E_N_ACCOUNTS: u32 = 1;
const E_DUPLICATE_ACCOUNT_RECIPIENT: u32 = 2;
const E_DUPLICATE_ACCOUNT_SYSTEM_PROGRAM: u32 = 3;
const E_INVALID_INSTRUCTION_DATA_LENGTH: u32 = 4;
const E_INSUFFICIENT_LAMPORTS: u32 = 5;

const TRANSFER_AMOUNT: u64 = 10;

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
    instruction.accounts = vec![
        sender_meta.clone(),
        recipient_meta.clone(),
        sender_meta.clone(),
    ];
    accounts = vec![
        (sender_pubkey, sender_account.clone()),
        (recipient_pubkey, recipient_account.clone()),
        (system_program, system_account.clone()),
    ];
    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            E_DUPLICATE_ACCOUNT_SYSTEM_PROGRAM,
        ))],
    );

    // Check invalid instruction data length.
    instruction.accounts = vec![sender_meta, recipient_meta, system_meta];
    accounts = vec![
        (sender_pubkey, sender_account.clone()),
        (recipient_pubkey, recipient_account.clone()),
        (system_program, system_account.clone()),
    ];
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

    // Recipient.
    const RECIPIENT_OFFSET: usize = 10344;

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
    assert_eq!(
        INSTRUCTION_DATA_OFFSET,
        INSTRUCTION_DATA_LENGTH_OFFSET + size_of::<u64>(),
    );
}
