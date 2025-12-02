# Resources

## Content

<!-- markdownlint-disable MD013 -->

1. [Scale or Die at Accelerate 2025: Writing Optimized Solana Programs (Dean Little | Blueshift)]
   ([transcript][writing optimized solana programs transcript])
1. [What is the Solana Virtual Machine (SVM)?]

<!-- markdownlint-enable MD013 -->

## Examples

1. [`sbpf` examples]
1. [`hello-solana-asm`]
1. [SBPF opcode test suite]

## Guides

1. [How to Write Solana Programs with SBPF Assembly]
1. [`Learn-Solana-BPF-Assembly`]
1. [sBPF Assembly 101]
1. [Optimizing Solana Programs]

## SBPF references

1. [`solana_sbpf::ebpf` documentation]
1. [SBPF instruction set architecture]
1. [SBPF memory map layout]

## Syscall references

1. [`agave` syscall library]
1. [`sbpf` syscall registry]
1. [`pinocchio` syscall registry]
1. [`solana-sdk` syscall registry]

## Tools

1. [`sbpf`]
1. [LiteSVM]
1. [sbpf.xyz]
1. [VS Code `sbpf-asm` extension]

[`pinocchio` syscall registry]: https://github.com/anza-xyz/pinocchio/blob/pinocchio@v0.9.2/sdk/pinocchio/src/syscalls.rs
[SBPF memory map layout]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/src/ebpf.rs#L37-L51
[What is the Solana Virtual Machine (SVM)?]: https://www.helius.dev/blog/solana-virtual-machine
[how to write solana programs with sbpf assembly]: https://www.helius.dev/blog/sbpf-assembly
[litesvm]: https://www.litesvm.com/
[optimizing solana programs]: https://www.helius.dev/blog/optimizing-solana-programs
[sbpf assembly 101]: https://learn.blueshift.gg/en/courses/introduction-to-assembly/assembly-101
[sbpf instruction set architecture]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/doc/bytecode.md
[sbpf opcode test suite]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/tests/execution.rs
[`solana_sbpf::ebpf` documentation]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/
[sbpf.xyz]: https://sbpf.xyz/
[scale or die at accelerate 2025: writing optimized solana programs (dean little | blueshift)]: https://youtu.be/Fk_UtbEny0c
[vs code `sbpf-asm` extension]: https://marketplace.visualstudio.com/items?itemName=deanmlittle.vscode-sbpf-asm
[writing optimized solana programs transcript]: https://github.com/Laugharne/solana_optimized_programs
[`hello-solana-asm`]: https://github.com/deanmlittle/hello-solana-asm
[`learn-solana-bpf-assembly`]: https://github.com/7etsuo/Learn-Solana-BPF-Assembly
[`sbpf`]: https://github.com/blueshift-gg/sbpf
[`sbpf` examples]: https://github.com/blueshift-gg/sbpf/tree/b7ac3d8/examples
[`sbpf` syscall registry]: https://github.com/blueshift-gg/sbpf/blob/b7ac3d8/crates/common/src/syscalls.rs
[`solana-sdk` syscall registry]: https://github.com/anza-xyz/solana-sdk/blob/frozen-abi-macro@v3.2.0/define-syscall/src/definitions.rs
[`agave` syscall library]: https://github.com/anza-xyz/agave/blob/v3.1.2/syscalls/src/lib.rs
