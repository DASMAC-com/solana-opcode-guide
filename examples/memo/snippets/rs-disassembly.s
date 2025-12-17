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
  jne r3, 2, jmp_00d0
  ldxb r2, [r1+0]
  jne r2, 255, jmp_0590
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
  add64 r1, 8
  call sol_log_
  mov64 r0, 0
  exit

jmp_00d0:
  mov64 r2, r10
  add64 r2, 16
  jlt r3, 6, jmp_03a0
  mov64 r2, r10
  add64 r2, 16

jmp_00f8:
  ldxb r4, [r1+0]
  jne r4, 255, jmp_0230
  stxdw [r2+8], r1
  ldxdw r4, [r1+80]
  add64 r1, r4
  add64 r1, 10343
  and64 r1, -8
  ldxb r4, [r1+0]
  jne r4, 255, jmp_0278

jmp_0140:
  stxdw [r2+16], r1
  ldxdw r4, [r1+80]
  add64 r1, r4
  add64 r1, 10343
  and64 r1, -8
  ldxb r4, [r1+0]
  jne r4, 255, jmp_02c0

jmp_0178:
  stxdw [r2+24], r1
  ldxdw r4, [r1+80]
  add64 r1, r4
  add64 r1, 10343
  and64 r1, -8
  ldxb r4, [r1+0]
  jne r4, 255, jmp_0308

jmp_01b0:
  stxdw [r2+32], r1
  ldxdw r4, [r1+80]
  add64 r1, r4
  add64 r1, 10343
  and64 r1, -8
  add64 r2, 40
  ldxb r4, [r1+0]
  jne r4, 255, jmp_0358

jmp_01f0:
  stxdw [r2+0], r1
  ldxdw r4, [r1+80]
  add64 r1, r4
  add64 r1, 10343
  and64 r1, -8
  add64 r3, -5
  jgt r3, 5, jmp_00f8
  ja jmp_03a0

jmp_0230:
  lsh64 r4, 3
  mov64 r5, r10
  add64 r5, 16
  add64 r5, r4
  ldxdw r4, [r5+0]
  stxdw [r2+8], r4
  add64 r1, 8
  ldxb r4, [r1+0]
  jeq r4, 255, jmp_0140

jmp_0278:
  lsh64 r4, 3
  mov64 r5, r10
  add64 r5, 16
  add64 r5, r4
  ldxdw r4, [r5+0]
  stxdw [r2+16], r4
  add64 r1, 8
  ldxb r4, [r1+0]
  jeq r4, 255, jmp_0178

jmp_02c0:
  lsh64 r4, 3
  mov64 r5, r10
  add64 r5, 16
  add64 r5, r4
  ldxdw r4, [r5+0]
  stxdw [r2+24], r4
  add64 r1, 8
  ldxb r4, [r1+0]
  jeq r4, 255, jmp_01b0

jmp_0308:
  lsh64 r4, 3
  mov64 r5, r10
  add64 r5, 16
  add64 r5, r4
  ldxdw r4, [r5+0]
  stxdw [r2+32], r4
  add64 r1, 8
  add64 r2, 40
  ldxb r4, [r1+0]
  jeq r4, 255, jmp_01f0

jmp_0358:
  lsh64 r4, 3
  mov64 r5, r10
  add64 r5, 16
  add64 r5, r4
  ldxdw r4, [r5+0]
  stxdw [r2+0], r4
  add64 r1, 8
  add64 r3, -5
  jgt r3, 5, jmp_00f8

jmp_03a0:
  jsle r3, 2, jmp_0448
  jeq r3, 3, jmp_0470
  jne r3, 4, jmp_04c8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0650
  stxdw [r2+8], r1
  ldxdw r3, [r1+80]
  add64 r1, r3
  add64 r1, 10343
  and64 r1, -8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0698

jmp_0400:
  stxdw [r2+16], r1
  ldxdw r3, [r1+80]
  add64 r1, r3
  add64 r1, 10343
  and64 r1, -8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_06e0

jmp_0438:
  stxdw [r2+24], r1
  ja jmp_0078

jmp_0448:
  jeq r3, 1, jmp_00a8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0828
  stxdw [r2+8], r1
  ja jmp_0078

jmp_0470:
  ldxb r3, [r1+0]
  jne r3, 255, jmp_05d0
  stxdw [r2+8], r1
  ldxdw r3, [r1+80]
  add64 r1, r3
  add64 r1, 10343
  and64 r1, -8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0618

jmp_04b8:
  stxdw [r2+16], r1
  ja jmp_0078

jmp_04c8:
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0718
  stxdw [r2+8], r1
  ldxdw r3, [r1+80]
  add64 r1, r3
  add64 r1, 10343
  and64 r1, -8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0760

jmp_0510:
  stxdw [r2+16], r1
  ldxdw r3, [r1+80]
  add64 r1, r3
  add64 r1, 10343
  and64 r1, -8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_07a8

jmp_0548:
  stxdw [r2+24], r1
  ldxdw r3, [r1+80]
  add64 r1, r3
  add64 r1, 10343
  and64 r1, -8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_07f0

jmp_0580:
  stxdw [r2+32], r1
  ja jmp_0078

jmp_0590:
  lsh64 r2, 3
  mov64 r3, r10
  add64 r3, 16
  add64 r3, r2
  ldxdw r2, [r3+0]
  stxdw [r10+24], r2

jmp_05c0:
  add64 r1, 8
  ja jmp_00a8

jmp_05d0:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+8], r3
  add64 r1, 8
  ldxb r3, [r1+0]
  jeq r3, 255, jmp_04b8

jmp_0618:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+16], r3
  ja jmp_05c0

jmp_0650:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+8], r3
  add64 r1, 8
  ldxb r3, [r1+0]
  jeq r3, 255, jmp_0400

jmp_0698:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+16], r3
  add64 r1, 8
  ldxb r3, [r1+0]
  jeq r3, 255, jmp_0438

jmp_06e0:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+24], r3
  ja jmp_05c0

jmp_0718:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+8], r3
  add64 r1, 8
  ldxb r3, [r1+0]
  jeq r3, 255, jmp_0510

jmp_0760:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+16], r3
  add64 r1, 8
  ldxb r3, [r1+0]
  jeq r3, 255, jmp_0548

jmp_07a8:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+24], r3
  add64 r1, 8
  ldxb r3, [r1+0]
  jeq r3, 255, jmp_0580

jmp_07f0:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+32], r3
  ja jmp_05c0

jmp_0828:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+8], r3
  ja jmp_05c0
  ldxdw r2, [r1+8]
  ldxdw r1, [r2+0]
  ldxdw r2, [r2+8]
  add64 r2, -1
  call sol_log_
  mov32 r1, 2512
  hor64 r1, 0
  mov64 r2, 14
  call sol_log_
  exit

.rodata
  str_0000: .ascii "** PANICKED **"
