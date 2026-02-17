.globl entrypoint

entrypoint:
  add64 r10, -384
  ldxdw r4, [r1+0]
  ldxdw r3, [r2-8]
  ldxb r5, [r2+0]
  jne r5, 1, jmp_07d8
  jne r3, 5, jmp_0c30
  jlt r4, 2, jmp_0c40
  ldxdw r3, [r1+88]
  jne r3, 0, jmp_0c50
  ldxb r3, [r1+10344]
  jne r3, 255, jmp_0c60
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
  ldxh r1, [r2+1]
  ja jmp_00d8

jmp_00b8:
  mov64 r5, r2
  add64 r5, r4
  ldxdw r4, [r5+0]
  jeq r4, 0, jmp_0120

jmp_00d8:
  mov64 r2, r4
  mov64 r4, 16
  ldxh r5, [r2+24]
  mov64 r0, r1
  jgt r0, r5, jmp_00b8
  mov64 r4, 8
  jlt r0, r5, jmp_00b8
  mov64 r0, 14
  ja jmp_07d0

jmp_0120:
  stxdw [r10+64], r6
  stxdw [r3+0], r2
  stb [r3+28], 1
  mov64 r4, r1
  ldxh r5, [r2+24]
  mov32 r1, 1
  jgt r4, r5, jmp_0160
  mov32 r1, 0

jmp_0160:
  lsh64 r1, 3
  mov64 r4, r2
  add64 r4, r1
  stxdw [r4+8], r3
  mov64 r0, 0

jmp_0188:
  ldxb r1, [r2+28]
  jeq r1, 0, jmp_07d0
  ldxdw r1, [r2+0]
  jeq r1, 0, jmp_0620
  ldxdw r4, [r1+16]
  mov32 r5, 1
  jeq r2, r4, jmp_01c8
  mov32 r5, 0

jmp_01c8:
  mov64 r8, r1
  add64 r8, 8
  mov64 r7, r5
  xor64 r7, 1
  mov64 r4, r7
  lsh64 r4, 3
  mov64 r9, r8
  add64 r9, r4
  ldxdw r9, [r9+0]
  jeq r9, 0, jmp_05e8
  ldxb r6, [r9+28]
  jeq r6, 0, jmp_05e8
  stb [r2+28], 0
  stb [r9+28], 0
  stb [r1+28], 1
  ldxdw r2, [r1+0]
  mov64 r3, r1
  jne r2, 0, jmp_0188
  ja jmp_07d0

jmp_0260:
  jne r4, 4, jmp_0c90
  ldxdw r3, [r1+10424]
  mov64 r5, r3
  add64 r5, 7
  and64 r5, -8
  mov64 r4, r1
  add64 r4, r5
  ldxb r5, [r4+20680]
  jne r5, 255, jmp_0c70
  ldxdw r5, [r4+20760]
  jne r5, 14, jmp_0c80
  ldxb r5, [r4+31032]
  jne r5, 255, jmp_0ca0
  mov64 r0, 8
  mov32 r5, 399877894
  hor64 r5, 1364995097
  ldxdw r7, [r4+31040]
  jne r7, r5, jmp_07d0
  mov32 r5, 1288277025
  hor64 r5, 2146519613
  ldxdw r7, [r4+31048]
  jne r7, r5, jmp_07d0
  mov32 r5, 149871192
  hor64 r5, 1157472667
  ldxdw r7, [r4+31056]
  jne r7, r5, jmp_07d0
  mov64 r7, r2
  ldxdw r2, [r4+31064]
  mov32 r5, -1965433885
  jne r2, r5, jmp_07d0
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

jmp_05c0:
  stdw [r3+0], 0
  stb [r3+28], 1
  stxdw [r6+0], r3

jmp_05d8:
  mov64 r0, 0
  ja jmp_07d0

jmp_05e8:
  mov64 r9, r2
  add64 r9, r4
  ldxdw r4, [r9+8]
  jeq r3, r4, jmp_0630
  mov64 r3, r4
  mov64 r4, r2
  ja jmp_0700

jmp_0620:
  stb [r2+28], 0
  ja jmp_07d0

jmp_0630:
  add64 r9, 8
  mov64 r6, r5
  lsh64 r6, 3
  mov64 r3, r4
  add64 r3, 8
  stxdw [r10+40], r3
  stxdw [r10+48], r6
  add64 r3, r6
  stxdw [r10+56], r3
  ldxdw r3, [r3+0]
  stxdw [r9+0], r3
  jeq r3, 0, jmp_0698
  stxdw [r3+0], r2

jmp_0698:
  ldxdw r3, [r10+56]
  stxdw [r3+0], r2
  stxdw [r4+0], r1
  stxdw [r2+0], r4
  mov64 r2, r8
  ldxdw r3, [r10+48]
  add64 r2, r3
  stxdw [r2+0], r4
  mov64 r2, r7
  lsh64 r2, 3
  ldxdw r3, [r10+40]
  add64 r3, r2
  ldxdw r3, [r3+0]

