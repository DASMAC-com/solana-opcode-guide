# Input buffer layout.
# --------------------
.equ N_ACCOUNTS, 2 # Number of accounts expected.

# Error codes.
# ------------
.equ E_N_ACCOUNTS, 1 # An invalid number of accounts were passed.
.equ E_USER_DATA, 2 # The user account has nonzero data length.

.globl entrypoint

entrypoint:
    mov64 r0, 0
    exit
