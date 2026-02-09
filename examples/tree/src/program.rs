use core::mem::transmute;
use pinocchio::{
    address::address_eq,
    hint::{likely, unlikely},
    no_allocator, nostd_panic_handler, AccountView, Address, SUCCESS,
};
use tree_interface::{data, error_codes::error, input_buffer};
#[cfg(target_os = "solana")]
use {
    core::{mem::MaybeUninit, ptr::null},
    pinocchio::syscalls::sol_try_find_program_address,
    tree_interface::cpi,
};

#[inline(always)]
unsafe fn account_at(input_buffer_ptr: *mut u8, offset: i16) -> AccountView {
    AccountView::new_unchecked(input_buffer_ptr.add(offset as usize).cast())
}

#[inline(always)]
unsafe fn ldxdw(ptr: *const u8, offset: i16) -> u64 {
    *ptr.add(offset as usize).cast::<u64>()
}

/// Checks if the account is a duplicate by checking if it's borrowed, since this is equivalent
/// via the underlying API due to the borrow state implementation.
#[inline(always)]
fn is_duplicate(account: &AccountView) -> bool {
    account.is_borrowed()
}

macro_rules! if_err {
    ($condition:expr, $error:expr) => {
        if unlikely($condition) {
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
    let user = account_at(input_buffer_ptr, input_buffer::USER_ACCOUNT_OFF);
    if_err!(!user.is_data_empty(), error::USER_DATA_LEN);

    // Error if tree is duplicate or has data.
    let tree = account_at(input_buffer_ptr, input_buffer::TREE_ACCOUNT_OFF);
    if_err!(is_duplicate(&tree), error::TREE_DUPLICATE);
    if_err!(!tree.is_data_empty(), error::TREE_DATA_LEN);

    // Error if System Program is duplicate or has invalid data length.
    let system_program = account_at(input_buffer_ptr, input_buffer::SYSTEM_PROGRAM_ACCOUNT_OFF);
    if_err!(
        is_duplicate(&system_program),
        error::SYSTEM_PROGRAM_DUPLICATE
    );
    if_err!(
        system_program.data_len() != input_buffer::SYSTEM_PROGRAM_DATA_LEN,
        error::SYSTEM_PROGRAM_DATA_LEN
    );

    // Error if instruction data provided.
    let instruction_data_len = ldxdw(
        input_buffer_ptr,
        input_buffer::INIT_INSTRUCTION_DATA_LEN_OFF,
    );
    if_err!(
        instruction_data_len != data::DATA_LEN_ZERO,
        error::INSTRUCTION_DATA
    );
    // ANCHOR_END: initialize-input-checks

    // ANCHOR: initialize-pda-checks
    #[cfg(target_os = "solana")]
    // Invoke syscall.
    let (pda, _bump) = {
        let mut pda = MaybeUninit::<Address>::uninit();
        let mut bump = MaybeUninit::<u8>::uninit();
        sol_try_find_program_address(
            // Pass a declared pointer instead of null to prevent unnecessary register assignment.
            input_buffer_ptr,
            cpi::N_SEEDS_TRY_FIND_PDA,
            input_buffer_ptr.add(input_buffer::INIT_PROGRAM_ID_OFF as usize),
            pda.as_mut_ptr().cast(),
            bump.as_mut_ptr(),
        );
        (pda.assume_init(), bump.assume_init())
    };
    // Dummy block for non-Solana target, to satisfy clippy.
    #[cfg(not(target_os = "solana"))]
    let (pda, _bump) = (Address::default(), 0u8);

    // Compare result with passed PDA.
    if !address_eq(
        &pda,
        #[allow(clippy::transmute_ptr_to_ref)]
        transmute::<*const u8, &Address>(
            input_buffer_ptr.add(input_buffer::TREE_ADDRESS_OFF as usize),
        ),
    ) {
        return error::PDA_MISMATCH.into();
    }
    // ANCHOR_END: initialize-pda-checks

    SUCCESS
}
