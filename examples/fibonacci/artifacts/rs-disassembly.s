.globl entrypoint

entrypoint:
  add64 r10, -2048
  mov64 r2, r1
  add64 r2, 8
  ldxdw r3, [r1+0]
  jeq r3, 0, jmp_00a0
  stxdw [r10+16], r2
  ldxdw r2, [r1+88]
  add64 r1, r2
  add64 r1, 10351
  and64 r1, -8
  jeq r3, 1, jmp_00a8
  jne r3, 2, jmp_0170
  ldxb r2, [r1+0]
  jne r2, 255, jmp_0630
  stxdw [r10+24], r1

jmp_0078:
  ldxdw r2, [r1+80]
  add64 r1, r2
  add64 r1, 10343
  and64 r1, -8
  ja jmp_00a8

jmp_00a0:
  mov64 r1, r2

jmp_00a8:
  ldxdw r2, [r1+0]
  jeq r2, 0, jmp_0900
  ldxb r1, [r1+8]
  jeq r1, 0, jmp_0160
  mov32 r2, -2
  jgt r1, 47, jmp_0150
  mov32 r5, 1
  mov32 r3, 0
  mov32 r4, 0

jmp_00f0:
  mov64 r2, r5
  add32 r4, r2
  add32 r3, 1
  mov64 r0, r3
  and32 r0, 255
  mov64 r5, r4
  mov64 r4, r2
  jlt r0, r1, jmp_00f0
  mov32 r0, 0
  hor64 r0, 1
  mov32 r1, r2
  jeq r1, 0, jmp_0168

jmp_0150:
  mov64 r0, r2
  ja jmp_0168

jmp_0160:
  mov64 r0, 0

jmp_0168:
  exit

jmp_0170:
  mov64 r2, r10
  add64 r2, 16
  jlt r3, 6, jmp_0440
  mov64 r2, r10
  add64 r2, 16

jmp_0198:
  ldxb r4, [r1+0]
  jne r4, 255, jmp_02d0
  stxdw [r2+8], r1
  ldxdw r4, [r1+80]
  add64 r1, r4
  add64 r1, 10343
  and64 r1, -8
  ldxb r4, [r1+0]
  jne r4, 255, jmp_0318

jmp_01e0:
  stxdw [r2+16], r1
  ldxdw r4, [r1+80]
  add64 r1, r4
  add64 r1, 10343
  and64 r1, -8
  ldxb r4, [r1+0]
  jne r4, 255, jmp_0360

jmp_0218:
  stxdw [r2+24], r1
  ldxdw r4, [r1+80]
  add64 r1, r4
  add64 r1, 10343
  and64 r1, -8
  ldxb r4, [r1+0]
  jne r4, 255, jmp_03a8

jmp_0250:
  stxdw [r2+32], r1
  ldxdw r4, [r1+80]
  add64 r1, r4
  add64 r1, 10343
  and64 r1, -8
  add64 r2, 40
  ldxb r4, [r1+0]
  jne r4, 255, jmp_03f8

jmp_0290:
  stxdw [r2+0], r1
  ldxdw r4, [r1+80]
  add64 r1, r4
  add64 r1, 10343
  and64 r1, -8
  add64 r3, -5
  jgt r3, 5, jmp_0198
  ja jmp_0440

jmp_02d0:
  lsh64 r4, 3
  mov64 r5, r10
  add64 r5, 16
  add64 r5, r4
  ldxdw r4, [r5+0]
  stxdw [r2+8], r4
  add64 r1, 8
  ldxb r4, [r1+0]
  jeq r4, 255, jmp_01e0

jmp_0318:
  lsh64 r4, 3
  mov64 r5, r10
  add64 r5, 16
  add64 r5, r4
  ldxdw r4, [r5+0]
  stxdw [r2+16], r4
  add64 r1, 8
  ldxb r4, [r1+0]
  jeq r4, 255, jmp_0218

