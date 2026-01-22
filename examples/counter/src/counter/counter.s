# Error codes.
# ------------
.equ E_N_ACCOUNTS, 1 # Invalid number of accounts.
.equ E_USER_DATA_LEN, 2 # User data length is nonzero.
.equ E_PDA_DATA_LEN, 3 # PDA data length is nonzero.
.equ E_SYSTEM_PROGRAM_DATA_LEN, 4 # System Program data length is nonzero.
.equ E_PDA_DUPLICATE, 5 # PDA is a duplicate account.
.equ E_SYSTEM_PROGRAM_DUPLICATE, 6 # System Program is a duplicate account.

# Input memory map layout.
# ------------------------
.equ NON_DUP_MARKER, 0xff # Flag that an account is not a duplicate.
.equ DATA_LEN_ZERO, 0 # Data length of zero.
.equ DATA_LEN_SYSTEM_PROGRAM, 14 # Data length of System Program.
.equ N_ACCOUNTS_INCREMENT, 2 # Number of accounts for increment operation.
.equ N_ACCOUNTS_INIT, 3 # Number of accounts for initialize operation.
.equ N_ACCOUNTS_OFF, 0 # Number of accounts in virtual memory map.
.equ USER_DATA_LEN_OFF, 88 # User data length.
.equ USER_PUBKEY_OFF, 16 # User pubkey.
.equ PDA_NON_DUP_MARKER_OFF, 10344 # PDA non-duplicate marker.
.equ PDA_DATA_LEN_OFF, 10424 # PDA data length.
# System Program non-duplicate marker.
.equ SYSTEM_PROGRAM_NON_DUP_MARKER_OFF, 20680
.equ SYSTEM_PROGRAM_DATA_LEN_OFF, 20760 # System program data length.

# Stack frame layout for initialize operation.
# --------------------------------------------
.equ STK_INIT_INSN_OFF, 328 # SolInstruction for CreateAccount CPI.
.equ STK_INIT_SEED_1_ADDR_OFF, 88 # Pointer to user pubkey.
.equ STK_INIT_SEED_1_LEN_OFF, 80 # Length of user pubkey.
.equ STK_INIT_SEED_2_ADDR_OFF, 72 # Pointer to bump seed.
.equ STK_INIT_SEED_2_LEN_OFF, 64 # Length of bump seed.
.equ STK_INIT_PDA_OFF, 40 # PDA.

.global entrypoint

entrypoint:
    ldxdw r2, [r1 + N_ACCOUNTS_OFF] # Get n accounts from input buffer.
    jeq r2, N_ACCOUNTS_INCREMENT, increment # Fast path to cheap operation.
    jeq r2, N_ACCOUNTS_INIT, initialize # Low priority, expensive anyways.
    mov64 r0, E_N_ACCOUNTS # Else fail.
    exit

initialize:

    # Check input memory map.
    # -----------------------
    ldxdw r2, [r1 + USER_DATA_LEN_OFF] # Get user data length.
    jne r2, DATA_LEN_ZERO, e_user_data_len # Exit if user account has data.
    ldxb r2, [r1 + PDA_NON_DUP_MARKER_OFF] # Check if PDA is a duplicate.
    jne r2, NON_DUP_MARKER, e_pda_duplicate # Exit if PDA is a duplicate.
    ldxdw r2, [r1 + PDA_DATA_LEN_OFF] # Get PDA data length.
    jne r2, DATA_LEN_ZERO, e_pda_data_len # Exit if PDA account has data.
    # Exit early if System Program is a duplicate.
    ldxb r2, [r1 + SYSTEM_PROGRAM_NON_DUP_MARKER_OFF]
    jne r2, NON_DUP_MARKER, e_system_program_duplicate
    # Exit early if System Program data length is invalid.
    ldxdw r2, [r1 + SYSTEM_PROGRAM_DATA_LEN_OFF]
    jne r2, DATA_LEN_SYSTEM_PROGRAM, e_system_program_data_len

    # Initialize user pubkey signer seed.
    # -----------------------------------

    exit

increment:
    exit

e_user_data_len:
    mov32 r0, E_USER_DATA_LEN
    exit

e_pda_data_len:
    mov32 r0, E_PDA_DATA_LEN
    exit

e_system_program_data_len:
    mov32 r0, E_SYSTEM_PROGRAM_DATA_LEN
    exit

e_pda_duplicate:
    mov32 r0, E_PDA_DUPLICATE
    exit

e_system_program_duplicate:
    mov32 r0, E_SYSTEM_PROGRAM_DUPLICATE
    exit
