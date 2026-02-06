.globl entrypoint

entrypoint:
  ldxdw r2, [r1+0]
  jne r2, 2, jmp_0038
  ldxdw r1, [r1+88]
  mov64 r0, 6677
  jeq r1, 67, jmp_0030
  mov64 r0, 666777

jmp_0030:
  exit

jmp_0038:
  jne r2, 3, jmp_00c8
  ldxdw r2, [r1+88]
  jne r2, 0, jmp_00b8
  ldxb r2, [r1+10344]
  jne r2, 255, jmp_00d8
  ldxdw r2, [r1+10424]
  jne r2, 0, jmp_00e8
  ldxb r2, [r1+20680]
  jne r2, 255, jmp_00f8
  ldxdw r2, [r1+20760]
  jne r2, 14, jmp_0108
  mov64 r0, 0
  ldxdw r1, [r1+31032]
  jeq r1, 0, jmp_0030
  mov64 r0, 7
  ja jmp_0030

jmp_00b8:
  mov64 r0, 2
  ja jmp_0030

jmp_00c8:
  mov64 r0, 1
  ja jmp_0030

jmp_00d8:
  mov64 r0, 5
  ja jmp_0030

jmp_00e8:
  mov64 r0, 3
  ja jmp_0030

jmp_00f8:
  mov64 r0, 6
  ja jmp_0030

jmp_0108:
  mov64 r0, 4
  ja jmp_0030
