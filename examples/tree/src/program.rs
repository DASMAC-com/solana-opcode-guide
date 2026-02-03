use interface::*;
use pinocchio::{
    entrypoint::InstructionContext, error::ProgramError, lazy_program_entrypoint, no_allocator,
    nostd_panic_handler, ProgramResult,
};

/// If condition is true, return the given error.
macro_rules! if_err {
    ($cond:expr, $variant:ident) => {
        if $cond {
            return Err(ProgramError::Custom(Error::$variant.into()));
        }
    };
}

lazy_program_entrypoint!(process_instruction);
nostd_panic_handler!();
no_allocator!();

pub fn process_instruction(context: InstructionContext) -> ProgramResult {
    if_err!(context.remaining() != input_buffer::N_ACCOUNTS, N_ACCOUNTS_INVALID);
    Ok(())
}
