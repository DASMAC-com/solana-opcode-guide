extern crate alloc;

use crate::bindings::{
    SolAccountInfo, SolAccountMeta, SolInstruction, SolSignerSeed, SolSignerSeeds,
};
use crate::common::{cpi, CreateAccountInstructionData, InitInputBuffer, InputBufferHeader};
use macros::{asm_constant_group, extend_constant_group, pubkey_chunk_group, sizes, stack_frame};
use pinocchio::{
    entrypoint::NON_DUP_MARKER,
    sysvars::rent::{Rent, RENT_ID},
    Address,
};

pubkey_chunk_group!();

sizes! {
    u8,
    u64,
}

extend_constant_group!(data {
    /// No offset.
    OFFSET_ZERO = 0,
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
    /// Tree address field.
    pubkey_offset!(TREE_ADDRESS, InputBufferHeader.tree_header.address),
    /// Tree data length field.
    offset!(TREE_DATA_LEN, InputBufferHeader.tree_header.data_len),
    /// System Program non-duplicate marker field.
    offset!(
        SYSTEM_PROGRAM_NON_DUP_MARKER,
        InitInputBuffer.header.system_program.header.borrow_state
    ),
    /// System Program data length field.
    offset!(SYSTEM_PROGRAM_DATA_LEN, InitInputBuffer.header.system_program.header.data_len),
    /// Rent account non-duplicate marker field.
    offset!(RENT_NON_DUP_MARKER, InitInputBuffer.header.rent.header.borrow_state),
    /// Rent address field.
    pubkey_offset!(RENT_ADDRESS, InitInputBuffer.header.rent.header.address),
    /// Rent sysvar ID.
    pubkey_value!(RENT_ID, RENT_ID),
    /// Program ID field for initialize instruction.
    offset_immediate!(INIT_PROGRAM_ID, InitInputBuffer.footer.program_id),
});

#[stack_frame]
struct InitStackFrame {
    bump_seed: u8,
    /// Zero-initialized on stack.
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
        /// Bump signer seed address field.
        stack_frame_offset!(SIGNER_SEED_ADDR, InitStackFrame.signer_seeds[0].addr),
        /// Bump signer seed length field.
        stack_frame_offset!(SIGNER_SEED_LEN, InitStackFrame.signer_seeds[0].len),
        /// PDA address field.
        stack_frame_offset!(PDA, InitStackFrame.pda),
        /// Lamports field in CreateAccount instruction data.
        stack_frame_offset_unaligned!(
            CREATE_ACCOUNT_LAMPORTS,
            InitStackFrame.instruction_data.lamports
        ),
        /// Space address field in CreateAccount instruction data.
        stack_frame_offset_unaligned!(CREATE_ACCOUNT_SPACE, InitStackFrame.instruction_data.space),
        /// Owner field in CreateAccount instruction data.
        stack_frame_pubkey_offset_unaligned!(
            CREATE_ACCOUNT_OWNER,
            InitStackFrame.instruction_data.owner
        ),
    }
}
