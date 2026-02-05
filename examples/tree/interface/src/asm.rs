extern crate alloc;

use crate::bindings::*;
use crate::common::*;
use macros::{asm_constant_group, extend_constant_group};
use pinocchio::{
    account::{RuntimeAccount, MAX_PERMITTED_DATA_INCREASE},
    entrypoint::NON_DUP_MARKER,
    sysvars::rent::Rent,
    Address,
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
    /// Instruction data length field for empty tree account.
    offset!(PACKED_INSTRUCTION_DATA_LEN, PackedInputBuffer.instruction_data_len),
    /// Program ID field for empty tree account.
    offset!(PACKED_PROGRAM_ID, PackedInputBuffer.program_id),
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

#[repr(C, packed)]
/// Input buffer for empty tree account and no instruction data (during initialization).
struct PackedInputBuffer {
    n_accounts: u64,
    user: EmptyRuntimeAccount,
    tree: EmptyRuntimeAccount,
    instruction_data_len: u64,
    instruction_data: [u8; 0],
    program_id: Address,
}

/// User and tree accounts must sign CPI.
const CPI_N_ACCOUNTS: usize = 2;
/// The tree account is a PDA.
const CPI_N_PDA_SIGNERS: usize = 1;
/// The bump seed is required for tree PDA signer.
const CPI_N_SEEDS: usize = 1;

#[repr(C)]
struct InitStackFrame {
    /// Zero-initialized on stack.
    system_program_address: Address,
    instruction: SolInstruction,
    account_metas: [SolAccountMeta; CPI_N_ACCOUNTS],
    account_infos: [SolAccountInfo; CPI_N_ACCOUNTS],
    signers_seeds: [SolSignerSeeds; CPI_N_PDA_SIGNERS],
    signer_seeds: [SolSignerSeed; CPI_N_SEEDS],
    pda: Address,
    rent: Rent,
    instruction_data: CreateAccountInstructionData,
    bump_seed: u8,
}
