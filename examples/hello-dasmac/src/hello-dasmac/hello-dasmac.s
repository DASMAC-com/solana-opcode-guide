.equ MESSAGE_LENGTH, 14

.globl entrypoint

entrypoint:
  lddw r1, message
  lddw r2, MESSAGE_LENGTH
  call sol_log_
  exit

.rodata
  message: .ascii "Hello, DASMAC!"
