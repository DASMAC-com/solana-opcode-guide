use pinocchio::{entrypoint, pubkey::Pubkey, ProgramResult};

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[pinocchio::account_info::AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}
