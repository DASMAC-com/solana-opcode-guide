# ANCHOR: constants
# Error codes.
# ------------
.equ E_N_ACCOUNTS, 1 # An invalid number of accounts were passed.
.equ E_USER_DATA_LEN, 2 # The user account has invalid data length.
.equ E_TREE_DATA_LEN, 3 # The tree account has invalid data length.
# The System Program account has invalid data length.
.equ E_SYSTEM_PROGRAM_DATA_LEN, 4
.equ E_TREE_DUPLICATE, 5 # The tree account is a duplicate.
# The System Program account is a duplicate.
.equ E_SYSTEM_PROGRAM_DUPLICATE, 6
.equ E_RENT_DUPLICATE, 7 # The rent sysvar account is a duplicate.
.equ E_RENT_ADDRESS, 8 # The rent sysvar account has invalid data length.
# Instruction data provided during initialization instruction.
.equ E_INSTRUCTION_DATA, 9
# The passed PDA does not match the expected address.
.equ E_PDA_MISMATCH, 10
.equ E_INSTRUCTION_DISCRIMINATOR, 11 # Invalid instruction discriminator.
.equ E_INSTRUCTION_DATA_LEN, 12 # Invalid instruction data length.
# Not enough accounts passed for insertion allocation.
.equ E_N_ACCOUNTS_INSERT_ALLOCATION, 13

# Type sizes.
# -----------
.equ SIZE_OF_U8, 1 # Size of u8.
.equ SIZE_OF_U64, 8 # Size of u64.
.equ SIZE_OF_ADDRESS, 32 # Size of Address.
.equ SIZE_OF_U128, 16 # Size of u128.
.equ SIZE_OF_TREE_HEADER, 24 # Size of TreeHeader.
.equ SIZE_OF_INITIALIZE_INSTRUCTION, 1 # Size of InitializeInstruction.
.equ SIZE_OF_INSERT_INSTRUCTION, 5 # Size of InsertInstruction.

# Data layout constants.
# ----------------------
.equ DATA_LEN_ZERO, 0 # Data length of zero.
.equ BPF_ALIGN_OF_U128, 8 # Data alignment during runtime.
.equ OFFSET_ZERO, 0 # No offset.
.equ DATA_LEN_AND_MASK, -8 # And mask for data length alignment.
.equ MAX_DATA_PAD, 7 # Maximum possible data length padding.

# Pubkey chunking offsets.
# ------------------------
.equ PUBKEY_CHUNK_OFF_0, 0 # Offset for the first 8 bytes.
.equ PUBKEY_CHUNK_OFF_1, 8 # Offset for the second 8 bytes.
.equ PUBKEY_CHUNK_OFF_2, 16 # Offset for the third 8 bytes.
.equ PUBKEY_CHUNK_OFF_3, 24 # Offset for the fourth 8 bytes.

