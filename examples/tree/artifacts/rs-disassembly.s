.globl entrypoint

entrypoint:
  add64 r10, -320
  ldxdw r4, [r1+0]
  ldxdw r3, [r2-8]
  ldxb r5, [r2+0]
  jne r5, 1, jmp_0800
  jne r3, 5, jmp_0c58
  jlt r4, 2, jmp_0c68
  ldxdw r3, [r1+88]
  jne r3, 0, jmp_0c78
  ldxb r3, [r1+10344]
  jne r3, 255, jmp_0c88
  mov64 r6, r1
  add64 r6, 10432
  ldxdw r3, [r1+10440]
  jeq r3, 0, jmp_0260
  ldxdw r4, [r3+0]
  stxdw [r1+10440], r4
  ldxw r1, [r2+1]
  stxw [r3+24], r1
  ldxdw r4, [r6+0]
  jeq r4, 0, jmp_05c0

jmp_00a8:
  ldxh r2, [r2+1]
  ja jmp_00d8

jmp_00b8:
  mov64 r5, r1
  add64 r5, r4
  ldxdw r4, [r5+0]
  jeq r4, 0, jmp_0120

jmp_00d8:
  mov64 r1, r4
  mov64 r4, 16
  ldxh r5, [r1+24]
  mov64 r0, r2
  jgt r0, r5, jmp_00b8
  mov64 r4, 8
  jlt r0, r5, jmp_00b8
  mov64 r0, 14
  ja jmp_07f8

jmp_0120:
  stxdw [r10+0], r6
  stxdw [r3+0], r1
  stb [r3+28], 1
  mov64 r4, r2
  ldxh r5, [r1+24]
  mov32 r2, 1
  jgt r4, r5, jmp_0160
  mov32 r2, 0

jmp_0160:
  lsh64 r2, 3
  mov64 r4, r1
  add64 r4, r2
  stxdw [r4+8], r3
  mov64 r0, 0

jmp_0188:
  ldxb r2, [r1+28]
  jeq r2, 0, jmp_07f8
  ldxdw r2, [r1+0]
  jeq r2, 0, jmp_0638
  ldxdw r5, [r2+16]
  mov32 r4, 1
  jeq r1, r5, jmp_01c8
  mov32 r4, 0

jmp_01c8:
  mov64 r5, r2
  add64 r5, 8
  mov64 r7, r4
  xor64 r7, 1
  mov64 r8, r7
  lsh64 r8, 3
  mov64 r9, r5
  add64 r9, r8
  ldxdw r9, [r9+0]
  jeq r9, 0, jmp_05e8
  ldxb r6, [r9+28]
  jeq r6, 0, jmp_05e8
  stb [r1+28], 0
  stb [r9+28], 0
  stb [r2+28], 1
  ldxdw r1, [r2+0]
  mov64 r3, r2
  jne r1, 0, jmp_0188
  ja jmp_07f8

