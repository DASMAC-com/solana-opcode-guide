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
const E_INSTRUCTION_DATA_LENGTH: u32 = 6;
const E_INSUFFICIENT_LAMPORTS: u32 = 7;

enum AccountPosition {
    Sender = 0,
    Recipient = 1,
    SystemProgram = 2,
}

const TRANSFER_AMOUNT: u64 = 10;
const COMPUTE_UNIT_OVERHEAD: u64 = 10_000;
const ALIGNMENT: usize = 8;

#[test]
fn test_asm() {
    let setup = setup_test(ProgramLanguage::Assembly);

    // Set up happy path accounts and instruction data.
    let (system_program, system_account) = program::keyed_account_for_system_program();
    let happy_path_instruction = Instruction::new_with_bytes(
        setup.program_id,
        &TRANSFER_AMOUNT.to_le_bytes(),
        vec![
            AccountMeta::new(Pubkey::new_unique(), true),
            AccountMeta::new(Pubkey::new_unique(), false),
            AccountMeta::new_readonly(system_program, false),
        ],
    );
    let happy_path_accounts = vec![
        (
            happy_path_instruction.accounts[AccountPosition::Sender as usize].pubkey,
            Account::new(TRANSFER_AMOUNT + COMPUTE_UNIT_OVERHEAD, 0, &system_program),
        ),
        (
            happy_path_instruction.accounts[AccountPosition::Recipient as usize].pubkey,
            Account::new(0, 0, &system_program),
        ),
        (system_program, system_account.clone()),
    ];

    // Check no accounts passed.
    let mut instruction = happy_path_instruction.clone();
    instruction.accounts.clear();
    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &[],
        &[Check::err(ProgramError::Custom(E_N_ACCOUNTS))],
    );

    // Check nonzero sender data length.
    let mut accounts = happy_path_accounts.clone();
    accounts[AccountPosition::Sender as usize].1.data = vec![0];
    setup.mollusk.process_and_validate_instruction(
        &happy_path_instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            E_DATA_LENGTH_NONZERO_SENDER,
        ))],
    );

    // Check duplicate recipient account.
    instruction = happy_path_instruction.clone();
    instruction.accounts[AccountPosition::Recipient as usize] =
        happy_path_instruction.accounts[AccountPosition::Sender as usize].clone();
    accounts = happy_path_accounts.clone();
    accounts[AccountPosition::Recipient as usize] =
        happy_path_accounts[AccountPosition::Sender as usize].clone();
    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            E_DUPLICATE_ACCOUNT_RECIPIENT,
        ))],
    );

    // Check nonzero recipient data length.
    accounts = happy_path_accounts.clone();
    accounts[AccountPosition::Recipient as usize].1.data = vec![0];
    setup.mollusk.process_and_validate_instruction(
        &happy_path_instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            E_DATA_LENGTH_NONZERO_RECIPIENT,
        ))],
    );

    // Check duplicate system program account.
    instruction = happy_path_instruction.clone();
    instruction.accounts[AccountPosition::SystemProgram as usize] =
        happy_path_instruction.accounts[AccountPosition::Recipient as usize].clone();
    accounts = happy_path_accounts.clone();
    accounts[AccountPosition::SystemProgram as usize] =
        happy_path_accounts[AccountPosition::Recipient as usize].clone();
    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            E_DUPLICATE_ACCOUNT_SYSTEM_PROGRAM,
        ))],
    );

    // Check invalid instruction data length.
    instruction = happy_path_instruction.clone();
    instruction.data.clear();
    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &happy_path_accounts,
        &[Check::err(ProgramError::Custom(E_INSTRUCTION_DATA_LENGTH))],
    );

    // Check insufficient lamports.
    accounts = happy_path_accounts.clone();
    accounts[AccountPosition::Sender as usize].1.lamports = TRANSFER_AMOUNT - 1;
    setup.mollusk.process_and_validate_instruction(
        &happy_path_instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(E_INSUFFICIENT_LAMPORTS))],
    );
}

