# Opcodes

This table links opcodes from the [SBPF bytecode ISA] with their corresponding
[Rust implementation constant name]. Each opcode also has a link to a select
example from this guide where it is used.

<!-- markdownlint-disable MD013 -->

| Opcode hex | Opcode name   | Assembler mnemonic                 | Select example |
| ---------- | ------------- | ---------------------------------- | -------------- |
| [`0x07`]   | [`ADD64_IMM`] | [`add64 dst, imm`][`0x07`]         | [Memo]         |
| [`0x14`]   | [`SUB32_IMM`] | [`sub32 dst, imm`][`0x14`]         | [Fibonacci]    |
| [`0x18`]   | [`LD_DW_IMM`] | [`lddw dst, imm`][`0x18`]          | [Quickstart]   |
| [`0x25`]   | [`JGT_IMM`]   | [`jgt dst, imm, off`][`0x25`]      | [Fibonacci]    |
| [`0x5d`]   | [`JNE_REG`]   | [`jne dst, src, off`][`0x5d`]      | [Memo]         |
| [`0x71`]   | [`LD_B_REG`]  | [`ldxb dst, [src + off]`][`0x71`]  | [Fibonacci]    |
| [`0x79`]   | [`LD_DW_REG`] | [`ldxdw dst, [src + off]`][`0x79`] | [Memo]         |
| [`0x85`]   | [`CALL_IMM`]  | [`call imm`][`0x85`]               | [Quickstart]   |
| [`0x95`]   | [`EXIT`]      | [`exit`][`0x95`]                   | [Quickstart]   |
| [`0xb4`]   | [`MOV32_IMM`] | [`mov32 dst, imm`][`0xb4`]         | [Fibonacci]    |
| [`0xb7`]   | [`MOV64_IMM`] | [`mov64 dst, imm`][`0xb7`]         | [Fibonacci]    |
| [`0xbf`]   | [`MOV64_REG`] | [`mov64 dst, src`][`0xbf`]         | [Fibonacci]    |

<!-- markdownlint-enable MD013 -->

[fibonacci]: examples/fibonacci
[memo]: examples/memo
[quickstart]: quickstart
[rust implementation constant name]: https://docs.rs/solana-sbpf/latest/solana_sbpf/ebpf/index.html
[sbpf bytecode isa]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md
[`0x07`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L130
[`0x14`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L87
[`0x18`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L222
[`0x25`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L278
[`0x5d`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L285
[`0x71`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L230
[`0x79`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L231
[`0x85`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L290
[`0x95`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L294
[`0xb4`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L117
[`0xb7`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L161
[`0xbf`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L162
[`add64_imm`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.ADD64_IMM.html
[`call_imm`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.CALL_IMM.html
[`exit`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.EXIT.html
[`jgt_imm`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.JGT_IMM.html
[`jne_reg`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.JNE_REG.html
[`ld_b_reg`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.LD_B_REG.html
[`ld_dw_imm`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.LD_DW_IMM.html
[`ld_dw_reg`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.LD_DW_REG.html
[`mov32_imm`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.MOV32_IMM.html
[`mov64_imm`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.MOV64_IMM.html
[`mov64_reg`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.MOV64_REG.html
[`sub32_imm`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.SUB32_IMM.html