jmp_0360:
  lsh64 r4, 3
  mov64 r5, r10
  add64 r5, 16
  add64 r5, r4
  ldxdw r4, [r5+0]
  stxdw [r2+24], r4
  add64 r1, 8
  ldxb r4, [r1+0]
  jeq r4, 255, jmp_0250

jmp_03a8:
  lsh64 r4, 3
  mov64 r5, r10
  add64 r5, 16
  add64 r5, r4
  ldxdw r4, [r5+0]
  stxdw [r2+32], r4
  add64 r1, 8
  add64 r2, 40
  ldxb r4, [r1+0]
  jeq r4, 255, jmp_0290

jmp_03f8:
  lsh64 r4, 3
  mov64 r5, r10
  add64 r5, 16
  add64 r5, r4
  ldxdw r4, [r5+0]
  stxdw [r2+0], r4
  add64 r1, 8
  add64 r3, -5
  jgt r3, 5, jmp_0198

jmp_0440:
  jsle r3, 2, jmp_04e8
  jeq r3, 3, jmp_0510
  jne r3, 4, jmp_0568
  ldxb r3, [r1+0]
  jne r3, 255, jmp_06f0
  stxdw [r2+8], r1
  ldxdw r3, [r1+80]
  add64 r1, r3
  add64 r1, 10343
  and64 r1, -8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0738

jmp_04a0:
  stxdw [r2+16], r1
  ldxdw r3, [r1+80]
  add64 r1, r3
  add64 r1, 10343
  and64 r1, -8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0780

jmp_04d8:
  stxdw [r2+24], r1
  ja jmp_0078

jmp_04e8:
  jeq r3, 1, jmp_00a8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_08c8
  stxdw [r2+8], r1
  ja jmp_0078

jmp_0510:
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0670
  stxdw [r2+8], r1
  ldxdw r3, [r1+80]
  add64 r1, r3
  add64 r1, 10343
  and64 r1, -8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_06b8

jmp_0558:
  stxdw [r2+16], r1
  ja jmp_0078

jmp_0568:
  ldxb r3, [r1+0]
  jne r3, 255, jmp_07b8
  stxdw [r2+8], r1
  ldxdw r3, [r1+80]
  add64 r1, r3
  add64 r1, 10343
  and64 r1, -8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0800

jmp_05b0:
  stxdw [r2+16], r1
  ldxdw r3, [r1+80]
  add64 r1, r3
  add64 r1, 10343
  and64 r1, -8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0848

jmp_05e8:
  stxdw [r2+24], r1
  ldxdw r3, [r1+80]
  add64 r1, r3
  add64 r1, 10343
  and64 r1, -8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0890

jmp_0620:
  stxdw [r2+32], r1
  ja jmp_0078

jmp_0630:
  lsh64 r2, 3
  mov64 r3, r10
  add64 r3, 16
  add64 r3, r2
  ldxdw r2, [r3+0]
  stxdw [r10+24], r2

jmp_0660:
  add64 r1, 8
  ja jmp_00a8

jmp_0670:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+8], r3
  add64 r1, 8
  ldxb r3, [r1+0]
  jeq r3, 255, jmp_0558

jmp_06b8:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+16], r3
  ja jmp_0660

jmp_06f0:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+8], r3
  add64 r1, 8
  ldxb r3, [r1+0]
  jeq r3, 255, jmp_04a0

jmp_0738:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+16], r3
  add64 r1, 8
  ldxb r3, [r1+0]
  jeq r3, 255, jmp_04d8

jmp_0780:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+24], r3
  ja jmp_0660

jmp_07b8:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+8], r3
  add64 r1, 8
  ldxb r3, [r1+0]
  jeq r3, 255, jmp_05b0

jmp_0800:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+16], r3
  add64 r1, 8
  ldxb r3, [r1+0]
  jeq r3, 255, jmp_05e8

jmp_0848:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+24], r3
  add64 r1, 8
  ldxb r3, [r1+0]
  jeq r3, 255, jmp_0620

jmp_0890:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+32], r3
  ja jmp_0660

jmp_08c8:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+8], r3
  ja jmp_0660

