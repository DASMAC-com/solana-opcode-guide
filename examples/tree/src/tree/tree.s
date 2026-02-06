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
.equ BPF_ALIGN_OF_U128, 8 # Data alignment during runtime.
.equ DATA_LEN_AND_MASK, -8 # And mask for data length alignment.
.equ MAX_DATA_PAD, 7 # Maximum possible data length padding.

# Init stack frame layout.
# ------------------------
.equ SF_SYSTEM_PROGRAM_ADDRESS_OFF, -360 # System program address.
.equ SF_INSTRUCTION_OFF, -328 # CPI instruction.
.equ SF_ACCOUNT_META_0_OFF, -288 # First account meta.
.equ SF_ACCOUNT_META_1_OFF, -272 # Second account meta.
.equ SF_ACCOUNT_INFO_0_OFF, -256 # First account info.
.equ SF_ACCOUNT_INFO_1_OFF, -200 # Second account info.
.equ SF_SIGNERS_SEEDS_OFF, -144 # Signer seeds array.
.equ SF_SIGNER_SEEDS_OFF, -128 # Signer seed entry.
.equ SF_PDA_OFF, -112 # PDA address.
.equ SF_RENT_OFF, -80 # Rent sysvar.
.equ SF_INSTRUCTION_DATA_OFF, -64 # Instruction data.
.equ SF_BUMP_SEED_OFF, -8 # Bump seed.

# Type sizes.
# -----------
.equ SIZE_OF_SOL_INSTRUCTION, 40 # Size of SolInstruction.
.equ SIZE_OF_SOL_ACCOUNT_META, 16 # Size of SolAccountMeta.
.equ SIZE_OF_SOL_ACCOUNT_INFO, 56 # Size of SolAccountInfo.
.equ SIZE_OF_SOL_SIGNER_SEED, 16 # Size of SolSignerSeed.
.equ SIZE_OF_SOL_SIGNER_SEEDS, 16 # Size of SolSignerSeeds.
# Size of CreateAccountInstructionData.
.equ SIZE_OF_CREATE_ACCOUNT_INSTRUCTION_DATA, 52
.equ SIZE_OF_RENT, 16 # Size of Rent.
.equ SIZE_OF_ADDRESS, 32 # Size of Address.
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

    # Initialize signer seed for PDA bump key.
    # ----------------------------------------
    mov64 r2, r10 # Get stack frame pointer.

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
