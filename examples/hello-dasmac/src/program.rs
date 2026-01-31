use pinocchio::{entrypoint, AccountView, Address, ProgramResult};

#[cfg(target_os = "solana")]
use pinocchio::syscalls::sol_log_;

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    #[cfg(target_os = "solana")]
    {
        const MESSAGE: &[u8] = b"Hello, DASMAC!";
        unsafe { sol_log_(MESSAGE.as_ptr(), MESSAGE.len() as u64) };
    }
    Ok(())
}
