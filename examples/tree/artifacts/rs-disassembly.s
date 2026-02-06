.globl entrypoint

entrypoint:
  ldxdw r2, [r1+0]
  jeq r2, 3, jmp_0030
  mov64 r0, 1
  jne r2, 2, jmp_00c8
  mov64 r0, 0
  ja jmp_00c8

jmp_0030:
  mov64 r0, 2
  ldxdw r2, [r1+88]
  jne r2, 0, jmp_00c8
  mov64 r0, 5
  ldxb r2, [r1+10344]
  jne r2, 255, jmp_00c8
  mov64 r0, 3
  ldxdw r2, [r1+10424]
  jne r2, 0, jmp_00c8
  mov64 r0, 6
  ldxb r2, [r1+20680]
  jne r2, 255, jmp_00c8
  mov64 r0, 4
  ldxdw r2, [r1+20760]
  jne r2, 14, jmp_00c8
  ldxdw r1, [r1+31032]
  mov64 r0, 0
  jeq r1, 0, jmp_00c8
  mov64 r0, 7

jmp_00c8:
  exit
