#![no_std]

extern crate alloc;
use build_macros::AsmErrorCodes;

#[cfg(feature = "std")]
mod injection;
#[cfg(feature = "std")]
pub use injection::inject_asm;
#[cfg(feature = "std")]
extern crate std;

#[derive(AsmErrorCodes)]
#[repr(u64)]
pub enum ErrorCodes {
    /// An invalid number of accounts were passed.
    NAccounts,
    /// The user account has nonzero data length.
    UserData,
}
