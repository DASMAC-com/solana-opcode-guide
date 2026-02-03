# Input buffer layout.
# --------------------
.equ IB_N_ACCOUNTS, 2 # Number of accounts expected.
.equ IB_N_ACCOUNTS_OFF, 0 # Number of accounts passed in input.

# Error codes.
# ------------
.equ E_N_ACCOUNTS_INVALID, 1 # An invalid number of accounts were passed.
.equ E_USER_HAS_DATA, 2 # The user account has nonzero data length.

.globl entrypoint

entrypoint:
    mov64 r0, 0
    exit
