# Opcodes

| Opcode hex | Assembler mnemonic | Opcode name | Select example |
| ------ | --------- | -------------- | - |
| [`0x07`] | `add64 dst, imm` | [`ADD64_IMM`] | [Memo] |
| [`0x18`] | `lddw dst, imm` | [`LD_DW_IMM`] | [Quickstart] |
| [`0x5d`] | `jne dst, src, off` | [`JNE_REG`] | [Memo] |
| [`0x79`] | `ldxdw dst, [src + off]` | [`LD_DW_REG`] | [Memo] |
| [`0x85`] | `call imm` | [`CALL_IMM`] | [Quickstart] |
| [`0x95`] | `exit` | [`EXIT`] | [Quickstart] |

[`0x07`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L130
[`0x18`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L222
[`0x5d`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L285
[`0x79`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L231
[`0x85`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L290
[`0x95`]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md?plain=1#L294
[`ADD64_IMM`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.ADD64_IMM.html
[memo]: examples/memo
[`JNE_REG`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.JNE_REG.html
[`LD_DW_REG`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.LD_DW_REG.html
[`CALL_IMM`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.CALL_IMM.html
[`exit`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.EXIT.html
[`LD_DW_IMM`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.LD_DW_IMM.html
[quickstart]: quickstart