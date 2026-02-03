use pinocchio::Address;

struct InputBuffer {
    n_accounts: u64,
    user: InputAccount<0>,
    tree: InputAccount<0>,
}

#[repr(C)]
struct InputAccount<const DATA_PADDED_LEN: usize> {
    non_dup_marker: u8,
    is_signer: u8,
    is_writable: u8,
    is_executable: u8,
    original_data_len: [u8; 4],
    pubkey: [u8; size_of::<Address>()],
    owner: [u8; size_of::<Address>()],
    lamports: u64,
    data_len: u64,
    data_padded: [u8; DATA_PADDED_LEN],
    rent_epoch: u64,
}
