use macros::{constant_group, error_codes};
use pinocchio::Address;

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

/// Value in r0.
#[repr(C, packed)]
struct Return {
    /// If a value is retrieved from the tree, it's encoded in high bits.
    maybe_value: u16,
    /// Nonzero iff error.
    status: u16,
}

#[repr(C, packed)]
/// For CPI to create tree account.
pub struct CreateAccountInstructionData {
    instruction_tag: u32,
    lamports: u64,
    space: u64,
    owner: Address,
}