jmp_0900:
  mov32 r3, 7288
  hor64 r3, 0
  mov64 r1, 0
  mov64 r2, 0
  call fn_09c8
  ldxdw r2, [r1+8]
  ldxdw r1, [r2+0]
  ldxdw r2, [r2+8]
  add64 r2, -1
  call sol_log_
  mov32 r1, 7017
  hor64 r1, 0
  mov64 r2, 14
  call sol_log_
  exit

fn_0978:
  call fn_0980

fn_0980:
  call custom_panic
  call abort

fn_0990:
  add64 r10, -64
  stxdw [r10+48], r2
  stxdw [r10+40], r1
  sth [r10+56], 1
  mov64 r1, r10
  add64 r1, 40
  call fn_0978

fn_09c8:
  add64 r10, -128
  stxdw [r10+40], r2
  stxdw [r10+32], r1
  mov32 r1, 7312
  hor64 r1, 0
  stxdw [r10+48], r1
  mov64 r1, r10
  add64 r1, 96
  stxdw [r10+64], r1
  mov64 r1, r10
  add64 r1, 32
  stxdw [r10+112], r1
  mov32 r1, 6240
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
  call fn_0990

fn_0a98:
  add64 r10, -128
  mov64 r0, r1
  ldxdw r9, [r10+120]
  stxdw [r10+80], r9
  jeq r2, 0, jmp_0af8
  mov32 r2, 1114112
  ldxw r6, [r0+16]
  mov64 r1, r6
  and32 r1, 2097152
  jeq r1, 0, jmp_0b10
  mov32 r2, 43
  ja jmp_0b08

jmp_0af8:
  mov32 r2, 45
  ldxw r6, [r0+16]

jmp_0b08:
  add64 r9, 1

jmp_0b10:
  mov64 r1, r6
  and32 r1, 8388608
  stxdw [r10+96], r4
  stxw [r10+88], r2
  jne r1, 0, jmp_0bf0
  mov64 r1, 0
  stxdw [r10+104], r1
  mov64 r7, r9
  ldxh r2, [r0+20]
  jlt r7, r2, jmp_0cf0

jmp_0b60:
  mov64 r9, r5
  ldxdw r8, [r0+8]
  ldxdw r7, [r0+0]
  mov64 r1, r7
  mov64 r2, r8
  ldxw r3, [r10+88]
  ldxdw r4, [r10+104]
  ldxdw r5, [r10+96]
  call fn_10c0
  mov32 r3, 1
  jne r0, 0, jmp_10a8
  ldxdw r4, [r8+24]
  mov64 r1, r7
  mov64 r2, r9
  ldxdw r3, [r10+80]
  callx r4
  mov64 r3, r0
  ja jmp_10a8

jmp_0bf0:
  stxdw [r10+104], r3
  jge r4, 32, jmp_0c88
  mov64 r7, 0
  jeq r4, 0, jmp_0cd8
  ldxdw r1, [r10+104]
  mov64 r2, r4
  ja jmp_0c48

jmp_0c28:
  add64 r7, r3
  add64 r1, 1
  add64 r2, -1
  jeq r2, 0, jmp_0cd8

jmp_0c48:
  ldxb r4, [r1+0]
  lsh32 r4, 24
  arsh32 r4, 24
  mov32 r3, 1
  mov32 r4, r4
  jsgt r4, -65, jmp_0c28
  mov32 r3, 0
  ja jmp_0c28

jmp_0c88:
  mov64 r1, r3
  mov64 r2, r4
  mov64 r7, r6
  mov64 r6, r5
  mov64 r8, r0
  call fn_1168
  mov64 r5, r6
  mov64 r6, r7
  mov64 r7, r0
  mov64 r0, r8

jmp_0cd8:
  add64 r7, r9
  ldxh r2, [r0+20]
  jge r7, r2, jmp_0b60

