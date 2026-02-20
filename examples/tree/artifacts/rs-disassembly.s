.globl entrypoint

entrypoint:
  add64 r10, -320
  ldxdw r4, [r1+0]
  ldxdw r3, [r2-8]
  ldxb r5, [r2+0]
  jne r5, 1, jmp_0780
  jne r3, 5, jmp_0cb0
  jlt r4, 2, jmp_0cc0
  ldxdw r3, [r1+88]
  jne r3, 0, jmp_0cd0
  ldxb r3, [r1+10344]
  jne r3, 255, jmp_0ce0
  mov64 r6, r1
  add64 r6, 10432
  ldxdw r3, [r1+10440]
  jeq r3, 0, jmp_0278
  ldxdw r4, [r3+0]
  stxdw [r1+10440], r4
  ldxw r1, [r2+1]
  stxw [r3+24], r1
  ldxdw r4, [r6+0]
  jeq r4, 0, jmp_05d8

jmp_00a8:
  ldxh r1, [r2+1]

jmp_00b0:
  mov64 r2, r4
  ldxh r4, [r2+24]
  mov64 r5, r1
  jle r5, r4, jmp_00e8
  ldxdw r4, [r2+16]
  jne r4, 0, jmp_00b0
  ja jmp_0120

jmp_00e8:
  jge r5, r4, jmp_0838
  ldxdw r4, [r2+8]
  jne r4, 0, jmp_00b0
  stxdw [r3+0], r2
  stb [r3+28], 1
  stxdw [r2+8], r3
  ja jmp_0138

jmp_0120:
  stxdw [r3+0], r2
  stb [r3+28], 1
  stxdw [r2+16], r3

jmp_0138:
  ldxb r1, [r2+28]
  jeq r1, 0, jmp_05f0
  mov64 r0, 0
  ja jmp_0188

jmp_0158:
  stb [r2+28], 0
  stb [r4+28], 0
  stb [r1+28], 1
  ldxdw r2, [r1+0]
  mov64 r3, r1
  jeq r2, 0, jmp_06a8

jmp_0188:
  ldxb r1, [r2+28]
  jeq r1, 0, jmp_06a8
  ldxdw r1, [r2+0]
  jeq r1, 0, jmp_0680
  ldxdw r4, [r1+8]
  jeq r2, r4, jmp_01d8
  jeq r4, 0, jmp_0600
  ldxb r5, [r4+28]
  jne r5, 0, jmp_0158
  ja jmp_0600

jmp_01d8:
  ldxdw r4, [r1+16]
  jeq r4, 0, jmp_01f8
  ldxb r5, [r4+28]
  jne r5, 0, jmp_0158

jmp_01f8:
  ldxdw r4, [r2+16]
  jeq r3, r4, jmp_0718
  mov64 r3, r4
  mov64 r4, r2
  stxdw [r1+8], r3
  ldxdw r2, [r1+0]
  jeq r3, 0, jmp_0238

jmp_0230:
  stxdw [r3+0], r1

jmp_0238:
  stxdw [r4+0], r2
  stxdw [r4+16], r1
  stxdw [r1+0], r4
  jeq r2, 0, jmp_0690
  ldxdw r3, [r2+16]
  jne r1, r3, jmp_0670

jmp_0268:
  stxdw [r2+16], r4
  ja jmp_0698

jmp_0278:
  jne r4, 4, jmp_0828
  ldxdw r3, [r1+10424]
  mov64 r5, r3
  add64 r5, 7
  and64 r5, -8
  mov64 r4, r1
  add64 r4, r5
  ldxb r5, [r4+20680]
  jne r5, 255, jmp_0cf0
  ldxdw r5, [r4+20760]
  jne r5, 14, jmp_0d00
  ldxb r5, [r4+31032]
  jne r5, 255, jmp_0d10
  mov64 r0, 8
  mov32 r5, 399877894
  hor64 r5, 1364995097
  ldxdw r7, [r4+31040]
  jne r7, r5, jmp_06a8
  mov32 r5, 1288277025
  hor64 r5, 2146519613
  ldxdw r7, [r4+31048]
  jne r7, r5, jmp_06a8
  mov32 r5, 149871192
  hor64 r5, 1157472667
  ldxdw r7, [r4+31056]
  jne r7, r5, jmp_06a8
  mov64 r7, r2
  ldxdw r2, [r4+31064]
  mov32 r5, -1965433885
  jne r2, r5, jmp_06a8
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

jmp_05d8:
  stdw [r3+0], 0
  stb [r3+28], 1
  stxdw [r6+0], r3

jmp_05f0:
  mov64 r0, 0
  ja jmp_06a8

jmp_0600:
  ldxdw r4, [r2+8]
  jeq r3, r4, jmp_06b0
  mov64 r3, r4
  mov64 r4, r2
  stxdw [r1+16], r3
  ldxdw r2, [r1+0]
  jeq r3, 0, jmp_0640

jmp_0638:
  stxdw [r3+0], r1

jmp_0640:
  stxdw [r4+0], r2
  stxdw [r4+8], r1
  stxdw [r1+0], r4
  jeq r2, 0, jmp_0690
  ldxdw r3, [r2+16]
  jeq r1, r3, jmp_0268

jmp_0670:
  stxdw [r2+8], r4
  ja jmp_0698

jmp_0680:
  stb [r2+28], 0
  ja jmp_06a8

jmp_0690:
  stxdw [r6+0], r4

jmp_0698:
  stb [r4+28], 0
  stb [r1+28], 1

jmp_06a8:
  exit

jmp_06b0:
  ldxdw r3, [r4+16]
  stxdw [r2+8], r3
  jeq r3, 0, jmp_06d0
  stxdw [r3+0], r2