#[test]
fn test_input_offsets() {
    const MAX_PERMITTED_DATA_INCREASE: usize = 10240;

    #[allow(dead_code)]
    #[repr(C)]
    struct AccountLayout<const PADDED_DATA_SIZE: usize> {
        non_dup_marker: u8,
        is_signer: u8,
        is_writable: u8,
        is_executable: u8,
        original_data_len: [u8; 4],
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
    const SENDER_PUBKEY_OFFSET: usize = 16;
    const SENDER_IS_SIGNER_OFFSET: usize = 9;
    const SENDER_IS_WRITABLE_OFFSET: usize = 10;
    const SENDER_LAMPORTS_OFFSET: usize = 80;
    const SENDER_DATA_LENGTH_OFFSET: usize = 88;

    // Recipient.
    const RECIPIENT_OFFSET: usize = 10344;
    const RECIPIENT_PUBKEY_OFFSET: usize = 10352;
    const RECIPIENT_IS_SIGNER_OFFSET: usize = 10345;
    const RECIPIENT_IS_WRITABLE_OFFSET: usize = 10346;
    const RECIPIENT_DATA_LENGTH_OFFSET: usize = 10424;

    // System program.
    const SYSTEM_PROGRAM_OFFSET: usize = 20680;
    const SYSTEM_PROGRAM_PUBKEY_OFFSET: usize = 20688;

    // Instruction data.
    const INSTRUCTION_DATA_LENGTH_OFFSET: usize = 31032;
    const INSTRUCTION_DATA_OFFSET: usize = 31040;

    // Sender checks.
    assert_eq!(
        SENDER_IS_SIGNER_OFFSET,
        SENDER_OFFSET + offset_of!(StandardAccount, is_signer),
    );
    assert_eq!(
        SENDER_IS_WRITABLE_OFFSET,
        SENDER_OFFSET + offset_of!(StandardAccount, is_writable),
    );
    assert_eq!(
        SENDER_LAMPORTS_OFFSET,
        SENDER_OFFSET + offset_of!(StandardAccount, lamports),
    );
    assert_eq!(
        SENDER_DATA_LENGTH_OFFSET,
        SENDER_OFFSET + offset_of!(StandardAccount, data_length),
    );
    assert_eq!(
        SENDER_PUBKEY_OFFSET,
        SENDER_OFFSET + offset_of!(StandardAccount, pubkey),
    );

    // Recipient checks.
    assert_eq!(
        RECIPIENT_OFFSET,
        SENDER_OFFSET + size_of::<StandardAccount>()
    );
    assert_eq!(
        RECIPIENT_DATA_LENGTH_OFFSET,
        RECIPIENT_OFFSET + offset_of!(StandardAccount, data_length),
    );
    assert_eq!(
        RECIPIENT_PUBKEY_OFFSET,
        RECIPIENT_OFFSET + offset_of!(StandardAccount, pubkey),
    );
    assert_eq!(
        RECIPIENT_IS_SIGNER_OFFSET,
        RECIPIENT_OFFSET + offset_of!(StandardAccount, is_signer),
    );
    assert_eq!(
        RECIPIENT_IS_WRITABLE_OFFSET,
        RECIPIENT_OFFSET + offset_of!(StandardAccount, is_writable),
    );

    // System program checks.
    assert_eq!(
        SYSTEM_PROGRAM_OFFSET,
        RECIPIENT_OFFSET + size_of::<StandardAccount>()
    );
    assert_eq!(
        SYSTEM_PROGRAM_PUBKEY_OFFSET,
        SYSTEM_PROGRAM_OFFSET + offset_of!(SystemProgramAccount, pubkey),
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

#[test]
fn test_cpi_offsets() {
    #[repr(C)]
    struct SolInstruction {
        program_id_addr: u64,
        accounts_addr: u64,
        accounts_len: u64,
        data_addr: u64,
        data_len: u64,
    }

    #[repr(C)]
    struct SolAccountMeta {
        pubkey_addr: u64,
        is_writable: bool,
        is_signer: bool,
        padding: [u8; 6],
    }

    #[repr(C)]
    struct SolAccountInfo {
        key_addr: u64,
        lamports_addr: u64,
        data_len: u64,
        data_addr: u64,
        owner_addr: u64,
        rent_epoch: u64,
        is_signer: bool,
        is_writable: bool,
        executable: bool,
        padding: [u8; 5],
    }

    #[repr(C)]
    struct InstructionData {
        variant: [u8; 4],
        amount: [u8; 8],
        padding: [u8; 4],
    }

    // CPI instruction offsets.
    const CPI_INSN_PROGRAM_ID_ADDR_OFFSET: usize = 0;
    const CPI_INSN_ACCOUNTS_ADDR_OFFSET: usize = 8;
    const CPI_INSN_ACCOUNTS_LEN_OFFSET: usize = 16;
    const CPI_INSN_DATA_ADDR_OFFSET: usize = 24;
    const CPI_INSN_DATA_LEN_OFFSET: usize = 32;

    // CPI account meta offsets.
    const CPI_ACCT_META_PUBKEY_ADDR_OFFSET: usize = 0;
    const CPI_ACCT_META_IS_WRITABLE_OFFSET: usize = 8;
    const CPI_ACCT_META_IS_SIGNER_OFFSET: usize = 9;
    const CPI_ACCT_META_SIZE_OF: usize = 16;

    // CPI account info offsets.
    const CPI_ACCT_INFO_KEY_ADDR_OFFSET: usize = 0;
    const CPI_ACCT_INFO_LAMPORTS_ADDR_OFFSET: usize = 8;
    const CPI_ACCT_INFO_DATA_LEN_OFFSET: usize = 16;
    const CPI_ACCT_INFO_DATA_ADDR_OFFSET: usize = 24;
    const CPI_ACCT_INFO_OWNER_ADDR_OFFSET: usize = 32;
    const CPI_ACCT_INFO_RENT_EPOCH_OFFSET: usize = 40;
    const CPI_ACCT_INFO_IS_SIGNER_OFFSET: usize = 48;
    const CPI_ACCT_INFO_IS_WRITABLE_OFFSET: usize = 49;
    const CPI_ACCT_INFO_EXECUTABLE_OFFSET: usize = 50;

    // CPI instruction data offsets.
    const CPI_INSN_DATA_VARIANT_OFFSET: usize = 0;
    const CPI_INSN_DATA_AMOUNT_OFFSET: usize = 4;
    const CPI_INSN_DATA_LEN: usize = 12;

    // Stack offsets.
    const STACK_INSN_OFFSET: usize = 200;
    const STACK_INSN_DATA_OFFSET: usize = 160;
    const STACK_ACCT_METAS_OFFSET: usize = 144;
    const STACK_ACCT_INFOS_OFFSET: usize = 112;

    // CPI instruction checks.
    assert_eq!(
        CPI_INSN_PROGRAM_ID_ADDR_OFFSET,
        offset_of!(SolInstruction, program_id_addr)
    );
    assert_eq!(
        CPI_INSN_ACCOUNTS_ADDR_OFFSET,
        offset_of!(SolInstruction, accounts_addr)
    );
    assert_eq!(
        CPI_INSN_ACCOUNTS_LEN_OFFSET,
        offset_of!(SolInstruction, accounts_len)
    );
    assert_eq!(
        CPI_INSN_DATA_ADDR_OFFSET,
        offset_of!(SolInstruction, data_addr)
    );
    assert_eq!(
        CPI_INSN_DATA_LEN_OFFSET,
        offset_of!(SolInstruction, data_len)
    );

    // CPI account meta checks.
    assert_eq!(
        CPI_ACCT_META_PUBKEY_ADDR_OFFSET,
        offset_of!(SolAccountMeta, pubkey_addr)
    );
    assert_eq!(
        CPI_ACCT_META_IS_WRITABLE_OFFSET,
        offset_of!(SolAccountMeta, is_writable)
    );
    assert_eq!(
        CPI_ACCT_META_IS_SIGNER_OFFSET,
        offset_of!(SolAccountMeta, is_signer)
    );
    assert!(size_of::<SolAccountMeta>().is_multiple_of(ALIGNMENT));
    assert_eq!(CPI_ACCT_META_SIZE_OF, size_of::<SolAccountMeta>());

    // CPI account info checks.
    assert_eq!(
        CPI_ACCT_INFO_KEY_ADDR_OFFSET,
        offset_of!(SolAccountInfo, key_addr)
    );
    assert_eq!(
        CPI_ACCT_INFO_LAMPORTS_ADDR_OFFSET,
        offset_of!(SolAccountInfo, lamports_addr)
    );
    assert_eq!(
        CPI_ACCT_INFO_DATA_LEN_OFFSET,
        offset_of!(SolAccountInfo, data_len)
    );
    assert_eq!(
        CPI_ACCT_INFO_DATA_ADDR_OFFSET,
        offset_of!(SolAccountInfo, data_addr)
    );
    assert_eq!(
        CPI_ACCT_INFO_OWNER_ADDR_OFFSET,
        offset_of!(SolAccountInfo, owner_addr)
    );
    assert_eq!(
        CPI_ACCT_INFO_RENT_EPOCH_OFFSET,
        offset_of!(SolAccountInfo, rent_epoch)
    );
    assert_eq!(
        CPI_ACCT_INFO_IS_SIGNER_OFFSET,
        offset_of!(SolAccountInfo, is_signer)
    );
    assert_eq!(
        CPI_ACCT_INFO_IS_WRITABLE_OFFSET,
        offset_of!(SolAccountInfo, is_writable)
    );
    assert_eq!(
        CPI_ACCT_INFO_EXECUTABLE_OFFSET,
        offset_of!(SolAccountInfo, executable)
    );
    assert!(size_of::<SolAccountMeta>().is_multiple_of(ALIGNMENT));

    // CPI instruction data checks.
    assert_eq!(
        CPI_INSN_DATA_VARIANT_OFFSET,
        offset_of!(InstructionData, variant)
    );
    assert_eq!(
        CPI_INSN_DATA_AMOUNT_OFFSET,
        offset_of!(InstructionData, amount)
    );
    assert!(size_of::<InstructionData>().is_multiple_of(ALIGNMENT));
    assert_eq!(CPI_INSN_DATA_LEN, size_of::<u32>() + size_of::<u64>(),);

    // Stack offset checks.
    assert_eq!(STACK_ACCT_INFOS_OFFSET, 2 * size_of::<SolAccountInfo>());
    assert_eq!(
        STACK_ACCT_METAS_OFFSET,
        STACK_ACCT_INFOS_OFFSET + 2 * size_of::<SolAccountMeta>()
    );
    assert_eq!(
        STACK_INSN_DATA_OFFSET,
        STACK_ACCT_METAS_OFFSET + size_of::<InstructionData>()
    );
    assert_eq!(
        STACK_INSN_OFFSET,
        STACK_INSN_DATA_OFFSET + size_of::<SolInstruction>()
    );
    assert!(STACK_ACCT_INFOS_OFFSET.is_multiple_of(ALIGNMENT));
    assert!(STACK_ACCT_METAS_OFFSET.is_multiple_of(ALIGNMENT));
    assert!(STACK_INSN_DATA_OFFSET.is_multiple_of(ALIGNMENT));
    assert!(STACK_INSN_OFFSET.is_multiple_of(ALIGNMENT));
}
