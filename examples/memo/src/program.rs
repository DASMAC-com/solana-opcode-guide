use pinocchio::{entrypoint, msg, pubkey::Pubkey, ProgramResult};

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[pinocchio::account_info::AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Log the instruction data (assumes valid UTF-8 bytes).
    unsafe {
        msg!(core::str::from_utf8_unchecked(instruction_data));
    }
    Ok(())
}
