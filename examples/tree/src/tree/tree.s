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
# Instruction data provided during initialization instruction.
.equ E_INSTRUCTION_DATA, 7
# The passed PDA does not match the expected address.
.equ E_PDA_MISMATCH, 8

# Type sizes.
# -----------
.equ SIZE_OF_U8, 1 # Size of u8.

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
# Expected number of accounts for general instructions.
.equ IB_N_ACCOUNTS_GENERAL, 2
# Expected number of accounts for tree initialization.
.equ IB_N_ACCOUNTS_INIT, 3
# Expected data length of system program account.
.equ IB_SYSTEM_PROGRAM_DATA_LEN, 14
.equ IB_USER_ADDRESS_OFF, 16 # User address field.
.equ IB_USER_DATA_LEN_OFF, 88 # User data length field.
.equ IB_NON_DUP_MARKER, 255 # Non-duplicate marker value.
.equ IB_TREE_NON_DUP_MARKER_OFF, 10344 # Tree non-duplicate marker field.
.equ IB_TREE_ADDRESS_OFF, 10352 # Tree address field.
.equ IB_TREE_DATA_LEN_OFF, 10424 # Tree data length field.
# Instruction data length field for empty tree account.
.equ IB_INIT_INSTRUCTION_DATA_LEN_OFF, 31032
# Program ID field for initialize instruction.
.equ IB_INIT_PROGRAM_ID_OFF, 31040
# System Program non-duplicate marker field.
.equ IB_SYSTEM_PROGRAM_NON_DUP_MARKER_OFF, 20680
# System Program data length field.
.equ IB_SYSTEM_PROGRAM_DATA_LEN_OFF, 20760

# Init stack frame layout.
# ------------------------
.equ SF_INIT_BUMP_SEED_OFF, -360 # Bump seed.
.equ SF_INIT_SIGNER_SEED_ADDR_OFF, -120 # Bump signer seed address field.
.equ SF_INIT_SIGNER_SEED_LEN_OFF, -112 # Bump signer seed length field.
.equ SF_INIT_PDA_OFF, -104 # PDA address field.

# CPI-specific constants.
# -----------------------
.equ CPI_N_ACCOUNTS, 2 # User and tree accounts must sign CPI.
.equ CPI_N_PDA_SIGNERS, 1 # The tree account is a PDA.
.equ CPI_N_SEEDS, 1 # The bump seed is required for tree PDA signer.
.equ CPI_N_SEEDS_TRY_FIND_PDA, 0 # Number of seeds for PDA generation.
# ANCHOR_END: constants

# ANCHOR: entrypoint-branching
.globl entrypoint

entrypoint:
    # Check input buffer accounts.
    # ----------------------------
    ldxdw r2, [r1 + IB_N_ACCOUNTS_OFF] # Get n input buffer accounts.
    jeq r2, IB_N_ACCOUNTS_GENERAL, general # Fast path to general case.
    jeq r2, IB_N_ACCOUNTS_INIT, initialize # Branch to init case.
    mov64 r0, E_N_ACCOUNTS # Else fail.
    exit
    # ANCHOR_END: entrypoint-branching

general:
    ldxdw r2, [r1 + IB_TREE_DATA_LEN_OFF] # Get tree data length.
    add64 r2, MAX_DATA_PAD # Speculatively add max possible padding.
    and64 r2, DATA_LEN_AND_MASK # Get data length plus required padding.
    add64 r2, r1 # Get input buffer pointer shifted for tree data.
    exit

# ANCHOR: initialize-input-checks
initialize:

    # Error if user has data.
    # -----------------------
    ldxdw r2, [r1 + IB_USER_DATA_LEN_OFF]
    jne r2, DATA_LEN_ZERO, e_user_data_len

    # Error if tree is duplicate or has data.
    # ---------------------------------------
    ldxb r2, [r1 + IB_TREE_NON_DUP_MARKER_OFF]
    jne r2, IB_NON_DUP_MARKER, e_tree_duplicate
    ldxdw r2, [r1 + IB_TREE_DATA_LEN_OFF]
    jne r2, DATA_LEN_ZERO, e_tree_data_len

    # Error if System Program is duplicate or has invalid data length.
    # ----------------------------------------------------------------
    ldxb r2, [r1 + IB_SYSTEM_PROGRAM_NON_DUP_MARKER_OFF]
    jne r2, IB_NON_DUP_MARKER, e_system_program_duplicate
    ldxdw r2, [r1 + IB_SYSTEM_PROGRAM_DATA_LEN_OFF]
    jne r2, IB_SYSTEM_PROGRAM_DATA_LEN, e_system_program_data_len

    # Error if instruction data provided.
    # -----------------------------------
    ldxdw r2, [r1 + IB_INIT_INSTRUCTION_DATA_LEN_OFF]
    jne r2, DATA_LEN_ZERO, e_instruction_data
    # ANCHOR_END: initialize-input-checks

    # ANCHOR: initialize-check-pda
    # Compute PDA.
    # ------------
    mov64 r2, CPI_N_SEEDS_TRY_FIND_PDA # Indicate no signer seeds.
    mov64 r3, r1 # Get input buffer pointer.
    add64 r3, IB_INIT_PROGRAM_ID_OFF # Point at program ID in input buffer.
    mov64 r4, r10 # Get stack frame pointer.
    add64 r4, SF_INIT_PDA_OFF # Point to PDA region on stack.
    mov64 r5, r10 # Get stack frame pointer.
    add64 r5, SF_INIT_BUMP_SEED_OFF # Point to bump seed region on stack.
    call sol_try_find_program_address # Find PDA.

    # Compare computed PDA against passed account.
    # --------------------------------------------
    mov64 r3, r1 # Get input buffer pointer.
    add64 r3, IB_TREE_ADDRESS_OFF # Point at tree address.
    ldxdw r5, [r3 + PUBKEY_CHUNK_OFF_0]
    ldxdw r6, [r4 + PUBKEY_CHUNK_OFF_0]
    jne r5, r6, e_pda_mismatch
    ldxdw r5, [r3 + PUBKEY_CHUNK_OFF_1]
    ldxdw r6, [r4 + PUBKEY_CHUNK_OFF_1]
    jne r5, r6, e_pda_mismatch
    ldxdw r5, [r3 + PUBKEY_CHUNK_OFF_2]
    ldxdw r6, [r4 + PUBKEY_CHUNK_OFF_2]
    jne r5, r6, e_pda_mismatch
    ldxdw r5, [r3 + PUBKEY_CHUNK_OFF_3]
    ldxdw r6, [r4 + PUBKEY_CHUNK_OFF_3]
    jne r5, r6, e_pda_mismatch
    # ANCHOR_END: initialize-check-pda

    # Initialize signer seed for PDA bump key.
    # ----------------------------------------
    mov64 r2, r10 # Get stack frame pointer.
    add64 r2, SF_INIT_BUMP_SEED_OFF # Point at bump seed on stack.
    stxdw [r10 + SF_INIT_SIGNER_SEED_ADDR_OFF], r2 # Store in signer seed.
    stdw [r10 + SF_INIT_SIGNER_SEED_LEN_OFF], SIZE_OF_U8 # Store length.

    exit

e_instruction_data:
    mov64 r0, E_INSTRUCTION_DATA
    exit

e_pda_mismatch:
    mov64 r0, E_PDA_MISMATCH
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
