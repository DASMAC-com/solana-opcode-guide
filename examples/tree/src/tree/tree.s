# ANCHOR: constants
# Error codes.
# ------------
.equ E_N_ACCOUNTS, 1 # An invalid number of accounts were passed.
.equ E_USER_DATA_LEN, 2 # The user account has nonzero data length.
.equ E_TREE_DUPLICATE, 3 # The tree account is a duplicate.

# Input buffer layout.
# --------------------
.equ IB_N_ACCOUNTS, 2 # Expected number of accounts.
.equ IB_N_ACCOUNTS_OFF, 0 # Number of accounts field.
.equ IB_USER_ADDRESS_OFF, 16 # User address field.
.equ IB_USER_DATA_LEN_OFF, 88 # User data length field.
.equ IB_NON_DUP_MARKER, 255 # Non-duplicate marker value.
.equ IB_TREE_NON_DUP_MARKER_OFF, 10344 # Tree non-duplicate marker field.
.equ IB_TREE_DATA_LEN_OFF, 10424 # Tree data length field.

# Miscellaneous constants.
# ------------------------
.equ DATA_LEN_ZERO, 0 # Data length of zero.
.equ DATA_LEN_AND_MASK, -8 # And mask for data length alignment.
.equ MAX_DATA_PAD, 7 # Maximum possible data length padding.
# ANCHOR_END: constants

# ANCHOR: check-input-buffer
.globl entrypoint

entrypoint:
    ldxdw r2, [r1 + IB_N_ACCOUNTS_OFF] # Get n input buffer accounts.
    jne r2, IB_N_ACCOUNTS, e_n_accounts # Error if invalid number.
    ldxdw r2, [r1 + IB_USER_DATA_LEN_OFF] # Get user data length.
    jne r2, DATA_LEN_ZERO, e_user_data_len # Error if user has data.
    ldxb r2, [r1 + IB_TREE_NON_DUP_MARKER_OFF] # Load tree non-dup marker.
    jne r2, IB_NON_DUP_MARKER, e_tree_duplicate # Error if duplicate.
    # ANCHOR_END: check-input-buffer

    ldxdw r2, [r1 + IB_TREE_DATA_LEN_OFF] # Get tree data length.
    add64 r2, MAX_DATA_PAD # Speculatively add max possible padding.
    and64 r2, DATA_LEN_AND_MASK # Get data length plus required padding.
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
