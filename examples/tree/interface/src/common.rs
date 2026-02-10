use core::mem::size_of;
use macros::{constant_group, error_codes};
use pinocchio::{
    account::{RuntimeAccount as RuntimeAccountHeader, MAX_PERMITTED_DATA_INCREASE},
    sysvars::rent::Rent,
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
    /// The rent sysvar account is a duplicate.
    RENT_DUPLICATE,
    /// The rent sysvar account has invalid data length.
    RENT_DATA_LEN,
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
        /// User runtime account.
        offset!(USER_ACCOUNT, InputBufferHeader.user),
        /// Tree runtime account header.
        offset!(TREE_ACCOUNT, InputBufferHeader.tree_header),
        /// System Program runtime account header.
        offset!(SYSTEM_PROGRAM_ACCOUNT, InitInputBuffer.header.system_program),
        /// Rent sysvar account header, in footer.
        offset!(RENT_ACCOUNT, InitInputBuffer.header.rent),
        /// Expected number of accounts for general instructions.
        N_ACCOUNTS_GENERAL: u64 = 2,
        /// Expected number of accounts for tree initialization.
        N_ACCOUNTS_INIT: u64 = 4,
        /// Expected data length of system program account.
        SYSTEM_PROGRAM_DATA_LEN: usize = b"system_program".len(),
        /// Expected data length of rent sysvar account.
        // Includes extra byte for deprecated burn_percent field that is still present in test
        // framework.
        RENT_DATA_LEN: usize = size_of::<Rent>() + size_of::<u8>(),
    }
}

constant_group! {
    /// CPI-specific constants.
    cpi {
        prefix = "CPI",
        /// User and tree accounts must sign CPI.
        N_ACCOUNTS: usize = 2,
        /// The tree account is a PDA.
        N_PDA_SIGNERS: usize = 1,
        /// The bump seed is required for tree PDA signer.
        N_SEEDS: usize = 1,
        /// Number of seeds for PDA generation.
        N_SEEDS_TRY_FIND_PDA: u64 = 0,
    }
}

#[repr(C, packed)]
/// For CPI to create tree account.
pub struct CreateAccountInstructionData {
    instruction_tag: u32,
    lamports: u64,
    space: u64,
    owner: Address,
}

constant_group! {
    /// Data layout constants.
    data {
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
/// Input buffer for tree initialization instruction. Broken up to fit relative offsets in i16.
pub struct InitInputBuffer {
    pub header: InitInputBufferHeader,
    pub footer: InitInputBufferFooter,
}

#[repr(C, packed)]
pub struct InitInputBufferHeader {
    pub _n_accounts: u64,
    pub _user: EmptyRuntimeAccount,
    pub _tree: EmptyRuntimeAccount,
    pub system_program: SystemProgramRuntimeAccount,
    pub rent: RentRuntimeAccount,
}

#[repr(C, packed)]
pub struct InitInputBufferFooter {
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

type EmptyRuntimeAccount = RuntimeAccount<{ runtime_data_size(data::DATA_LEN_ZERO as usize) }>;
type SystemProgramRuntimeAccount =
    RuntimeAccount<{ runtime_data_size(input_buffer::SYSTEM_PROGRAM_DATA_LEN) }>;
type RentRuntimeAccount = RuntimeAccount<{ runtime_data_size(size_of::<Rent>()) }>;

/// Compute the data buffer size for a runtime account with the given data length.
const fn runtime_data_size(data_len: usize) -> usize {
    MAX_PERMITTED_DATA_INCREASE + data_len.next_multiple_of(data::BPF_ALIGN_OF_U128)
}
