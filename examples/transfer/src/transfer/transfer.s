# Invalid number of accounts.
.equ E_N_ACCOUNTS, 1
# Sender data length is nonzero.
.equ E_DATA_LENGTH_NONZERO_SENDER, 2
# Recipient account is a duplicate.
.equ E_DUPLICATE_ACCOUNT_RECIPIENT, 3
# Recipient data length is nonzero.
.equ E_DATA_LENGTH_NONZERO_RECIPIENT, 4
# System program account is a duplicate.
.equ E_DUPLICATE_ACCOUNT_SYSTEM_PROGRAM, 5
# Invalid instruction data length.
.equ E_INSTRUCTION_DATA_LENGTH, 6
# Sender has insufficient Lamports.
.equ E_INSUFFICIENT_LAMPORTS, 7

# CPI instruction offsets.
.equ CPI_INSN_PROGRAM_ID_OFFSET, 0
.equ CPI_INSN_ACCOUNTS_ADDR_OFFSET, 8
.equ CPI_INSN_ACCOUNTS_LEN_OFFSET, 16
.equ CPI_INSN_DATA_ADDR_OFFSET, 24
.equ CPI_INSN_DATA_LEN_OFFSET, 32

# CPI account meta offsets.
.equ CPI_ACCT_META_PUBKEY_ADDR_OFFSET, 0
.equ CPI_ACCT_META_IS_WRITABLE_OFFSET, 8
.equ CPI_ACCT_META_IS_SIGNER_OFFSET, 9

# CPI account info offsets.
.equ CPI_ACCT_INFO_KEY_ADDR_OFFSET, 0
.equ CPI_ACCT_INFO_LAMPORTS_ADDR_OFFSET, 8
.equ CPI_ACCT_INFO_DATA_LEN_OFFSET, 16
.equ CPI_ACCT_INFO_DATA_ADDR_OFFSET, 24
.equ CPI_ACCT_INFO_OWNER_ADDR_OFFSET, 32
.equ CPI_ACCT_INFO_RENT_EPOCH_OFFSET, 40
.equ CPI_ACCT_INFO_IS_SIGNER_OFFSET, 48
.equ CPI_ACCT_INFO_IS_WRITABLE_OFFSET, 49
.equ CPI_ACCT_INFO_EXECUTABLE_OFFSET, 50

# CPI instruction data.
.equ INSTRUCTION_DISCRIMINATOR, 2

# Account layout.
.equ N_ACCOUNTS_OFFSET, 0
.equ N_ACCOUNTS_EXPECTED, 3
.equ NON_DUP_MARKER, 0xff
.equ DATA_LENGTH_ZERO, 0

# Sender account.
.equ SENDER_OFFSET, 8
.equ SENDER_LAMPORTS_OFFSET, 80
.equ SENDER_DATA_LENGTH_OFFSET, 88

# Recipient account.
.equ RECIPIENT_OFFSET, 10344
.equ RECIPIENT_DATA_LENGTH_OFFSET, 10424

# System program account.
.equ SYSTEM_PROGRAM_OFFSET, 20680

# Transfer input.
.equ INSTRUCTION_DATA_LENGTH_OFFSET, 31032
.equ INSTRUCTION_DATA_LENGTH_EXPECTED, 8
.equ INSTRUCTION_DATA_OFFSET, 31040

.global entrypoint

entrypoint:
    # Check number of accounts.
    ldxdw r2, [r1 + N_ACCOUNTS_OFFSET]
    jne r2, N_ACCOUNTS_EXPECTED, e_n_accounts

    # Check sender data length, since a nonzero length would invalidate
    # subsequent offsets.
    ldxdw r2, [r1 + SENDER_DATA_LENGTH_OFFSET]
    jne r2, DATA_LENGTH_ZERO, e_data_length_nonzero_sender

    # Check if the recipient account is a duplicate, since duplicate
    # accounts have different field layouts.
    ldxb r2, [r1 + RECIPIENT_OFFSET]
    jne r2, NON_DUP_MARKER, e_duplicate_account_recipient

    # Check recipient data length, since a nonzero length would invalidate
    # subsequent offsets.
    ldxdw r2, [r1 + RECIPIENT_DATA_LENGTH_OFFSET]
    jne r2, DATA_LENGTH_ZERO, e_data_length_nonzero_recipient

    # Check if the System Account is a duplicate, since duplicate accounts
    # have different field layouts.
    ldxb r2, [r1 + SYSTEM_PROGRAM_OFFSET]
    jne r2, NON_DUP_MARKER, e_duplicate_account_system_program

    # Check instruction data length.
    ldxdw r4, [r1 + INSTRUCTION_DATA_LENGTH_OFFSET]
    jne r4, INSTRUCTION_DATA_LENGTH_EXPECTED, e_instruction_data_length

    # Verify sender has at least as many Lamports as they are trying to
    # send. Technically this could be done after checking the number of
    # accounts since Lamports balance comes before account data length, but
    # in the happy path both checks need to be done anyways and it is
    # cleaner to do all layout validation first.
    ldxdw r4, [r1 + INSTRUCTION_DATA_OFFSET]
    ldxdw r2, [r1 + SENDER_LAMPORTS_OFFSET]
    jlt r2, r4, e_insufficient_lamports

    exit

e_duplicate_account_recipient:
    mov32 r0, E_DUPLICATE_ACCOUNT_RECIPIENT
    exit

e_duplicate_account_system_program:
    mov32 r0, E_DUPLICATE_ACCOUNT_SYSTEM_PROGRAM
    exit

e_instruction_data_length:
    mov32 r0, E_INSTRUCTION_DATA_LENGTH
    exit

e_insufficient_lamports:
    mov32 r0, E_INSUFFICIENT_LAMPORTS
    exit

e_n_accounts:
    mov32 r0, E_N_ACCOUNTS
    exit

e_data_length_nonzero_recipient:
    mov32 r0, E_DATA_LENGTH_NONZERO_RECIPIENT
    exit

e_data_length_nonzero_sender:
    mov32 r0, E_DATA_LENGTH_NONZERO_SENDER
    exit
