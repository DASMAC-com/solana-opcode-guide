/// Generated from Agave using bindgen.
use pinocchio::Address;

/// SolInstruction from cpi.h.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SolInstruction {
    /// Pubkey of the instruction processor that executes this instruction.
    pub program_id: *mut Address,
    /// Metadata for what accounts should be passed to the instruction processor.
    pub accounts: *mut SolAccountMeta,
    /// Number of SolAccountMetas.
    pub account_len: u64,
    /// Opaque data passed to the instruction processor.
    pub data: *mut u8,
    /// Length of the data in bytes.
    pub data_len: u64,
}

/// SolAccountMeta from cpi.h.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SolAccountMeta {
    /// An account's public key.
    pub pubkey: *mut Address,
    /// True if the `pubkey` can be loaded as a read-write account.
    pub is_writable: bool,
    /// True if an Instruction requires a Transaction signature matching `pubkey`.
    pub is_signer: bool,
}

/// SolAccountInfo from entrypoint.h.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SolAccountInfo {
    /// Public key of the account.
    pub key: *mut Address,
    /// Number of lamports owned by this account.
    pub lamports: *mut u64,
    /// Length of data in bytes.
    pub data_len: u64,
    /// On-chain data within this account.
    pub data: *mut u8,
    /// Program that owns this account.
    pub owner: *mut Address,
    /// The epoch at which this account will next owe rent.
    pub rent_epoch: u64,
    /// Transaction was signed by this account's key?
    pub is_signer: bool,
    /// Is the account writable?
    pub is_writable: bool,
    /// This account's data contains a loaded program (and is now read-only).
    pub executable: bool,
}

/// SolSignerSeed from pubkey.h.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SolSignerSeed {
    /// Seed bytes.
    pub addr: *const u8,
    /// Length of the seed bytes.
    pub len: u64,
}

/// SolSignerSeeds from pubkey.h.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SolSignerSeeds {
    /// An array of a signer's seeds.
    pub addr: *const SolSignerSeed,
    /// Number of seeds.
    pub len: u64,
}