jmp_0cf0:
  and32 r2, -1
  mov64 r1, r6
  and32 r1, 16777216
  stxdw [r10+64], r5
  jne r1, 0, jmp_0d68
  and32 r7, -1
  sub32 r2, r7
  mov64 r1, r6
  rsh32 r1, 29
  and32 r1, 3
  jsgt r1, 1, jmp_0e80
  mov32 r8, 0
  jeq r1, 0, jmp_0ea8
  mov64 r8, r2
  ja jmp_0ea8

jmp_0d68:
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
  call fn_10c0
  mov32 r3, 1
  jne r0, 0, jmp_10a8
  and32 r7, -1
  ldxdw r2, [r10+72]
  sub32 r2, r7
  mov32 r7, 0
  and32 r2, 65535

jmp_0e20:
  mov64 r1, r7
  and32 r1, 65535
  jge r1, r2, jmp_1050
  ldxdw r3, [r9+32]
  mov64 r1, r8
  mov64 r6, r2
  mov32 r2, 48
  callx r3
  mov64 r2, r6
  add32 r7, 1
  jeq r0, 0, jmp_0e20
  ja jmp_0f30

jmp_0e80:
  mov64 r8, r2
  jne r1, 2, jmp_0ea8
  mov64 r8, r2
  and32 r8, 65534
  rsh32 r8, 1

jmp_0ea8:
  stxdw [r10+72], r2
  and32 r6, 2097151
  stxw [r10+112], r6
  mov32 r6, 0
  ldxdw r7, [r0+8]
  ldxdw r9, [r0+0]

jmp_0ed8:
  mov64 r1, r8
  and32 r1, 65535
  mov64 r2, r6
  and32 r2, 65535
  jge r2, r1, jmp_0f40
  ldxdw r3, [r7+32]
  mov64 r1, r9
  ldxw r2, [r10+112]
  callx r3
  add32 r6, 1
  jeq r0, 0, jmp_0ed8

jmp_0f30:
  mov32 r3, 1
  ja jmp_10a8

jmp_0f40:
  mov64 r1, r9
  mov64 r2, r7
  ldxw r3, [r10+88]
  ldxdw r4, [r10+104]
  ldxdw r5, [r10+96]
  call fn_10c0
  mov32 r3, 1
  jne r0, 0, jmp_10a8
  ldxdw r4, [r7+24]
  mov64 r1, r9
  ldxdw r2, [r10+64]
  ldxdw r3, [r10+80]
  callx r4
  mov32 r3, 1
  ldxdw r6, [r10+72]
  jne r0, 0, jmp_10a8
  sub32 r6, r8
  mov32 r8, 0
  and32 r6, 65535

jmp_0fd8:
  mov64 r1, r8
  and32 r1, 65535
  mov32 r3, 1
  jlt r1, r6, jmp_1000
  mov32 r3, 0

jmp_1000:
  jge r1, r6, jmp_10a8
  stxw [r10+104], r3
  ldxdw r3, [r7+32]
  mov64 r1, r9
  ldxw r2, [r10+112]
  callx r3
  ldxw r3, [r10+104]
  add32 r8, 1
  jeq r0, 0, jmp_0fd8
  ja jmp_10a8

jmp_1050:
  ldxdw r4, [r9+24]
  mov64 r1, r8
  ldxdw r2, [r10+64]
  ldxdw r3, [r10+80]
  callx r4
  mov32 r3, 1
  jne r0, 0, jmp_10a8
  ldxdw r1, [r10+112]
  ldxdw r2, [r10+56]
  stxdw [r1+16], r2
  mov32 r3, 0

jmp_10a8:
  and32 r3, 1
  mov64 r0, r3
  exit

fn_10c0:
  mov64 r6, r5
  mov64 r7, r4
  mov64 r8, r2
  mov32 r2, r3
  jeq r2, 1114112, jmp_1128
  ldxdw r4, [r8+32]
  mov64 r9, r1
  mov64 r2, r3
  callx r4
  mov64 r1, r9
  mov64 r2, r0
  mov32 r0, 1
  jne r2, 0, jmp_1160

jmp_1128:
  jeq r7, 0, jmp_1158
  ldxdw r4, [r8+24]
  mov64 r2, r7
  mov64 r3, r6
  callx r4
  ja jmp_1160

