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
    Address,
    u128,
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
    /// Relative offset from user data field to tree pubkey field.
    relative_offset_immediate!(
        USER_DATA,
        TREE_ADDRESS,
        InputBufferHeader.user.data,
        InputBufferHeader.tree_header.address
    ),
});

#[stack_frame]
struct InitStackFrame {
    bump_seed: u8,
    instruction_data: CreateAccountInstructionData,
    instruction: SolInstruction,
    account_metas: [SolAccountMeta; cpi::N_ACCOUNTS],
    account_infos: [SolAccountInfo; cpi::N_ACCOUNTS],
    signers_seeds: [SolSignerSeeds; cpi::N_PDA_SIGNERS],
    signer_seeds: [SolSignerSeed; cpi::N_SEEDS],
    pda: Address,
    rent: Rent,
    /// Zero-initialized on stack.
    system_program_address: Address,
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
        /// Signers seeds address field.
        stack_frame_offset!(SIGNERS_SEEDS_ADDR, InitStackFrame.signers_seeds),
        /// Signers seeds length field.
        stack_frame_offset!(SIGNERS_SEEDS_LEN, InitStackFrame.signers_seeds[0].len),
        /// System Program address.
        stack_frame_offset!(SYSTEM_PROGRAM_ADDRESS, InitStackFrame.system_program_address),
        /// SolInstruction program_id field.
        stack_frame_offset!(INSN_PROGRAM_ID, InitStackFrame.instruction.program_id),
        /// SolInstruction accounts field.
        stack_frame_offset!(INSN_ACCOUNTS, InitStackFrame.instruction.accounts),
        /// SolInstruction account_len field.
        stack_frame_offset!(INSN_ACCOUNT_LEN, InitStackFrame.instruction.account_len),
        /// SolInstruction data field.
        stack_frame_offset!(INSN_DATA, InitStackFrame.instruction.data),
        /// SolInstruction data_len field.
        stack_frame_offset!(INSN_DATA_LEN, InitStackFrame.instruction.data_len),
        /// SolAccountMeta is_writable field for user account.
        stack_frame_offset!(
            USER_META_IS_WRITABLE,
            InitStackFrame.account_metas[cpi::USER_ACCOUNT_INDEX].is_writable
        ),
        /// SolAccountMeta is_writable field for tree account.
        stack_frame_offset!(
            TREE_META_IS_WRITABLE,
            InitStackFrame.account_metas[cpi::TREE_ACCOUNT_INDEX].is_writable
        ),
        /// SolAccountInfo is_signer field for user account.
        stack_frame_offset!(
            USER_INFO_IS_SIGNER,
            InitStackFrame.account_infos[cpi::USER_ACCOUNT_INDEX].is_signer
        ),
        /// SolAccountMeta pubkey field for user account.
        stack_frame_offset!(
            USER_META_PUBKEY,
            InitStackFrame.account_metas[cpi::USER_ACCOUNT_INDEX].pubkey
        ),
        /// SolAccountInfo pubkey field for user account.
        stack_frame_offset!(
            USER_INFO_PUBKEY,
            InitStackFrame.account_infos[cpi::USER_ACCOUNT_INDEX].key
        ),
        /// SolAccountInfo owner field for user account.
        stack_frame_offset!(
            USER_INFO_OWNER,
            InitStackFrame.account_infos[cpi::USER_ACCOUNT_INDEX].owner
        ),
        /// SolAccountInfo lamports field for user account.
        stack_frame_offset!(
            USER_INFO_LAMPORTS,
            InitStackFrame.account_infos[cpi::USER_ACCOUNT_INDEX].lamports
        ),
        /// SolAccountInfo data_len field for user account.
        stack_frame_offset!(
            USER_INFO_DATA,
            InitStackFrame.account_infos[cpi::USER_ACCOUNT_INDEX].data
        ),
        /// SolAccountInfo is_signer field for tree account.
        stack_frame_offset!(
            TREE_INFO_IS_SIGNER,
            InitStackFrame.account_infos[cpi::TREE_ACCOUNT_INDEX].is_signer
        ),
        /// SolAccountMeta pubkey field for tree account.
        stack_frame_offset!(
            TREE_META_PUBKEY,
            InitStackFrame.account_metas[cpi::TREE_ACCOUNT_INDEX].pubkey
        ),
        /// SolAccountInfo pubkey field for tree account.
        stack_frame_offset!(
            TREE_INFO_PUBKEY,
            InitStackFrame.account_infos[cpi::TREE_ACCOUNT_INDEX].key
        ),
        /// SolAccountInfo owner field for tree account.
        stack_frame_offset!(
            TREE_INFO_OWNER,
            InitStackFrame.account_infos[cpi::TREE_ACCOUNT_INDEX].owner
        ),
        /// SolAccountInfo lamports field for tree account.
        stack_frame_offset!(
            TREE_INFO_LAMPORTS,
            InitStackFrame.account_infos[cpi::TREE_ACCOUNT_INDEX].lamports
        ),
        /// SolAccountInfo data_len field for tree account.
        stack_frame_offset!(
            TREE_INFO_DATA,
            InitStackFrame.account_infos[cpi::TREE_ACCOUNT_INDEX].data
        ),
        /// Relative offset from PDA on stack to System Program ID.
        relative_offset_immediate!(
            PDA,
            SYSTEM_PROGRAM_ID,
            InitStackFrame.pda,
            InitStackFrame.system_program_address
        ),
        /// Relative offset from System Program ID to first SolAccountMeta.
        relative_offset_immediate!(
            SYSTEM_PROGRAM_ID,
            ACCT_METAS,
            InitStackFrame.system_program_address,
            InitStackFrame.account_metas
        ),
        /// Relative offset from SolAccountMeta array to instruction data.
        relative_offset_immediate!(
            ACCT_METAS,
            INSN_DATA,
            InitStackFrame.account_metas,
            InitStackFrame.instruction_data
        ),
        /// Relative offset from instruction data to signer seeds.
        relative_offset_immediate!(
            INSN_DATA,
            SIGNER_SEEDS,
            InitStackFrame.instruction_data,
            InitStackFrame.signer_seeds
        ),
        /// Relative offset from signer seeds to signers seeds.
        relative_offset_immediate!(
            SIGNER_SEEDS,
            SIGNERS_SEEDS,
            InitStackFrame.signer_seeds,
            InitStackFrame.signers_seeds
        ),
        /// Account infos array.
        stack_frame_offset!(ACCT_INFOS, InitStackFrame.account_infos),
    }
}
