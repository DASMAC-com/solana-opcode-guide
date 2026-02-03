use macros::{constant_group, error_codes};

error_codes! {
    /// An invalid number of accounts were passed.
    N_ACCOUNTS_INVALID,
    /// The user account has nonzero data length.
    USER_HAS_DATA,
}

constant_group! {
    /// Input buffer layout.
    input_buffer {
        /// Number of accounts expected.
        N_ACCOUNTS: u64 = 2,
    }
}
