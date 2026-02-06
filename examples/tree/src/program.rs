use core::mem::transmute;
use interface::{error_codes::error, input_buffer};
use pinocchio::{
    address::address_eq,
    entrypoint::{lazy::InstructionContext, MaybeAccount},
    error::ProgramError,
    no_allocator, nostd_panic_handler, AccountView, Address, ProgramResult, SUCCESS,
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

// ANCHOR: entrypoint-branching
no_allocator!();
nostd_panic_handler!();

#[no_mangle]
pub unsafe extern "C" fn entrypoint(input_buffer_ptr: *mut u8) -> u64 {
    match *transmute::<*mut u8, *const u64>(
        input_buffer_ptr.add(input_buffer::N_ACCOUNTS_OFF as usize),
    ) {
        input_buffer::N_ACCOUNTS_GENERAL => SUCCESS,
        input_buffer::N_ACCOUNTS_INIT => SUCCESS,
        _ => error::N_ACCOUNTS.into(),
    }
}
// ANCHOR_END: entrypoint-branching

#[inline(always)]
fn general_branch() -> u64 {
    10
}

#[inline(always)]
fn initialize_branch() -> u64 {
    5
}

// ANCHOR: initialize-input-checks
#[inline(always)]
/// SAFETY: Called by entrypoint after verifying the right number of accounts for initialization.
fn initialize(mut context: InstructionContext) -> ProgramResult {
    // Error if user has data.
    // SAFETY: number of accounts has been checked.
    let user = unsafe { context.next_account_unchecked().assume_account() };
    ensure_is_data_empty(&user, error::USER_DATA_LEN)?;

    // Error if tree is duplicate or has data.
    // SAFETY: number of accounts has been checked.
    let tree = unsafe { next_account_non_duplicate(&mut context, error::TREE_DUPLICATE) }?;
    ensure_is_data_empty(&tree, error::TREE_DATA_LEN)?;

    // Error if System Program is duplicate or has invalid data length.
    // SAFETY: number of accounts has been checked.
    let system_program =
        unsafe { next_account_non_duplicate(&mut context, error::SYSTEM_PROGRAM_DUPLICATE) }?;
    ensure(
        system_program.data_len() == input_buffer::SYSTEM_PROGRAM_DATA_LEN,
        error::SYSTEM_PROGRAM_DATA_LEN,
    )?;

    // Error if instruction data provided.
    // SAFETY: all accounts have been consumed.
    ensure(
        unsafe { context.instruction_data_unchecked().is_empty() },
        error::INSTRUCTION_DATA,
    )?;
    // ANCHOR_END: initialize-input-checks

    // Verify tree PDA.
    let program_id = unsafe { context.program_id_unchecked() };
    let (expected_pda, bump) = Address::find_program_address(&[], program_id);
    ensure(
        address_eq(tree.address(), &expected_pda),
        error::PDA_MISMATCH,
    )?;
    Ok(())
}
