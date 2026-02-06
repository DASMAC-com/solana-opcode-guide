extern crate alloc;

use crate::bindings;
use crate::common::{cpi, CreateAccountInstructionData, InitInputBuffer, InputBufferHeader};
use macros::{asm_constant_group, extend_constant_group, stack_frame};
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
    /// Foo.
    size_of!(u8),
});

#[stack_frame]
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
    /// Padding for alignment.
    _pad: [u8; 4],
    bump_seed: u8,
}

asm_constant_group! {
    /// Init stack frame layout.
    init_stack_frame {
        prefix = "SF",
        align = misc::BPF_ALIGN_OF_U128,
        /// System program address.
        stack_frame_offset!(SYSTEM_PROGRAM_ADDRESS, InitStackFrame.system_program_address),
        /// CPI instruction.
        stack_frame_offset!(INSTRUCTION, InitStackFrame.instruction),
        /// First account meta.
        stack_frame_offset!(ACCOUNT_META_0, InitStackFrame.account_metas[0]),
        /// Second account meta.
        stack_frame_offset!(ACCOUNT_META_1, InitStackFrame.account_metas[1]),
        /// First account info.
        stack_frame_offset!(ACCOUNT_INFO_0, InitStackFrame.account_infos[0]),
        /// Second account info.
        stack_frame_offset!(ACCOUNT_INFO_1, InitStackFrame.account_infos[1]),
        /// Signer seeds array.
        stack_frame_offset!(SIGNERS_SEEDS, InitStackFrame.signers_seeds),
        /// Signer seed entry.
        stack_frame_offset!(SIGNER_SEEDS, InitStackFrame.signer_seeds),
        /// PDA address.
        stack_frame_offset!(PDA, InitStackFrame.pda),
        /// Rent sysvar.
        stack_frame_offset!(RENT, InitStackFrame.rent),
        /// Instruction data.
        stack_frame_offset!(INSTRUCTION_DATA, InitStackFrame.instruction_data),
        /// Bump seed.
        stack_frame_offset!(BUMP_SEED, InitStackFrame.bump_seed),
    }
}
