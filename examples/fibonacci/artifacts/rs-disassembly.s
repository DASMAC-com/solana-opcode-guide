.globl entrypoint

entrypoint:
  ldxdw r2, [r1+0]
  jeq r2, 0, jmp_0068
  ldxdw r2, [r1+88]
  add64 r1, r2
  add64 r1, 10351
  and64 r1, -8
  ldxdw r2, [r1+0]
  jeq r2, 0, jmp_0080

jmp_0040:
  ldxb r1, [r1+8]
  jeq r1, 0, jmp_00a8
  jne r1, 1, jmp_00b8
  mov64 r0, 1
  ja jmp_0110

jmp_0068:
  add64 r1, 8
  ldxdw r2, [r1+0]
  jne r2, 0, jmp_0040

jmp_0080:
  mov32 r3, 5224
  hor64 r3, 0
  mov64 r1, 0
  mov64 r2, 0
  call fn_01b8

jmp_00a8:
  mov64 r0, r1
  ja jmp_0110

jmp_00b8:
  mov32 r0, -2
  jgt r1, 47, jmp_0110
  mov32 r2, 1
  mov32 r3, 0
  add64 r1, -1
  mov32 r0, 1

jmp_00e8:
  add32 r0, r3
  add64 r1, -1
  mov64 r3, r2
  mov64 r2, r0
  jne r1, 0, jmp_00e8

jmp_0110:
  exit
  ldxdw r2, [r1+8]
  ldxdw r1, [r2+0]
  ldxdw r2, [r2+8]
  add64 r2, -1
  call sol_log_
  mov32 r1, 4953
  hor64 r1, 0
  mov64 r2, 14
  call sol_log_
  exit

fn_0168:
  call fn_0170

fn_0170:
  call custom_panic
  call abort

fn_0180:
  add64 r10, -64
  stxdw [r10+48], r2
  stxdw [r10+40], r1
  sth [r10+56], 1
  mov64 r1, r10
  add64 r1, 40
  call fn_0168

fn_01b8:
  add64 r10, -128
  stxdw [r10+40], r2
  stxdw [r10+32], r1
  mov32 r1, 5248
  hor64 r1, 0
  stxdw [r10+48], r1
  mov64 r1, r10
  add64 r1, 96
  stxdw [r10+64], r1
  mov64 r1, r10
  add64 r1, 32
  stxdw [r10+112], r1
  mov32 r1, 4176
  hor64 r1, 0
  stxdw [r10+120], r1
  stxdw [r10+104], r1
  mov64 r1, r10
  add64 r1, 40
  stxdw [r10+96], r1
  stdw [r10+80], 0
  stdw [r10+56], 2
  stdw [r10+72], 2
  mov64 r1, r10
  add64 r1, 48
  mov64 r2, r3
  call fn_0180

fn_0288:
  add64 r10, -128
  mov64 r0, r1
  ldxdw r9, [r10+120]
  stxdw [r10+80], r9
  jeq r2, 0, jmp_02e8
  mov32 r2, 1114112
  ldxw r6, [r0+16]
  mov64 r1, r6
  and32 r1, 2097152
  jeq r1, 0, jmp_0300
  mov32 r2, 43
  ja jmp_02f8

jmp_02e8:
  mov32 r2, 45
  ldxw r6, [r0+16]

jmp_02f8:
  add64 r9, 1

jmp_0300:
  mov64 r1, r6
  and32 r1, 8388608
  stxdw [r10+96], r4
  stxw [r10+88], r2
  jne r1, 0, jmp_03e0
  mov64 r1, 0
  stxdw [r10+104], r1
  mov64 r7, r9
  ldxh r2, [r0+20]
  jlt r7, r2, jmp_04e0

jmp_0350:
  mov64 r9, r5
  ldxdw r8, [r0+8]
  ldxdw r7, [r0+0]
  mov64 r1, r7
  mov64 r2, r8
  ldxw r3, [r10+88]
  ldxdw r4, [r10+104]
  ldxdw r5, [r10+96]
  call fn_08b0
  mov32 r3, 1
  jne r0, 0, jmp_0898
  ldxdw r4, [r8+24]
  mov64 r1, r7
  mov64 r2, r9
  ldxdw r3, [r10+80]
  callx r4
  mov64 r3, r0
  ja jmp_0898

jmp_03e0:
  stxdw [r10+104], r3
  jge r4, 32, jmp_0478
  mov64 r7, 0
  jeq r4, 0, jmp_04c8
  ldxdw r1, [r10+104]
  mov64 r2, r4
  ja jmp_0438

