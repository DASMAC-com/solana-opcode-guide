.globl entrypoint

entrypoint:
  add64 r10, -384
  ldxdw r4, [r1+0]
  ldxdw r3, [r2-8]
  ldxb r5, [r2+0]
  jne r5, 1, jmp_0780
  jne r3, 5, jmp_1518
  jlt r4, 2, jmp_1528
  ldxdw r3, [r1+88]
  jne r3, 0, jmp_1538
  ldxb r3, [r1+10344]
  jne r3, 255, jmp_1548
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
  jge r5, r4, jmp_0ce0
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
  jne r4, 4, jmp_0830
  ldxdw r3, [r1+10424]
  mov64 r5, r3
  add64 r5, 7
  and64 r5, -8
  mov64 r4, r1
  add64 r4, r5
  ldxb r5, [r4+20680]
  jne r5, 255, jmp_1558
  ldxdw r5, [r4+20760]
  jne r5, 14, jmp_1568
  ldxb r5, [r4+31032]
  jne r5, 255, jmp_1578
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
  stxdw [r10+372], r2
  stw [r10+368], 2
  mov64 r4, r1
  add64 r4, 10352
  stxdw [r10+296], r4
  mov64 r2, r1
  add64 r2, 16
  stxdw [r10+280], r2
  sth [r10+304], 1
  sth [r10+288], 257
  mov64 r5, r1
  add64 r5, 10384
  stxdw [r10+256], r5
  stxdw [r10+248], r6
  stxdw [r10+240], r3
  mov64 r3, r1
  add64 r3, 10416
  stxdw [r10+232], r3
  stxdw [r10+224], r4
  mov64 r3, r1
  add64 r3, 48
  stxdw [r10+200], r3
  mov64 r3, r1
  add64 r3, 96
  stxdw [r10+192], r3
  mov64 r3, r1
  add64 r3, 80
  stxdw [r10+176], r3
  stxdw [r10+168], r2
  stb [r10+274], 0
  sth [r10+272], 256
  stdw [r10+264], 0
  stb [r10+218], 0
  sth [r10+216], 257
  stdw [r10+208], 0
  stdw [r10+184], 0
  stdw [r10+336], 0
  stdw [r10+328], 0
  stdw [r10+320], 0
  stdw [r10+312], 0
  mov64 r2, r10
  add64 r2, 368
  stxdw [r10+104], r2
  mov64 r2, r10
  add64 r2, 280
  stxdw [r10+88], r2
  mov64 r2, r10
  add64 r2, 312
  stxdw [r10+80], r2
  stdw [r10+112], 12
  stdw [r10+96], 2
  stdw [r10+144], 0
  stdw [r10+136], 0
  mov64 r3, r10
  add64 r3, 80
  mov64 r2, r10
  add64 r2, 168
  mov64 r4, r10
  add64 r4, 136
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
  jne r5, 2, jmp_10b0
  jne r3, 3, jmp_1518
  jlt r4, 2, jmp_1528
  ldxdw r3, [r1+88]
  jne r3, 0, jmp_1538
  ldxb r3, [r1+10344]
  jne r3, 255, jmp_1548
  mov64 r0, 15
  ldxdw r3, [r1+10432]
  jeq r3, 0, jmp_06a8
  ldxh r4, [r2+1]
  ja jmp_07f0

jmp_07e0:
  ldxdw r3, [r3+16]
  jeq r3, 0, jmp_06a8

jmp_07f0:
  ldxh r5, [r3+24]
  mov64 r6, r4
  jgt r6, r5, jmp_07e0
  ldxdw r2, [r3+8]
  jge r6, r5, jmp_0840
  mov64 r3, r2
  jne r2, 0, jmp_07f0
  ja jmp_06a8

jmp_0830:
  mov64 r0, 13
  ja jmp_06a8

jmp_0840:
  stxdw [r10+64], r1
  jeq r2, 0, jmp_0890
  ldxdw r1, [r3+16]
  jeq r1, 0, jmp_0938

jmp_0860:
  mov64 r2, r1
  ldxdw r1, [r2+8]
  jne r1, 0, jmp_0860
  ldxw r1, [r2+24]
  stxw [r3+24], r1
  ja jmp_0898

jmp_0890:
  mov64 r2, r3

jmp_0898:
  ldxdw r4, [r2+0]
  mov64 r1, r2
  add64 r1, 8
  stxdw [r10+48], r1
  ldxdw r1, [r2+16]
  jeq r1, 0, jmp_0900
  stxdw [r1+0], r4
  stb [r1+28], 0
  jeq r4, 0, jmp_0988
  ldxdw r3, [r4+16]
  jeq r2, r3, jmp_0b10
  stxdw [r4+8], r1
  ja jmp_1070