jmp_06d0:
  stxdw [r4+0], r1
  stxdw [r4+16], r2
  stxdw [r2+0], r4
  stxdw [r1+16], r4
  ldxdw r3, [r4+8]
  stxdw [r1+16], r3
  ldxdw r2, [r1+0]
  jne r3, 0, jmp_0638
  ja jmp_0640

jmp_0718:
  ldxdw r3, [r4+8]
  stxdw [r2+16], r3
  jeq r3, 0, jmp_0738
  stxdw [r3+0], r2

jmp_0738:
  stxdw [r4+0], r1
  stxdw [r4+8], r2
  stxdw [r2+0], r4
  stxdw [r1+8], r4
  ldxdw r3, [r4+16]
  stxdw [r1+8], r3
  ldxdw r2, [r1+0]
  jne r3, 0, jmp_0230
  ja jmp_0238

jmp_0780:
  jne r5, 2, jmp_0848
  jne r3, 3, jmp_0cb0
  jlt r4, 2, jmp_0cc0
  ldxdw r3, [r1+88]
  jne r3, 0, jmp_0cd0
  ldxb r3, [r1+10344]
  jne r3, 255, jmp_0ce0
  mov64 r0, 15
  ldxdw r1, [r1+10432]
  jeq r1, 0, jmp_06a8
  ldxh r2, [r2+1]
  ja jmp_07f0

jmp_07e0:
  ldxdw r1, [r1+16]
  jeq r1, 0, jmp_06a8

jmp_07f0:
  ldxh r3, [r1+24]
  mov64 r4, r2
  jgt r4, r3, jmp_07e0
  jge r4, r3, jmp_05f0
  ldxdw r1, [r1+8]
  jne r1, 0, jmp_07f0
  ja jmp_06a8

jmp_0828:
  mov64 r0, 13
  ja jmp_06a8

jmp_0838:
  mov64 r0, 14
  ja jmp_06a8

jmp_0848:
  mov64 r8, r1
  jne r5, 0, jmp_0d30
  jne r3, 1, jmp_0cb0
  jne r4, 4, jmp_0cc0
  ldxdw r1, [r8+88]
  jne r1, 0, jmp_0cd0
  ldxb r1, [r8+10344]
  jne r1, 255, jmp_0ce0
  ldxdw r1, [r8+10424]
  jne r1, 0, jmp_0d20
  ldxb r1, [r8+20680]
  jne r1, 255, jmp_0cf0
  ldxdw r1, [r8+20760]
  jne r1, 14, jmp_0d00
  ldxb r1, [r8+31032]
  jne r1, 255, jmp_0d10
  mov64 r0, 8
  mov32 r1, 399877894
  hor64 r1, 1364995097
  ldxdw r2, [r8+31040]
  jne r2, r1, jmp_06a8
  mov32 r1, 1288277025
  hor64 r1, 2146519613
  ldxdw r2, [r8+31048]
  jne r2, r1, jmp_06a8
  mov32 r1, 149871192
  hor64 r1, 1157472667
  ldxdw r2, [r8+31056]
  jne r2, r1, jmp_06a8
  mov64 r7, r8
  ldxdw r1, [r7+31064]
  mov32 r2, -1965433885
  jne r1, r2, jmp_06a8
  mov64 r6, r7
  add64 r6, 41401
  mov64 r4, r10
  add64 r4, 104
  mov64 r5, r10
  add64 r5, 15
  mov64 r1, r7
  mov64 r2, 0
  mov64 r3, r6
  call sol_try_find_program_address
  mov64 r0, 10
  ldxdw r1, [r10+104]
  ldxdw r2, [r7+10352]
  jne r1, r2, jmp_06a8
  ldxdw r1, [r10+112]
  ldxdw r2, [r8+10360]
  jne r1, r2, jmp_06a8
  ldxdw r1, [r10+120]
  ldxdw r2, [r8+10368]
  jne r1, r2, jmp_06a8
  ldxdw r1, [r10+128]
  ldxdw r2, [r8+10376]
  jne r1, r2, jmp_06a8
  mov64 r1, r8
  add64 r1, 10352
  ldxdw r2, [r8+31120]
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
  mov64 r2, r8
  add64 r2, 16
  stxdw [r10+72], r2
  sth [r10+96], 257
  sth [r10+80], 257
  mov64 r3, r8
  add64 r3, 10384
  stxdw [r10+192], r3
  mov64 r3, r8
  add64 r3, 10432
  stxdw [r10+184], r3
  mov64 r3, r8
  add64 r3, 10416
  stxdw [r10+168], r3
  stxdw [r10+160], r1
  mov64 r1, r8
  add64 r1, 48
  stxdw [r10+136], r1
  mov64 r1, r8
  add64 r1, 96
  stxdw [r10+128], r1
  mov64 r1, r8
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
  mov64 r1, r8
  add64 r1, 10456
  stxdw [r8+10448], r1
  ja jmp_05f0

jmp_0cb0:
  mov64 r0, 12
  ja jmp_06a8

jmp_0cc0:
  mov64 r0, 1
  ja jmp_06a8

jmp_0cd0:
  mov64 r0, 2
  ja jmp_06a8

jmp_0ce0:
  mov64 r0, 5
  ja jmp_06a8

jmp_0cf0:
  mov64 r0, 6
  ja jmp_06a8

jmp_0d00:
  mov64 r0, 4
  ja jmp_06a8

jmp_0d10:
  mov64 r0, 7
  ja jmp_06a8

jmp_0d20:
  mov64 r0, 3
  ja jmp_06a8

jmp_0d30:
  mov64 r0, 11
  ja jmp_06a8