jmp_0418:
  add64 r7, r3
  add64 r1, 1
  add64 r2, -1
  jeq r2, 0, jmp_04c8

jmp_0438:
  ldxb r4, [r1+0]
  lsh32 r4, 24
  arsh32 r4, 24
  mov32 r3, 1
  mov32 r4, r4
  jsgt r4, -65, jmp_0418
  mov32 r3, 0
  ja jmp_0418

jmp_0478:
  mov64 r1, r3
  mov64 r2, r4
  mov64 r7, r6
  mov64 r6, r5
  mov64 r8, r0
  call fn_0958
  mov64 r5, r6
  mov64 r6, r7
  mov64 r7, r0
  mov64 r0, r8

jmp_04c8:
  add64 r7, r9
  ldxh r2, [r0+20]
  jge r7, r2, jmp_0350

jmp_04e0:
  and32 r2, -1
  mov64 r1, r6
  and32 r1, 16777216
  stxdw [r10+64], r5
  jne r1, 0, jmp_0558
  and32 r7, -1
  sub32 r2, r7
  mov64 r1, r6
  rsh32 r1, 29
  and32 r1, 3
  jsgt r1, 1, jmp_0670
  mov32 r8, 0
  jeq r1, 0, jmp_0698
  mov64 r8, r2
  ja jmp_0698

jmp_0558:
  stxdw [r10+72], r2
  ldxdw r1, [r0+16]
  stxdw [r10+56], r1
  and32 r1, -1
  and32 r1, -1612709888
  or32 r1, 536870960
  stxw [r0+16], r1
  ldxdw r8, [r0+0]
  stxdw [r10+112], r0
  ldxdw r9, [r0+8]
  mov64 r1, r8
  mov64 r2, r9
  ldxw r3, [r10+88]
  ldxdw r4, [r10+104]
  ldxdw r5, [r10+96]
  call fn_08b0
  mov32 r3, 1
  jne r0, 0, jmp_0898
  and32 r7, -1
  ldxdw r2, [r10+72]
  sub32 r2, r7
  mov32 r7, 0
  and32 r2, 65535

jmp_0610:
  mov64 r1, r7
  and32 r1, 65535
  jge r1, r2, jmp_0840
  ldxdw r3, [r9+32]
  mov64 r1, r8
  mov64 r6, r2
  mov32 r2, 48
  callx r3
  mov64 r2, r6
  add32 r7, 1
  jeq r0, 0, jmp_0610
  ja jmp_0720

jmp_0670:
  mov64 r8, r2
  jne r1, 2, jmp_0698
  mov64 r8, r2
  and32 r8, 65534
  rsh32 r8, 1

jmp_0698:
  stxdw [r10+72], r2
  and32 r6, 2097151
  stxw [r10+112], r6
  mov32 r6, 0
  ldxdw r7, [r0+8]
  ldxdw r9, [r0+0]

jmp_06c8:
  mov64 r1, r8
  and32 r1, 65535
  mov64 r2, r6
  and32 r2, 65535
  jge r2, r1, jmp_0730
  ldxdw r3, [r7+32]
  mov64 r1, r9
  ldxw r2, [r10+112]
  callx r3
  add32 r6, 1
  jeq r0, 0, jmp_06c8

jmp_0720:
  mov32 r3, 1
  ja jmp_0898

jmp_0730:
  mov64 r1, r9
  mov64 r2, r7
  ldxw r3, [r10+88]
  ldxdw r4, [r10+104]
  ldxdw r5, [r10+96]
  call fn_08b0
  mov32 r3, 1
  jne r0, 0, jmp_0898
  ldxdw r4, [r7+24]
  mov64 r1, r9
  ldxdw r2, [r10+64]
  ldxdw r3, [r10+80]
  callx r4
  mov32 r3, 1
  ldxdw r6, [r10+72]
  jne r0, 0, jmp_0898
  sub32 r6, r8
  mov32 r8, 0
  and32 r6, 65535

jmp_07c8:
  mov64 r1, r8
  and32 r1, 65535
  mov32 r3, 1
  jlt r1, r6, jmp_07f0
  mov32 r3, 0

jmp_07f0:
  jge r1, r6, jmp_0898
  stxw [r10+104], r3
  ldxdw r3, [r7+32]
  mov64 r1, r9
  ldxw r2, [r10+112]
  callx r3
  ldxw r3, [r10+104]
  add32 r8, 1
  jeq r0, 0, jmp_07c8
  ja jmp_0898

