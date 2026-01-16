use core::mem::size_of;
use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke,
    entrypoint,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};

/// System program transfer instruction variant.
const SYSTEM_PROGRAM_TRANSFER: u32 = 2;

/// System program ID (all zeros).
const SYSTEM_PROGRAM_ID: Pubkey = [0u8; 32];

const N_CPI_ACCOUNTS: usize = 2;
const N_INSTRUCTION_ACCOUNTS: usize = 3;

const E_N_ACCOUNTS: u32 = 1;
const E_INSTRUCTION_DATA_LENGTH: u32 = 6;
const E_INSUFFICIENT_LAMPORTS: u32 = 7;

const CPI_DATA_SIZE: usize = size_of::<u32>() + size_of::<u64>();

enum AccountIndex {
    Sender = 0,
    Recipient = 1,
}

entrypoint!(process_instruction, N_INSTRUCTION_ACCOUNTS);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Validate accounts length.
    if accounts.len() != N_INSTRUCTION_ACCOUNTS {
        return Err(ProgramError::Custom(E_N_ACCOUNTS));
    }

    // Parse accounts.
    let sender = &accounts[AccountIndex::Sender as usize];
    let recipient = &accounts[AccountIndex::Recipient as usize];

    // Parse transfer amount.
    let amount = u64::from_le_bytes(
        instruction_data
            .try_into()
            .map_err(|_| ProgramError::Custom(E_INSTRUCTION_DATA_LENGTH))?,
    );

    // Validate sender has sufficient Lamports.
    if sender.lamports() < amount {
        return Err(ProgramError::Custom(E_INSUFFICIENT_LAMPORTS));
    }

    // Build CPI instruction data: [variant (4 bytes), amount (8 bytes)].
    let mut cpi_data = [0u8; CPI_DATA_SIZE];
    cpi_data[0..size_of::<u32>()].copy_from_slice(&SYSTEM_PROGRAM_TRANSFER.to_le_bytes());
    cpi_data[size_of::<u32>()..CPI_DATA_SIZE].copy_from_slice(&amount.to_le_bytes());

    // Build account metas for CPI.
    let account_metas = [
        AccountMeta::writable_signer(sender.key()),
        AccountMeta::writable(recipient.key()),
    ];

    // Build CPI instruction.
    let instruction = Instruction {
        program_id: &SYSTEM_PROGRAM_ID,
        accounts: &account_metas,
        data: &cpi_data,
    };

    // Invoke the System Program transfer.
    invoke::<N_CPI_ACCOUNTS>(&instruction, &[sender, recipient])
}
