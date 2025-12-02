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

<<< ../../../examples/memo/src/memo/memo.s{1-3 asm:line-numbers}

## Error checking

The value in `r0` at the conclusion of an SBPF program
[is considered the return value], where [a return value of 0] indicates
[success].

Hence the [`ldxdw` (load indexed double word) `LD_DW_REG`] opcode effectively
triggers an error code if a nonzero number of accounts are passed, by loading
into `r0` the value pointed to by `r1 + NUM_ACCOUNTS_OFFSET`: the number of
accounts passed.

Once the return code is set, the [`jne` (jump if not equal) `JNE_REG`] opcode
then compares it against the value in `r4`, which is initialized to zero: by
default
[all registers are initialized to zero in a new virtual machine instance] except
for an [immediate modification] to the [frame pointer register (`r10`)], and
[pre-execution modifications to `r1` and optionally `r2`]. If `r0` and `r4` are
unequal (if the number of accounts is nonzero), the program jumps immediately to
the [`exit`] opcode.

Notably this comparison is performed using registers instead of using an
[immediate value] of zero, for example `jne r0, 0, 3`, since this approach would
use [`JNE_IMM`] and therefore only compare `r0` against
[32 bits from an immediate][immediate value] as opposed to
[all 64 register bits] from `r4`.

<<< ../../../examples/memo/src/memo/memo.s{5-11 asm:line-numbers}

Note the minimal [compute unit] consumption for a failure:

::: details `test_asm_fail` {open}

<<< ../../../examples/memo/test-runs/asm_fail.txt{2 sh:line-numbers}

:::

## Logging

Assuming no accounts are passed, the length of the message is similarly loaded
via [`ldxdw` (load indexed double word) `LD_DW_REG`] into `r2` via an offset
reference to `r1`. Then `r1` is itself incremented via
[`add64` (add to 64-bit register an immediate value) `ADD64_IMM`] by the
instruction data offset, a value known to fit in 32 bits.

These operations preposition a [`call` via `CALL_IMM`] to [`sol_log_`], which
[takes the following arguments]:

| Register | Value |
| - | - |
| `r1` | The address of the message to log |
| `r2` | The number of bytes to log |

After the logging operation, the program concludes.

<<< ../../../examples/memo/src/memo/memo.s{12-17 asm:line-numbers}

Note the [compute unit] consumption for a successful log:

<<< ../../../examples/memo/test-runs/asm_pass.txt{3 sh:line-numbers}

## Rust implementation

The rust implementation similarly calls [the `pinocchio` version of `sol_log_`]
with the passed instruction data.

<<< ../../../examples/memo/src/program.rs{12,16 rs:line-numbers}

Notably, however, it introduces [compute unit] overhead:

<<< ../../../examples/memo/test-runs/rs.txt{3 sh:line-numbers}

## Tests

::: details Tests

<<< ../../../examples/memo/src/tests.rs{rs:line-numbers}

:::

> [!note]
> The assembly file in this example was adapted from [a Helius Blog post]

[the `pinocchio` version of `sol_log_`]: https://github.com/anza-xyz/pinocchio/blob/pinocchio@v0.9.2/sdk/pinocchio/src/syscalls.rs#L42
[compute unit]: https://solana.com/docs/references/terminology#compute-units
[takes the following arguments]: https://github.com/anza-xyz/agave/blob/v3.1.3/syscalls/src/logging.rs#L7-L16
[`sol_log_`]: https://github.com/anza-xyz/agave/blob/v3.1.3/syscalls/src/lib.rs#L345
[`call` via `CALL_IMM`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.CALL_IMM.html
[`add64` (add to 64-bit register an immediate value) `ADD64_IMM`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.ADD64_IMM.html
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