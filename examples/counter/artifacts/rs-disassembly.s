.globl entrypoint

entrypoint:
  add64 r10, -384
  ldxdw r2, [r1+0]
  jeq r2, 3, jmp_0190
  mov64 r0, 1
  jne r2, 2, jmp_0848
  ldxb r2, [r1+8]
  jne r2, 255, jmp_0900
  ldxdw r2, [r1+88]
  mov64 r6, r1
  add64 r6, r2
  mov64 r0, 5
  add64 r6, 10351
  and64 r6, -8
  ldxb r2, [r6+0]
  jne r2, 255, jmp_0848
  mov64 r0, 3
  ldxdw r2, [r6+80]
  jne r2, 9, jmp_0848
  mov64 r3, r6
  add64 r3, r2
  add64 r3, 10343
  and64 r3, -8
  mov64 r0, 9
  ldxdw r2, [r3+0]
  jne r2, 8, jmp_0848
  add64 r1, 16
  ldxdw r2, [r6+88]
  ldxdw r4, [r3+8]
  add64 r4, r2
  stxdw [r6+88], r4
  ldxb r2, [r6+96]
  mov64 r4, r10
  add64 r4, 304
  stxdw [r10+104], r4
  stxdw [r10+88], r1
  stxb [r10+304], r2
  add64 r3, 16
  stdw [r10+112], 1
  stdw [r10+96], 32
  mov64 r1, r10
  add64 r1, 88
  mov64 r4, r10
  add64 r4, 176
  mov64 r2, 2
  call sol_create_program_address
  jeq r0, 0, jmp_0788
  mov64 r1, r0
  call fn_0998
  mov64 r0, 7
  ja jmp_0848

jmp_0190:
  ldxb r2, [r1+8]
  jne r2, 255, jmp_0900
  mov64 r0, 2
  ldxdw r2, [r1+88]
  jne r2, 0, jmp_0848
  mov64 r7, r1
  add64 r7, r2
  add64 r7, 10351
  and64 r7, -8
  mov64 r0, 5
  ldxb r2, [r7+0]
  jne r2, 255, jmp_0848
  mov64 r0, 3
  ldxdw r3, [r7+80]
  jne r3, 0, jmp_0848
  mov64 r2, r7
  add64 r2, r3
  add64 r2, 10343
  and64 r2, -8
  mov64 r0, 6
  ldxb r3, [r2+0]
  jne r3, 255, jmp_0848
  mov64 r0, 4
  ldxdw r3, [r2+80]
  jne r3, 14, jmp_0848
  mov64 r9, r1
  mov64 r8, r1
  add64 r8, 16
  add64 r2, r3
  mov64 r6, r2
  add64 r6, 10336
  add64 r2, 10343
  and64 r2, -8
  sub64 r2, r6
  add64 r6, r2
  ldxdw r1, [r6+0]
  stxdw [r10+88], r8
  add64 r6, r1
  stdw [r10+96], 32
  add64 r6, 8
  stb [r10+344], 255
  mov64 r1, r10
  add64 r1, 88
  mov64 r4, r10
  add64 r4, 176
  mov64 r5, r10
  add64 r5, 344
  mov64 r2, 1
  mov64 r3, r6
  call sol_try_find_program_address
  jne r0, 0, jmp_08d0
  mov64 r0, 8
  ldxdw r1, [r10+176]
  ldxdw r2, [r7+8]
  jne r2, r1, jmp_0848
  ldxdw r1, [r10+184]
  ldxdw r2, [r7+16]
  jne r2, r1, jmp_0848
  ldxdw r1, [r10+192]
  ldxdw r2, [r7+24]
  jne r2, r1, jmp_0848
  ldxdw r1, [r10+200]
  ldxdw r2, [r7+32]
  jne r2, r1, jmp_0848
  ldxb r1, [r10+344]
  stxw [r10+40], r1
  mov64 r1, r7
  add64 r1, 8
  stxdw [r10+56], r1
  mov64 r1, r10
  add64 r1, 72
  call sol_get_rent_sysvar
  mov64 r1, r9
  ldxdw r2, [r10+72]
  ldxdw r3, [r6+24]
  stxdw [r10+132], r3
  ldxdw r3, [r6+16]
  stxdw [r10+124], r3
  ldxdw r3, [r6+8]
  stxdw [r10+116], r3
  ldxdw r3, [r6+0]
  stxdw [r10+108], r3
  lmul64 r2, 137
  stxdw [r10+92], r2
  stdw [r10+100], 9
  stw [r10+88], 0
  ldxdw r2, [r10+56]
  stxdw [r10+160], r2
  stxdw [r10+144], r8
  sth [r10+168], 257
  sth [r10+152], 257
  mov32 r2, 1
  stxw [r10+48], r2
  ldxb r2, [r1+9]
  mov32 r4, r2
  mov32 r2, 1
  jeq r4, 0, jmp_0850
  ldxb r4, [r1+10]
  mov32 r5, r4
  mov32 r3, 1
  jeq r5, 0, jmp_0878