jmp_0840:
  ldxdw r4, [r9+24]
  mov64 r1, r8
  ldxdw r2, [r10+64]
  ldxdw r3, [r10+80]
  callx r4
  mov32 r3, 1
  jne r0, 0, jmp_0898
  ldxdw r1, [r10+112]
  ldxdw r2, [r10+56]
  stxdw [r1+16], r2
  mov32 r3, 0

jmp_0898:
  and32 r3, 1
  mov64 r0, r3
  exit

fn_08b0:
  mov64 r6, r5
  mov64 r7, r4
  mov64 r8, r2
  mov32 r2, r3
  jeq r2, 1114112, jmp_0918
  ldxdw r4, [r8+32]
  mov64 r9, r1
  mov64 r2, r3
  callx r4
  mov64 r1, r9
  mov64 r2, r0
  mov32 r0, 1
  jne r2, 0, jmp_0950

jmp_0918:
  jeq r7, 0, jmp_0948
  ldxdw r4, [r8+24]
  mov64 r2, r7
  mov64 r3, r6
  callx r4
  ja jmp_0950

jmp_0948:
  mov32 r0, 0

jmp_0950:
  exit

fn_0958:
  add64 r10, -64
  mov64 r7, r1
  add64 r7, 7
  and64 r7, -8
  mov64 r3, r7
  sub64 r3, r1
  jge r2, r3, jmp_0a08

jmp_0990:
  mov64 r0, 0
  jne r2, 0, jmp_09c8
  ja jmp_0f28

jmp_09a8:
  add64 r0, r3
  add64 r1, 1
  add64 r2, -1
  jeq r2, 0, jmp_0f28

jmp_09c8:
  ldxb r4, [r1+0]
  lsh32 r4, 24
  arsh32 r4, 24
  mov32 r3, 1
  mov32 r4, r4
  jsgt r4, -65, jmp_09a8
  mov32 r3, 0
  ja jmp_09a8

jmp_0a08:
  mov64 r5, r2
  sub64 r5, r3
  jlt r5, 8, jmp_0990
  stxdw [r10+56], r3
  mov64 r2, r5
  and64 r2, 7
  mov64 r0, 0
  mov64 r3, 0
  jne r7, r1, jmp_0d60

jmp_0a50:
  ldxdw r4, [r10+56]
  add64 r1, r4
  jeq r2, 0, jmp_0af8
  mov64 r0, r5
  and64 r0, -8
  mov64 r4, r1
  add64 r4, r0
  mov64 r0, 0
  ja jmp_0ab8

jmp_0a98:
  add64 r0, r6
  add64 r4, 1
  add64 r2, -1
  jeq r2, 0, jmp_0af8

jmp_0ab8:
  ldxb r7, [r4+0]
  lsh32 r7, 24
  arsh32 r7, 24
  mov32 r6, 1
  mov32 r7, r7
  jsgt r7, -65, jmp_0a98
  mov32 r6, 0
  ja jmp_0a98

jmp_0af8:
  rsh64 r5, 3
  add64 r0, r3
  ja jmp_0bb8

jmp_0b10:
  mov64 r1, r2
  add64 r1, r5
  ldxdw r9, [r10+56]
  mov64 r7, r9
  and64 r7, 3
  mov64 r5, r3
  sub64 r5, r9
  mov32 r6, 16711935
  hor64 r6, 16711935
  mov64 r8, r4
  and64 r8, r6
  rsh64 r4, 8
  and64 r4, r6
  add64 r4, r8
  mov32 r6, 65537
  hor64 r6, 65537
  lmul64 r4, r6
  rsh64 r4, 48
  add64 r4, r0
  mov64 r0, r4
  jne r7, 0, jmp_0e18

jmp_0bb8:
  mov64 r3, r5
  mov64 r2, r1
  jeq r3, 0, jmp_0f28
  mov64 r5, r3
  jlt r3, 192, jmp_0be8
  mov64 r5, 192

jmp_0be8:
  stxdw [r10+56], r5
  lsh64 r5, 3
  mov64 r4, 0
  jlt r3, 4, jmp_0b10
  mov64 r4, r5
  and64 r4, 2016
  mov64 r1, r2
  add64 r1, r4
  mov64 r4, 0
  mov64 r7, r2

