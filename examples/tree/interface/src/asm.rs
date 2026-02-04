extern crate alloc;

use macros::{asm_constant_group, extend_constant_group};
use pinocchio::{
    account::{RuntimeAccount, MAX_PERMITTED_DATA_INCREASE},
    entrypoint::NON_DUP_MARKER,
};

extend_constant_group!(input_buffer {
    prefix = "IB",
    /// Number of accounts field.
    offset!(N_ACCOUNTS, InputBuffer.n_accounts),
    /// User address field.
    offset!(USER_ADDRESS, InputBuffer.user.header.address),
    /// User data length field.
    offset!(USER_DATA_LEN, InputBuffer.user.header.data_len),
    /// Non-duplicate marker value.
    NON_DUP_MARKER = NON_DUP_MARKER,
    /// Tree non-duplicate marker field.
    offset!(TREE_NON_DUP_MARKER, InputBuffer.tree_header.borrow_state),
    /// Tree data length field.
    offset!(TREE_DATA_LEN, InputBuffer.tree_header.data_len),
});

asm_constant_group! {
    /// Miscellaneous constants.
    misc {
        /// Data length of zero.
        DATA_LEN_ZERO = 0,
        /// And mask for data length alignment.
        DATA_LEN_AND_MASK = -8,
        /// Maximum possible data length padding.
        MAX_DATA_PAD = 7,
    }
}

#[repr(C, packed)]
struct EmptyRuntimeAccount {
    header: RuntimeAccount,
    data: [u8; MAX_PERMITTED_DATA_INCREASE],
    rent_epoch: u64,
}

#[repr(C, packed)]
struct InputBuffer {
    n_accounts: u64,
    user: EmptyRuntimeAccount,
    tree_header: RuntimeAccount,
}
