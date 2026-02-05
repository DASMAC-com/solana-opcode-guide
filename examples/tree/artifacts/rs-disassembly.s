.globl entrypoint

entrypoint:
  ldxdw r1, [r1+0]
  mov32 r0, 1
  jne r1, 3, jmp_0020
  mov32 r0, 0

jmp_0020:
  exit
