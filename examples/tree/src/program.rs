use interface::{error_codes::error, input_buffer};
use pinocchio::{
    address::address_eq,
    entrypoint::{InstructionContext, MaybeAccount},
    error::ProgramError,
    lazy_program_entrypoint, no_allocator, nostd_panic_handler, AccountView, Address,
    ProgramResult,
};

#[inline(always)]
/// Return an error code for early return at call site.
fn err<T>(error_code: error) -> Result<T, ProgramError> {
    Err(ProgramError::Custom(error_code.into()))
}

#[inline(always)]
/// Ensure a condition is met else return error code for early return at call site.
fn ensure(condition: bool, error_code: error) -> Result<(), ProgramError> {
    if condition {
        Ok(())
    } else {
        err(error_code)
    }
}

#[inline(always)]
/// Ensure an account has empty data else return error code for early return at call site.
fn ensure_is_data_empty(account: &AccountView, error_code: error) -> Result<(), ProgramError> {
    ensure(account.is_data_empty(), error_code)
}

#[inline(always)]
unsafe fn next_account_non_duplicate(
    context: &mut InstructionContext,
    error_code: error,
) -> Result<AccountView, ProgramError> {
    match unsafe { context.next_account_unchecked() } {
        MaybeAccount::Account(account) => Ok(account),
        MaybeAccount::Duplicated(_) => err(error_code),
    }
}

// ANCHOR: entrypoint-branch
nostd_panic_handler!();
no_allocator!();
lazy_program_entrypoint!(process_instruction);

pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
    match context.remaining() {
        input_buffer::N_ACCOUNTS_GENERAL => Ok(()),
        // SAFETY: number of accounts has been checked.
        input_buffer::N_ACCOUNTS_INIT => unsafe { initialize(context) },
        _ => err(error::N_ACCOUNTS),
    }
}
// ANCHOR_END: entrypoint-branch

#[inline(always)]
fn initialize(mut context: InstructionContext) -> ProgramResult {
    // Verify user has no data.
    // SAFETY: number of accounts has been checked.
    let user = unsafe { context.next_account_unchecked().assume_account() };
    ensure_is_data_empty(&user, error::USER_DATA_LEN)?;

    // Verify tree is non-duplicate and has no data.
    // SAFETY: number of accounts has been checked.
    let tree = unsafe { next_account_non_duplicate(&mut context, error::TREE_DUPLICATE) }?;
    ensure_is_data_empty(&tree, error::TREE_DATA_LEN)?;

    // Verify system program is non-duplicate and has expected data length.
    // SAFETY: number of accounts has been checked.
    let system_program =
        unsafe { next_account_non_duplicate(&mut context, error::SYSTEM_PROGRAM_DUPLICATE) }?;
    ensure(
        system_program.data_len() == input_buffer::SYSTEM_PROGRAM_DATA_LEN,
        error::SYSTEM_PROGRAM_DATA_LEN,
    )?;

    // Verify no instruction data provided.
    // SAFETY: all accounts have been consumed.
    ensure(
        unsafe { context.instruction_data_unchecked().is_empty() },
        error::INSTRUCTION_DATA,
    )?;

    // Verify tree PDA.
    let program_id = unsafe { context.program_id_unchecked() };
    let (expected_pda, bump) = Address::find_program_address(&[], program_id);
    ensure(
        address_eq(tree.address(), &expected_pda),
        error::PDA_MISMATCH,
    )?;
    Ok(())
}
