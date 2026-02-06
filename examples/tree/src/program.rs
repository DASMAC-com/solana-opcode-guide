use core::mem::transmute;
use interface::{error_codes::error, input_buffer, misc};
use pinocchio::{
    address::address_eq,
    entrypoint::{lazy::InstructionContext, MaybeAccount, NON_DUP_MARKER},
    error::ProgramError,
    hint::{likely, unlikely},
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

#[inline(always)]
unsafe fn ldxdw(ptr: *const u8, offset: i16) -> u64 {
    *transmute::<*const u8, *const u64>(ptr.add(offset as usize))
}

#[inline(always)]
unsafe fn ldxb(ptr: *const u8, offset: i16) -> u8 {
    *ptr.add(offset as usize)
}

#[no_mangle]
pub unsafe extern "C" fn entrypoint(input_buffer_ptr: *mut u8) -> u64 {
    let n_accounts = ldxdw(input_buffer_ptr, input_buffer::N_ACCOUNTS_OFF);
    if likely(n_accounts == input_buffer::N_ACCOUNTS_GENERAL) {
        return general(input_buffer_ptr);
    };
    if likely(n_accounts == input_buffer::N_ACCOUNTS_INIT) {
        return initialize(input_buffer_ptr);
    };
    error::N_ACCOUNTS.into()
}
// ANCHOR_END: entrypoint-branching

#[inline(always)]
unsafe fn general(input_buffer_ptr: *mut u8) -> u64 {
    if ldxdw(input_buffer_ptr, input_buffer::USER_DATA_LEN_OFF) == 67 {
        6677
    } else {
        666777
    }
}

// ANCHOR: initialize-input-checks
#[inline(always)]
#[cold]
unsafe fn initialize(input_buffer_ptr: *mut u8) -> u64 {
    // Error if user has data.
    if unlikely(ldxdw(input_buffer_ptr, input_buffer::USER_DATA_LEN_OFF) != misc::DATA_LEN_ZERO) {
        return error::USER_DATA_LEN.into();
    }

    // Error if tree is duplicate or has data.
    if unlikely(ldxb(input_buffer_ptr, input_buffer::TREE_NON_DUP_MARKER_OFF) != NON_DUP_MARKER) {
        return error::TREE_DUPLICATE.into();
    }
    if unlikely(ldxdw(input_buffer_ptr, input_buffer::TREE_DATA_LEN_OFF) != misc::DATA_LEN_ZERO) {
        return error::TREE_DATA_LEN.into();
    }

    // Error if System Program is duplicate or has invalid data length.
    if unlikely(
        ldxb(
            input_buffer_ptr,
            input_buffer::SYSTEM_PROGRAM_NON_DUP_MARKER_OFF,
        ) != NON_DUP_MARKER,
    ) {
        return error::SYSTEM_PROGRAM_DUPLICATE.into();
    }
    if unlikely(
        ldxdw(input_buffer_ptr, input_buffer::SYSTEM_PROGRAM_DATA_LEN_OFF)
            != input_buffer::SYSTEM_PROGRAM_DATA_LEN as u64,
    ) {
        return error::SYSTEM_PROGRAM_DATA_LEN.into();
    }

    // Error if instruction data provided.
    if unlikely(
        ldxdw(
            input_buffer_ptr,
            input_buffer::INIT_INSTRUCTION_DATA_LEN_OFF,
        ) != misc::DATA_LEN_ZERO,
    ) {
        return error::INSTRUCTION_DATA.into();
    }
    // ANCHOR_END: initialize-input-checks

    SUCCESS
}
