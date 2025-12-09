.globl entrypoint

entrypoint:
  add64 r10, -2048
  ldxdw r3, [r1+0]
  jeq r3, 0, jmp_0038
  mov64 r2, r1
  add64 r2, 8
  stxdw [r10+16], r2
  jge r3, 3, jmp_0068

jmp_0038:
  mov32 r1, 2344
  hor64 r1, 0
  mov64 r2, 14
  call sol_log_
  mov64 r0, 0
  exit

jmp_0068:
  ldxdw r2, [r1+88]
  add64 r1, r2
  mov64 r2, r10
  add64 r2, 16
  add64 r1, 10351
  and64 r1, -8
  jlt r3, 6, jmp_0358
  mov64 r2, r10
  add64 r2, 16

jmp_00b0:
  ldxb r4, [r1+0]
  jne r4, 255, jmp_01e8
  stxdw [r2+8], r1
  ldxdw r4, [r1+80]
  add64 r1, r4
  add64 r1, 10343
  and64 r1, -8
  ldxb r4, [r1+0]
  jne r4, 255, jmp_0230

jmp_00f8:
  stxdw [r2+16], r1
  ldxdw r4, [r1+80]
  add64 r1, r4
  add64 r1, 10343
  and64 r1, -8
  ldxb r4, [r1+0]
  jne r4, 255, jmp_0278

jmp_0130:
  stxdw [r2+24], r1
  ldxdw r4, [r1+80]
  add64 r1, r4
  add64 r1, 10343
  and64 r1, -8
  ldxb r4, [r1+0]
  jne r4, 255, jmp_02c0

jmp_0168:
  stxdw [r2+32], r1
  ldxdw r4, [r1+80]
  add64 r1, r4
  add64 r1, 10343
  and64 r1, -8
  add64 r2, 40
  ldxb r4, [r1+0]
  jne r4, 255, jmp_0310

jmp_01a8:
  stxdw [r2+0], r1
  ldxdw r4, [r1+80]
  add64 r1, r4
  add64 r1, 10343
  and64 r1, -8
  add64 r3, -5
  jgt r3, 5, jmp_00b0
  ja jmp_0358

jmp_01e8:
  lsh64 r4, 3
  mov64 r5, r10
  add64 r5, 16
  add64 r5, r4
  ldxdw r4, [r5+0]
  stxdw [r2+8], r4
  add64 r1, 8
  ldxb r4, [r1+0]
  jeq r4, 255, jmp_00f8

jmp_0230:
  lsh64 r4, 3
  mov64 r5, r10
  add64 r5, 16
  add64 r5, r4
  ldxdw r4, [r5+0]
  stxdw [r2+16], r4
  add64 r1, 8
  ldxb r4, [r1+0]
  jeq r4, 255, jmp_0130

jmp_0278:
  lsh64 r4, 3
  mov64 r5, r10
  add64 r5, 16
  add64 r5, r4
  ldxdw r4, [r5+0]
  stxdw [r2+24], r4
  add64 r1, 8
  ldxb r4, [r1+0]
  jeq r4, 255, jmp_0168

jmp_02c0:
  lsh64 r4, 3
  mov64 r5, r10
  add64 r5, 16
  add64 r5, r4
  ldxdw r4, [r5+0]
  stxdw [r2+32], r4
  add64 r1, 8
  add64 r2, 40
  ldxb r4, [r1+0]
  jeq r4, 255, jmp_01a8

jmp_0310:
  lsh64 r4, 3
  mov64 r5, r10
  add64 r5, 16
  add64 r5, r4
  ldxdw r4, [r5+0]
  stxdw [r2+0], r4
  add64 r1, 8
  add64 r3, -5
  jgt r3, 5, jmp_00b0

jmp_0358:
  jsle r3, 2, jmp_0400
  jeq r3, 3, jmp_0428
  jne r3, 4, jmp_0480
  ldxb r3, [r1+0]
  jne r3, 255, jmp_05c0
  stxdw [r2+8], r1
  ldxdw r3, [r1+80]
  add64 r1, r3
  add64 r1, 10343
  and64 r1, -8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0608

jmp_03b8:
  stxdw [r2+16], r1
  ldxdw r3, [r1+80]
  add64 r1, r3
  add64 r1, 10343
  and64 r1, -8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0650

jmp_03f0:
  stxdw [r2+24], r1
  ja jmp_0038

jmp_0400:
  jeq r3, 1, jmp_0038
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0790

jmp_0418:
  stxdw [r2+8], r1
  ja jmp_0038

jmp_0428:
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0548
  stxdw [r2+8], r1
  ldxdw r3, [r1+80]
  add64 r1, r3
  add64 r1, 10343
  and64 r1, -8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0590

jmp_0470:
  stxdw [r2+16], r1
  ja jmp_0038

jmp_0480:
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0688
  stxdw [r2+8], r1
  ldxdw r3, [r1+80]
  add64 r1, r3
  add64 r1, 10343
  and64 r1, -8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_06d0

jmp_04c8:
  stxdw [r2+16], r1
  ldxdw r3, [r1+80]
  add64 r1, r3
  add64 r1, 10343
  and64 r1, -8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0718

jmp_0500:
  stxdw [r2+24], r1
  ldxdw r3, [r1+80]
  add64 r1, r3
  add64 r1, 10343
  and64 r1, -8
  ldxb r3, [r1+0]
  jne r3, 255, jmp_0760

jmp_0538:
  stxdw [r2+32], r1
  ja jmp_0038

jmp_0548:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+8], r3
  add64 r1, 8
  ldxb r3, [r1+0]
  jeq r3, 255, jmp_0470

jmp_0590:
  lsh64 r3, 3
  mov64 r1, r10
  add64 r1, 16
  add64 r1, r3
  ldxdw r1, [r1+0]
  ja jmp_0470

jmp_05c0:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+8], r3
  add64 r1, 8
  ldxb r3, [r1+0]
  jeq r3, 255, jmp_03b8

jmp_0608:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+16], r3
  add64 r1, 8
  ldxb r3, [r1+0]
  jeq r3, 255, jmp_03f0

jmp_0650:
  lsh64 r3, 3
  mov64 r1, r10
  add64 r1, 16
  add64 r1, r3
  ldxdw r1, [r1+0]
  stxdw [r2+24], r1
  ja jmp_0038

jmp_0688:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+8], r3
  add64 r1, 8
  ldxb r3, [r1+0]
  jeq r3, 255, jmp_04c8

jmp_06d0:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+16], r3
  add64 r1, 8
  ldxb r3, [r1+0]
  jeq r3, 255, jmp_0500

jmp_0718:
  lsh64 r3, 3
  mov64 r4, r10
  add64 r4, 16
  add64 r4, r3
  ldxdw r3, [r4+0]
  stxdw [r2+24], r3
  add64 r1, 8
  ldxb r3, [r1+0]
  jeq r3, 255, jmp_0538

jmp_0760:
  lsh64 r3, 3
  mov64 r1, r10
  add64 r1, 16
  add64 r1, r3
  ldxdw r1, [r1+0]
  ja jmp_0538

jmp_0790:
  lsh64 r3, 3
  mov64 r1, r10
  add64 r1, 16
  add64 r1, r3
  ldxdw r1, [r1+0]
  ja jmp_0418
  ldxdw r1, [r1+8]
  ldxdw r2, [r1+8]
  ldxdw r1, [r1+0]
  call sol_log_
  mov32 r1, 2358
  hor64 r1, 0
  mov64 r2, 14
  call sol_log_
  exit

.rodata
  str_0000: .ascii "Hello, DASMAC!** PANICKED **"
