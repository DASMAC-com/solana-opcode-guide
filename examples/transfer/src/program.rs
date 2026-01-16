use core::mem::size_of;
use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke,
    instruction::{AccountMeta, Instruction},
    no_allocator, nostd_panic_handler, program_entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};

const SYSTEM_PROGRAM_TRANSFER_DISCRIMINANT: u32 = 2;
const SYSTEM_PROGRAM_ID: Pubkey = [0u8; 32];
const N_CPI_ACCOUNTS: usize = 2;
const N_INSTRUCTION_ACCOUNTS: usize = 3;

const E_N_ACCOUNTS: u32 = 1;
const E_INSTRUCTION_DATA_LENGTH: u32 = 6;
const E_INSUFFICIENT_LAMPORTS: u32 = 7;

const CPI_DATA_SIZE: usize = size_of::<u32>() + size_of::<u64>();

program_entrypoint!(process_instruction, N_INSTRUCTION_ACCOUNTS);
nostd_panic_handler!();
no_allocator!();

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [sender, recipient, _system_program] = accounts else {
        return Err(ProgramError::Custom(E_N_ACCOUNTS));
    };

    // Parse transfer amount.
    if instruction_data.len() != size_of::<u64>() {
        return Err(ProgramError::Custom(E_INSTRUCTION_DATA_LENGTH));
    };
    // SAFETY: instruction_data is validated to be the correct length.
    let amount = unsafe {
        u64::from_le_bytes(*(instruction_data.as_ptr() as *const [u8; size_of::<u64>()]))
    };

    // Validate sender has sufficient Lamports.
    if sender.lamports() < amount {
        return Err(ProgramError::Custom(E_INSUFFICIENT_LAMPORTS));
    }

    // Build CPI instruction data.
    let mut cpi_data = core::mem::MaybeUninit::<[u8; CPI_DATA_SIZE]>::uninit();
    // SAFETY: Sources are aligned, initialized, and valid.
    let cpi_data = unsafe {
        let ptr = cpi_data.as_mut_ptr() as *mut u8;
        core::ptr::copy_nonoverlapping(
            SYSTEM_PROGRAM_TRANSFER_DISCRIMINANT.to_le_bytes().as_ptr(),
            ptr,
            size_of::<u32>(),
        );
        core::ptr::copy_nonoverlapping(
            amount.to_le_bytes().as_ptr(),
            ptr.add(size_of::<u32>()),
            size_of::<u64>(),
        );
        cpi_data.assume_init()
    };

    // Build CPI instruction.
    let instruction = Instruction {
        program_id: &SYSTEM_PROGRAM_ID,
        accounts: &[
            AccountMeta::writable_signer(sender.key()),
            AccountMeta::writable(recipient.key()),
        ],
        data: &cpi_data,
    };

    // Invoke the System Program transfer.
    invoke::<N_CPI_ACCOUNTS>(&instruction, &[sender, recipient])
}
