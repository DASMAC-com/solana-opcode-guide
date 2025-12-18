use pinocchio::{entrypoint, pubkey::Pubkey, ProgramResult};

entrypoint!(process_instruction);

#[cfg_attr(not(target_os = "solana"), allow(unused_variables))]
fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[pinocchio::account_info::AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}
