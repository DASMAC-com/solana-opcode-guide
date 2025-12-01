<!--- cspell:word helius -->

# Memo

<!--@include: ./disclaimer.md-->

## Memory layout background

The [SBPF bytecode instruction set architecture] defines 12 registers, including
10 general-purpose registers `r0` through `r9`. At the start of program
execution, `r1` [is initialized to] the [input buffer address `MM_INPUT_START`],
corresponding to one of [several runtime memory regions].

> [!note]
> The assembly file in this example was adapted from [a Helius Blog post]

[several runtime memory regions]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/src/ebpf.rs#L37-L51
[input buffer address `MM_INPUT_START`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.MM_INPUT_START.html
[is initialized to]: https://github.com/anza-xyz/agave/blob/v3.1.3/programs/bpf_loader/src/lib.rs#L1523
[sbpf bytecode instruction set architecture]: https://github.com/anza-xyz/sbpf/blob/main/doc/bytecode.md
[a helius blog post]: https://www.helius.dev/blog/sbpf-assembly
