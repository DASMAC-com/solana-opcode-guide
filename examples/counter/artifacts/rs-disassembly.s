.globl entrypoint

entrypoint:
  add64 r10, -128
  mov64 r0, 0
  ldxdw r3, [r1+0]
  jeq r3, 2, jmp_0228
  mov64 r2, 1
  jne r3, 3, jmp_0220
  ldxb r2, [r1+8]
  jne r2, 255, jmp_0290
  mov64 r2, 2
  ldxdw r3, [r1+88]
  jne r3, 0, jmp_0220
  mov64 r6, r1
  add64 r6, r3
  add64 r6, 10351
  and64 r6, -8
  mov64 r2, 5
  ldxb r3, [r6+0]
  jne r3, 255, jmp_0220
  mov64 r2, 3
  ldxdw r3, [r6+80]
  jne r3, 0, jmp_0220
  mov64 r4, r6
  add64 r4, r3
  add64 r4, 10343
  and64 r4, -8
  mov64 r2, 6
  ldxb r3, [r4+0]
  jne r3, 255, jmp_0220
  mov64 r2, 4
  ldxdw r3, [r4+80]
  jne r3, 14, jmp_0220
  add64 r1, 16
  add64 r4, r3
  mov64 r3, r4
  add64 r3, 10336
  add64 r4, 10343
  and64 r4, -8
  sub64 r4, r3
  add64 r3, r4
  ldxdw r2, [r3+0]
  stxdw [r10+56], r1
  add64 r3, r2
  stdw [r10+64], 32
  add64 r3, 8
  stb [r10+79], 255
  mov64 r1, r10
  add64 r1, 56
  mov64 r4, r10
  add64 r4, 80
  mov64 r5, r10
  add64 r5, 79
  mov64 r2, 1
  call sol_try_find_program_address
  jne r0, 0, jmp_0230
  mov64 r2, 8
  ldxdw r1, [r10+80]
  ldxdw r3, [r6+8]
  jne r3, r1, jmp_0220
  ldxdw r1, [r10+88]
  ldxdw r3, [r6+16]
  jne r3, r1, jmp_0220
  ldxdw r1, [r10+96]
  ldxdw r3, [r6+24]
  jne r3, r1, jmp_0220
  ldxdw r1, [r10+104]
  ldxdw r3, [r6+32]
  mov64 r0, 0
  jeq r3, r1, jmp_0228

jmp_0220:
  mov64 r0, r2

jmp_0228:
  exit

jmp_0230:
  mov32 r1, 1216
  hor64 r1, 0
  stxdw [r10+80], r1
  mov32 r2, 1232
  hor64 r2, 0

jmp_0258:
  stdw [r10+112], 0
  stdw [r10+88], 1
  stdw [r10+104], 0
  stdw [r10+96], 8
  mov64 r1, r10
  add64 r1, 80
  call fn_02f8

jmp_0290:
  mov32 r1, 1256
  hor64 r1, 0
  stxdw [r10+80], r1
  mov32 r2, 1272
  hor64 r2, 0
  ja jmp_0258

fn_02c0:
  ldxdw r2, [r1+8]
  ldxw r4, [r2+20]
  ldxw r3, [r2+16]
  ldxdw r1, [r2+0]
  ldxdw r2, [r2+8]
  add64 r2, -1
  call sol_panic_

fn_02f8:
  add64 r10, -64
  stxdw [r10+48], r2
  stxdw [r10+40], r1
  sth [r10+56], 1
  mov64 r1, r10
  add64 r1, 40
  call fn_02c0

.rodata
  data_0000: .byte 0x55, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x66, 0x69, 0x6e, 0x64, 0x20, 0x61, 0x20, 0x76, 0x69, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x62, 0x75, 0x6d, 0x70, 0x20, 0x73, 0x65, 0x65, 0x64, 0x73, 0x72, 0x63, 0x2f, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x2f, 0x6c, 0x61, 0x7a, 0x79, 0x2e, 0x72, 0x73, 0x00, 0x73, 0x72, 0x63, 0x2f, 0x73, 0x79, 0x73, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x2e, 0x72, 0x73, 0x00, 0x44, 0x75, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74
