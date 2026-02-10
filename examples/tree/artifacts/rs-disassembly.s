.globl entrypoint

entrypoint:
  add64 r10, -64
  ldxdw r3, [r1+0]
  jne r3, 2, jmp_0040
  ldxdw r1, [r1+88]
  mov64 r0, 6677
  jeq r1, 67, jmp_0038
  mov64 r0, 666777

jmp_0038:
  exit

jmp_0040:
  jne r3, 3, jmp_0178
  ldxdw r3, [r1+88]
  jne r3, 0, jmp_0168
  ldxb r3, [r1+10344]
  jne r3, 255, jmp_0188
  ldxdw r3, [r1+10424]
  jne r3, 0, jmp_0198
  ldxb r3, [r1+20680]
  jne r3, 255, jmp_01a8
  ldxdw r3, [r1+20760]
  jne r3, 14, jmp_01b8
  ldxdw r2, [r2-8]
  jne r2, 0, jmp_01c8
  mov64 r3, r1
  add64 r3, 31040
  mov64 r4, r10
  add64 r4, 24
  mov64 r5, r10
  add64 r5, 63
  mov64 r6, r1
  mov64 r2, 0
  call sol_try_find_program_address
  mov64 r0, 8
  ldxdw r1, [r10+24]
  ldxdw r2, [r6+10352]
  jne r1, r2, jmp_0038
  ldxdw r1, [r10+32]
  ldxdw r2, [r6+10360]
  jne r1, r2, jmp_0038
  ldxdw r1, [r10+40]
  ldxdw r2, [r6+10368]
  jne r1, r2, jmp_0038
  ldxdw r1, [r10+48]
  ldxdw r2, [r6+10376]
  jne r1, r2, jmp_0038
  mov64 r0, 0
  ja jmp_0038

jmp_0168:
  mov64 r0, 2
  ja jmp_0038

jmp_0178:
  mov64 r0, 1
  ja jmp_0038

jmp_0188:
  mov64 r0, 5
  ja jmp_0038

jmp_0198:
  mov64 r0, 3
  ja jmp_0038

jmp_01a8:
  mov64 r0, 6
  ja jmp_0038

jmp_01b8:
  mov64 r0, 4
  ja jmp_0038

jmp_01c8:
  mov64 r0, 7
  ja jmp_0038
