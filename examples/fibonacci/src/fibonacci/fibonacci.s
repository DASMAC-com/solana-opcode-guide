.equ NUM_ACCOUNTS_OFFSET, 0
.equ INSTRUCTION_DATA_LENGTH_OFFSET, 8
.equ INSTRUCTION_DATA_OFFSET, 16
.equ E_ACCOUNTS, 1
.equ E_MAX_N, 2
.equ MAX_N, 93
.equ MAX_N_SPECIAL_CASE, 1

.global entrypoint

entrypoint:
    # Indexed load double word the number of accounts into r3, for use as a
    # scratch register.
    ldxdw r3, [r1 + NUM_ACCOUNTS_OFFSET]
    # If number of accounts is nonzero, jump to abort_accounts. Note r4
    # initially contains zero.
    jne r3, r4, abort_accounts

    # Indexed load single byte the sequence number into r8. Only check a single
    # byte since MAX_N < 256.
    ldxb r8, [r1 + INSTRUCTION_DATA_OFFSET]
    # If sequence number > MAX_N, jump to exit with error code E_MAX_N.
    jgt r8, MAX_N, abort_max_n

    # Prepare call-preserved registers for loop.
    # r6 = F(0) = 0, r7 = F(1) = 1. (r6 defaults to 0).
    mov64 r7, 1

    # F(n) = n for n = 0, 1. So compare sequence number to
    # MAX_N_SPECIAL_CASE then loop if not special case.
    jgt r8, MAX_N_SPECIAL_CASE, loop
    mov64 r0, r8
    exit

loop:
    # Decrement sequence number tracker for iteration. Using r9 as a scratch
    # register, increment the sequence numbers of the two Fibonacci numbers
    # being tracked. For example on the first iteration,
    # r6 = F(0), r7 = F(1)
    # ->
    # r6 = F(1), r7 = F(2).
    mov64 r9, r6
    mov64 r6, r7
    add64 r7, r9

    # Decrement sequence number counter.
    sub32 r8, 1
    # If sequence number counter > 1, continue loop.
    jgt r8, MAX_N_SPECIAL_CASE, loop
    # Now result in r7 = F(n), move into return code register.
    mov64 r0, r7
    exit

abort_accounts:
    sub64 r0, E_ACCOUNTS
    exit

abort_max_n:
    sub64 r0, E_MAX_N
    exit
