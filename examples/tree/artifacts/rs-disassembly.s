.globl entrypoint

entrypoint:
  add64 r10, -320
  ldxdw r3, [r1+0]
  jne r3, 2, jmp_0068
  ldxdw r3, [r1+88]
  jne r3, 0, jmp_04c8
  ldxb r1, [r1+10344]
  jne r1, 255, jmp_04d8
  ldxb r1, [r2+0]
  jne r1, 1, jmp_04e8
  mov64 r0, 0
  ldxdw r1, [r2-8]
  jne r1, 5, jmp_04f8

jmp_0060:
  exit

jmp_0068:
  jne r3, 4, jmp_0508
  ldxdw r3, [r1+88]
  jne r3, 0, jmp_04c8
  ldxb r3, [r1+10344]
  jne r3, 255, jmp_04d8
  ldxdw r3, [r1+10424]
  jne r3, 0, jmp_0518
  ldxb r3, [r1+20680]
  jne r3, 255, jmp_0528
  ldxdw r3, [r1+20760]
  jne r3, 14, jmp_0538
  ldxb r3, [r1+31032]
  jne r3, 255, jmp_0548
  mov64 r0, 8
  mov32 r3, 399877894
  hor64 r3, 1364995097
  ldxdw r4, [r1+31040]
  jne r4, r3, jmp_0060
  mov32 r3, 1288277025
  hor64 r3, 2146519613
  ldxdw r4, [r1+31048]
  jne r4, r3, jmp_0060
  mov32 r3, 149871192
  hor64 r3, 1157472667
  ldxdw r4, [r1+31056]
  jne r4, r3, jmp_0060
  ldxdw r3, [r1+31064]
  mov32 r4, -1965433885
  jne r3, r4, jmp_0060
  ldxdw r2, [r2-8]
  jne r2, 0, jmp_0558
  mov64 r6, r1
  add64 r6, 41400
  mov64 r4, r10
  add64 r4, 104
  mov64 r5, r10
  add64 r5, 19
  mov64 r7, r1
  mov64 r2, 0
  mov64 r3, r6
  call sol_try_find_program_address
  mov64 r0, 10
  ldxdw r1, [r10+104]
  ldxdw r2, [r7+10352]
  jne r1, r2, jmp_0060
  ldxdw r1, [r10+112]
  ldxdw r2, [r7+10360]
  jne r1, r2, jmp_0060
  ldxdw r1, [r10+120]
  ldxdw r2, [r7+10368]
  jne r1, r2, jmp_0060
  ldxdw r1, [r10+128]
  ldxdw r2, [r7+10376]
  jne r1, r2, jmp_0060
  mov64 r1, r7
  add64 r1, 10352
  ldxdw r2, [r7+31120]
  ldxdw r3, [r6+24]
  stxdw [r10+64], r3
  ldxdw r3, [r6+16]
  stxdw [r10+56], r3
  ldxdw r3, [r6+8]
  stxdw [r10+48], r3
  ldxdw r3, [r6+0]
  stxdw [r10+40], r3
  lmul64 r2, 152
  stxdw [r10+24], r2
  stdw [r10+32], 24
  stw [r10+20], 0
  stxdw [r10+88], r1
  mov64 r2, r7
  add64 r2, 16
  stxdw [r10+72], r2
  sth [r10+96], 257
  sth [r10+80], 257
  mov64 r3, r7
  add64 r3, 10384
  stxdw [r10+192], r3
  mov64 r3, r7
  add64 r3, 10432
  stxdw [r10+184], r3
  mov64 r3, r7
  add64 r3, 10416
  stxdw [r10+168], r3
  stxdw [r10+160], r1
  mov64 r1, r7
  add64 r1, 48
  stxdw [r10+136], r1
  mov64 r1, r7
  add64 r1, 96
  stxdw [r10+128], r1
  mov64 r1, r7
  add64 r1, 80
  stxdw [r10+112], r1
  stxdw [r10+104], r2
  stb [r10+210], 0
  sth [r10+208], 257
  stdw [r10+200], 0
  stdw [r10+176], 0
  stb [r10+154], 0
  sth [r10+152], 257
  stdw [r10+144], 0
  stdw [r10+120], 0
  stdw [r10+240], 0
  stdw [r10+232], 0
  stdw [r10+224], 0
  stdw [r10+216], 0
  mov64 r1, r10
  add64 r1, 20
  stxdw [r10+272], r1
  mov64 r1, r10
  add64 r1, 72
  stxdw [r10+256], r1
  mov64 r1, r10
  add64 r1, 216
  stxdw [r10+248], r1
  stdw [r10+280], 52
  stdw [r10+264], 2
  mov64 r1, r10
  add64 r1, 19
  stxdw [r10+288], r1
  stdw [r10+296], 1
  mov64 r1, r10
  add64 r1, 288
  stxdw [r10+304], r1
  stdw [r10+312], 1
  mov64 r1, r10
  add64 r1, 248
  mov64 r2, r10
  add64 r2, 104
  mov64 r4, r10
  add64 r4, 304
  mov64 r3, 2
  mov64 r5, 1
  call sol_invoke_signed_c
  mov64 r1, r7
  add64 r1, 10456
  stxdw [r7+10448], r1
  mov64 r0, 0
  ja jmp_0060

jmp_04c8:
  mov64 r0, 2
  ja jmp_0060

jmp_04d8:
  mov64 r0, 5
  ja jmp_0060

jmp_04e8:
  mov64 r0, 11
  ja jmp_0060

jmp_04f8:
  mov64 r0, 12
  ja jmp_0060

jmp_0508:
  mov64 r0, 1
  ja jmp_0060

jmp_0518:
  mov64 r0, 3
  ja jmp_0060

jmp_0528:
  mov64 r0, 6
  ja jmp_0060

jmp_0538:
  mov64 r0, 4
  ja jmp_0060

jmp_0548:
  mov64 r0, 7
  ja jmp_0060

jmp_0558:
  mov64 r0, 9
  ja jmp_0060