# Input buffer layout.
# --------------------
.equ IB_N_ACCOUNTS_OFF, 0 # Number of accounts field.
.equ IB_USER_ACCOUNT_OFF, 8 # User runtime account.
.equ IB_USER_LAMPORTS_OFF, 80 # User Lamports field.
.equ IB_USER_DATA_OFF, 96 # User data field.
.equ IB_USER_OWNER_OFF, 48 # User owner field.
.equ IB_TREE_LAMPORTS_OFF, 10416 # Tree Lamports field.
.equ IB_TREE_DATA_OFF, 10432 # Tree data field.
.equ IB_TREE_OWNER_OFF, 10384 # Tree owner field.
.equ IB_TREE_ACCOUNT_OFF, 10344 # Tree runtime account header.
.equ IB_TREE_ADDRESS_OFF, 10352 # Tree address field.
# System Program runtime account header.
.equ IB_SYSTEM_PROGRAM_ACCOUNT_OFF, 20680
.equ IB_RENT_ACCOUNT_OFF, 31032 # Rent sysvar account header.
.equ IB_RENT_DATA_OFF, 31120 # Rent sysvar account data.
# Expected number of accounts for general instructions.
.equ IB_N_ACCOUNTS_GENERAL, 2
# Expected number of accounts for tree initialization.
.equ IB_N_ACCOUNTS_INIT, 4
# Expected data length of system program account.
.equ IB_SYSTEM_PROGRAM_DATA_LEN, 14
.equ IB_RENT_DATA_LEN, 17 # Expected data length of rent sysvar account.
.equ IB_USER_ADDRESS_OFF, 16 # User address field.
.equ IB_USER_DATA_LEN_OFF, 88 # User data length field.
.equ IB_NON_DUP_MARKER, 255 # Non-duplicate marker value.
.equ IB_TREE_NON_DUP_MARKER_OFF, 10344 # Tree non-duplicate marker field.
.equ IB_TREE_ADDRESS_OFF_0, 10352 # Tree address field (chunk index 0).
.equ IB_TREE_ADDRESS_OFF_1, 10360 # Tree address field (chunk index 1).
.equ IB_TREE_ADDRESS_OFF_2, 10368 # Tree address field (chunk index 2).
.equ IB_TREE_ADDRESS_OFF_3, 10376 # Tree address field (chunk index 3).
.equ IB_TREE_DATA_LEN_OFF, 10424 # Tree data length field.
# System Program non-duplicate marker field.
.equ IB_SYSTEM_PROGRAM_NON_DUP_MARKER_OFF, 20680
# System Program data length field.
.equ IB_SYSTEM_PROGRAM_DATA_LEN_OFF, 20760
# Rent account non-duplicate marker field.
.equ IB_RENT_NON_DUP_MARKER_OFF, 31032
.equ IB_RENT_ADDRESS_OFF_0, 31040 # Rent address field (chunk index 0).
.equ IB_RENT_ADDRESS_OFF_1, 31048 # Rent address field (chunk index 1).
.equ IB_RENT_ADDRESS_OFF_2, 31056 # Rent address field (chunk index 2).
.equ IB_RENT_ADDRESS_OFF_3, 31064 # Rent address field (chunk index 3).
.equ IB_RENT_ID_CHUNK_0, 5862609301215225606 # Rent sysvar ID (chunk 0).
.equ IB_RENT_ID_CHUNK_0_LO, 399877894 # Rent sysvar ID (chunk 0 lo).
.equ IB_RENT_ID_CHUNK_0_HI, 1364995097 # Rent sysvar ID (chunk 0 hi).
.equ IB_RENT_ID_CHUNK_1, 9219231539345853473 # Rent sysvar ID (chunk 1).
.equ IB_RENT_ID_CHUNK_1_LO, 1288277025 # Rent sysvar ID (chunk 1 lo).
.equ IB_RENT_ID_CHUNK_1_HI, 2146519613 # Rent sysvar ID (chunk 1 hi).
.equ IB_RENT_ID_CHUNK_2, 4971307250928769624 # Rent sysvar ID (chunk 2).
.equ IB_RENT_ID_CHUNK_2_LO, 149871192 # Rent sysvar ID (chunk 2 lo).
.equ IB_RENT_ID_CHUNK_2_HI, 1157472667 # Rent sysvar ID (chunk 2 hi).
.equ IB_RENT_ID_CHUNK_3, 2329533411 # Rent sysvar ID (chunk 3).
.equ IB_RENT_ID_CHUNK_3_LO, -1965433885 # Rent sysvar ID (chunk 3 lo).
.equ IB_RENT_ID_CHUNK_3_HI, 0 # Rent sysvar ID (chunk 3 hi).
# Program ID field for initialize instruction.
.equ IB_INIT_PROGRAM_ID_OFF_IMM, 41401
# Relative offset from user data field to tree pubkey field.
.equ IB_USER_DATA_TO_TREE_ADDRESS_REL_OFF_IMM, 10256