jmp_0260:
  jne r4, 4, jmp_0cb8
  ldxdw r3, [r1+10424]
  mov64 r5, r3
  add64 r5, 7
  and64 r5, -8
  mov64 r4, r1
  add64 r4, r5
  ldxb r5, [r4+20680]
  jne r5, 255, jmp_0c98
  ldxdw r5, [r4+20760]
  jne r5, 14, jmp_0ca8
  ldxb r5, [r4+31032]
  jne r5, 255, jmp_0cc8
  mov64 r0, 8
  mov32 r5, 399877894
  hor64 r5, 1364995097
  ldxdw r7, [r4+31040]
  jne r7, r5, jmp_07f8
  mov32 r5, 1288277025
  hor64 r5, 2146519613
  ldxdw r7, [r4+31048]
  jne r7, r5, jmp_07f8
  mov32 r5, 149871192
  hor64 r5, 1157472667
  ldxdw r7, [r4+31056]
  jne r7, r5, jmp_07f8
  mov64 r7, r2
  ldxdw r2, [r4+31064]
  mov32 r5, -1965433885
  jne r2, r5, jmp_07f8
  ldxdw r2, [r4+31120]
  lmul64 r2, 29
  stxdw [r10+308], r2
  stw [r10+304], 2
  mov64 r4, r1
  add64 r4, 10352
  stxdw [r10+232], r4
  mov64 r2, r1
  add64 r2, 16
  stxdw [r10+216], r2
  sth [r10+240], 1
  sth [r10+224], 257
  mov64 r5, r1
  add64 r5, 10384
  stxdw [r10+192], r5
  stxdw [r10+184], r6
  stxdw [r10+176], r3
  mov64 r3, r1
  add64 r3, 10416
  stxdw [r10+168], r3
  stxdw [r10+160], r4
  mov64 r3, r1
  add64 r3, 48
  stxdw [r10+136], r3
  mov64 r3, r1
  add64 r3, 96
  stxdw [r10+128], r3
  mov64 r3, r1
  add64 r3, 80
  stxdw [r10+112], r3
  stxdw [r10+104], r2
  stb [r10+210], 0
  sth [r10+208], 256
  stdw [r10+200], 0
  stb [r10+154], 0
  sth [r10+152], 257
  stdw [r10+144], 0
  stdw [r10+120], 0
  stdw [r10+272], 0
  stdw [r10+264], 0
  stdw [r10+256], 0
  stdw [r10+248], 0
  mov64 r2, r10
  add64 r2, 304
  stxdw [r10+40], r2
  mov64 r2, r10
  add64 r2, 216
  stxdw [r10+24], r2
  mov64 r2, r10
  add64 r2, 248
  stxdw [r10+16], r2
  stdw [r10+48], 12
  stdw [r10+32], 2
  stdw [r10+80], 0
  stdw [r10+72], 0
  mov64 r3, r10
  add64 r3, 16
  mov64 r2, r10
  add64 r2, 104
  mov64 r4, r10
  add64 r4, 72
  mov64 r8, r1
  mov64 r1, r3
  mov64 r3, 2
  mov64 r5, 0
  call sol_invoke_signed_c
  ldxdw r1, [r8+10424]
  add64 r1, 29
  stxdw [r8+10424], r1
  ldxdw r3, [r8+10448]
  mov64 r1, r3
  add64 r1, 29
  stxdw [r8+10448], r1
  mov64 r2, r7
  ldxw r1, [r2+1]
  stxw [r3+24], r1
  ldxdw r4, [r6+0]
  jne r4, 0, jmp_00a8

jmp_05c0:
  stdw [r3+0], 0
  stb [r3+28], 1
  stxdw [r6+0], r3

jmp_05d8:
  mov64 r0, 0
  ja jmp_07f8

jmp_05e8:
  mov64 r9, r1
  add64 r9, r8
  ldxdw r8, [r9+8]
  jeq r3, r8, jmp_0648
  mov64 r3, r4
  lsh64 r3, 3
  mov64 r6, r5
  add64 r6, r3
  ldxdw r3, [r6+0]
  ja jmp_0720

jmp_0638:
  stb [r1+28], 0
  ja jmp_07f8

jmp_0648:
  add64 r9, 8
  mov64 r6, r4
  lsh64 r6, 3
  mov64 r3, r8
  add64 r3, r6
  ldxdw r6, [r3+8]
  stxdw [r9+0], r6
  add64 r3, 8
  jeq r6, 0, jmp_0698
  stxdw [r6+0], r1

jmp_0698:
  stxdw [r3+0], r1
  stxdw [r8+0], r2
  stxdw [r1+0], r8
  ldxdw r6, [r2+16]
  mov32 r3, 1
  jeq r1, r6, jmp_06d0
  mov32 r3, 0

jmp_06d0:
  lsh64 r3, 3
  mov64 r1, r5
  add64 r1, r3
  stxdw [r1+0], r8
  mov64 r1, r4
  lsh64 r1, 3
  mov64 r3, r5
  add64 r3, r1
  ldxdw r3, [r3+0]
  mov64 r1, r3

jmp_0720:
  lsh64 r4, 3
  add64 r5, r4
  lsh64 r7, 3
  mov64 r8, r3
  add64 r8, r7
  ldxdw r4, [r2+0]
  ldxdw r7, [r8+8]
  stxdw [r5+0], r7
  add64 r8, 8
  jeq r7, 0, jmp_0778
  stxdw [r7+0], r2

