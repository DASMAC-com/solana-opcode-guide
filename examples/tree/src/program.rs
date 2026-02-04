use interface::*;
use pinocchio::{
    entrypoint::{InstructionContext, MaybeAccount},
    error::ProgramError,
    lazy_program_entrypoint, no_allocator, nostd_panic_handler, ProgramResult,
};

/// If condition is true, return the given error.
macro_rules! if_err {
    ($cond:expr, $variant:ident) => {
        if $cond {
            err!($variant);
        }
    };
}

/// Return the given error.
macro_rules! err {
    ($variant:ident) => {
        return Err(ProgramError::Custom(Error::$variant.into()))
    };
}

lazy_program_entrypoint!(process_instruction);
nostd_panic_handler!();
no_allocator!();

pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
    // Verify the input memory map: user has no data, tree is not duplicate.
    if_err!(context.remaining() != input_buffer::N_ACCOUNTS, N_ACCOUNTS);
    // SAFETY: number of accounts has been checked.
    let user = unsafe { context.next_account_unchecked().assume_account() };
    if_err!(user.data_len() != 0, USER_DATA_LEN);
    // SAFETY: number of accounts has been checked.
    let tree = match unsafe { context.next_account_unchecked() } {
        MaybeAccount::Account(account) => account,
        MaybeAccount::Duplicated(_) => err!(TREE_DUPLICATE),
    };
    // SAFETY: all accounts have been read.
    let instruction_data = unsafe { context.instruction_data_unchecked() };
    let program_id = unsafe { context.program_id_unchecked() };
    Ok(())
}
