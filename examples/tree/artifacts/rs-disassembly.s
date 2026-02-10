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
  jne r3, 4, jmp_0298
  ldxdw r3, [r1+88]
  jne r3, 0, jmp_0288
  ldxb r3, [r1+10344]
  jne r3, 255, jmp_02a8
  ldxdw r3, [r1+10424]
  jne r3, 0, jmp_02b8
  ldxb r3, [r1+20680]
  jne r3, 255, jmp_02c8
  ldxdw r3, [r1+20760]
  jne r3, 14, jmp_02d8
  ldxb r3, [r1+31032]
  jne r3, 255, jmp_02e8
  mov64 r0, 8
  mov32 r3, 399877894
  hor64 r3, 1364995097
  ldxdw r4, [r1+31040]
  jne r4, r3, jmp_0038
  mov32 r3, 1288277025
  hor64 r3, 2146519613
  ldxdw r4, [r1+31048]
  jne r4, r3, jmp_0038
  mov32 r3, 149871192
  hor64 r3, 1157472667
  ldxdw r4, [r1+31056]
  jne r4, r3, jmp_0038
  ldxdw r3, [r1+31064]
  mov32 r4, -1965433885
  jne r3, r4, jmp_0038
  ldxdw r2, [r2-8]
  jne r2, 0, jmp_02f8
  mov64 r6, r1
  add64 r6, 41400
  mov64 r4, r10
  add64 r4, 8
  mov64 r5, r10
  add64 r5, 7
  mov64 r7, r1
  mov64 r2, 0
  mov64 r3, r6
  call sol_try_find_program_address
  mov64 r0, 10
  ldxdw r1, [r10+8]
  ldxdw r2, [r7+10352]
  jne r1, r2, jmp_0038
  ldxdw r1, [r10+16]
  ldxdw r2, [r7+10360]
  jne r1, r2, jmp_0038
  ldxdw r1, [r10+24]
  ldxdw r2, [r7+10368]
  jne r1, r2, jmp_0038
  ldxdw r1, [r10+32]
  ldxdw r2, [r7+10376]
  jne r1, r2, jmp_0038
  ldxdw r1, [r7+31120]
  ldxdw r2, [r6+24]
  stxdw [r10+52], r2
  ldxdw r2, [r6+16]
  stxdw [r10+44], r2
  ldxdw r2, [r6+8]
  stxdw [r10+36], r2
  ldxdw r2, [r6+0]
  stxdw [r10+28], r2
  lmul64 r1, 144
  stxdw [r10+12], r1
  stdw [r10+20], 16
  stw [r10+8], 0
  mov64 r1, r10
  add64 r1, 8
  mov64 r2, 52
  call sol_set_return_data
  mov64 r0, 0
  ja jmp_0038

jmp_0288:
  mov64 r0, 2
  ja jmp_0038

jmp_0298:
  mov64 r0, 1
  ja jmp_0038

jmp_02a8:
  mov64 r0, 5
  ja jmp_0038

jmp_02b8:
  mov64 r0, 3
  ja jmp_0038

jmp_02c8:
  mov64 r0, 6
  ja jmp_0038

jmp_02d8:
  mov64 r0, 4
  ja jmp_0038

jmp_02e8:
  mov64 r0, 7
  ja jmp_0038

jmp_02f8:
  mov64 r0, 9
  ja jmp_0038
