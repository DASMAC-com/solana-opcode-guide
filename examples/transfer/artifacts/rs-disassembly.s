.globl entrypoint

entrypoint:
  add64 r10, -2240
  mov64 r0, 1
  ldxdw r3, [r1+0]
  jeq r3, 0, jmp_08a0
  mov64 r2, r1
  add64 r2, 8
  stxdw [r10+8], r2
  jlt r3, 3, jmp_08a0
  ldxdw r4, [r1+88]
  mov64 r2, r1
  add64 r2, r4
  mov64 r4, r10
  add64 r4, 8
  add64 r2, 10351
  and64 r2, -8
  jlt r3, 6, jmp_0348
  mov64 r4, r10
  add64 r4, 8
  mov64 r5, r3

jmp_0098:
  ldxb r6, [r2+0]
  jne r6, 255, jmp_01d0
  stxdw [r4+8], r2
  ldxdw r6, [r2+80]
  add64 r2, r6
  add64 r2, 10343
  and64 r2, -8
  ldxb r6, [r2+0]
  jne r6, 255, jmp_0218

jmp_00e0:
  stxdw [r4+16], r2
  ldxdw r6, [r2+80]
  add64 r2, r6
  add64 r2, 10343
  and64 r2, -8
  ldxb r6, [r2+0]
  jne r6, 255, jmp_0260

jmp_0118:
  stxdw [r4+24], r2
  ldxdw r6, [r2+80]
  add64 r2, r6
  add64 r2, 10343
  and64 r2, -8
  ldxb r6, [r2+0]
  jne r6, 255, jmp_02a8

jmp_0150:
  stxdw [r4+32], r2
  ldxdw r6, [r2+80]
  add64 r2, r6
  add64 r2, 10343
  and64 r2, -8
  add64 r4, 40
  ldxb r6, [r2+0]
  jne r6, 255, jmp_02f8

jmp_0190:
  stxdw [r4+0], r2
  ldxdw r6, [r2+80]
  add64 r2, r6
  add64 r2, 10343
  and64 r2, -8
  add64 r5, -5
  jgt r5, 5, jmp_0098
  ja jmp_0350

jmp_01d0:
  lsh64 r6, 3
  mov64 r7, r10
  add64 r7, 8
  add64 r7, r6
  ldxdw r6, [r7+0]
  stxdw [r4+8], r6
  add64 r2, 8
  ldxb r6, [r2+0]
  jeq r6, 255, jmp_00e0

jmp_0218:
  lsh64 r6, 3
  mov64 r7, r10
  add64 r7, 8
  add64 r7, r6
  ldxdw r6, [r7+0]
  stxdw [r4+16], r6
  add64 r2, 8
  ldxb r6, [r2+0]
  jeq r6, 255, jmp_0118

jmp_0260:
  lsh64 r6, 3
  mov64 r7, r10
  add64 r7, 8
  add64 r7, r6
  ldxdw r6, [r7+0]
  stxdw [r4+24], r6
  add64 r2, 8
  ldxb r6, [r2+0]
  jeq r6, 255, jmp_0150

jmp_02a8:
  lsh64 r6, 3
  mov64 r7, r10
  add64 r7, 8
  add64 r7, r6
  ldxdw r6, [r7+0]
  stxdw [r4+32], r6
  add64 r2, 8
  add64 r4, 40
  ldxb r6, [r2+0]
  jeq r6, 255, jmp_0190

jmp_02f8:
  lsh64 r6, 3
  mov64 r7, r10
  add64 r7, 8
  add64 r7, r6
  ldxdw r6, [r7+0]
  stxdw [r4+0], r6
  add64 r2, 8
  add64 r5, -5
  jgt r5, 5, jmp_0098
  ja jmp_0350

jmp_0348:
  mov64 r5, r3

jmp_0350:
  jsle r5, 2, jmp_03f8
  jeq r5, 3, jmp_0420
  jne r5, 4, jmp_0478
  ldxb r5, [r2+0]
  jne r5, 255, jmp_0978
  stxdw [r4+8], r2
  ldxdw r5, [r2+80]
  add64 r2, r5
  add64 r2, 10343
  and64 r2, -8
  ldxb r5, [r2+0]
  jne r5, 255, jmp_09c0

jmp_03b0:
  stxdw [r4+16], r2
  ldxdw r5, [r2+80]
  add64 r2, r5
  add64 r2, 10343
  and64 r2, -8
  ldxb r5, [r2+0]
  jne r5, 255, jmp_0a08