jmp_04b8:
  ldxb r5, [r1+11]
  mov32 r0, r5
  mov32 r4, 1
  jeq r0, 0, jmp_08a0

jmp_04d8:
  ldxb r0, [r7+1]
  mov32 r6, r0
  mov32 r0, 1
  jne r6, 0, jmp_0500

jmp_04f8:
  mov32 r0, 0

jmp_0500:
  stxw [r10+32], r4
  ldxb r6, [r7+2]
  mov32 r9, r6
  mov32 r6, 1
  jne r9, 0, jmp_0530
  mov32 r6, 0

jmp_0530:
  mov64 r5, r3
  mov64 r4, r2
  ldxb r9, [r7+3]
  mov32 r9, r9
  jne r9, 0, jmp_0568
  mov32 r2, 0
  stxw [r10+48], r2

jmp_0568:
  mov64 r9, r7
  add64 r9, 40
  ldxdw r2, [r1+88]
  ldxdw r3, [r7+80]
  stxdw [r10+264], r9
  mov64 r9, r7
  add64 r9, 88
  stxdw [r10+256], r9
  stxdw [r10+248], r3
  mov64 r3, r7
  add64 r3, 72
  stxdw [r10+240], r3
  ldxdw r3, [r10+56]
  stxdw [r10+232], r3
  mov64 r3, r1
  add64 r3, 48
  stxdw [r10+208], r3
  mov64 r3, r1
  add64 r3, 96
  stxdw [r10+200], r3
  stxdw [r10+192], r2
  add64 r1, 80
  stxdw [r10+184], r1
  ldxw r1, [r10+48]
  stxb [r10+282], r1
  stxb [r10+281], r6
  stxb [r10+280], r0
  ldxw r1, [r10+32]
  stxb [r10+226], r1
  stxb [r10+225], r5
  stxb [r10+224], r4
  stxdw [r10+176], r8
  stdw [r10+272], 0
  stdw [r10+216], 0
  mov64 r1, r10
  add64 r1, 304
  stxdw [r10+288], r1
  mov64 r1, r10
  add64 r1, 343
  stxdw [r10+320], r1
  stxdw [r10+304], r8
  ldxw r6, [r10+40]
  stxb [r10+343], r6
  stdw [r10+296], 2
  stdw [r10+328], 1
  stdw [r10+312], 32
  mov64 r1, r10
  add64 r1, 88
  stxdw [r10+368], r1
  mov64 r1, r10
  add64 r1, 144
  stxdw [r10+352], r1
  mov32 r1, 3055
  hor64 r1, 0
  stxdw [r10+344], r1
  stdw [r10+376], 52
  stdw [r10+360], 2
  mov64 r1, r10
  add64 r1, 344
  mov64 r2, r10
  add64 r2, 176
  mov64 r4, r10
  add64 r4, 288
  mov64 r3, 2
  mov64 r5, 1
  call sol_invoke_signed_c
  stxb [r7+96], r6
  ja jmp_0840

