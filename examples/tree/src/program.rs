use pinocchio::{
    entrypoint::InstructionContext, lazy_program_entrypoint, no_allocator, nostd_panic_handler,
    ProgramResult,
};

lazy_program_entrypoint!(process_instruction);
nostd_panic_handler!();
no_allocator!();

pub fn process_instruction(_context: InstructionContext) -> ProgramResult {
    Ok(())
}