jmp_0c38:
  ldxdw r9, [r7+0]
  mov64 r8, r9
  rsh64 r8, 6
  xor64 r9, -1
  rsh64 r9, 7
  or64 r9, r8
  mov32 r8, 16843009
  hor64 r8, 16843009
  and64 r9, r8
  add64 r9, r4
  ldxdw r4, [r7+8]
  mov64 r6, r4
  rsh64 r6, 6
  xor64 r4, -1
  rsh64 r4, 7
  or64 r4, r6
  and64 r4, r8
  add64 r4, r9
  ldxdw r9, [r7+16]
  mov64 r6, r9
  rsh64 r6, 6
  xor64 r9, -1
  rsh64 r9, 7
  or64 r9, r6
  and64 r9, r8
  add64 r9, r4
  ldxdw r4, [r7+24]
  mov64 r6, r4
  rsh64 r6, 6
  xor64 r4, -1
  rsh64 r4, 7
  or64 r4, r6
  and64 r4, r8
  add64 r4, r9
  add64 r7, 32
  jne r7, r1, jmp_0c38
  ja jmp_0b10

jmp_0d60:
  mov64 r6, r1
  sub64 r6, r7
  mov64 r7, r1
  ja jmp_0da0

jmp_0d80:
  add64 r3, r9
  add64 r7, 1
  mov32 r4, r8
  jeq r4, 1, jmp_0a50

jmp_0da0:
  ldxb r9, [r7+0]
  lsh32 r9, 24
  arsh32 r9, 24
  mov32 r8, 1
  mov32 r4, r9
  mov32 r9, 1
  jsle r4, -65, jmp_0df0
  add64 r6, 1
  jeq r6, 0, jmp_0d80
  ja jmp_0e08

jmp_0df0:
  mov32 r9, 0
  add64 r6, 1
  jeq r6, 0, jmp_0d80

jmp_0e08:
  mov32 r8, 0
  ja jmp_0d80

jmp_0e18:
  and64 r9, 252
  lsh64 r9, 3
  jlt r3, 192, jmp_0e38
  mov64 r3, 192

jmp_0e38:
  add64 r2, r9
  mov64 r1, 0
  and64 r3, 3
  lsh64 r3, 3

jmp_0e58:
  ldxdw r0, [r2+0]
  mov64 r5, r0
  rsh64 r5, 6
  xor64 r0, -1
  rsh64 r0, 7
  or64 r0, r5
  mov32 r5, 16843009
  hor64 r5, 16843009
  and64 r0, r5
  add64 r0, r1
  add64 r2, 8
  add64 r3, -8
  mov64 r1, r0
  jne r3, 0, jmp_0e58
  mov32 r1, 16711935
  hor64 r1, 16711935
  mov64 r2, r0
  and64 r2, r1
  rsh64 r0, 8
  and64 r0, r1
  add64 r0, r2
  mov32 r1, 65537
  hor64 r1, 65537
  lmul64 r0, r1
  rsh64 r0, 48
  add64 r0, r4

jmp_0f28:
  exit
  add64 r10, -64
  mov64 r3, 20
  ldxdw r1, [r1+0]
  mov64 r4, r1
  jlt r1, 1000, jmp_1088
  mov64 r3, 16
  mov64 r4, r1

jmp_0f68:
  mov64 r5, r4
  mov64 r0, r5
  and32 r0, -1
  udiv64 r4, 10000
  mov64 r6, r4
  and32 r6, -1
  lmul32 r6, 10000
  sub32 r0, r6
  mov64 r6, r0
  and32 r6, 65535
  udiv32 r6, 100
  mov64 r7, r6
  lmul32 r7, 100
  sub32 r0, r7
  mov32 r7, 5018
  hor64 r7, 0
  lsh32 r0, 1
  and64 r0, 65534
  lsh32 r6, 1
  mov64 r8, r7
  add64 r8, r0
  add64 r7, r6
  mov64 r0, r10
  add64 r0, 44
  add64 r0, r3
  ldxb r6, [r7+1]
  stxb [r0+1], r6
  ldxb r6, [r7+0]
  stxb [r0+0], r6
  ldxb r6, [r8+1]
  stxb [r0+3], r6
  ldxb r6, [r8+0]
  stxb [r0+2], r6
  add64 r3, -4
  jgt r5, 9999999, jmp_0f68
  add64 r3, 4

