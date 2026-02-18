use core::mem::size_of;
use error_codes::error;
use macros::{array_fields, constant_group, error_codes};
use pinocchio::{
    account::{RuntimeAccount as RuntimeAccountHeader, MAX_PERMITTED_DATA_INCREASE},
    sysvars::rent::{Rent, ACCOUNT_STORAGE_OVERHEAD},
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
    RENT_ADDRESS,
    /// Instruction data provided during initialization instruction.
    INSTRUCTION_DATA,
    /// The passed PDA does not match the expected address.
    PDA_MISMATCH,
    /// Invalid instruction discriminator.
    INSTRUCTION_DISCRIMINATOR,
    /// Invalid instruction data length.
    INSTRUCTION_DATA_LEN,
    /// Not enough accounts passed for insertion allocation.
    N_ACCOUNTS_INSERT_ALLOCATION,
    /// Key already exists in tree during insertion.
    KEY_EXISTS,
}

constant_group! {
    /// Input buffer layout.
    input_buffer {
    /// Number of accounts field.
        offset!(N_ACCOUNTS, InputBufferHeader.n_accounts),
        /// User runtime account.
        offset!(USER_ACCOUNT, InputBufferHeader.user),
        /// User Lamports field.
        offset!(USER_LAMPORTS, InputBufferHeader.user.header.lamports),
        /// User data field.
        offset!(USER_DATA, InputBufferHeader.user.data),
        /// User owner field.
        offset!(USER_OWNER, InputBufferHeader.user.header.owner),
        /// Tree Lamports field.
        offset!(TREE_LAMPORTS, InputBufferHeader.tree_header.lamports),
        /// Tree data field.
        offset!(TREE_DATA, InitInputBuffer.header.tree.data),
        /// Tree owner field.
        offset!(TREE_OWNER, InputBufferHeader.tree_header.owner),
        /// Tree runtime account header.
        offset!(TREE_ACCOUNT, InputBufferHeader.tree_header),
        /// Tree address field.
        offset!(TREE_ADDRESS, InputBufferHeader.tree_header.address),
        /// System Program runtime account header.
        offset!(SYSTEM_PROGRAM_ACCOUNT, InitInputBuffer.header.system_program),
        /// Rent sysvar account header.
        offset!(RENT_ACCOUNT, InitInputBuffer.header.rent),
        /// Rent sysvar account data.
        offset!(RENT_DATA, InitInputBuffer.header.rent.data),
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
        /// User and tree accounts.
        N_ACCOUNTS: usize = 2,
        /// The tree account is a PDA for CreateAccount CPI.
        N_PDA_SIGNERS: usize = 1,
        /// Number of seeds for CreateAccount PDA signer (bump only).
        N_SEEDS_CREATE_ACCOUNT: usize = 1,
        /// PDA signers for Transfer CPI (none — user is already a signer).
        N_PDA_SIGNERS_TRANSFER: u64 = 0,
        /// Number of seeds for PDA generation.
        N_SEEDS_TRY_FIND_PDA: u64 = 0,
        /// Tree account data length.
        TREE_DATA_LEN: usize = size_of::<TreeHeader>(),
        /// Account data scalar for base rent calculation.
        ACCOUNT_DATA_SCALAR: usize = (ACCOUNT_STORAGE_OVERHEAD as usize) + TREE_DATA_LEN,
        /// CreateAccount discriminator for CPI.
        CREATE_ACCOUNT_DISCRIMINATOR: u32 = 0,
        /// Length of CreateAccount instruction data.
        CREATE_ACCOUNT_INSN_DATA_LEN: usize = size_of::<CreateAccountInstructionData>(),
        /// Transfer discriminator for CPI.
        TRANSFER_DISCRIMINATOR: u32 = 2,
        /// Length of Transfer instruction data.
        TRANSFER_INSN_DATA_LEN: usize = size_of::<TransferInstructionData>(),
        /// Mask for writable signer.
        WRITABLE_SIGNER: u64 = 0x0101,
        /// Account index for user account in CPI.
        USER_ACCOUNT_INDEX: usize = 0,
        /// Account index for tree account in CPI.
        TREE_ACCOUNT_INDEX: usize = 1,
        /// Null rent epoch.
        RENT_EPOCH_NULL: u64 = 0,
    }
}

#[repr(C, packed)]
/// For CPI to create tree account.
pub struct CreateAccountInstructionData {
    pub discriminator: u32,
    pub lamports: u64,
    pub space: u64,
    pub owner: Address,
}

