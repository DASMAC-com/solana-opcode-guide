# Invalid number of accounts.
.equ E_N_ACCOUNTS, 1
# Recipient account is a duplicate.
.equ E_DUPLICATE_ACCOUNT_RECIPIENT, 2
# System program account is a duplicate.
.equ E_DUPLICATE_ACCOUNT_SYSTEM_PROGRAM, 3
# Invalid instruction data length.
.equ E_INVALID_INSTRUCTION_DATA_LENGTH, 4

# Account positioning.
.equ N_ACCOUNTS_OFFSET, 0
.equ N_ACCOUNTS_EXPECTED, 3
.equ NON_DUP_MARKER, 0xff

# Sender account.
.equ SENDER_OFFSET, 8

# Recipient account.
.equ RECIPIENT_OFFSET, 10344

# System program account.
.equ SYSTEM_PROGRAM_OFFSET, 20680

# Transfer amount.
.equ INSTRUCTION_DATA_LENGTH_OFFSET, 31032
.equ INSTRUCTION_DATA_OFFSET, 31040

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

    # Check instruction data, storing transfer amount in r4.
    ldxdw r4, [r1 + INSTRUCTION_DATA_LENGTH_OFFSET]
    jne r4, 8, e_invalid_instruction_data_length
    ldxdw r4, [r1 + INSTRUCTION_DATA_OFFSET]

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

e_invalid_instruction_data_length:
    mov32 r0, E_INVALID_INSTRUCTION_DATA_LENGTH
    exit