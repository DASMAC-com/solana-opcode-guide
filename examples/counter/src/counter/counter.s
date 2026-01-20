# Error codes.
# ------------
.equ E_N_ACCOUNTS, 1 # Invalid number of accounts.

# Input memory map account layout.
# --------------------------------
.equ N_ACCOUNTS_OFF, 0 # Number of accounts in virtual memory map.
.equ NON_DUP_MARKER, 0xff # Flag that an account is not a duplicate.
.equ N_ACCOUNTS_INCREMENT, 2 # Number of accounts for increment operation.
.equ N_ACCOUNTS_INIT, 3 # Number of accounts for init operation.

.global entrypoint

entrypoint:
    # Check number of accounts.
    # -------------------------
    ldxdw r2, [r1 + N_ACCOUNTS_OFF] # Get n accounts from input buffer.
    jeq r2, N_ACCOUNTS_INCREMENT, increment # Fast path to cheap operation.
    jeq r3, N_ACCOUNTS_INIT, init # Second priority, is expensive anyways.
    mov64 r0, E_N_ACCOUNTS # Else fail.
    exit

init:
    exit

increment:
    exit