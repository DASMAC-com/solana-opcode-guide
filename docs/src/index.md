# Solana Opcode Guide

*Courtesy of Distributed Atomic State Machine Algorithms Corporation (DASMAC)*

## Background

[Solana programs] are typically written in [Rust], then compiled via [LLVM] into
an Executable and Linkable Format ([ELF]) file that can be deployed to a cluster
like mainnet. In practice, most developers do not concern themselves with the
compilation process or the contents of the executable, even though the resultant
[bytecode] is what actually runs their program logic.

This is because Solana programs, in particular [native Rust] implementations,
are already considered "low-level" due to the manual memory management
operations they perform (which may include byte- or even bit-specific logic).
Moreover, native Rust programs tend to incorporate various other paradigms
typically not encountered outside of embedded systems or other hardware-adjacent
engineering contexts. Hence the proliferation of development frameworks like
[Anchor], which simplify some of the development process at the cost of
execution overhead.

Nevertheless, Solana at its core is a [virtual machine] that simply runs opcodes
to manipulate bytes, and a full mastery of the system's execution mechanics
requires sufficient grasp of the underlying [instruction set architecture].
Specifically, this includes a rudimentary understanding of the [SBPF opcodes],
which are based on [eBPF] and include [syscall] support for utilities like
[logging].

In particular for high-performance applications, opcode-aware programming
methods are an effective tool for optimizing transaction costs and for designing
robust program architectures, and it is the goal of this guide that through an
in-depth exploration of Solana opcodes, developers may improve their command
of the enduring Solana Virtual Machine.

## Example

A Rust operation that checks if `a` is less than `b` looks like:

```rust
if a < b
```

In bytecode this corresponds to the [assembler mnemonic]:

```text
jlt dst, src, off
```

| Term | Meaning |
|-|-|
| `jlt` | The "jump if less than" operation |
| `dst` | The destination register (`a` in Rust) |
| `src` | The source register (`b` in Rust) |
| `off` | How much to increment the program counter by (the "offset") if `dst` is less than `src` |

Inside an ELF file, this `jlt` operation is represented using the number `173`
(or `0xad` in [hexadecimal]), and is encoded in a single byte, corresponding to
the constant [`JLT_REG`] from the [SBPF opcodes].

## How to use this guide

Start by heading over to the [quickstart](quickstart), which will help you set
up your environment and run a simple example.

See also the [resources](resources) page for a curated list of resources to
explore on your journey.

[`jlt_reg`]: https://docs.rs/solana-sbpf/latest/solana_sbpf/ebpf/constant.JLT_REG.html
[anchor]: https://www.anchor-lang.com/docs
[assembler mnemonic]: https://en.wikipedia.org/wiki/Assembly_language#Opcode_mnemonics_and_extended_mnemonics
[bytecode]: https://en.wikipedia.org/wiki/Bytecode
[ebpf]: https://www.rfc-editor.org/rfc/rfc9669
[elf]: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
[hexadecimal]: https://en.wikipedia.org/wiki/Hexadecimal
[instruction set architecture]: https://github.com/anza-xyz/sbpf/blob/main/doc/bytecode.md
[llvm]: https://llvm.org/
[logging]: https://docs.rs/solana-msg/3.0.0/src/solana_msg/lib.rs.html#45
[native rust]: https://solana.com/docs/programs/rust
[rust]: https://en.wikipedia.org/wiki/Rust_(programming_language)
[sbpf opcodes]: https://docs.rs/solana-sbpf/latest/solana_sbpf/ebpf/index.html
[solana programs]: https://solana.com/docs/core/programs
[syscall]: https://en.wikipedia.org/wiki/System_call
[virtual machine]: https://en.wikipedia.org/wiki/Virtual_machine