# Offsets for instruction processing.
# -----------------------------------
.equ INSN_DISCRIMINATOR_OFF, 0 # Offset to instruction discriminator byte.
# Initialize instruction discriminator.
.equ INSN_DISCRIMINATOR_INITIALIZE, 0
.equ INSN_DISCRIMINATOR_INSERT, 1 # Insert instruction discriminator.

# Init stack frame layout.
# ------------------------
.equ SF_INIT_BUMP_SEED_OFF, -352 # Bump seed.
.equ SF_INIT_SIGNER_SEED_ADDR_OFF, -96 # Bump signer seed address field.
.equ SF_INIT_SIGNER_SEED_LEN_OFF, -88 # Bump signer seed length field.
.equ SF_INIT_PDA_OFF, -80 # PDA address field.
# Lamports field in CreateAccount instruction data.
.equ SF_INIT_CREATE_ACCOUNT_LAMPORTS_UOFF, -347
# Space address field in CreateAccount instruction data.
.equ SF_INIT_CREATE_ACCOUNT_SPACE_UOFF, -339
# Owner field in CreateAccount instruction data (chunk index 0).
.equ SF_INIT_CREATE_ACCOUNT_OWNER_UOFF_0, -331
# Owner field in CreateAccount instruction data (chunk index 1).
.equ SF_INIT_CREATE_ACCOUNT_OWNER_UOFF_1, -323
# Owner field in CreateAccount instruction data (chunk index 2).
.equ SF_INIT_CREATE_ACCOUNT_OWNER_UOFF_2, -315
# Owner field in CreateAccount instruction data (chunk index 3).
.equ SF_INIT_CREATE_ACCOUNT_OWNER_UOFF_3, -307
.equ SF_INIT_SIGNERS_SEEDS_ADDR_OFF, -112 # Signers seeds address field.
.equ SF_INIT_SIGNERS_SEEDS_LEN_OFF, -104 # Signers seeds length field.
.equ SF_INIT_SYSTEM_PROGRAM_ADDRESS_OFF, -32 # System Program address.
.equ SF_INIT_INSN_PROGRAM_ID_OFF, -296 # SolInstruction program_id field.
.equ SF_INIT_INSN_ACCOUNTS_OFF, -288 # SolInstruction accounts field.
.equ SF_INIT_INSN_ACCOUNT_LEN_OFF, -280 # SolInstruction account_len field.
.equ SF_INIT_INSN_DATA_OFF, -272 # SolInstruction data field.
.equ SF_INIT_INSN_DATA_LEN_OFF, -264 # SolInstruction data_len field.
# SolAccountMeta is_writable field for user account.
.equ SF_INIT_USER_META_IS_WRITABLE_OFF, -248
# SolAccountMeta is_writable field for tree account.
.equ SF_INIT_TREE_META_IS_WRITABLE_OFF, -232
# SolAccountInfo is_signer field for user account.
.equ SF_INIT_USER_INFO_IS_SIGNER_OFF, -176
# SolAccountMeta pubkey field for user account.
.equ SF_INIT_USER_META_PUBKEY_OFF, -256
# SolAccountInfo pubkey field for user account.
.equ SF_INIT_USER_INFO_PUBKEY_OFF, -224
# SolAccountInfo owner field for user account.
.equ SF_INIT_USER_INFO_OWNER_OFF, -192
# SolAccountInfo lamports field for user account.
.equ SF_INIT_USER_INFO_LAMPORTS_OFF, -216
# SolAccountInfo data_len field for user account.
.equ SF_INIT_USER_INFO_DATA_OFF, -200
# SolAccountInfo is_signer field for tree account.
.equ SF_INIT_TREE_INFO_IS_SIGNER_OFF, -120
# SolAccountMeta pubkey field for tree account.
.equ SF_INIT_TREE_META_PUBKEY_OFF, -240
# SolAccountInfo pubkey field for tree account.
.equ SF_INIT_TREE_INFO_PUBKEY_OFF, -168
# SolAccountInfo owner field for tree account.
.equ SF_INIT_TREE_INFO_OWNER_OFF, -136
# SolAccountInfo lamports field for tree account.
.equ SF_INIT_TREE_INFO_LAMPORTS_OFF, -160
# SolAccountInfo data_len field for tree account.
.equ SF_INIT_TREE_INFO_DATA_OFF, -144
# Relative offset from PDA on stack to System Program ID.
.equ SF_INIT_PDA_TO_SYSTEM_PROGRAM_ID_REL_OFF_IMM, 48
# Relative offset from System Program ID to first SolAccountMeta.
.equ SF_INIT_SYSTEM_PROGRAM_ID_TO_ACCT_METAS_REL_OFF_IMM, -224
# Relative offset from SolAccountMeta array to instruction data.
.equ SF_INIT_ACCT_METAS_TO_INSN_DATA_REL_OFF_IMM, -95
# Relative offset from instruction data to signer seeds.
.equ SF_INIT_INSN_DATA_TO_SIGNER_SEEDS_REL_OFF_IMM, 255
# Relative offset from signer seeds to signers seeds.
.equ SF_INIT_SIGNER_SEEDS_TO_SIGNERS_SEEDS_REL_OFF_IMM, -16
.equ SF_INIT_ACCT_INFOS_OFF, -224 # Account infos array.

