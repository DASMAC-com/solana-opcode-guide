use macros::{constant_group, error_codes};

error_codes! {
    /// An invalid number of accounts were passed.
    N_ACCOUNTS,
    /// The user account has nonzero data length.
    USER_DATA_LEN,
    /// The tree account is a duplicate.
    TREE_DUPLICATE,
}

constant_group! {
    /// Input buffer layout.
    input_buffer {
        /// Number of accounts expected.
        N_ACCOUNTS: u64 = 2,
    }
}
