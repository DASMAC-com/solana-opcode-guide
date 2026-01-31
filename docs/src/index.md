# Solana Opcode Guide

<!-- markdownlint-disable MD013 -->

> [!important] Courtesy of Distributed Atomic State Machine Algorithms Corporation (DASMAC)

<!-- markdownlint-enable MD013 -->

## :books: Background

[Solana programs] are typically written in [Rust], then compiled via [LLVM] into
an [ELF] file with [bytecode] that runs on a [virtual machine]. While
[native Rust] is already considered low-level due to manual memory management,
frameworks like [Anchor] provide further abstractions at the cost of execution
overhead.

At its core, Solana simply runs [SBPF opcodes] (based on [eBPF]) to manipulate
bytes. Mastery of these [opcodes][instruction set architecture] and their
[syscall] support (for things like [logging]) enables high-performance program
development. For example,
[compute unit analysis from this guide](examples/counter#compute-unit-analysis)
shows that Rust can easily introduce 50-100% overhead compared with hand-written
assembly, even when using high-performance frameworks like [`pinocchio`].

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
incrementally learn more advanced [SBPF][instruction set architecture] concepts.

> [!tip]
> See the [resources](resources) page for curated links and the
> [opcodes](opcodes) page for a reference table linking to examples.

["hello, world!" program]: https://en.wikipedia.org/wiki/%22Hello,_World!%22_program
[anchor]: https://www.anchor-lang.com/docs
[bytecode]: https://en.wikipedia.org/wiki/Bytecode
[ebpf]: https://www.rfc-editor.org/rfc/rfc9669
[elf]: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
[instruction set architecture]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/doc/bytecode.md
[llvm]: https://llvm.org/
[logging]: https://docs.rs/solana-msg/3.0.0/src/solana_msg/lib.rs.html#45
[native rust]: https://solana.com/docs/programs/rust
[rust]: https://en.wikipedia.org/wiki/Rust_(programming_language)
[sbpf opcodes]: https://docs.rs/solana-sbpf/latest/solana_sbpf/ebpf/index.html
[solana programs]: https://solana.com/docs/core/programs
[syscall]: https://en.wikipedia.org/wiki/System_call
[virtual machine]: https://en.wikipedia.org/wiki/Virtual_machine
[`pinocchio`]: https://github.com/anza-xyz/pinocchio
