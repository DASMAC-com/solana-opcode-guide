.equ INSTRUCTION_DATA_LENGTH_OFFSET, 8
.equ INSTRUCTION_DATA_OFFSET, 16

.globl entrypoint
entrypoint:
    // Exit with error if any accounts are passed.
    mov64 r0, r1
    // Load into r2 a pointer to the instruction data length.
    ldxdw r2, [r1 + INSTRUCTION_DATA_LENGTH_OFFSET]
    // Increment pointer in r1 by the instruction data offset.
    add64 r1, INSTRUCTION_DATA_OFFSET
    call sol_log_
    exit