jmp_0788:
  ldxdw r1, [r10+179]
  stxdw [r10+347], r1
  ldxh r1, [r10+177]
  stxh [r10+345], r1
  ldxb r1, [r10+176]
  stxb [r10+344], r1
  ldxb r1, [r10+191]
  stxb [r10+359], r1
  ldxw r1, [r10+187]
  stxw [r10+355], r1
  mov64 r0, 8
  ldxdw r1, [r6+8]
  ldxdw r2, [r10+344]
  jne r1, r2, jmp_0848
  ldxdw r1, [r6+16]
  ldxdw r2, [r10+352]
  jne r1, r2, jmp_0848
  ldxdw r1, [r10+192]
  ldxdw r2, [r6+24]
  jne r2, r1, jmp_0848
  ldxdw r1, [r10+200]
  ldxdw r2, [r6+32]
  jne r2, r1, jmp_0848

jmp_0840:
  mov64 r0, 0

jmp_0848:
  exit

jmp_0850:
  mov32 r2, 0
  ldxb r4, [r1+10]
  mov32 r5, r4
  mov32 r3, 1
  jne r5, 0, jmp_04b8

jmp_0878:
  mov32 r3, 0
  ldxb r5, [r1+11]
  mov32 r0, r5
  mov32 r4, 1
  jne r0, 0, jmp_04d8

jmp_08a0:
  mov32 r4, 0
  ldxb r0, [r7+1]
  mov32 r6, r0
  mov32 r0, 1
  jeq r6, 0, jmp_04f8
  ja jmp_0500

jmp_08d0:
  mov32 r1, 3112
  hor64 r1, 0
  stxdw [r10+176], r1
  mov32 r2, 3128
  hor64 r2, 0
  ja jmp_0928

jmp_0900:
  mov32 r1, 3152
  hor64 r1, 0
  stxdw [r10+176], r1
  mov32 r2, 3168
  hor64 r2, 0

jmp_0928:
  stdw [r10+208], 0
  stdw [r10+184], 1
  stdw [r10+200], 0
  stdw [r10+192], 8
  mov64 r1, r10
  add64 r1, 176
  call fn_0a20

fn_0960:
  ldxdw r2, [r1+8]
  ldxw r4, [r2+20]
  ldxw r3, [r2+16]
  ldxdw r1, [r2+0]
  ldxdw r2, [r2+8]
  add64 r2, -1
  call sol_panic_

fn_0998:
  add64 r10, -64
  jge r1, 3, jmp_09c0
  and32 r1, -1
  mov64 r0, r1
  exit

jmp_09c0:
  mov32 r1, 3192
  hor64 r1, 0
  stxdw [r10+16], r1
  mov32 r2, 3208
  hor64 r2, 0
  stdw [r10+48], 0
  stdw [r10+24], 1
  stdw [r10+40], 0
  stdw [r10+32], 8
  mov64 r1, r10
  add64 r1, 16
  call fn_0a20

fn_0a20:
  add64 r10, -64
  stxdw [r10+48], r2
  stxdw [r10+40], r1
  sth [r10+56], 1
  mov64 r1, r10
  add64 r1, 40
  call fn_0960

.rodata
  data_0000: .byte 0x55, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x66, 0x69, 0x6e, 0x64, 0x20, 0x61, 0x20, 0x76, 0x69, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x62, 0x75, 0x6d, 0x70, 0x20, 0x73, 0x65, 0x65, 0x64, 0x73, 0x72, 0x63, 0x2f, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x2f, 0x6c, 0x61, 0x7a, 0x79, 0x2e, 0x72, 0x73, 0x00, 0x73, 0x72, 0x63, 0x2f, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2e, 0x72, 0x73, 0x00, 0x73, 0x72, 0x63, 0x2f, 0x73, 0x79, 0x73, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x2e, 0x72, 0x73, 0x00, 0x44, 0x75, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x55, 0x6e, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x45, 0x72, 0x72, 0x6f, 0x72