jmp_0778:
  stxdw [r8+0], r2
  stxdw [r3+0], r4
  stxdw [r2+0], r3
  jeq r4, 0, jmp_07d8
  ldxdw r6, [r4+16]
  mov32 r5, 1
  jeq r2, r6, jmp_07b8
  mov32 r5, 0

jmp_07b8:
  lsh64 r5, 3
  add64 r4, r5
  stxdw [r4+8], r3
  ja jmp_07e8

jmp_07d8:
  ldxdw r4, [r10+0]
  stxdw [r4+0], r3

jmp_07e8:
  stb [r1+28], 0
  stb [r2+28], 1

jmp_07f8:
  exit

jmp_0800:
  jne r5, 0, jmp_0cd8
  jne r3, 1, jmp_0c58
  jne r4, 4, jmp_0c68
  ldxdw r2, [r1+88]
  jne r2, 0, jmp_0c78
  ldxb r2, [r1+10344]
  jne r2, 255, jmp_0c88
  ldxdw r2, [r1+10424]
  jne r2, 0, jmp_0ce8
  ldxb r2, [r1+20680]
  jne r2, 255, jmp_0c98
  ldxdw r2, [r1+20760]
  jne r2, 14, jmp_0ca8
  ldxb r2, [r1+31032]
  jne r2, 255, jmp_0cc8
  mov64 r0, 8
  mov32 r2, 399877894
  hor64 r2, 1364995097
  ldxdw r3, [r1+31040]
  jne r3, r2, jmp_07f8
  mov32 r2, 1288277025
  hor64 r2, 2146519613
  ldxdw r3, [r1+31048]
  jne r3, r2, jmp_07f8
  mov32 r2, 149871192
  hor64 r2, 1157472667
  ldxdw r3, [r1+31056]
  jne r3, r2, jmp_07f8
  ldxdw r2, [r1+31064]
  mov32 r3, -1965433885
  jne r2, r3, jmp_07f8
  mov64 r6, r1
  add64 r6, 41401
  mov64 r4, r10
  add64 r4, 104
  mov64 r5, r10
  add64 r5, 15
  mov64 r7, r1
  mov64 r2, 0
  mov64 r3, r6
  call sol_try_find_program_address
  mov64 r0, 10
  ldxdw r1, [r10+104]
  ldxdw r2, [r7+10352]
  jne r1, r2, jmp_07f8
  ldxdw r1, [r10+112]
  ldxdw r2, [r7+10360]
  jne r1, r2, jmp_07f8
  ldxdw r1, [r10+120]
  ldxdw r2, [r7+10368]
  jne r1, r2, jmp_07f8
  ldxdw r1, [r10+128]
  ldxdw r2, [r7+10376]
  jne r1, r2, jmp_07f8
  mov64 r1, r7
  add64 r1, 10352
  ldxdw r2, [r7+31120]
  ldxdw r3, [r6+24]
  stxdw [r10+60], r3
  ldxdw r3, [r6+16]
  stxdw [r10+52], r3
  ldxdw r3, [r6+8]
  stxdw [r10+44], r3
  ldxdw r3, [r6+0]
  stxdw [r10+36], r3
  lmul64 r2, 152
  stxdw [r10+20], r2
  stdw [r10+28], 24
  stw [r10+16], 0
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
  add64 r1, 16
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
  add64 r1, 15
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
  ja jmp_05d8

jmp_0c58:
  mov64 r0, 12
  ja jmp_07f8

jmp_0c68:
  mov64 r0, 1
  ja jmp_07f8

jmp_0c78:
  mov64 r0, 2
  ja jmp_07f8

jmp_0c88:
  mov64 r0, 5
  ja jmp_07f8

jmp_0c98:
  mov64 r0, 6
  ja jmp_07f8

jmp_0ca8:
  mov64 r0, 4
  ja jmp_07f8

jmp_0cb8:
  mov64 r0, 13
  ja jmp_07f8

jmp_0cc8:
  mov64 r0, 7
  ja jmp_07f8

jmp_0cd8:
  mov64 r0, 11
  ja jmp_07f8

jmp_0ce8:
  mov64 r0, 3
  ja jmp_07f8
