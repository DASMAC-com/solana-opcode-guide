# Error codes.
# ------------
.equ E_N_ACCOUNTS, 1 # Invalid number of accounts.
.equ E_USER_DATA_LEN, 2 # User data length is nonzero.
.equ E_PDA_DATA_LEN, 3 # PDA data length is nonzero.
.equ E_SYSTEM_PROGRAM_DATA_LEN, 4 # System Program data length is nonzero.

# Input memory map account layout.
# --------------------------------
.equ N_ACCOUNTS_OFF, 0 # Number of accounts in virtual memory map.
.equ NON_DUP_MARKER, 0xff # Flag that an account is not a duplicate.
.equ DATA_LEN_ZERO, 0 # Data length of zero.
.equ DATA_LEN_SYSTEM_PROGRAM, 14 # Data length of System Program.
.equ N_ACCOUNTS_INCREMENT, 2 # Number of accounts for increment operation.
.equ N_ACCOUNTS_INIT, 3 # Number of accounts for initialize operation.
.equ USER_OFF, 8 # Serialized user account.
.equ USER_DATA_LEN_OFF, 88 # User data length.
.equ USER_PUBKEY_OFF, 16 # User pubkey.
.equ PDA_DATA_LEN_OFF, 10424 # PDA data length.
.equ SYSTEM_PROGRAM_DATA_LEN_OFF, 20760 # System program data length.

# Stack frame layout for initialize operation.
# --------------------------------------------
.equ STK_INIT_INSN_OFF, 288 # SolInstruction for CreateAccount CPI.
.equ STK_INIT_SEED_1_ADDR_OFF, 48 # Pointer to user pubkey.
.equ STK_INIT_SEED_1_LEN_OFF, 40 # Length of user pubkey.
.equ STK_INIT_SEED_2_ADDR_OFF, 32 # Pointer to bump seed.
.equ STK_INIT_SEED_2_LEN_OFF, 24 # Length of bump seed.

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
    ldxdw r2, [r1 + PDA_DATA_LEN_OFF] # Get PDA data length.
    jne r2, DATA_LEN_ZERO, e_pda_data_len # Exit if PDA account has data.
    # Get System Program account data length.
    ldxdw r2, [r1 + SYSTEM_PROGRAM_DATA_LEN_OFF]
    # Exit if System Program account has invalid data length.
    jne r2, DATA_LEN_SYSTEM_PROGRAM, e_system_program_data_len
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