jmp_0900:
  jeq r4, 0, jmp_1588
  ldxdw r1, [r4+16]
  ldxb r3, [r2+28]
  jne r3, 1, jmp_09a0
  jeq r2, r1, jmp_0cd0
  stdw [r4+8], 0
  ja jmp_1070

jmp_0938:
  mov64 r1, r3
  add64 r1, 8
  ldxdw r4, [r3+0]
  stxdw [r2+0], r4
  stb [r2+28], 0
  jeq r4, 0, jmp_0b20
  ldxdw r5, [r4+16]
  jeq r3, r5, jmp_0c90
  stxdw [r4+8], r2
  ja jmp_0c98

jmp_0988:
  ldxdw r3, [r10+64]
  stxdw [r3+10432], r1
  ja jmp_1070

jmp_09a0:
  mov32 r3, 1
  stxdw [r10+32], r3
  jeq r2, r1, jmp_09c8
  mov32 r1, 0
  stxdw [r10+32], r1

jmp_09c8:
  mov64 r8, r4
  add64 r8, 8
  ldxdw r5, [r10+32]
  mov64 r6, r5
  lsh64 r6, 3
  mov64 r1, r8
  add64 r1, r6
  stdw [r1+0], 0
  xor64 r5, 1
  mov64 r9, r5
  lsh64 r9, 3
  add64 r8, r9
  ldxdw r3, [r8+0]
  mov64 r7, r3
  add64 r7, 28
  mov64 r1, r3
  add64 r1, 8
  mov64 r0, r1
  add64 r0, r6
  ldxdw r6, [r0+0]
  stxdw [r10+56], r6
  stxdw [r10+40], r3
  ldxb r6, [r3+28]
  jeq r6, 0, jmp_0b38
  mov64 r6, r4
  ldxdw r1, [r6+0]
  ldxdw r3, [r10+56]
  stxdw [r8+0], r3
  jeq r3, 0, jmp_0ac0

jmp_0ab0:
  ldxdw r3, [r10+56]
  stxdw [r3+0], r6

jmp_0ac0:
  stxdw [r0+0], r6
  ldxdw r3, [r10+40]
  stxdw [r3+0], r1
  stxdw [r6+0], r3
  jeq r1, 0, jmp_0cf0
  ldxdw r4, [r1+16]
  jeq r6, r4, jmp_0d10
  ldxdw r3, [r10+40]
  stxdw [r1+8], r3
  ja jmp_0d20

jmp_0b10:
  stxdw [r4+16], r1
  ja jmp_1070

jmp_0b20:
  ldxdw r4, [r10+64]
  stxdw [r4+10432], r2
  ja jmp_0c98

jmp_0b38:
  add64 r1, r9
  ja jmp_0c00

jmp_0b48:
  mov64 r5, r3
  xor64 r5, 1
  mov64 r4, r5
  lsh64 r4, 3
  mov64 r8, r6
  add64 r8, r4
  ldxdw r9, [r8+8]
  mov64 r0, r9
  add64 r0, 8
  mov64 r1, r0
  add64 r1, r4
  stxdw [r10+32], r3
  mov64 r4, r3
  lsh64 r4, 3
  add64 r0, r4
  mov64 r7, r9
  add64 r7, 28
  ldxdw r3, [r0+0]
  stxdw [r10+56], r3
  stxdw [r10+40], r9
  ldxb r3, [r9+28]
  mov64 r4, r6
  jne r3, 0, jmp_0ef8

jmp_0c00:
  ldxdw r1, [r1+0]
  jeq r1, 0, jmp_0c20
  ldxb r0, [r1+28]
  jne r0, 0, jmp_0f40

jmp_0c20:
  ldxdw r9, [r10+56]
  jeq r9, 0, jmp_0c40
  ldxb r1, [r9+28]
  jne r1, 0, jmp_0df8

jmp_0c40:
  ldxb r1, [r4+28]
  stb [r7+0], 1
  jeq r1, 1, jmp_0ee8
  ldxdw r6, [r4+0]
  jeq r6, 0, jmp_1070
  ldxdw r1, [r6+16]
  mov32 r3, 1
  jeq r4, r1, jmp_0b48
  mov32 r3, 0
  ja jmp_0b48

jmp_0c90:
  stxdw [r4+16], r2

jmp_0c98:
  stdw [r1+8], 0
  stdw [r1+0], 0
  ldxdw r1, [r10+64]
  ldxdw r2, [r1+10440]
  stxdw [r3+0], r2
  stxdw [r1+10440], r3
  ja jmp_05f0

jmp_0cd0:
  stdw [r4+16], 0
  ja jmp_1070

jmp_0ce0:
  mov64 r0, 14
  ja jmp_06a8

