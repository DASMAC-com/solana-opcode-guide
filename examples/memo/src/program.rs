use pinocchio::{entrypoint, pubkey::Pubkey, ProgramResult};

#[cfg(target_os = "solana")]
use pinocchio::syscalls::sol_log_;

entrypoint!(process_instruction);

#[cfg_attr(not(target_os = "solana"), allow(unused_variables))]
fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[pinocchio::account_info::AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    #[cfg(target_os = "solana")]
    unsafe {
        sol_log_(instruction_data.as_ptr(), instruction_data.len() as u64);
    }
    Ok(())
}
