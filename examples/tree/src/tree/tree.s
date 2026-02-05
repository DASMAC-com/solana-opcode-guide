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

# Input buffer layout.
# --------------------
# Expected number of accounts for general instructions.
.equ IB_N_ACCOUNTS_GENERAL, 2
# Expected number of accounts for tree initialization.
.equ IB_N_ACCOUNTS_INIT, 3
# Expected data length of system program account.
.equ IB_SYSTEM_PROGRAM_DATA_LEN, 14
.equ IB_N_ACCOUNTS_OFF, 0 # Number of accounts field.
.equ IB_USER_ADDRESS_OFF, 16 # User address field.
.equ IB_USER_DATA_LEN_OFF, 88 # User data length field.
.equ IB_NON_DUP_MARKER, 255 # Non-duplicate marker value.
.equ IB_TREE_NON_DUP_MARKER_OFF, 10344 # Tree non-duplicate marker field.
.equ IB_TREE_DATA_LEN_OFF, 10424 # Tree data length field.
# Instruction data length field for empty tree account.
.equ IB_INIT_INSTRUCTION_DATA_LEN_OFF, 31032
# Program ID field for initialize instruction.
.equ IB_INIT_PROGRAM_ID_OFF, 31040
# System Program non-duplicate marker field.
.equ IB_SYSTEM_PROGRAM_NON_DUP_MARKER_OFF, 20680
# System Program data length field.
.equ IB_SYSTEM_PROGRAM_DATA_LEN_OFF, 20760

# Miscellaneous constants.
# ------------------------
.equ DATA_LEN_ZERO, 0 # Data length of zero.
.equ DATA_LEN_AND_MASK, -8 # And mask for data length alignment.
.equ MAX_DATA_PAD, 7 # Maximum possible data length padding.
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

    exit

e_instruction_data:
    mov64 r0, E_INSTRUCTION_DATA
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