jmp_0cf0:
  ldxdw r1, [r10+64]
  ldxdw r3, [r10+40]
  stxdw [r1+10432], r3
  ja jmp_0d20

jmp_0d10:
  ldxdw r3, [r10+40]
  stxdw [r1+16], r3

jmp_0d20:
  stb [r6+28], 1
  stb [r7+0], 0
  mov64 r3, r5
  lsh64 r3, 3
  ldxdw r1, [r10+56]
  add64 r1, 8
  mov64 r4, r1
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r10+40], r3
  jeq r3, 0, jmp_0da8
  ldxdw r3, [r10+40]
  ldxb r4, [r3+28]
  jeq r4, 0, jmp_0da8
  mov64 r4, r6
  ldxdw r9, [r10+56]
  ja jmp_0f78

jmp_0da8:
  ldxdw r3, [r10+32]
  lsh64 r3, 3
  add64 r1, r3
  ldxdw r9, [r1+0]
  jeq r9, 0, jmp_0ec8
  ldxb r1, [r9+28]
  jeq r1, 0, jmp_0ec8
  mov64 r4, r6
  ldxdw r1, [r10+56]
  stxdw [r10+40], r1

jmp_0df8:
  mov64 r1, r5
  lsh64 r1, 3
  mov64 r0, r9
  add64 r0, r1
  ldxdw r1, [r10+32]
  lsh64 r1, 3
  ldxdw r6, [r10+40]
  mov64 r3, r6
  add64 r3, r1
  ldxdw r1, [r6+0]
  ldxdw r6, [r0+8]
  stxdw [r3+8], r6
  add64 r0, 8
  jeq r6, 0, jmp_0e78
  ldxdw r3, [r10+40]
  stxdw [r6+0], r3

jmp_0e78:
  ldxdw r3, [r10+40]
  stxdw [r0+0], r3
  stxdw [r9+0], r1
  stxdw [r3+0], r9
  jeq r1, 0, jmp_0f28
  ldxdw r3, [r1+16]
  ldxdw r0, [r10+40]
  jeq r0, r3, jmp_0f58
  stxdw [r1+8], r9
  ja jmp_0f60

jmp_0ec8:
  ldxdw r1, [r10+56]
  stb [r1+28], 1
  stb [r6+28], 0
  ja jmp_1070

jmp_0ee8:
  stb [r4+28], 0
  ja jmp_1070

jmp_0ef8:
  add64 r8, 8
  ldxdw r1, [r6+0]
  ldxdw r3, [r10+56]
  stxdw [r8+0], r3
  jne r3, 0, jmp_0ab0
  ja jmp_0ac0

jmp_0f28:
  ldxdw r1, [r10+64]
  stxdw [r1+10432], r9
  ja jmp_0f60

jmp_0f40:
  ldxdw r9, [r10+40]
  stxdw [r10+40], r1
  ja jmp_0f78

jmp_0f58:
  stxdw [r1+16], r9

jmp_0f60:
  ldxdw r1, [r10+40]
  stb [r1+28], 1
  stb [r9+28], 0

jmp_0f78:
  lsh64 r5, 3
  mov64 r3, r4
  add64 r3, r5
  ldxdw r5, [r10+32]
  lsh64 r5, 3
  ldxdw r1, [r3+8]
  mov64 r0, r1
  add64 r0, r5
  ldxdw r5, [r4+0]
  ldxdw r6, [r0+8]
  stxdw [r3+8], r6
  add64 r0, 8
  jeq r6, 0, jmp_0fe8
  stxdw [r6+0], r4

jmp_0fe8:
  stxdw [r0+0], r4
  stxdw [r1+0], r5
  stxdw [r4+0], r1
  jeq r5, 0, jmp_1028
  ldxdw r3, [r5+16]
  jeq r4, r3, jmp_1040
  stxdw [r5+8], r1
  ja jmp_1048

jmp_1028:
  ldxdw r5, [r10+64]
  stxdw [r5+10432], r1
  ja jmp_1048

jmp_1040:
  stxdw [r5+16], r1

jmp_1048:
  ldxb r1, [r4+28]
  stxb [r9+28], r1
  stb [r4+28], 0
  ldxdw r1, [r10+40]
  stb [r1+28], 0

jmp_1070:
  ldxdw r1, [r10+48]
  stdw [r1+8], 0
  stdw [r1+0], 0
  ldxdw r1, [r10+64]

jmp_1090:
  ldxdw r3, [r1+10440]
  stxdw [r2+0], r3
  stxdw [r1+10440], r2
  ja jmp_05f0

