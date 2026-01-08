use mollusk_svm::program;
use mollusk_svm::result::Check;
use solana_sdk::instruction::Instruction;
use solana_sdk::program_error::ProgramError;
use std::mem::size_of;
use test_utils::{setup_test, single_mock_account, ProgramLanguage};

const E_DUPLICATE_ACCOUNTS: u32 = 2;

#[test]
fn test_asm() {
    let setup = setup_test(ProgramLanguage::Assembly);
}

#[test]
fn test_offsets() {
    const SENDER_OFFSET: usize = 8;
    const MAX_PERMITTED_DATA_INCREASE: usize = 10240;

    const RECIPIENT_OFFSET: usize = 10344;

    const SYSTEM_PROGRAM_OFFSET: usize = 20680;

    const INSTRUCTION_DATA_LENGTH_OFFSET: usize = 31032;

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
