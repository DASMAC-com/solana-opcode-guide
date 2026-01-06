# Opcodes

| Opcode hex | Opcode name   | Assembler mnemonic                 | Select example |
| ---------- | ------------- | ---------------------------------- | -------------- |
| [`0x07`]   | [`ADD64_IMM`] | [`add64 dst, imm`][`0x07`]         | [Memo]         |
| [`0x18`]   | [`LD_DW_IMM`] | [`lddw dst, imm`][`0x18`]          | [Quickstart]   |
| [`0x5d`]   | [`JNE_REG`]   | [`jne dst, src, off`][`0x5d`]      | [Memo]         |
| [`0x71`]   | [`LD_B_REG`]  | [`ldxb dst, [src + off]`][`0x71`]  | [Fibonacci]    |
| [`0x79`]   | [`LD_DW_REG`] | [`ldxdw dst, [src + off]`][`0x79`] | [Memo]         |
| [`0x85`]   | [`CALL_IMM`]  | [`call imm`][`0x85`]               | [Quickstart]   |
| [`0x95`]   | [`EXIT`]      | [`exit`][`0x95`]                   | [Quickstart]   |

[fibonacci]: examples/fibonacci
[memo]: examples/memo
[quickstart]: quickstart
[`0x07`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L130
[`0x18`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L222
[`0x5d`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L285
[`0x71`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L230
[`0x79`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L231
[`0x85`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L290
[`0x95`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L294
[`add64_imm`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.ADD64_IMM.html
[`call_imm`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.CALL_IMM.html
[`exit`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.EXIT.html
[`jne_reg`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.JNE_REG.html
[`ld_b_reg`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.LD_B_REG.html
[`ld_dw_imm`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.LD_DW_IMM.html
[`ld_dw_reg`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.LD_DW_REG.html
