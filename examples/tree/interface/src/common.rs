use macros::{constant_group, error_codes};
use pinocchio::Address;

error_codes! {
    /// An invalid number of accounts were passed.
    N_ACCOUNTS,
    /// The user account has invalid data length.
    USER_DATA_LEN,
    /// The tree account has invalid data length.
    TREE_DATA_LEN,
    /// The System Program account has invalid data length.
    SYSTEM_PROGRAM_DATA_LEN,
    /// The tree account is a duplicate.
    TREE_DUPLICATE,
    /// The System Program account is a duplicate.
    SYSTEM_PROGRAM_DUPLICATE,
    /// The passed PDA does not match the expected address.
    PDA_MISMATCH,
}

constant_group! {
    /// Input buffer layout.
    input_buffer {
        /// Expected number of accounts for general instructions.
        N_ACCOUNTS_GENERAL: u64 = 2,
        /// Expected number of accounts for tree initialization.
        N_ACCOUNTS_INIT: u64 = 3,
        /// Expected data length of system program account.
        SYSTEM_PROGRAM_DATA_LEN: usize = b"system_program".len(),
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
