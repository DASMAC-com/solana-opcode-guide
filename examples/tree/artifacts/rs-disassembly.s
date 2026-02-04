.globl entrypoint

fn_0000:
  ldxdw r2, [r1+8]
  ldxw r4, [r2+20]
  ldxw r3, [r2+16]
  ldxdw r1, [r2+0]
  ldxdw r2, [r2+8]
  add64 r2, -1
  call sol_panic_

entrypoint:
  add64 r10, -64
  mov64 r0, 1
  ldxdw r2, [r1+0]
  jne r2, 2, jmp_00b8
  ldxb r2, [r1+8]
  jne r2, 255, jmp_00c0
  mov64 r0, 2
  ldxdw r2, [r1+88]
  jne r2, 0, jmp_00b8
  add64 r1, r2
  add64 r1, 10351
  and64 r1, -8
  mov64 r0, 3
  ldxb r1, [r1+0]
  jne r1, 255, jmp_00b8
  mov64 r0, 0

jmp_00b8:
  exit

jmp_00c0:
  mov32 r1, 680
  hor64 r1, 0
  stxdw [r10+16], r1
  mov32 r2, 696
  hor64 r2, 0
  stdw [r10+48], 0
  stdw [r10+24], 1
  stdw [r10+40], 0
  stdw [r10+32], 8
  mov64 r1, r10
  add64 r1, 16
  call fn_0120

fn_0120:
  add64 r10, -64
  stxdw [r10+48], r2
  stxdw [r10+40], r1
  sth [r10+56], 1
  mov64 r1, r10
  add64 r1, 40
  call fn_0000

.rodata
  data_0000: .byte 0x73, 0x72, 0x63, 0x2f, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x2f, 0x6c, 0x61, 0x7a, 0x79, 0x2e, 0x72, 0x73, 0x00, 0x44, 0x75, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74
