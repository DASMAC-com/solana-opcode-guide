use interface::*;
use pinocchio::{
    address::address_eq,
    entrypoint::{InstructionContext, MaybeAccount},
    error::ProgramError,
    lazy_program_entrypoint, no_allocator, nostd_panic_handler, Address, ProgramResult,
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
        return Err(ProgramError::Custom(error_codes::error::$variant.into()))
    };
}

nostd_panic_handler!();
no_allocator!();

// ANCHOR: entrypoint-branch
lazy_program_entrypoint!(process_instruction);

pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
    match context.remaining() {
        input_buffer::N_ACCOUNTS_GENERAL => Ok(()),
        input_buffer::N_ACCOUNTS_INIT => initialize(context),
        _ => err!(N_ACCOUNTS),
    }
}
// ANCHOR_END: entrypoint-branch

#[inline(always)]
fn initialize(mut context: InstructionContext) -> ProgramResult {
    // Verify user has no data.
    // SAFETY: number of accounts has been checked.
    let user = unsafe { context.next_account_unchecked().assume_account() };
    if_err!(!user.is_data_empty(), USER_DATA_LEN);

    // Verify tree is non-duplicate and has no data.
    // SAFETY: number of accounts has been checked.
    let tree = match unsafe { context.next_account_unchecked() } {
        MaybeAccount::Account(account) => account,
        MaybeAccount::Duplicated(_) => err!(TREE_DUPLICATE),
    };
    if_err!(!tree.is_data_empty(), TREE_DATA_LEN);

    // Verify system program is non-duplicate and has expected data length.
    let system_program = match unsafe { context.next_account_unchecked() } {
        MaybeAccount::Account(account) => account,
        MaybeAccount::Duplicated(_) => err!(SYSTEM_PROGRAM_DUPLICATE),
    };
    if_err!(
        system_program.data_len() != input_buffer::SYSTEM_PROGRAM_DATA_LEN,
        SYSTEM_PROGRAM_DATA_LEN
    );

    // Verify tree PDA.
    let program_id = unsafe { context.program_id_unchecked() };
    let (expected_pda, bump) = Address::find_program_address(&[], program_id);
    if_err!(!address_eq(tree.address(), &expected_pda), PDA_MISMATCH);
    Ok(())
}
