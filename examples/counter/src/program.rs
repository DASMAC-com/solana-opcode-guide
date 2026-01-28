use pinocchio::{
    entrypoint::InstructionContext, lazy_program_entrypoint, no_allocator, nostd_panic_handler,
    ProgramResult,
};

lazy_program_entrypoint!(process_instruction);
nostd_panic_handler!();
no_allocator!();

const E_N_ACCOUNTS: u32 = 1;

const N_ACCOUNTS_INCREMENT: u64 = 2;
const N_ACCOUNTS_INITIALIZE: u64 = 3;

pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
    match context.remaining() {
        N_ACCOUNTS_INCREMENT => {}
        N_ACCOUNTS_INITIALIZE => {}
        _ => return Err(pinocchio::error::ProgramError::Custom(E_N_ACCOUNTS)),
    }
    Ok(())
}
