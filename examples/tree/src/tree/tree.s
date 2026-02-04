# Error codes.
# ------------
.equ E_N_ACCOUNTS, 1 # An invalid number of accounts were passed.
.equ E_USER_DATA_LEN, 2 # The user account has nonzero data length.
.equ E_TREE_DUPLICATE, 3 # The tree account is a duplicate.

# Input buffer layout.
# --------------------
.equ IB_N_ACCOUNTS, 2 # Expected number of accounts.
.equ IB_N_ACCOUNTS_OFF, 0 # Number of accounts field.
.equ IB_USER_DATA_LEN_OFF, 88 # User data length field.
.equ IB_NON_DUP_MARKER, 0xff # Non-duplicate marker value.
.equ IB_TREE_NON_DUP_MARKER_OFF, 104 # Tree non-duplicate marker field.

# Miscellaneous constants.
# ------------------------
.equ DATA_LENGTH_ZERO, 0 # Data length of zero.

.globl entrypoint

entrypoint:
    ldxdw r2, [r1 + IB_N_ACCOUNTS_OFF] # Get n input buffer accounts.
    jne r2, IB_N_ACCOUNTS, e_n_accounts # Error if invalid number.
    ldxdw r2, [r1 + IB_USER_DATA_LEN_OFF] # Get user data length.
    jne r2, DATA_LENGTH_ZERO, e_user_data_len # Error if user has data.
    ldxb r2, [r1 + IB_TREE_NON_DUP_MARKER_OFF] # Load tree non-dup marker.
    # Error if tree is duplicate account.
    jne r2, IB_NON_DUP_MARKER, e_tree_duplicate
    exit

e_n_accounts:
    mov64 r0, E_N_ACCOUNTS
    exit

e_user_data_len:
    mov64 r0, E_USER_DATA_LEN
    exit

e_tree_duplicate:
    mov64 r0, E_TREE_DUPLICATE
    exit