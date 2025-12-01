<!--- cspell:word helius -->

# Memo

<!--@include: ./disclaimer.md-->

## Memory layout background

The [SBPF instruction set architecture] defines 12 registers, including
10 general-purpose registers `r0` through `r9`. At the start of program
execution, `r1` [is initialized to] the [input buffer address `MM_INPUT_START`],
corresponding to one of [several runtime memory map regions].

Within the input buffer, data is serialized in the following order:

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

> [!note]
> The assembly file in this example was adapted from [a Helius Blog post]

[The number of accounts as a `u64`]: https://github.com/anza-xyz/agave/blob/v3.1.3/program-runtime/src/serialization.rs#L531
[A sequence of serialized accounts]: https://github.com/anza-xyz/agave/blob/v3.1.3/program-runtime/src/serialization.rs#L532-L566
[The length of instruction data as a `u64`]: https://github.com/anza-xyz/agave/blob/v3.1.3/program-runtime/src/serialization.rs#L567
[The instruction data itself]: https://github.com/anza-xyz/agave/blob/v3.1.3/program-runtime/src/serialization.rs#L568
[several runtime memory map regions]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/src/ebpf.rs#L37-L51
[input buffer address `MM_INPUT_START`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.MM_INPUT_START.html
[is initialized to]: https://github.com/anza-xyz/agave/blob/v3.1.3/programs/bpf_loader/src/lib.rs#L1523
[sbpf instruction set architecture]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/doc/bytecode.md
[a helius blog post]: https://www.helius.dev/blog/sbpf-assembly
