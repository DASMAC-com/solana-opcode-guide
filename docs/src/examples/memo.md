<!--- cspell:word helius -->

# Memo

<!--@include: ./disclaimer.md-->

## Memory layout background

The [SBPF instruction set architecture] defines 12 registers, including
10 general-purpose registers `r0` through `r9`. At the start of program
execution, `r1` [is initialized to] the [input buffer address `MM_INPUT_START`],
corresponding to one of [several runtime memory map regions].

[Within the input buffer], data is serialized in the following order:

1. [The number of accounts as a `u64`]
1. [A sequence of serialized accounts]
1. [The length of instruction data as a `u64`]
1. [The instruction data itself]

Hence for a transaction that accepts no accounts and includes a memo string to
print, the input buffer layout is as follows:

| Offset | Size      | Description                |
| ------ | --------- | -------------------------- |
| `0`    | `8` bytes | Number of accounts (`0`)   |
| `8`    | `8` bytes | Length of instruction data |
| `16`   | N bytes   | Instruction data (memo)    |

Related constants are defined at the top of the assembly implementation and used
throughout the remainder of the program:

<<< ../../../examples/memo/src/memo/memo.s{asm:line-numbers}

## Register operations

The value in `r0` at the conclusion of an SBPF program
[is considered the return value], where [a return value of 0] indicates
[success].

Hence the [`ldxdw` (load indexed double word) `LD_DW_REG`] opcode effectively
triggers an error code if a nonzero number of accounts are passed, by loading
into `r0` the value pointed to by `r1 + NUM_ACCOUNTS_OFFSET`: the number of
accounts passed.

Once the return code is set, the [`jne` (jump if not equal) `JNE_REG`] opcode then
compares it against the value in `r4`, which is initialized to zero: by default
[all registers are initialized to zero in a new virtual machine instance] except
for an [immediate modification] to the [frame pointer register (`r10`)], and
[pre-execution modifications to `r1` and optionally `r2`]. If `r0` and `r4` are
unequal, the program jumps immediately to the [`exit`] opcode.

Notably this comparison is performed using registers instead of using an
[immediate value] of zero, for example `jne r0, 0, 3`, since this approach would
use [`JNE_IMM`] and therefore only compare `r0` against
[32 bits from an immediate][immediate value] as opposed to
[all 64 register bits] from `r4`.

> [!note]
> The assembly file in this example was adapted from [a Helius Blog post]

[`exit`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.EXIT.html
[all 64 register bits]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/doc/bytecode.md#registers
[immediate value]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/doc/bytecode.md#instruction-layout
[`JNE_IMM`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.JNE_IMM.html
[pre-execution modifications to `r1` and optionally `r2`]: https://github.com/anza-xyz/agave/blob/v3.1.3/programs/bpf_loader/src/lib.rs#L1523-L1528
[immediate modification]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/src/vm.rs#L318
[frame pointer register (`r10`)]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.FRAME_PTR_REG.html
[all registers are initialized to zero in a new virtual machine instance]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/src/vm.rs#L317
[`jne` (jump if not equal) `JNE_REG`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.JNE_REG.html
[`ldxdw` (load indexed double word) `LD_DW_REG`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.LD_DW_REG.html
[a return value of 0]: https://github.com/anza-xyz/agave/blob/v3.1.3/programs/bpf_loader/src/lib.rs#L1557-L1560
[success]: https://docs.rs/solana-program-entrypoint/3.1.1/solana_program_entrypoint/constant.SUCCESS.html
[Within the input buffer]: https://github.com/anza-xyz/agave/blob/v3.1.3/program-runtime/src/serialization.rs#L524
[The number of accounts as a `u64`]: https://github.com/anza-xyz/agave/blob/v3.1.3/program-runtime/src/serialization.rs#L531
[A sequence of serialized accounts]: https://github.com/anza-xyz/agave/blob/v3.1.3/program-runtime/src/serialization.rs#L532-L566
[The length of instruction data as a `u64`]: https://github.com/anza-xyz/agave/blob/v3.1.3/program-runtime/src/serialization.rs#L567
[The instruction data itself]: https://github.com/anza-xyz/agave/blob/v3.1.3/program-runtime/src/serialization.rs#L568
[several runtime memory map regions]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/src/ebpf.rs#L37-L51
[input buffer address `MM_INPUT_START`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.MM_INPUT_START.html
[is initialized to]: https://github.com/anza-xyz/agave/blob/v3.1.3/programs/bpf_loader/src/lib.rs#L1523
[sbpf instruction set architecture]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/doc/bytecode.md
[a helius blog post]: https://www.helius.dev/blog/sbpf-assembly
[is considered the return value]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/src/interpreter.rs#L574