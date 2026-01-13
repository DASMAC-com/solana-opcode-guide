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
.equ CPI_INSN_PROGRAM_ID_ADDR_OFFSET, 0
.equ CPI_INSN_ACCOUNTS_ADDR_OFFSET, 8
.equ CPI_INSN_ACCOUNTS_LEN_OFFSET, 16
.equ CPI_INSN_DATA_ADDR_OFFSET, 24
.equ CPI_INSN_DATA_LEN_OFFSET, 32

# CPI account meta offsets.
.equ CPI_ACCT_META_PUBKEY_ADDR_OFFSET, 0
.equ CPI_ACCT_META_IS_WRITABLE_OFFSET, 8
.equ CPI_ACCT_META_IS_SIGNER_OFFSET, 9
.equ CPI_ACCT_META_SIZE_OF, 16

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

# CPI instruction data offsets.
.equ CPI_INSN_DATA_VARIANT_OFFSET, 0
.equ CPI_INSN_DATA_AMOUNT_OFFSET, 4
.equ CPI_INSN_DATA_LEN, 12

# CPI general constants.
.equ CPI_INSN_DATA_VARIANT, 2
.equ CPI_ACCOUNTS_LEN, 2

# Stack offsets.
.equ STACK_INSN_OFFSET, 200
.equ STACK_INSN_DATA_OFFSET, 160
.equ STACK_ACCT_METAS_OFFSET, 144
.equ STACK_ACCT_INFOS_OFFSET, 112

# Account layout.
.equ N_ACCOUNTS_OFFSET, 0
.equ N_ACCOUNTS_EXPECTED, 3
.equ NON_DUP_MARKER, 0xff
.equ DATA_LENGTH_ZERO, 0
.equ PUBKEY_SIZE_OF, 32
.equ U8_SIZE_OF, 8
.equ U16_SIZE_OF, 16

# Sender account.
.equ SENDER_OFFSET, 8
.equ SENDER_IS_SIGNER_OFFSET, 9
.equ SENDER_IS_WRITABLE_OFFSET, 10
.equ SENDER_IS_EXECUTABLE_OFFSET, 11
.equ SENDER_PUBKEY_OFFSET, 16
.equ SENDER_LAMPORTS_OFFSET, 80
.equ SENDER_DATA_LENGTH_OFFSET, 88
.equ SENDER_RENT_EPOCH_OFFSET, 10336

# Recipient account.
.equ RECIPIENT_OFFSET, 10344
.equ RECIPIENT_PUBKEY_OFFSET, 10352
.equ RECIPIENT_IS_SIGNER_OFFSET, 10345
.equ RECIPIENT_IS_WRITABLE_OFFSET, 10346
.equ RECIPIENT_DATA_LENGTH_OFFSET, 10424

# System program account.
.equ SYSTEM_PROGRAM_OFFSET, 20680
.equ SYSTEM_PROGRAM_PUBKEY_OFFSET, 20688

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

    # Allocate CPI data regions on stack.
    mov64 r9, r10
    sub64 r9, STACK_INSN_OFFSET
    mov64 r8, r10
    sub64 r8, STACK_INSN_DATA_OFFSET
    mov64 r7, r10
    mov64 r7, STACK_ACCT_METAS_OFFSET
    mov64 r6, r10
    mov64 r6, STACK_ACCT_INFOS_OFFSET

    # Set up instruction.
    mov64 r2, r9 # Load pointer to CPI instruction on stack.
    mov64 r3, r1 # Load pointer to input buffer.
    add64 r3, SYSTEM_PROGRAM_PUBKEY_OFFSET
    stxdw [r2 + CPI_INSN_PROGRAM_ID_ADDR_OFFSET], r3
    mov64 r3, r7 # Load pointer to CPI instruction account metas on stack.
    stxdw [r2 + CPI_INSN_ACCOUNTS_ADDR_OFFSET], r3
    stxdw [r2 + CPI_INSN_ACCOUNTS_LEN_OFFSET], CPI_ACCOUNTS_LEN
    mov64 r3, r8 # Load pointer to CPI instruction data on stack.
    stxdw [r2 + CPI_INSN_DATA_ADDR_OFFSET], r3
    stxdw [r2 + CPI_INSN_DATA_LEN_OFFSET], CPI_INSN_DATA_LEN

    # Set up instruction data.
    mov64 r2, r8
    mov32 r3, CPI_INSN_DATA_VARIANT
    stxw [r2 + CPI_INSN_DATA_VARIANT_OFFSET], r3
    stxdw [r2 + CPI_INSN_DATA_AMOUNT_OFFSET], r4

    # Parse sender account from input buffer into CPI metadata and info.
    # Start with fields that are copied, then step through pointers.
    mov64 r2, r7 # Account metadata array pointer.
    mov64 r3, r6 # Account info array pointer.
    ldxb r4, [r1 + SENDER_IS_SIGNER_OFFSET]
    stxb [r2 + CPI_ACCT_META_IS_SIGNER_OFFSET], r4
    stxb [r3 + CPI_ACCT_INFO_IS_SIGNER_OFFSET], r4
    ldxb r4, [r1 + SENDER_IS_WRITABLE_OFFSET]
    stxb [r2 + CPI_ACCT_META_IS_WRITABLE_OFFSET], r4
    stxb [r3 + CPI_ACCT_INFO_IS_WRITABLE_OFFSET], r4
    ldxb r4, [r1 + SENDER_IS_EXECUTABLE_OFFSET]
    stxb [r3 + CPI_ACCT_INFO_EXECUTABLE_OFFSET], r4
    ldxdw r4, [r1 + SENDER_DATA_LENGTH_OFFSET]
    stxdw [r3 + CPI_ACCT_INFO_DATA_LEN_OFFSET], r4
    ldxdw r4, [r1 + SENDER_RENT_EPOCH_OFFSET]
    stxdw [r3 + CPI_ACCT_INFO_RENT_EPOCH_OFFSET], r4
    mov64 r4, r1 # Begin stepping through pointer fields.
    add64 r4, SENDER_PUBKEY_OFFSET # Step to sender pubkey field pointer.
    stxdw [r2 + CPI_ACCT_META_PUBKEY_ADDR_OFFSET], r4
    stxdw [r3 + CPI_ACCT_INFO_KEY_ADDR_OFFSET], r4
    add64 r4, PUBKEY_SIZE_OF # Step to owner field pointer.
    stxdw [r3 + CPI_ACCT_INFO_OWNER_ADDR_OFFSET], r4
    add64 r4, PUBKEY_SIZE_OF # Step to Lamports balance pointer.
    stxdw [r3 + CPI_ACCT_INFO_LAMPORTS_ADDR_OFFSET], r4
    add64 r4, U16_SIZE_OF # Step over data length, to account data pointer.
    stxdw [r3 + CPI_ACCT_INFO_DATA_ADDR_OFFSET], r4

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
