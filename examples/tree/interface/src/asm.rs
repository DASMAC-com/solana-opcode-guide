extern crate alloc;

use macros::{asm_constant_group, extend_constant_group};
use pinocchio::Address;

extend_constant_group!(input_buffer {
    prefix = "IB",
    /// Number of accounts field.
    offset!(N_ACCOUNTS, InputBuffer.n_accounts),
    /// User data length field.
    offset!(USER_DATA_LEN, InputBuffer.user.header.data_len),
    /// Non-duplicate marker value.
    NON_DUP_MARKER = 0xff,
    /// Tree non-duplicate marker field.
    offset!(TREE_NON_DUP_MARKER, InputBuffer.tree_header.non_dup_marker),
});

asm_constant_group! {
    /// Miscellaneous constants.
    misc {
        /// Data length of zero.
        DATA_LENGTH_ZERO = 0,
    }
}

#[repr(C, packed)]
struct InputAccountHeader {
    non_dup_marker: u8,
    is_signer: bool,
    is_writable: bool,
    is_executable: bool,
    original_data_len: u32,
    pubkey: Address,
    owner: Address,
    lamports: u64,
    data_len: u64,
}

#[repr(C, packed)]
struct EmptyInputAccount {
    header: InputAccountHeader,
    rent_epoch: u64,
}

#[repr(C, packed)]
struct InputBuffer {
    n_accounts: u64,
    user: EmptyInputAccount,
    tree_header: InputAccountHeader,
}