jmp_03e8:
  stxdw [r4+24], r2
  ja jmp_0538

jmp_03f8:
  jeq r5, 1, jmp_0558
  ldxb r5, [r2+0]
  jne r5, 255, jmp_0b50
  stxdw [r4+8], r2
  ja jmp_0538

jmp_0420:
  ldxb r5, [r2+0]
  jne r5, 255, jmp_08f8
  stxdw [r4+8], r2
  ldxdw r5, [r2+80]
  add64 r2, r5
  add64 r2, 10343
  and64 r2, -8
  ldxb r5, [r2+0]
  jne r5, 255, jmp_0940

jmp_0468:
  stxdw [r4+16], r2
  ja jmp_0538

jmp_0478:
  ldxb r5, [r2+0]
  jne r5, 255, jmp_0a40
  stxdw [r4+8], r2
  ldxdw r5, [r2+80]
  add64 r2, r5
  add64 r2, 10343
  and64 r2, -8
  ldxb r5, [r2+0]
  jne r5, 255, jmp_0a88

jmp_04c0:
  stxdw [r4+16], r2
  ldxdw r5, [r2+80]
  add64 r2, r5
  add64 r2, 10343
  and64 r2, -8
  ldxb r5, [r2+0]
  jne r5, 255, jmp_0ad0

jmp_04f8:
  stxdw [r4+24], r2
  ldxdw r5, [r2+80]
  add64 r2, r5
  add64 r2, 10343
  and64 r2, -8
  ldxb r5, [r2+0]
  jne r5, 255, jmp_0b18

jmp_0530:
  stxdw [r4+32], r2

jmp_0538:
  ldxdw r4, [r2+80]
  add64 r2, r4
  add64 r2, 10343
  and64 r2, -8

jmp_0558:
  jne r3, 3, jmp_08a0

jmp_0560:
  ldxdw r3, [r2+0]
  mov64 r0, 6
  jne r3, 8, jmp_08a0
  mov64 r0, 7
  ldxdw r2, [r2+8]
  ldxdw r3, [r1+80]
  jlt r3, r2, jmp_08a0
  stxdw [r10+2048], r2
  mov32 r0, 0
  hor64 r0, 12
  stw [r10+2044], 2
  ldxdw r3, [r10+16]
  mov64 r2, r3
  add64 r2, 8
  stxdw [r10+2072], r2
  mov64 r4, r1
  add64 r4, 16
  stxdw [r10+2056], r4
  sth [r10+2080], 1
  sth [r10+2064], 257
  ldxb r5, [r1+8]
  jne r5, 255, jmp_08a0
  ldxb r5, [r1+9]
  mov32 r0, 1
  mov32 r6, r5
  mov32 r5, 1
  jne r6, 0, jmp_0640
  mov32 r5, 0

jmp_0640:
  ldxb r6, [r1+10]
  mov32 r7, r6
  mov32 r6, 1
  jne r7, 0, jmp_0668
  mov32 r6, 0

jmp_0668:
  mov64 r7, r1
  add64 r7, 80
  ldxb r8, [r1+11]
  mov32 r8, r8
  jne r8, 0, jmp_0698
  mov32 r0, 0

jmp_0698:
  ldxdw r8, [r1+88]
  mov64 r9, r1
  add64 r9, 48
  stxdw [r10+2120], r9
  add64 r1, 96
  stxdw [r10+2112], r1
  stxdw [r10+2104], r8
  stxdw [r10+2096], r7
  stxdw [r10+2088], r4
  stxb [r10+2138], r0
  stxb [r10+2137], r6
  stxb [r10+2136], r5
  mov32 r0, 0
  hor64 r0, 12
  stdw [r10+2128], 0
  ldxb r1, [r3+0]
  jne r1, 255, jmp_08a0
  ldxb r1, [r3+1]
  mov32 r4, 1
  mov32 r5, r1
  mov32 r1, 1
  jeq r5, 0, jmp_08a8
  ldxb r5, [r3+2]
  mov32 r0, r5
  mov32 r5, 1
  jeq r0, 0, jmp_08d0

jmp_0768:
  ldxb r0, [r3+3]
  mov32 r0, r0
  jne r0, 0, jmp_0788

jmp_0780:
  mov32 r4, 0

