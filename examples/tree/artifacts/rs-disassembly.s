.globl entrypoint

entrypoint:
  ldxdw r1, [r1+0]
  and64 r1, -2
  mov32 r0, 1
  jne r1, 2, jmp_0028
  mov32 r0, 0

jmp_0028:
  exit
