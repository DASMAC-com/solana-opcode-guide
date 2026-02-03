use build_macros::{constant_group, error_codes};

#[error_codes]
pub enum Error {
    /// An invalid number of accounts were passed.
    NAccounts,
    /// The user account has nonzero data length.
    UserData,
}

constant_group! {
    /// Memory map layout.
    memory_map {
        /// Number of accounts expected.
        N_ACCOUNTS: u64 = 2,
    }
}
