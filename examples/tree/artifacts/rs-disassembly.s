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
  jne r3, 4, jmp_0180
  ldxdw r3, [r1+88]
  jne r3, 0, jmp_0170
  ldxb r3, [r1+10344]
  jne r3, 255, jmp_0190
  ldxdw r3, [r1+10424]
  jne r3, 0, jmp_01a0
  ldxb r3, [r1+20680]
  jne r3, 255, jmp_01b0
  ldxdw r3, [r1+20760]
  jne r3, 14, jmp_01c0
  ldxdw r2, [r2-8]
  jne r2, 0, jmp_01d0
  mov64 r6, r1
  add64 r6, 10360
  mov64 r4, r10
  add64 r4, 24
  mov64 r5, r10
  add64 r5, 63
  mov64 r7, r1
  mov64 r2, 0
  mov64 r3, r6
  call sol_try_find_program_address
  ldxdw r1, [r10+24]
  ldxdw r2, [r7+10352]
  jne r1, r2, jmp_0160
  ldxdw r1, [r10+32]
  ldxdw r2, [r6+0]
  jne r1, r2, jmp_0160
  ldxdw r1, [r10+40]
  ldxdw r2, [r7+10368]
  jne r1, r2, jmp_0160
  ldxdw r1, [r10+48]
  mov64 r0, 0
  ldxdw r2, [r7+10376]
  jeq r1, r2, jmp_0038

jmp_0160:
  mov64 r0, 8
  ja jmp_0038

jmp_0170:
  mov64 r0, 2
  ja jmp_0038

jmp_0180:
  mov64 r0, 1
  ja jmp_0038

jmp_0190:
  mov64 r0, 5
  ja jmp_0038

jmp_01a0:
  mov64 r0, 3
  ja jmp_0038

jmp_01b0:
  mov64 r0, 6
  ja jmp_0038

jmp_01c0:
  mov64 r0, 4
  ja jmp_0038

jmp_01d0:
  mov64 r0, 7
  ja jmp_0038