jmp_1158:
  mov32 r0, 0

jmp_1160:
  exit

fn_1168:
  add64 r10, -64
  mov64 r7, r1
  add64 r7, 7
  and64 r7, -8
  mov64 r3, r7
  sub64 r3, r1
  jge r2, r3, jmp_1218

jmp_11a0:
  mov64 r0, 0
  jne r2, 0, jmp_11d8
  ja jmp_1738

jmp_11b8:
  add64 r0, r3
  add64 r1, 1
  add64 r2, -1
  jeq r2, 0, jmp_1738

jmp_11d8:
  ldxb r4, [r1+0]
  lsh32 r4, 24
  arsh32 r4, 24
  mov32 r3, 1
  mov32 r4, r4
  jsgt r4, -65, jmp_11b8
  mov32 r3, 0
  ja jmp_11b8

jmp_1218:
  mov64 r5, r2
  sub64 r5, r3
  jlt r5, 8, jmp_11a0
  stxdw [r10+56], r3
  mov64 r2, r5
  and64 r2, 7
  mov64 r0, 0
  mov64 r3, 0
  jne r7, r1, jmp_1570

jmp_1260:
  ldxdw r4, [r10+56]
  add64 r1, r4
  jeq r2, 0, jmp_1308
  mov64 r0, r5
  and64 r0, -8
  mov64 r4, r1
  add64 r4, r0
  mov64 r0, 0
  ja jmp_12c8

jmp_12a8:
  add64 r0, r6
  add64 r4, 1
  add64 r2, -1
  jeq r2, 0, jmp_1308

jmp_12c8:
  ldxb r7, [r4+0]
  lsh32 r7, 24
  arsh32 r7, 24
  mov32 r6, 1
  mov32 r7, r7
  jsgt r7, -65, jmp_12a8
  mov32 r6, 0
  ja jmp_12a8

jmp_1308:
  rsh64 r5, 3
  add64 r0, r3
  ja jmp_13c8

jmp_1320:
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
  jne r7, 0, jmp_1628

jmp_13c8:
  mov64 r3, r5
  mov64 r2, r1
  jeq r3, 0, jmp_1738
  mov64 r5, r3
  jlt r3, 192, jmp_13f8
  mov64 r5, 192

jmp_13f8:
  stxdw [r10+56], r5
  lsh64 r5, 3
  mov64 r4, 0
  jlt r3, 4, jmp_1320
  mov64 r4, r5
  and64 r4, 2016
  mov64 r1, r2
  add64 r1, r4
  mov64 r4, 0
  mov64 r7, r2

jmp_1448:
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
  jne r7, r1, jmp_1448
  ja jmp_1320

jmp_1570:
  mov64 r6, r1
  sub64 r6, r7
  mov64 r7, r1
  ja jmp_15b0

jmp_1590:
  add64 r3, r9
  add64 r7, 1
  mov32 r4, r8
  jeq r4, 1, jmp_1260

jmp_15b0:
  ldxb r9, [r7+0]
  lsh32 r9, 24
  arsh32 r9, 24
  mov32 r8, 1
  mov32 r4, r9
  mov32 r9, 1
  jsle r4, -65, jmp_1600
  add64 r6, 1
  jeq r6, 0, jmp_1590
  ja jmp_1618

jmp_1600:
  mov32 r9, 0
  add64 r6, 1
  jeq r6, 0, jmp_1590

jmp_1618:
  mov32 r8, 0
  ja jmp_1590

jmp_1628:
  and64 r9, 252
  lsh64 r9, 3
  jlt r3, 192, jmp_1648
  mov64 r3, 192

jmp_1648:
  add64 r2, r9
  mov64 r1, 0
  and64 r3, 3
  lsh64 r3, 3

jmp_1668:
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
  jne r3, 0, jmp_1668
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

jmp_1738:
  exit
  add64 r10, -64
  mov64 r3, 20
  ldxdw r1, [r1+0]
  mov64 r4, r1
  jlt r1, 1000, jmp_1898
  mov64 r3, 16
  mov64 r4, r1

