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
unsafe fn ldxb(ptr: *const u8, offset: i16) -> u8 {
    *ptr.add(offset as usize)
}

macro_rules! ensure_ldxb {
    ($ptr:expr, $offset:expr, $expected:expr, $error:expr) => {
        if unlikely(ldxb($ptr, $offset) != $expected) {
            return $error.into();
        }
    };
}

#[inline(always)]
unsafe fn ldxdw(ptr: *const u8, offset: i16) -> u64 {
    *transmute::<*const u8, *const u64>(ptr.add(offset as usize))
}

macro_rules! ensure_ldxdw {
    ($ptr:expr, $offset:expr, $expected:expr, $error:expr) => {
        if unlikely(ldxdw($ptr, $offset) != $expected) {
            return $error.into();
        }
    };
}

// ANCHOR: entrypoint-branching
no_allocator!();
nostd_panic_handler!();

#[no_mangle]
pub unsafe extern "C" fn entrypoint(input_buffer_ptr: *mut u8) -> u64 {
    let n_accounts = ldxdw(input_buffer_ptr, input_buffer::N_ACCOUNTS_OFF);
    if likely(n_accounts == input_buffer::N_ACCOUNTS_GENERAL) {
        general(input_buffer_ptr)
    } else if likely(n_accounts == input_buffer::N_ACCOUNTS_INIT) {
        initialize(input_buffer_ptr)
    } else {
        error::N_ACCOUNTS.into()
    }
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
unsafe fn initialize(input_buffer_ptr: *mut u8) -> u64 {
    // Error if user has data.
    ensure_ldxdw!(
        input_buffer_ptr,
        input_buffer::USER_DATA_LEN_OFF,
        misc::DATA_LEN_ZERO,
        error::USER_DATA_LEN
    );

    // Error if tree is duplicate or has data.
    ensure_ldxb!(
        input_buffer_ptr,
        input_buffer::TREE_NON_DUP_MARKER_OFF,
        NON_DUP_MARKER,
        error::TREE_DUPLICATE
    );
    ensure_ldxdw!(
        input_buffer_ptr,
        input_buffer::TREE_DATA_LEN_OFF,
        misc::DATA_LEN_ZERO,
        error::TREE_DATA_LEN
    );

    // Error if System Program is duplicate or has invalid data length.
    ensure_ldxb!(
        input_buffer_ptr,
        input_buffer::SYSTEM_PROGRAM_NON_DUP_MARKER_OFF,
        NON_DUP_MARKER,
        error::SYSTEM_PROGRAM_DUPLICATE
    );
    ensure_ldxdw!(
        input_buffer_ptr,
        input_buffer::SYSTEM_PROGRAM_DATA_LEN_OFF,
        input_buffer::SYSTEM_PROGRAM_DATA_LEN as u64,
        error::SYSTEM_PROGRAM_DATA_LEN
    );

    // Error if instruction data provided.
    ensure_ldxdw!(
        input_buffer_ptr,
        input_buffer::INIT_INSTRUCTION_DATA_LEN_OFF,
        misc::DATA_LEN_ZERO,
        error::INSTRUCTION_DATA
    );
    // ANCHOR_END: initialize-input-checks

    SUCCESS
}
