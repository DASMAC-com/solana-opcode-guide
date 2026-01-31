# Solana Opcode Guide

<!-- markdownlint-disable MD013 -->

> [!important] Courtesy of Distributed Atomic State Machine Algorithms Corporation (DASMAC)

<!-- markdownlint-enable MD013 -->

## :books: Background

[Solana programs] are typically written in [Rust], then compiled via [LLVM] into
an [ELF] file with [bytecode] that runs on a [virtual machine].
[Native Rust techniques], generally considered low-level due to the
manual memory management operations they perform entail, are often further
supplemented by higher-level frameworks like [Anchor] that provide further
abstractions at the cost of execution overhead.

Yet at its core, Solana simply runs [SBPF opcodes] (based on [eBPF]) to
manipulate bytes. Mastery of these [opcodes][isa] and their [syscall] support
(for things like [logging]) enables high-performance program development in
excess of what traditional techniques can deliver.

For example,
[compute unit analysis from this guide](examples/counter#compute-unit-analysis)
shows that Rust can easily introduce 50-100% overhead compared with hand-written
assembly, even when using high-performance frameworks like [`pinocchio`].

Contrary to typical approaches of higher-level frameworks and abstractions, this
guide surveys optimization techniques that are only possible when writing in
assembly, helping you gain full control over program execution to squeeze as
much performance as possible out of the [Solana Virtual Machine].

## :bulb: Example

Here is a simple ["Hello, World!" program] implemented in both SBPF assembly and
Rust:

::: code-group

<<< ../../examples/hello-dasmac/src/hello-dasmac/hello-dasmac.s{asm} [Assembly]

<<< ../../examples/hello-dasmac/src/program.rs [Rust]

:::

## :rocket: Continue your journey

Start with the [quickstart](quickstart) to set up your environment and run the
above program, then follow the [examples](examples/index) in order to
incrementally learn more advanced [SBPF][isa] concepts.

See also the following pages:

| Page                   | Content                                           |
| ---------------------- | ------------------------------------------------- |
| [Resources](resources) | Curated resource links                            |
| [Opcodes](opcodes)     | [SBPF opcode][isa] reference linked with examples |
| [Syscalls](syscalls)   | [Solana syscalls] reference linked with examples  |

["hello, world!" program]: https://en.wikipedia.org/wiki/%22Hello,_World!%22_program
[anchor]: https://www.anchor-lang.com/docs
[bytecode]: https://en.wikipedia.org/wiki/Bytecode
[ebpf]: https://www.rfc-editor.org/rfc/rfc9669
[elf]: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
[isa]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/doc/bytecode.md
[llvm]: https://llvm.org/
[logging]: https://docs.rs/solana-msg/3.0.0/src/solana_msg/lib.rs.html#45
[native rust techniques]: https://solana.com/docs/programs/rust
[rust]: https://en.wikipedia.org/wiki/Rust_(programming_language)
[sbpf opcodes]: https://docs.rs/solana-sbpf/latest/solana_sbpf/ebpf/index.html
[solana programs]: https://solana.com/docs/core/programs
[solana syscalls]: https://github.com/anza-xyz/solana-sdk/blob/frozen-abi-macro@v3.2.0/define-syscall/src/definitions.rs
[solana virtual machine]: https://docs.rs/crate/solana-sbpf/latest
[syscall]: https://en.wikipedia.org/wiki/System_call
[virtual machine]: https://en.wikipedia.org/wiki/Virtual_machine
[`pinocchio`]: https://github.com/anza-xyz/pinocchio
