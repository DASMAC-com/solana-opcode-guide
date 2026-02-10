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

# Type sizes.
# -----------
.equ SIZE_OF_U8, 1 # Size of u8.
.equ SIZE_OF_U64, 8 # Size of u64.

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
.equ IB_TREE_ACCOUNT_OFF, 10344 # Tree runtime account header.
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
.equ IB_INIT_PROGRAM_ID_OFF_IMM, 41400

# Init stack frame layout.
# ------------------------
.equ SF_INIT_BUMP_SEED_OFF, -360 # Bump seed.
.equ SF_INIT_SIGNER_SEED_ADDR_OFF, -120 # Bump signer seed address field.
.equ SF_INIT_SIGNER_SEED_LEN_OFF, -112 # Bump signer seed length field.
.equ SF_INIT_PDA_OFF, -104 # PDA address field.
# Lamports field in CreateAccount instruction data.
.equ SF_INIT_CREATE_ACCOUNT_LAMPORTS_UOFF, -52
# Space address field in CreateAccount instruction data.
.equ SF_INIT_CREATE_ACCOUNT_SPACE_UOFF, -44

# CPI-specific constants.
# -----------------------
.equ CPI_N_ACCOUNTS, 2 # User and tree accounts must sign CPI.
.equ CPI_N_PDA_SIGNERS, 1 # The tree account is a PDA.
.equ CPI_N_SEEDS, 1 # The bump seed is required for tree PDA signer.
.equ CPI_N_SEEDS_TRY_FIND_PDA, 0 # Number of seeds for PDA generation.
.equ CPI_TREE_DATA_LEN, 16 # Tree account data length.
# Account data scalar for base rent calculation.
.equ CPI_ACCOUNT_DATA_SCALAR, 144
# CreateAccount discriminator for CPI.
.equ CPI_CREATE_ACCOUNT_DISCRIMINATOR, 0
# ANCHOR_END: constants

# ANCHOR: entrypoint-branching
.globl entrypoint

entrypoint:
    # Check input buffer accounts.
    # ----------------------------
    ldxdw r9, [r1 + IB_N_ACCOUNTS_OFF] # Get n input buffer accounts.
    jeq r9, IB_N_ACCOUNTS_GENERAL, general # Fast path to general case.
    jeq r9, IB_N_ACCOUNTS_INIT, initialize # Branch to init case.
    mov64 r0, E_N_ACCOUNTS # Else fail.
    exit
    # ANCHOR_END: entrypoint-branching

general:
    ldxdw r9, [r1 + IB_TREE_DATA_LEN_OFF] # Get tree data length.
    add64 r9, MAX_DATA_PAD # Speculatively add max possible padding.
    and64 r9, DATA_LEN_AND_MASK # Get data length plus required padding.
    add64 r9, r1 # Get input buffer pointer shifted for tree data.
    exit

# ANCHOR: initialize-input-checks
initialize:

    # Error if user has data.
    # -----------------------
    ldxdw r9, [r1 + IB_USER_DATA_LEN_OFF]
    jne r9, DATA_LEN_ZERO, e_user_data_len

    # Error if tree is duplicate or has data.
    # ---------------------------------------
    ldxb r9, [r1 + IB_TREE_NON_DUP_MARKER_OFF]
    jne r9, IB_NON_DUP_MARKER, e_tree_duplicate
    ldxdw r9, [r1 + IB_TREE_DATA_LEN_OFF]
    jne r9, DATA_LEN_ZERO, e_tree_data_len

    # Error if System Program is duplicate or has invalid data length.
    # ----------------------------------------------------------------
    ldxb r9, [r1 + IB_SYSTEM_PROGRAM_NON_DUP_MARKER_OFF]
    jne r9, IB_NON_DUP_MARKER, e_system_program_duplicate
    ldxdw r9, [r1 + IB_SYSTEM_PROGRAM_DATA_LEN_OFF]
    jne r9, IB_SYSTEM_PROGRAM_DATA_LEN, e_system_program_data_len

    # Error if Rent account is duplicate or has incorrect address.
    # ------------------------------------------------------------
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

    # Error if instruction data provided.
    # -----------------------------------
    ldxdw r9, [r2 - SIZE_OF_U64]
    jne r9, DATA_LEN_ZERO, e_instruction_data
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
    # --------------------------------------------
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

    # Initialize signer seed for PDA bump key.
    # ---------------------------------------------------------------------
    # Reuses r5 from PDA derivation syscall.
    # ---------------------------------------------------------------------
    # Store pointer to bump seed.
    stxdw [r10 + SF_INIT_SIGNER_SEED_ADDR_OFF], r5
    stdw [r10 + SF_INIT_SIGNER_SEED_LEN_OFF], SIZE_OF_U8 # Store length.

    // ANCHOR_END: initialize-create-account


    exit

e_instruction_data:
    mov64 r0, E_INSTRUCTION_DATA
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