# CPI-specific constants.
# -----------------------
.equ CPI_N_ACCOUNTS, 2 # User and tree accounts must sign CPI.
.equ CPI_N_PDA_SIGNERS, 1 # The tree account is a PDA.
.equ CPI_N_SEEDS, 1 # The bump seed is required for tree PDA signer.
.equ CPI_N_SEEDS_TRY_FIND_PDA, 0 # Number of seeds for PDA generation.
.equ CPI_TREE_DATA_LEN, 24 # Tree account data length.
# Account data scalar for base rent calculation.
.equ CPI_ACCOUNT_DATA_SCALAR, 152
# CreateAccount discriminator for CPI.
.equ CPI_CREATE_ACCOUNT_DISCRIMINATOR, 0
.equ CPI_INSN_DATA_LEN, 52 # Length of CreateAccount instruction data.
.equ CPI_WRITABLE_SIGNER, 0x0101 # Mask for writable signer.
.equ CPI_USER_ACCOUNT_INDEX, 0 # Account index for user account in CPI.
.equ CPI_TREE_ACCOUNT_INDEX, 1 # Account index for tree account in CPI.
.equ CPI_RENT_EPOCH_NULL, 0 # Null rent epoch.

# Tree constants.
# ---------------
.equ TREE_N_CHILDREN, 2 # Max number of children per node.
.equ TREE_DIR_L, 0 # Left direction.
.equ TREE_DIR_R, 1 # Right direction.
.equ TREE_COLOR_B, 0 # Black color.
.equ TREE_COLOR_R, 1 # Red color.
.equ TREE_HEADER_NEXT_OFF, 16 # Next node field in header.
.equ TREE_ROOT_OFF, 0 # Tree root.
.equ TREE_TOP_OFF, 8 # Stack top.
.equ TREE_DISCRIMINATOR_INSERT, 1 # Discriminator for insert instruction.
# ANCHOR_END: constants

# ANCHOR: entrypoint-branching
.globl entrypoint

entrypoint:
    # Read instruction data length and discriminator.
    # ---------------------------------------------------------------------
    ldxdw r9, [r2 - SIZE_OF_U64] # Get instruction data length.
    ldxdw r8, [r1 + IB_N_ACCOUNTS_OFF] # Get n input buffer accounts.
    ldxb r7, [r2 + OFFSET_ZERO] # Get discriminator.

    # Jump to branch for given discriminator.
    # ---------------------------------------------------------------------
    jeq r7, INSN_DISCRIMINATOR_INSERT, insert
    jeq r7, INSN_DISCRIMINATOR_INITIALIZE, initialize
    # Error if invalid discriminator provided.
    mov64 r0, E_INSTRUCTION_DISCRIMINATOR
    exit
    # ANCHOR_END: entrypoint-branching

