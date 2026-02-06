.globl entrypoint

entrypoint:
  ldxdw r2, [r1+0]
  jeq r2, 2, jmp_0098
  mov64 r0, 1
  jne r2, 3, jmp_00b8
  ldxdw r2, [r1+88]
  jne r2, 0, jmp_00c0
  ldxb r2, [r1+10344]
  jne r2, 255, jmp_00d0
  ldxdw r2, [r1+10424]
  jne r2, 0, jmp_00e0
  ldxb r2, [r1+20680]
  jne r2, 255, jmp_00f0
  ldxdw r2, [r1+20760]
  jne r2, 14, jmp_0100
  mov64 r0, 0
  ldxdw r1, [r1+31032]
  jeq r1, 0, jmp_00b8
  mov64 r0, 7
  ja jmp_00b8

jmp_0098:
  ldxdw r1, [r1+88]
  mov64 r0, 6677
  jeq r1, 67, jmp_00b8
  mov64 r0, 666777

jmp_00b8:
  exit

jmp_00c0:
  mov64 r0, 2
  ja jmp_00b8

jmp_00d0:
  mov64 r0, 5
  ja jmp_00b8

jmp_00e0:
  mov64 r0, 3
  ja jmp_00b8

jmp_00f0:
  mov64 r0, 6
  ja jmp_00b8

jmp_0100:
  mov64 r0, 4
  ja jmp_00b8
