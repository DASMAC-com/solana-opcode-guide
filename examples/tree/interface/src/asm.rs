extern crate alloc;

use macros::extend_constant_group;

use pinocchio::Address;

extend_constant_group!(input_buffer {
    prefix = "IB",
    /// Number of accounts passed in input.
    N_ACCOUNTS_OFF = 0,
});

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