jmp_0788:
  ldxdw r0, [r3+80]
  mov64 r6, r3
  add64 r6, 40
  stxdw [r10+2176], r6
  mov64 r6, r3
  add64 r6, 88
  stxdw [r10+2168], r6
  stxdw [r10+2160], r0
  add64 r3, 72
  stxdw [r10+2152], r3
  stxdw [r10+2144], r2
  stxb [r10+2194], r4
  stxb [r10+2193], r5
  stxb [r10+2192], r1
  stdw [r10+2184], 0
  mov64 r1, r10
  add64 r1, 2044
  stxdw [r10+2224], r1
  mov64 r1, r10
  add64 r1, 2056
  stxdw [r10+2208], r1
  mov32 r1, 3256
  hor64 r1, 0
  stxdw [r10+2200], r1
  stdw [r10+2232], 12
  stdw [r10+2216], 2
  mov64 r1, r10
  add64 r1, 2200
  mov64 r2, r10
  add64 r2, 2088
  mov64 r3, 2
  mov64 r4, 8
  mov64 r5, 0
  call sol_invoke_signed_c
  mov64 r0, 0

jmp_08a0:
  exit

jmp_08a8:
  mov32 r1, 0
  ldxb r5, [r3+2]
  mov32 r0, r5
  mov32 r5, 1
  jne r0, 0, jmp_0768

jmp_08d0:
  mov32 r5, 0
  ldxb r0, [r3+3]
  mov32 r0, r0
  jeq r0, 0, jmp_0780
  ja jmp_0788

jmp_08f8:
  lsh64 r5, 3
  mov64 r6, r10
  add64 r6, 8
  add64 r6, r5
  ldxdw r5, [r6+0]
  stxdw [r4+8], r5
  add64 r2, 8
  ldxb r5, [r2+0]
  jeq r5, 255, jmp_0468

jmp_0940:
  lsh64 r5, 3
  mov64 r6, r10
  add64 r6, 8
  add64 r6, r5
  ldxdw r5, [r6+0]
  stxdw [r4+16], r5
  ja jmp_0b80

jmp_0978:
  lsh64 r5, 3
  mov64 r6, r10
  add64 r6, 8
  add64 r6, r5
  ldxdw r5, [r6+0]
  stxdw [r4+8], r5
  add64 r2, 8
  ldxb r5, [r2+0]
  jeq r5, 255, jmp_03b0

jmp_09c0:
  lsh64 r5, 3
  mov64 r6, r10
  add64 r6, 8
  add64 r6, r5
  ldxdw r5, [r6+0]
  stxdw [r4+16], r5
  add64 r2, 8
  ldxb r5, [r2+0]
  jeq r5, 255, jmp_03e8

jmp_0a08:
  lsh64 r5, 3
  mov64 r6, r10
  add64 r6, 8
  add64 r6, r5
  ldxdw r5, [r6+0]
  stxdw [r4+24], r5
  ja jmp_0b80

jmp_0a40:
  lsh64 r5, 3
  mov64 r6, r10
  add64 r6, 8
  add64 r6, r5
  ldxdw r5, [r6+0]
  stxdw [r4+8], r5
  add64 r2, 8
  ldxb r5, [r2+0]
  jeq r5, 255, jmp_04c0

jmp_0a88:
  lsh64 r5, 3
  mov64 r6, r10
  add64 r6, 8
  add64 r6, r5
  ldxdw r5, [r6+0]
  stxdw [r4+16], r5
  add64 r2, 8
  ldxb r5, [r2+0]
  jeq r5, 255, jmp_04f8

jmp_0ad0:
  lsh64 r5, 3
  mov64 r6, r10
  add64 r6, 8
  add64 r6, r5
  ldxdw r5, [r6+0]
  stxdw [r4+24], r5
  add64 r2, 8
  ldxb r5, [r2+0]
  jeq r5, 255, jmp_0530

jmp_0b18:
  lsh64 r5, 3
  mov64 r6, r10
  add64 r6, 8
  add64 r6, r5
  ldxdw r5, [r6+0]
  stxdw [r4+32], r5
  ja jmp_0b80

jmp_0b50:
  lsh64 r5, 3
  mov64 r6, r10
  add64 r6, 8
  add64 r6, r5
  ldxdw r5, [r6+0]
  stxdw [r4+8], r5

jmp_0b80:
  add64 r2, 8
  jeq r3, 3, jmp_0560
  ja jmp_08a0
