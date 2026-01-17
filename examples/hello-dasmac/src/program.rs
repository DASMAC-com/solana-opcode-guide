use pinocchio::{entrypoint, AccountView, Address, ProgramResult};

#[cfg(target_os = "solana")]
use pinocchio::syscalls::sol_log_;

entrypoint!(process_instruction);

#[cfg_attr(not(target_os = "solana"), allow(unused_variables))]
fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    #[cfg(target_os = "solana")]
    {
        const MSG: &[u8] = b"Hello, DASMAC!";
        unsafe { sol_log_(MSG.as_ptr(), MSG.len() as u64) };
    }
    Ok(())
}
