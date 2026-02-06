extern crate alloc;

use crate::bindings::{
    SolAccountInfo, SolAccountMeta, SolInstruction, SolSignerSeed, SolSignerSeeds,
};
use crate::common::{
    cpi, CpiAccountIndex, CreateAccountInstructionData, InitInputBuffer, InputBufferHeader,
};
use macros::{asm_constant_group, extend_constant_group, sizes, stack_frame};
use pinocchio::{entrypoint::NON_DUP_MARKER, sysvars::rent::Rent, Address};

sizes! {
    u8,
}

extend_constant_group!(misc {
    /// And mask for data length alignment.
    DATA_LEN_AND_MASK = -8,
    /// Maximum possible data length padding.
    MAX_DATA_PAD = 7,
});

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

#[stack_frame]
struct InitStackFrame {
    /// Zero-initialized on stack.
    bump_seed: u8,
    system_program_address: Address,
    instruction: SolInstruction,
    account_metas: [SolAccountMeta; cpi::N_ACCOUNTS],
    account_infos: [SolAccountInfo; cpi::N_ACCOUNTS],
    signers_seeds: [SolSignerSeeds; cpi::N_PDA_SIGNERS],
    signer_seeds: [SolSignerSeed; cpi::N_SEEDS],
    pda: Address,
    rent: Rent,
    instruction_data: CreateAccountInstructionData,
}

asm_constant_group! {
    /// Init stack frame layout.
    init_stack_frame {
        prefix = "SF_INIT",
        /// Bump seed.
        stack_frame_offset!(BUMP_SEED, InitStackFrame.bump_seed),
    }
}
