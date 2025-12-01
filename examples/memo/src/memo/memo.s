.equ NUM_ACCOUNTS_OFFSET, 0
.equ INSTRUCTION_DATA_LENGTH_OFFSET, 8
.equ INSTRUCTION_DATA_OFFSET, 16

.globl entrypoint

entrypoint:
    // Indexed load the number of accounts into the return code.
    ldxdw r0, [r1 + NUM_ACCOUNTS_OFFSET]
    // If nonzero number of accounts, jump to exit instruction.
    jne r0, r4, 3
    // Indexed load the message data length.
    ldxdw r2, [r1 + INSTRUCTION_DATA_LENGTH_OFFSET]
    // Increment pointer in r1 the instruction data offset.
    add64 r1, INSTRUCTION_DATA_OFFSET
    call sol_log_
    exit

// Without mock .rodata the dump script fails.
.rodata
    null: .byte 0