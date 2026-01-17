use pinocchio::{entrypoint, AccountView, Address, ProgramResult};

#[cfg(target_os = "solana")]
use pinocchio::syscalls::sol_log_;

entrypoint!(process_instruction);

#[cfg_attr(not(target_os = "solana"), allow(unused_variables))]
fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    #[cfg(target_os = "solana")]
    unsafe {
        sol_log_(instruction_data.as_ptr(), instruction_data.len() as u64);
    }
    Ok(())
}