jmp_1778:
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
  mov32 r7, 7082
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
  jgt r5, 9999999, jmp_1778
  add64 r3, 4

jmp_1898:
  jle r4, 9, jmp_1960
  and32 r4, -1
  mov64 r5, r4
  and32 r5, 65535
  udiv32 r5, 100
  mov64 r0, r5
  lmul32 r0, 100
  sub32 r4, r0
  lsh32 r4, 1
  mov32 r0, 7082
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
  jne r1, 0, jmp_1970
  ja jmp_1978

jmp_1960:
  mov64 r5, r4
  jeq r1, 0, jmp_1978

jmp_1970:
  jeq r5, 0, jmp_19d0

jmp_1978:
  mov32 r1, 7082
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

jmp_19d0:
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
  call fn_0a98
  exit

.rodata
  data_0000: .byte 0x66, 0x69, 0x62, 0x6f, 0x6e, 0x61, 0x63, 0x63, 0x69, 0x2f, 0x73, 0x72, 0x63, 0x2f, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x2e, 0x72, 0x73, 0x00, 0x2a, 0x2a, 0x20, 0x50, 0x41, 0x4e, 0x49, 0x43, 0x4b, 0x45, 0x44, 0x20, 0x2a, 0x2a, 0x00, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x20, 0x6f, 0x75, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x73, 0x3a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x65, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x20, 0x62, 0x75, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x20, 0x69, 0x73, 0x20, 0x30, 0x30, 0x30, 0x31, 0x30, 0x32, 0x30, 0x33, 0x30, 0x34, 0x30, 0x35, 0x30, 0x36, 0x30, 0x37, 0x30, 0x38, 0x30, 0x39, 0x31, 0x30, 0x31, 0x31, 0x31, 0x32, 0x31, 0x33, 0x31, 0x34, 0x31, 0x35, 0x31, 0x36, 0x31, 0x37, 0x31, 0x38, 0x31, 0x39, 0x32, 0x30, 0x32, 0x31, 0x32, 0x32, 0x32, 0x33, 0x32, 0x34, 0x32, 0x35, 0x32, 0x36, 0x32, 0x37, 0x32, 0x38, 0x32, 0x39, 0x33, 0x30, 0x33, 0x31, 0x33, 0x32, 0x33, 0x33, 0x33, 0x34, 0x33, 0x35, 0x33, 0x36, 0x33, 0x37, 0x33, 0x38, 0x33, 0x39, 0x34, 0x30, 0x34, 0x31, 0x34, 0x32, 0x34, 0x33, 0x34, 0x34, 0x34, 0x35, 0x34, 0x36, 0x34, 0x37, 0x34, 0x38, 0x34, 0x39, 0x35, 0x30, 0x35, 0x31, 0x35, 0x32, 0x35, 0x33, 0x35, 0x34, 0x35, 0x35, 0x35, 0x36, 0x35, 0x37, 0x35, 0x38, 0x35, 0x39, 0x36, 0x30, 0x36, 0x31, 0x36, 0x32, 0x36, 0x33, 0x36, 0x34, 0x36, 0x35, 0x36, 0x36, 0x36, 0x37, 0x36, 0x38, 0x36, 0x39, 0x37, 0x30, 0x37, 0x31, 0x37, 0x32, 0x37, 0x33, 0x37, 0x34, 0x37, 0x35, 0x37, 0x36, 0x37, 0x37, 0x37, 0x38, 0x37, 0x39, 0x38, 0x30, 0x38, 0x31, 0x38, 0x32, 0x38, 0x33, 0x38, 0x34, 0x38, 0x35, 0x38, 0x36, 0x38, 0x37, 0x38, 0x38, 0x38, 0x39, 0x39, 0x30, 0x39, 0x31, 0x39, 0x32, 0x39, 0x33, 0x39, 0x34, 0x39, 0x35, 0x39, 0x36, 0x39, 0x37, 0x39, 0x38, 0x39, 0x39
