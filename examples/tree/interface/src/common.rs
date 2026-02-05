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
        /// Expected number of accounts.
        N_ACCOUNTS: u64 = 2,
    }
}

struct InitInstructionData {}

struct GetInstructionData {
    key: u16,
}

struct InsertInstructionData {
    key: u16,
    value: u16,
}

struct RemoveInstructionData {
    key: u16,
}

/// Value in r0.
#[repr(C, packed)]
struct Return {
    /// If a value is removed from the tree, it's placed here.
    maybe_value: u16,
    /// 0 for success, nonzero for error.
    status: u16,
}