#[repr(C, packed)]
/// For CPI to transfer lamports.
pub struct TransferInstructionData {
    pub discriminator: u32,
    pub lamports: u64,
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
    pub tree: EmptyRuntimeAccount,
    pub system_program: SystemProgramRuntimeAccount,
    pub rent: RentRuntimeAccount,
}

#[repr(C, packed)]
/// Input buffer header for general tree instructions.
pub struct GeneralInputBufferHeader {
    pub n_accounts: u64,
    pub user: EmptyRuntimeAccount,
    pub tree_header: RuntimeAccountHeader,
    pub tree_data: TreeHeader,
}

// ANCHOR: tree-defs-common
#[repr(u8)]
#[derive(PartialEq)]
pub enum Color {
    Black,
    Red,
}

#[repr(usize)]
pub enum Direction {
    Left,
    Right,
}

constant_group! {
    /// Tree constants.
    tree {
        /// Max number of children per node.
        N_CHILDREN: usize = 2,
        /// Left direction.
        DIR_L = Direction::Left as usize,
        /// Right direction.
        DIR_R = Direction::Right as usize,
        /// Black color.
        COLOR_B = Color::Black as u8,
        /// Red color.
        COLOR_R = Color::Red as u8,
        /// Stack top field in header.
        offset!(HEADER_TOP, TreeHeader.top),
        /// Next node field in header.
        offset!(HEADER_NEXT, TreeHeader.next),
    }
}

#[repr(C, packed)]
/// Tree account data header. Contains pointer to tree root and top of free node stack.
pub struct TreeHeader {
    /// Aboslute pointer to tree root in memory map.
    pub root: *mut TreeNode,
    /// Absolute pointer to stack top in memory map.
    pub top: *mut StackNode,
    /// Absolute pointer to where the next node should be allocated in memory map.
    pub next: *mut TreeNode,
}

#[array_fields]
#[repr(C, packed)]
pub struct TreeNode {
    pub parent: *mut TreeNode,
    pub child: [*mut TreeNode; tree::N_CHILDREN],
    pub key: u16,
    pub value: u16,
    pub color: Color,
}

#[repr(C, packed)]
pub struct StackNode {
    pub next: *mut StackNode,
}
// ANCHOR_END: tree-defs-common

// ANCHOR: instructions
#[repr(u8)]
pub enum Instruction {
    /// Initialize the tree.
    Initialize,
    /// Insert key-value pair.
    Insert,
    /// Remove key-value pair.
    Remove,
}

#[repr(C, packed)]
pub struct InstructionHeader {
    pub discriminator: u8,
}

#[repr(C, packed)]
pub struct InitializeInstruction {
    pub header: InstructionHeader,
}

#[repr(C, packed)]
pub struct InsertInstruction {
    pub header: InstructionHeader,
    pub key: u16,
    pub value: u16,
}

#[repr(C, packed)]
pub struct RemoveInstruction {
    pub header: InstructionHeader,
    pub key: u16,
}

#[repr(C, packed)]
/// Value in r0.
pub struct RemoveReturn {
    value: u16,
    status: u16,
}

constant_group! {
    /// Offsets for instruction processing.
    instruction {
        prefix = "INSN",
        /// Offset to instruction discriminator byte.
        offset!(DISCRIMINATOR, InstructionHeader.discriminator),
        /// Initialize instruction discriminator.
        DISCRIMINATOR_INITIALIZE: u8 = Instruction::Initialize as u8,
        /// Insert instruction discriminator.
        DISCRIMINATOR_INSERT: u8 = Instruction::Insert as u8,
        /// Remove instruction discriminator.
        DISCRIMINATOR_REMOVE: u8 = Instruction::Remove as u8,
        /// Key field in insert instruction.
        offset!(INSERT_KEY, InsertInstruction.key),
        /// Value field in insert instruction.
        offset!(INSERT_VALUE, InsertInstruction.value),
        /// Key field in remove instruction.
        offset!(REMOVE_KEY, RemoveInstruction.key),
        /// Status value for successful remove (first non-error code).
        REMOVE_STATUS_OK: u16 = error::N_CODES as u16,
    }
}

// ANCHOR_END: instructions

#[repr(C, packed)]
pub struct InitInputBufferFooter {
    pub instruction_data_len: u64,
    pub instruction: InitializeInstruction,
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
type RentRuntimeAccount = RuntimeAccount<{ runtime_data_size(input_buffer::RENT_DATA_LEN) }>;

/// Compute the data buffer size for a runtime account with the given data length.
const fn runtime_data_size(data_len: usize) -> usize {
    MAX_PERMITTED_DATA_INCREASE + data_len.next_multiple_of(data::BPF_ALIGN_OF_U128)
}
