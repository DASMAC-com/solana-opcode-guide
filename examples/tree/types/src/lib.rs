#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
mod injection;
#[cfg(feature = "std")]
pub use injection::inject_asm;
#[cfg(feature = "std")]
extern crate std;

use build_macros::{asm_constants, AsmErrorCodes};

#[derive(AsmErrorCodes)]
#[repr(u64)]
pub enum ErrorCodes {
    /// An invalid number of accounts were passed.
    NAccounts,
    /// The user account has nonzero data length.
    UserData,
}

asm_constants! {
    /// Memory map.
    pub mod memory_map {
        /// Number of accounts expected.
        N_ACCOUNTS = 2,
    }
}
