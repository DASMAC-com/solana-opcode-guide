extern crate alloc;

use crate::bindings;
use crate::common::{cpi, CreateAccountInstructionData, InitInputBuffer, InputBufferHeader};
use macros::extend_constant_group;
use pinocchio::{entrypoint::NON_DUP_MARKER, sysvars::rent::Rent, Address};

extend_constant_group!(input_buffer {
    prefix = "IB",
    /// User address field.
    offset!(USER_ADDRESS, InputBufferHeader.user.header.address),
    /// User data length field.
    offset!(USER_DATA_LEN, InputBufferHeader.user.header.data_len),
    /// Non-duplicate marker value.
    NON_DUP_MARKER = NON_DUP_MARKER,
    /// Tree non-duplicate marker field.
    offset!(TREE_NON_DUP_MARKER, InputBufferHeader.tree_header.borrow_state),
    /// Tree data length field.
    offset!(TREE_DATA_LEN, InputBufferHeader.tree_header.data_len),
    /// Instruction data length field for empty tree account.
    offset!(INIT_INSTRUCTION_DATA_LEN, InitInputBuffer.instruction_data_len),
    /// Program ID field for initialize instruction.
    offset!(INIT_PROGRAM_ID, InitInputBuffer.program_id),
    /// System Program non-duplicate marker field.
    offset!(SYSTEM_PROGRAM_NON_DUP_MARKER, InitInputBuffer.system_program.header.borrow_state),
    /// System Program data length field.
    offset!(SYSTEM_PROGRAM_DATA_LEN, InitInputBuffer.system_program.header.data_len),
});

extend_constant_group!(misc {
    /// And mask for data length alignment.
    DATA_LEN_AND_MASK = -8,
    /// Maximum possible data length padding.
    MAX_DATA_PAD = 7,
});

#[repr(C)]
struct InitStackFrame {
    /// Zero-initialized on stack.
    system_program_address: Address,
    instruction: bindings::SolInstruction,
    account_metas: [bindings::SolAccountMeta; cpi::N_ACCOUNTS],
    account_infos: [bindings::SolAccountInfo; cpi::N_ACCOUNTS],
    signers_seeds: [bindings::SolSignerSeeds; cpi::N_PDA_SIGNERS],
    signer_seeds: [bindings::SolSignerSeed; cpi::N_SEEDS],
    pda: Address,
    rent: Rent,
    instruction_data: CreateAccountInstructionData,
    bump_seed: u8,
}