jmp_10b0:
  mov64 r8, r1
  jne r5, 0, jmp_15c8
  jne r3, 1, jmp_1518
  jne r4, 4, jmp_1528
  ldxdw r1, [r8+88]
  jne r1, 0, jmp_1538
  ldxb r1, [r8+10344]
  jne r1, 255, jmp_1548
  ldxdw r1, [r8+10424]
  jne r1, 0, jmp_15b8
  ldxb r1, [r8+20680]
  jne r1, 255, jmp_1558
  ldxdw r1, [r8+20760]
  jne r1, 14, jmp_1568
  ldxb r1, [r8+31032]
  jne r1, 255, jmp_1578
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
  add64 r4, 168
  mov64 r5, r10
  add64 r5, 79
  mov64 r1, r7
  mov64 r2, 0
  mov64 r3, r6
  call sol_try_find_program_address
  mov64 r0, 10
  ldxdw r1, [r10+168]
  ldxdw r2, [r7+10352]
  jne r1, r2, jmp_06a8
  ldxdw r1, [r10+176]
  ldxdw r2, [r8+10360]
  jne r1, r2, jmp_06a8
  ldxdw r1, [r10+184]
  ldxdw r2, [r8+10368]
  jne r1, r2, jmp_06a8
  ldxdw r1, [r10+192]
  ldxdw r2, [r8+10376]
  jne r1, r2, jmp_06a8
  mov64 r1, r8
  add64 r1, 10352
  ldxdw r2, [r8+31120]
  ldxdw r3, [r6+24]
  stxdw [r10+124], r3
  ldxdw r3, [r6+16]
  stxdw [r10+116], r3
  ldxdw r3, [r6+8]
  stxdw [r10+108], r3
  ldxdw r3, [r6+0]
  stxdw [r10+100], r3
  lmul64 r2, 152
  stxdw [r10+84], r2
  stdw [r10+92], 24
  stw [r10+80], 0
  stxdw [r10+152], r1
  mov64 r2, r8
  add64 r2, 16
  stxdw [r10+136], r2
  sth [r10+160], 257
  sth [r10+144], 257
  mov64 r3, r8
  add64 r3, 10384
  stxdw [r10+256], r3
  mov64 r3, r8
  add64 r3, 10432
  stxdw [r10+248], r3
  mov64 r3, r8
  add64 r3, 10416
  stxdw [r10+232], r3
  stxdw [r10+224], r1
  mov64 r1, r8
  add64 r1, 48
  stxdw [r10+200], r1
  mov64 r1, r8
  add64 r1, 96
  stxdw [r10+192], r1
  mov64 r1, r8
  add64 r1, 80
  stxdw [r10+176], r1
  stxdw [r10+168], r2
  stb [r10+274], 0
  sth [r10+272], 257
  stdw [r10+264], 0
  stdw [r10+240], 0
  stb [r10+218], 0
  sth [r10+216], 257
  stdw [r10+208], 0
  stdw [r10+184], 0
  stdw [r10+304], 0
  stdw [r10+296], 0
  stdw [r10+288], 0
  stdw [r10+280], 0
  mov64 r1, r10
  add64 r1, 80
  stxdw [r10+336], r1
  mov64 r1, r10
  add64 r1, 136
  stxdw [r10+320], r1
  mov64 r1, r10
  add64 r1, 280
  stxdw [r10+312], r1
  stdw [r10+344], 52
  stdw [r10+328], 2
  mov64 r1, r10
  add64 r1, 79
  stxdw [r10+352], r1
  stdw [r10+360], 1
  mov64 r1, r10
  add64 r1, 352
  stxdw [r10+368], r1
  stdw [r10+376], 1
  mov64 r1, r10
  add64 r1, 312
  mov64 r2, r10
  add64 r2, 168
  mov64 r4, r10
  add64 r4, 368
  mov64 r3, 2
  mov64 r5, 1
  call sol_invoke_signed_c
  mov64 r1, r8
  add64 r1, 10456
  stxdw [r8+10448], r1
  ja jmp_05f0

jmp_1518:
  mov64 r0, 12
  ja jmp_06a8

jmp_1528:
  mov64 r0, 1
  ja jmp_06a8

jmp_1538:
  mov64 r0, 2
  ja jmp_06a8

jmp_1548:
  mov64 r0, 5
  ja jmp_06a8

jmp_1558:
  mov64 r0, 6
  ja jmp_06a8

jmp_1568:
  mov64 r0, 4
  ja jmp_06a8

jmp_1578:
  mov64 r0, 7
  ja jmp_06a8

jmp_1588:
  ldxdw r1, [r10+64]
  stdw [r1+10432], 0
  ldxdw r3, [r10+48]
  stdw [r3+8], 0
  stdw [r3+0], 0
  ja jmp_1090

jmp_15b8:
  mov64 r0, 3
  ja jmp_06a8

jmp_15c8:
  mov64 r0, 11
  ja jmp_06a8