# ANCHOR: initialize-input-checks
initialize:
    # Error if invalid instruction data length.
    # ---------------------------------------------------------------------
    jne r9, SIZE_OF_INITIALIZE_INSTRUCTION, e_instruction_data_len

    # Error if invalid number of accounts.
    # ---------------------------------------------------------------------
    jne r8, IB_N_ACCOUNTS_INIT, e_n_accounts

    # Error if user has data.
    # ---------------------------------------------------------------------
    ldxdw r9, [r1 + IB_USER_DATA_LEN_OFF]
    jne r9, DATA_LEN_ZERO, e_user_data_len

    # Error if tree is duplicate or has data.
    # ---------------------------------------------------------------------
    ldxb r9, [r1 + IB_TREE_NON_DUP_MARKER_OFF]
    jne r9, IB_NON_DUP_MARKER, e_tree_duplicate
    ldxdw r9, [r1 + IB_TREE_DATA_LEN_OFF]
    jne r9, DATA_LEN_ZERO, e_tree_data_len

    # Error if System Program is duplicate or has invalid data length.
    # ---------------------------------------------------------------------
    ldxb r9, [r1 + IB_SYSTEM_PROGRAM_NON_DUP_MARKER_OFF]
    jne r9, IB_NON_DUP_MARKER, e_system_program_duplicate
    ldxdw r9, [r1 + IB_SYSTEM_PROGRAM_DATA_LEN_OFF]
    jne r9, IB_SYSTEM_PROGRAM_DATA_LEN, e_system_program_data_len

    # Error if Rent account is duplicate or has incorrect address.
    # ---------------------------------------------------------------------
    ldxb r9, [r1 + IB_RENT_NON_DUP_MARKER_OFF]
    jne r9, IB_NON_DUP_MARKER, e_rent_duplicate
    ldxdw r9, [r1 + IB_RENT_ADDRESS_OFF_0]
    lddw r8, IB_RENT_ID_CHUNK_0
    jne r9, r8, e_rent_address
    ldxdw r9, [r1 + IB_RENT_ADDRESS_OFF_1]
    lddw r8, IB_RENT_ID_CHUNK_1
    jne r9, r8, e_rent_address
    ldxdw r9, [r1 + IB_RENT_ADDRESS_OFF_2]
    lddw r8, IB_RENT_ID_CHUNK_2
    jne r9, r8, e_rent_address
    ldxdw r9, [r1 + IB_RENT_ADDRESS_OFF_3]
    # Optimize out the following line, which costs two CUs due to two
    # 32-bit immediate loads across two opcodes:
    # ```
    # lddw r8, IB_RENT_ID_CHUNK_3
    # ```
    # Instead, replace with mov32, which only loads one 32-bit immediate,
    # since the rent sysvar address has all chunk 3 hi bits unset.
    mov32 r8, IB_RENT_ID_CHUNK_3_LO
    jne r9, r8, e_rent_address
    # ANCHOR_END: initialize-input-checks

    # ANCHOR: initialize-pda-checks
    # Compute PDA.
    # ---------------------------------------------------------------------
    # Skip assignment for r1, since no seeds need to be parsed and this
    # argument is effectively ignored.
    # ---------------------------------------------------------------------
    mov64 r2, CPI_N_SEEDS_TRY_FIND_PDA # Declare no seeds to parse.
    mov64 r3, r1 # Get input buffer pointer.
    add64 r3, IB_INIT_PROGRAM_ID_OFF_IMM # Point at program ID.
    mov64 r4, r10 # Get stack frame pointer.
    add64 r4, SF_INIT_PDA_OFF # Point to PDA region on stack.
    mov64 r5, r10 # Get stack frame pointer.
    add64 r5, SF_INIT_BUMP_SEED_OFF # Point to bump seed region on stack.
    call sol_try_find_program_address # Find PDA.

    # Compare computed PDA against passed account.
    # ---------------------------------------------------------------------
    ldxdw r9, [r1 + IB_TREE_ADDRESS_OFF_0]
    ldxdw r8, [r4 + PUBKEY_CHUNK_OFF_0]
    jne r9, r8, e_pda_mismatch
    ldxdw r9, [r1 + IB_TREE_ADDRESS_OFF_1]
    ldxdw r8, [r4 + PUBKEY_CHUNK_OFF_1]
    jne r9, r8, e_pda_mismatch
    ldxdw r9, [r1 + IB_TREE_ADDRESS_OFF_2]
    ldxdw r8, [r4 + PUBKEY_CHUNK_OFF_2]
    jne r9, r8, e_pda_mismatch
    ldxdw r9, [r1 + IB_TREE_ADDRESS_OFF_3]
    ldxdw r8, [r4 + PUBKEY_CHUNK_OFF_3]
    jne r9, r8, e_pda_mismatch
    # ANCHOR_END: initialize-pda-checks

    // ANCHOR: initialize-create-account
    # Pack SolInstruction.
    # ---------------------------------------------------------------------
    # Packed later during bulk pointer load operation:
    # - [x] System Program ID pointer.
    # - [x] Account metas pointer.
    # - [x] Instruction data pointer.
    # ---------------------------------------------------------------------
    stdw [r10 + SF_INIT_INSN_ACCOUNT_LEN_OFF], CPI_N_ACCOUNTS
    stdw [r10 + SF_INIT_INSN_DATA_LEN_OFF], CPI_INSN_DATA_LEN

    # Pack CreateAccount instruction data.
    # ---------------------------------------------------------------------
    # - Discriminator is already set to 0 since stack is zero initialized.
    # - Reuses r3 from PDA syscall.
    # ---------------------------------------------------------------------
    ldxdw r9, [r1 + IB_RENT_DATA_OFF] # Load lamports per byte
    mul64 r9, CPI_ACCOUNT_DATA_SCALAR # Multiply to get rent-exempt cost.
    # Store in instruction data.
    stxdw [r10 + SF_INIT_CREATE_ACCOUNT_LAMPORTS_UOFF], r9
    # Store new account data length.
    stdw [r10 + SF_INIT_CREATE_ACCOUNT_SPACE_UOFF], CPI_TREE_DATA_LEN
    # Copy in program ID to instruction data.
    ldxdw r9, [r3 + PUBKEY_CHUNK_OFF_0]
    stxdw [r10 + SF_INIT_CREATE_ACCOUNT_OWNER_UOFF_0], r9
    ldxdw r9, [r3 + PUBKEY_CHUNK_OFF_1]
    stxdw [r10 + SF_INIT_CREATE_ACCOUNT_OWNER_UOFF_1], r9
    ldxdw r9, [r3 + PUBKEY_CHUNK_OFF_2]
    stxdw [r10 + SF_INIT_CREATE_ACCOUNT_OWNER_UOFF_2], r9
    ldxdw r9, [r3 + PUBKEY_CHUNK_OFF_3]
    stxdw [r10 + SF_INIT_CREATE_ACCOUNT_OWNER_UOFF_3], r9

    # Pack SolAccountMeta for user and tree.
    # ---------------------------------------------------------------------
    # Packed later during bulk pointer load operation:
    # - [x] User pubkey pointer.
    # - [x] Tree pubkey pointer.
    # ---------------------------------------------------------------------
    sth [r10 + SF_INIT_USER_META_IS_WRITABLE_OFF], CPI_WRITABLE_SIGNER
    sth [r10 + SF_INIT_TREE_META_IS_WRITABLE_OFF], CPI_WRITABLE_SIGNER

    # Pack SolAccountInfo for user and tree.
    # ---------------------------------------------------------------------
    # Packed later during bulk pointer load operation:
    # - [x] User pubkey pointer.
    # - [x] Tree pubkey pointer.
    # - [x] User lamports pointer.
    # - [x] Tree lamports pointer.
    # - [x] User data pointer.
    # - [x] Tree data pointer.
    # - [x] User owner pointer.
    # - [x] Tree owner pointer.
    # Skipped due to zero-initialized stack memory:
    # - User data length (already checked as zero).
    # - Tree data length (already checked as zero).
    # - User rent epoch.
    # - Tree rent epoch.
    # - User executable.
    # - Tree executable.
    # ---------------------------------------------------------------------
    sth [r10 + SF_INIT_USER_INFO_IS_SIGNER_OFF], CPI_WRITABLE_SIGNER
    sth [r10 + SF_INIT_TREE_INFO_IS_SIGNER_OFF], CPI_WRITABLE_SIGNER

    # Initialize signer seed for PDA bump seed.
    # ---------------------------------------------------------------------
    # Reuses r5 from PDA derivation syscall.
    # ---------------------------------------------------------------------
    # Store pointer to bump seed.
    stxdw [r10 + SF_INIT_SIGNER_SEED_ADDR_OFF], r5
    stdw [r10 + SF_INIT_SIGNER_SEED_LEN_OFF], SIZE_OF_U8 # Store length.

    # Initialize signers seeds for PDA.
    # ---------------------------------------------------------------------
    # Packed later during bulk pointer load operation:
    # - [x] Signer seed pointer.
    # ---------------------------------------------------------------------
    stdw [r10 + SF_INIT_SIGNERS_SEEDS_LEN_OFF], CPI_N_SEEDS

    # Bulk assign/load pointers for account metas and infos.
    # ---------------------------------------------------------------------
    # Since pointers must be loaded from registers, this block steps
    # through the input buffer in order to reduce intermediate loads.
    # ---------------------------------------------------------------------
    add64 r1, IB_USER_ADDRESS_OFF # Point to user address in input buffer.
    stxdw [r10 + SF_INIT_USER_META_PUBKEY_OFF], r1 # Store in account meta.
    stxdw [r10 + SF_INIT_USER_INFO_PUBKEY_OFF], r1 # Store in account info.
    add64 r1, SIZE_OF_ADDRESS # Advance to user owner.
    stxdw [r10 + SF_INIT_USER_INFO_OWNER_OFF], r1 # Store in account info.
    add64 r1, SIZE_OF_ADDRESS # Advance to user lamports.
    stxdw [r10 + SF_INIT_USER_INFO_LAMPORTS_OFF], r1 # Store in acct info.
    add64 r1, SIZE_OF_U128 # Advance to user data.
    stxdw [r10 + SF_INIT_USER_INFO_DATA_OFF], r1 # Store in account info.
    # Advance to tree address field.
    add64 r1, IB_USER_DATA_TO_TREE_ADDRESS_REL_OFF_IMM
    stxdw [r10 + SF_INIT_TREE_META_PUBKEY_OFF], r1 # Store in account meta.
    stxdw [r10 + SF_INIT_TREE_INFO_PUBKEY_OFF], r1 # Store in account info.
    add64 r1, SIZE_OF_ADDRESS # Advance to tree owner.
    stxdw [r10 + SF_INIT_TREE_INFO_OWNER_OFF], r1 # Store in account info.
    add64 r1, SIZE_OF_ADDRESS # Advance to tree lamports.
    stxdw [r10 + SF_INIT_TREE_INFO_LAMPORTS_OFF], r1 # Store in acct info.
    add64 r1, SIZE_OF_U128 # Advance to tree data.
    stxdw [r10 + SF_INIT_TREE_INFO_DATA_OFF], r1 # Store in account info.
    mov64 r6, r1 # Store tree data pointer for later.

    # Bulk assign/load pointers for CPI bindings.
    # ---------------------------------------------------------------------
    # This block steps through the stack frame, optimizing assignments in
    # preparation for the impending CreateAccount CPI, which requires:
    # - [x] r1 = pointer to instruction.
    # - [x] r2 = pointer to account infos.
    # - [x] r4 = pointer to signers seeds.
    # Notably, it reuses r4 from the PDA derivation syscall to walk through
    # pointers on the stack, before advancing it to its final value.
    # ---------------------------------------------------------------------
    # Advance to System Program ID pointer on zero-initialized stack.
    add64 r4, SF_INIT_PDA_TO_SYSTEM_PROGRAM_ID_REL_OFF_IMM
    # Store in SolInstruction.
    stxdw [r10 + SF_INIT_INSN_PROGRAM_ID_OFF], r4
    # Advance to SolAccountMeta array pointer.
    add64 r4, SF_INIT_SYSTEM_PROGRAM_ID_TO_ACCT_METAS_REL_OFF_IMM
    stxdw [r10 + SF_INIT_INSN_ACCOUNTS_OFF], r4 # Store in SolInstruction.
    # Advance to instruction data pointer.
    add64 r4, SF_INIT_ACCT_METAS_TO_INSN_DATA_REL_OFF_IMM
    stxdw [r10 + SF_INIT_INSN_DATA_OFF], r4 # Store in SolInstruction.
    # Advance to signer seeds pointer.
    add64 r4, SF_INIT_INSN_DATA_TO_SIGNER_SEEDS_REL_OFF_IMM
    stxdw [r10 + SF_INIT_SIGNERS_SEEDS_ADDR_OFF], r4
    # Advance to signers seeds pointer.
    add64 r4, SF_INIT_SIGNER_SEEDS_TO_SIGNERS_SEEDS_REL_OFF_IMM
    # Assign remaining syscall pointers.
    mov64 r1, r10
    add64 r1, SF_INIT_INSN_PROGRAM_ID_OFF
    mov64 r2, r10
    add64 r2, SF_INIT_ACCT_INFOS_OFF

    # Invoke CPI.
    # ---------------------------------------------------------------------
    mov64 r3, CPI_N_ACCOUNTS
    mov64 r5, CPI_N_PDA_SIGNERS
    call sol_invoke_signed_c

    # Store next pointer in tree header.
    # ---------------------------------------------------------------------
    mov64 r7, r6 # Get copy of tree data pointer.
    add64 r7, SIZE_OF_TREE_HEADER # Advance to next node.
    stxdw [r6 + TREE_HEADER_NEXT_OFF], r7 # Store in next field.

    exit
    // ANCHOR_END: initialize-create-account

# ANCHOR: insert
insert:
    jne r9, SIZE_OF_INSERT_INSTRUCTION, e_instruction_data_len
    exit
# ANCHOR_END: insert

e_instruction_data:
    mov64 r0, E_INSTRUCTION_DATA
    exit

e_instruction_data_len:
    mov64 r0, E_INSTRUCTION_DATA_LEN
    exit

e_n_accounts:
    mov64 r0, E_N_ACCOUNTS
    exit

e_pda_mismatch:
    mov64 r0, E_PDA_MISMATCH
    exit

e_rent_address:
    mov64 r0, E_RENT_ADDRESS
    exit

e_rent_duplicate:
    mov64 r0, E_RENT_DUPLICATE
    exit

e_system_program_data_len:
    mov64 r0, E_SYSTEM_PROGRAM_DATA_LEN
    exit

e_system_program_duplicate:
    mov64 r0, E_SYSTEM_PROGRAM_DUPLICATE
    exit

e_tree_data_len:
    mov64 r0, E_TREE_DATA_LEN
    exit

e_tree_duplicate:
    mov64 r0, E_TREE_DUPLICATE
    exit

e_user_data_len:
    mov64 r0, E_USER_DATA_LEN
    exit
