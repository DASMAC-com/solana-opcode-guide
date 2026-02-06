use macros::{constant_group, error_codes};
use pinocchio::{
    account::{RuntimeAccount as RuntimeAccountHeader, MAX_PERMITTED_DATA_INCREASE},
    Address,
};

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
    /// Instruction data provided during initialization instruction.
    INSTRUCTION_DATA,
    /// The passed PDA does not match the expected address.
    PDA_MISMATCH,
}

constant_group! {
    /// Input buffer layout.
    input_buffer {
        /// Number of accounts field.
        offset!(N_ACCOUNTS, InputBufferHeader.n_accounts),
        /// Expected number of accounts for general instructions.
        N_ACCOUNTS_GENERAL: u64 = 2,
        /// Expected number of accounts for tree initialization.
        N_ACCOUNTS_INIT: u64 = 3,
        /// Expected data length of system program account.
        SYSTEM_PROGRAM_DATA_LEN: usize = b"system_program".len(),
    }
}

constant_group! {
    /// CPI-specific constants.
    cpi {
        /// User and tree accounts must sign CPI.
        CPI_N_ACCOUNTS: usize = 2,
        /// The tree account is a PDA.
        CPI_N_PDA_SIGNERS: usize = 1,
        /// The bump seed is required for tree PDA signer.
        CPI_N_SEEDS: usize = 1,
    }
}

constant_group! {
    /// Miscellaneous constants.
    misc {
        /// Data length of zero.
        DATA_LEN_ZERO: u64 = 0,
        /// Data alignment during runtime.
        BPF_ALIGN_OF_U128: usize = 8,
    }
}

#[repr(C, packed)]
/// Input buffer header for all instructions.
pub struct InputBufferHeader {
    pub n_accounts: u64,
    pub user: EmptyRuntimeAccount,
    pub tree_header: RuntimeAccountHeader,
}

#[repr(C, packed)]
/// Input buffer for tree initialization instruction.
pub struct InitInputBuffer {
    pub n_accounts: u64,
    pub user: EmptyRuntimeAccount,
    pub tree: EmptyRuntimeAccount,
    pub system_program: SystemProgramRuntimeAccount,
    /// No actual instruction data follows.
    pub instruction_data_len: u64,
    pub program_id: Address,
}

#[repr(C)]
pub struct RuntimeAccount<const DATA_SIZE: usize> {
    pub header: RuntimeAccountHeader,
    pub data: [u8; DATA_SIZE],
    pub rent_epoch: u64,
}

type EmptyRuntimeAccount = RuntimeAccount<{ runtime_data_size(misc::DATA_LEN_ZERO as usize) }>;
type SystemProgramRuntimeAccount =
    RuntimeAccount<{ runtime_data_size(input_buffer::SYSTEM_PROGRAM_DATA_LEN) }>;

/// Compute the data buffer size for a runtime account with the given data length.
const fn runtime_data_size(data_len: usize) -> usize {
    MAX_PERMITTED_DATA_INCREASE + data_len.next_multiple_of(misc::BPF_ALIGN_OF_U128)
}