jmp_1088:
  jle r4, 9, jmp_1150
  and32 r4, -1
  mov64 r5, r4
  and32 r5, 65535
  udiv32 r5, 100
  mov64 r0, r5
  lmul32 r0, 100
  sub32 r4, r0
  lsh32 r4, 1
  mov32 r0, 5018
  hor64 r0, 0
  and64 r4, 65534
  add64 r0, r4
  mov64 r4, r10
  add64 r4, 44
  mov64 r6, r4
  add64 r6, r3
  ldxb r7, [r0+1]
  stxb [r6-1], r7
  add64 r3, -2
  add64 r4, r3
  ldxb r0, [r0+0]
  stxb [r4+0], r0
  jne r1, 0, jmp_1160
  ja jmp_1168

jmp_1150:
  mov64 r5, r4
  jeq r1, 0, jmp_1168

jmp_1160:
  jeq r5, 0, jmp_11c0

jmp_1168:
  mov32 r1, 5018
  hor64 r1, 0
  lsh64 r5, 1
  and64 r5, 30
  add64 r1, r5
  add64 r3, -1
  mov64 r4, r10
  add64 r4, 44
  add64 r4, r3
  ldxb r1, [r1+1]
  stxb [r4+0], r1

jmp_11c0:
  mov64 r1, r3
  sub64 r1, 20
  stxdw [r10-8], r1
  mov64 r5, r10
  add64 r5, 44
  add64 r5, r3
  mov64 r1, r2
  mov32 r2, 1
  mov64 r3, 1
  mov64 r4, 0
  call fn_0288
  exit

.rodata
  data_0000: .byte 0x66, 0x69, 0x62, 0x6f, 0x6e, 0x61, 0x63, 0x63, 0x69, 0x2f, 0x73, 0x72, 0x63, 0x2f, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x2e, 0x72, 0x73, 0x00, 0x2a, 0x2a, 0x20, 0x50, 0x41, 0x4e, 0x49, 0x43, 0x4b, 0x45, 0x44, 0x20, 0x2a, 0x2a, 0x00, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x20, 0x6f, 0x75, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x73, 0x3a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x65, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x20, 0x62, 0x75, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x20, 0x69, 0x73, 0x20, 0x30, 0x30, 0x30, 0x31, 0x30, 0x32, 0x30, 0x33, 0x30, 0x34, 0x30, 0x35, 0x30, 0x36, 0x30, 0x37, 0x30, 0x38, 0x30, 0x39, 0x31, 0x30, 0x31, 0x31, 0x31, 0x32, 0x31, 0x33, 0x31, 0x34, 0x31, 0x35, 0x31, 0x36, 0x31, 0x37, 0x31, 0x38, 0x31, 0x39, 0x32, 0x30, 0x32, 0x31, 0x32, 0x32, 0x32, 0x33, 0x32, 0x34, 0x32, 0x35, 0x32, 0x36, 0x32, 0x37, 0x32, 0x38, 0x32, 0x39, 0x33, 0x30, 0x33, 0x31, 0x33, 0x32, 0x33, 0x33, 0x33, 0x34, 0x33, 0x35, 0x33, 0x36, 0x33, 0x37, 0x33, 0x38, 0x33, 0x39, 0x34, 0x30, 0x34, 0x31, 0x34, 0x32, 0x34, 0x33, 0x34, 0x34, 0x34, 0x35, 0x34, 0x36, 0x34, 0x37, 0x34, 0x38, 0x34, 0x39, 0x35, 0x30, 0x35, 0x31, 0x35, 0x32, 0x35, 0x33, 0x35, 0x34, 0x35, 0x35, 0x35, 0x36, 0x35, 0x37, 0x35, 0x38, 0x35, 0x39, 0x36, 0x30, 0x36, 0x31, 0x36, 0x32, 0x36, 0x33, 0x36, 0x34, 0x36, 0x35, 0x36, 0x36, 0x36, 0x37, 0x36, 0x38, 0x36, 0x39, 0x37, 0x30, 0x37, 0x31, 0x37, 0x32, 0x37, 0x33, 0x37, 0x34, 0x37, 0x35, 0x37, 0x36, 0x37, 0x37, 0x37, 0x38, 0x37, 0x39, 0x38, 0x30, 0x38, 0x31, 0x38, 0x32, 0x38, 0x33, 0x38, 0x34, 0x38, 0x35, 0x38, 0x36, 0x38, 0x37, 0x38, 0x38, 0x38, 0x39, 0x39, 0x30, 0x39, 0x31, 0x39, 0x32, 0x39, 0x33, 0x39, 0x34, 0x39, 0x35, 0x39, 0x36, 0x39, 0x37, 0x39, 0x38, 0x39, 0x39
