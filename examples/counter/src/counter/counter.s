# Error codes.
# ------------
.equ E_N_ACCOUNTS, 1 # Invalid number of accounts.

# Input memory map account layout.
# --------------------------------
.equ N_ACCOUNTS_OFF, 0 # Number of accounts in virtual memory map.
.equ NON_DUP_MARKER, 0xff # Flag that an account is not a duplicate.
.equ N_ACCOUNTS_INCREMENT, 2 # Number of accounts for increment operation.
.equ N_ACCOUNTS_INIT, 3 # Number of accounts for init operation.

# Stack frame layout for initialize operation.
# --------------------------------------------
.equ STK_INIT_INSN_OFF, 296 # SolInstruction for CreateAccount CPI.
.equ STK_INIT_SEED_1_ADDR_OFF, 56 # Pointer to user pubkey.
.equ STK_INIT_SEED_1_LEN_OFF, 48 # Length of user pubkey.
.equ STK_INIT_SEED_2_ADDR_OFF, 40 # Pointer to bump seed.
.equ STK_INIT_SEED_2_LEN_OFF, 32 # Length of bump seed.
.equ STK_INIT_BUMP_SEED_OFF, 8 # PDA bump seed.

.global entrypoint

entrypoint:
    ldxdw r2, [r1 + N_ACCOUNTS_OFF] # Get n accounts from input buffer.
    jeq r2, N_ACCOUNTS_INCREMENT, increment # Fast path to cheap operation.
    jeq r3, N_ACCOUNTS_INIT, initialize # Low priority, expensive anyways.
    mov64 r0, E_N_ACCOUNTS # Else fail.
    exit

initialize:
    exit

increment:
    exit
