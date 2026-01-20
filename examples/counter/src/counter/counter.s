# Input memory map account layout.
# --------------------------------
.equ N_ACCOUNTS_OFF, 0 # Number of accounts in virtual memory map.
.equ NON_DUP_MARKER, 0xff # Flag that an account is not a duplicate.

.global entrypoint

entrypoint:
    exit