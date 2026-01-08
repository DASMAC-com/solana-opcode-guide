# Invalid number of accounts.
.equ E_N_ACCOUNTS, 1
# Recipient account is a duplicate.
.equ E_DUPLICATE_ACCOUNT_RECIPIENT, 2
# System program account is a duplicate.
.equ E_DUPLICATE_ACCOUNT_SYSTEM_PROGRAM, 3

# Account positioning.
.equ N_ACCOUNTS_OFFSET, 0
.equ N_ACCOUNTS_EXPECTED, 3
.equ NON_DUP_MARKER, 0xff

# Sender accounts.
.equ SENDER_OFFSET, 8

# Recipient account.
.equ RECIPIENT_OFFSET, 10344

# System program account.
.equ SYSTEM_PROGRAM_OFFSET, 20680

# Transfer amount.
.equ INSTRUCTION_DATA_LEN, 31032

.global entrypoint

entrypoint:
    # Check number of accounts.
    ldxdw r2, [r1 + N_ACCOUNTS_OFFSET]
    jne r2, N_ACCOUNTS_EXPECTED, e_n_accounts

    # Check duplicate accounts.
    ldxb r2, [r1 + RECIPIENT_OFFSET]
    jne r2, NON_DUP_MARKER, e_duplicate_account_recipient
    ldxb r2, [r1 + SYSTEM_PROGRAM_OFFSET]
    jne r2, NON_DUP_MARKER, e_duplicate_account_system_program

    exit

e_n_accounts:
    mov32 r0, E_N_ACCOUNTS
    exit

e_duplicate_account_recipient:
    mov32 r0, E_DUPLICATE_ACCOUNT_RECIPIENT
    exit

e_duplicate_account_system_program:
    mov32 r0, E_DUPLICATE_ACCOUNT_SYSTEM_PROGRAM
    exit