jmp_0700:
  lsh64 r5, 3
  add64 r8, r5
  ldxdw r2, [r1+0]
  stxdw [r8+0], r3
  lsh64 r7, 3
  mov64 r5, r4
  add64 r5, r7
  add64 r5, 8
  jeq r3, 0, jmp_0750
  stxdw [r3+0], r1

jmp_0750:
  stxdw [r5+0], r1
  stxdw [r4+0], r2
  stxdw [r1+0], r4
  ldxdw r3, [r10+64]
  jeq r2, 0, jmp_07b8
  ldxdw r5, [r2+16]
  mov32 r3, 1
  jeq r1, r5, jmp_0798
  mov32 r3, 0

jmp_0798:
  lsh64 r3, 3
  add64 r2, r3
  stxdw [r2+8], r4
  ja jmp_07c0

jmp_07b8:
  stxdw [r3+0], r4

jmp_07c0:
  stb [r4+28], 0
  stb [r1+28], 1

jmp_07d0:
  exit

jmp_07d8:
  jne r5, 0, jmp_0cb0
  jne r3, 1, jmp_0c30
  jne r4, 4, jmp_0c40
  ldxdw r2, [r1+88]
  jne r2, 0, jmp_0c50
  ldxb r2, [r1+10344]
  jne r2, 255, jmp_0c60
  ldxdw r2, [r1+10424]
  jne r2, 0, jmp_0cc0
  ldxb r2, [r1+20680]
  jne r2, 255, jmp_0c70
  ldxdw r2, [r1+20760]
  jne r2, 14, jmp_0c80
  ldxb r2, [r1+31032]
  jne r2, 255, jmp_0ca0
  mov64 r0, 8
  mov32 r2, 399877894
  hor64 r2, 1364995097
  ldxdw r3, [r1+31040]
  jne r3, r2, jmp_07d0
  mov32 r2, 1288277025
  hor64 r2, 2146519613
  ldxdw r3, [r1+31048]
  jne r3, r2, jmp_07d0
  mov32 r2, 149871192
  hor64 r2, 1157472667
  ldxdw r3, [r1+31056]
  jne r3, r2, jmp_07d0
  ldxdw r2, [r1+31064]
  mov32 r3, -1965433885
  jne r2, r3, jmp_07d0
  mov64 r6, r1
  add64 r6, 41401
  mov64 r4, r10
  add64 r4, 168
  mov64 r5, r10
  add64 r5, 79
  mov64 r7, r1
  mov64 r2, 0
  mov64 r3, r6
  call sol_try_find_program_address
  mov64 r0, 10
  ldxdw r1, [r10+168]
  ldxdw r2, [r7+10352]
  jne r1, r2, jmp_07d0
  ldxdw r1, [r10+176]
  ldxdw r2, [r7+10360]
  jne r1, r2, jmp_07d0
  ldxdw r1, [r10+184]
  ldxdw r2, [r7+10368]
  jne r1, r2, jmp_07d0
  ldxdw r1, [r10+192]
  ldxdw r2, [r7+10376]
  jne r1, r2, jmp_07d0
  mov64 r1, r7
  add64 r1, 10352
  ldxdw r2, [r7+31120]
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
  mov64 r2, r7
  add64 r2, 16
  stxdw [r10+136], r2
  sth [r10+160], 257
  sth [r10+144], 257
  mov64 r3, r7
  add64 r3, 10384
  stxdw [r10+256], r3
  mov64 r3, r7
  add64 r3, 10432
  stxdw [r10+248], r3
  mov64 r3, r7
  add64 r3, 10416
  stxdw [r10+232], r3
  stxdw [r10+224], r1
  mov64 r1, r7
  add64 r1, 48
  stxdw [r10+200], r1
  mov64 r1, r7
  add64 r1, 96
  stxdw [r10+192], r1
  mov64 r1, r7
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
  mov64 r1, r7
  add64 r1, 10456
  stxdw [r7+10448], r1
  ja jmp_05d8

jmp_0c30:
  mov64 r0, 12
  ja jmp_07d0

jmp_0c40:
  mov64 r0, 1
  ja jmp_07d0

jmp_0c50:
  mov64 r0, 2
  ja jmp_07d0

jmp_0c60:
  mov64 r0, 5
  ja jmp_07d0

jmp_0c70:
  mov64 r0, 6
  ja jmp_07d0

jmp_0c80:
  mov64 r0, 4
  ja jmp_07d0

jmp_0c90:
  mov64 r0, 13
  ja jmp_07d0

jmp_0ca0:
  mov64 r0, 7
  ja jmp_07d0

jmp_0cb0:
  mov64 r0, 11
  ja jmp_07d0

jmp_0cc0:
  mov64 r0, 3
  ja jmp_07d0
