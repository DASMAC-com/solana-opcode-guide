<!--- cspell:word helius -->

# Memo

<!-- @include: ./disclaimer.md -->

## :world_map: Memory map background

The [SBPF instruction set architecture] defines 12 registers, including
10 general-purpose registers `r0` through `r9`. At the start of instruction
execution, `r1` [is initialized to] the [input buffer address `MM_INPUT_START`],
corresponding to one of [several runtime memory map regions].

[Within the input buffer], data is serialized as follows:

| Description                                 | Size (bytes) |
| ------------------------------------------- | ------------ |
| [The number of accounts as a `u64`]         | 8            |
| [A sequence of serialized accounts]         | Variable     |
| [The length of instruction data as a `u64`] | 8            |
| [Instruction data]                          | Variable     |
| [The calling program ID]                    | 32           |

> [!tip]
> A new virtual memory map is created for [every instruction] _and_ for every
> [instance] of a [CPI] (which contains an
> [inner call to an instruction processor] whose [own inner call] generates
> [a fresh memory map]).

Hence for an instruction that accepts no accounts and includes a memo string to
print, the input buffer layout is as follows:

| Offset | Size      | Description                |
| ------ | --------- | -------------------------- |
| `0`    | `8` bytes | Number of accounts (`0`)   |
| `8`    | `8` bytes | Length of instruction data |
| `16`   | N bytes   | Instruction data (memo)    |

Related constants are defined at the top of the assembly implementation and used
throughout the remainder of the program:

<<< ../../../examples/memo/artifacts/snippets/asm/constants.txt{1-3 asm}

::: details Full program

<<< ../../../examples/memo/src/memo/memo.s{asm}

:::

## :warning: Error checking

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
[all 64 register bits] from `r4`:

<<< ../../../examples/memo/artifacts/snippets/asm/error-checking.txt{4-7 asm}

::: details Full program

<<< ../../../examples/memo/src/memo/memo.s{asm}

:::

Note the minimal [compute unit] consumption for a failure:

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/memo/artifacts/tests/asm_fail/result.txt{3 sh:line-numbers}

<!-- markdownlint-enable MD013 -->

::: details `test_asm_fail`

<<< ../../../examples/memo/artifacts/tests/asm_fail/test.txt{rs:line-numbers}
:::

## :speech_balloon: Logging

Assuming no accounts are passed, the length of the message is similarly loaded
via [`ldxdw` (load indexed double word) `LD_DW_REG`] into `r2` via an offset
reference to `r1`. Then `r1` is itself incremented via
[`add64` (add to 64-bit register an immediate value) `ADD64_IMM`] by the
instruction data offset, a value known to fit in 32 bits.

These operations preposition a [`call` via `CALL_IMM`] to [`sol_log_`], which
[takes the following arguments]:

| Register | Value                             |
| -------- | --------------------------------- |
| `r1`     | The address of the message to log |
| `r2`     | The number of bytes to log        |

After the logging operation, the program concludes:

<<< ../../../examples/memo/artifacts/snippets/asm/logging.txt{6-11 asm}

::: details Full program

<<< ../../../examples/memo/src/memo/memo.s{asm}

:::

Note the [compute unit] consumption for a successful log:

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/memo/artifacts/tests/asm_pass/result.txt{4 sh:line-numbers}

<!-- markdownlint-enable MD013 -->

::: details `test_asm_pass`

<<< ../../../examples/memo/artifacts/tests/asm_pass/test.txt{rs:line-numbers}

:::

## :crab: Rust implementation

The Rust implementation similarly calls [the `pinocchio` version of `sol_log_`]
with the passed instruction data.

<<< ../../../examples/memo/src/program.rs{12,16 rs:line-numbers}

Notably, however, it introduces [compute unit] overhead:

<<< ../../../examples/memo/artifacts/tests/rs/result.txt{4 sh:line-numbers}

::: details `test_rs`

<<< ../../../examples/memo/artifacts/tests/rs/test.txt{rs:line-numbers}
:::

## :white_check_mark: All tests

::: details `tests.rs`

<<< ../../../examples/memo/src/tests.rs{rs:line-numbers}

:::

> [!note]
> The assembly file in this example was adapted from [a Helius Blog post]

[a fresh memory map]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/invoke_context.rs#L555-L556
[a helius blog post]: https://www.helius.dev/blog/sbpf-assembly
[a return value of 0]: https://github.com/anza-xyz/agave/blob/v3.1.3/programs/bpf_loader/src/lib.rs#L1557-L1560
[a sequence of serialized accounts]: https://github.com/anza-xyz/agave/blob/v3.1.3/program-runtime/src/serialization.rs#L532-L566
[all 64 register bits]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/doc/bytecode.md#registers
[all registers are initialized to zero in a new virtual machine instance]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/src/vm.rs#L317
[compute unit]: https://solana.com/docs/references/terminology#compute-units
[cpi]: https://solana.com/docs/core/cpi
[every instruction]: https://github.com/anza-xyz/agave/blob/v3.1.5/programs/bpf_loader/src/lib.rs#L1512
[frame pointer register (`r10`)]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.FRAME_PTR_REG.html
[immediate modification]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/src/vm.rs#L318
[immediate value]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/doc/bytecode.md#instruction-layout
[inner call to an instruction processor]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/cpi.rs#L882
[input buffer address `mm_input_start`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.MM_INPUT_START.html
[instance]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/cpi.rs#L802
[instruction data]: https://github.com/anza-xyz/agave/blob/v3.1.3/program-runtime/src/serialization.rs#L568
[is considered the return value]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/src/interpreter.rs#L574
[is initialized to]: https://github.com/anza-xyz/agave/blob/v3.1.3/programs/bpf_loader/src/lib.rs#L1523
[own inner call]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/invoke_context.rs#L484
[pre-execution modifications to `r1` and optionally `r2`]: https://github.com/anza-xyz/agave/blob/v3.1.3/programs/bpf_loader/src/lib.rs#L1523-L1528
[sbpf instruction set architecture]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/doc/bytecode.md
[several runtime memory map regions]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/src/ebpf.rs#L37-L51
[success]: https://docs.rs/solana-program-entrypoint/3.1.1/solana_program_entrypoint/constant.SUCCESS.html
[takes the following arguments]: https://github.com/anza-xyz/agave/blob/v3.1.3/syscalls/src/logging.rs#L7-L16
[the calling program id]: https://github.com/anza-xyz/agave/blob/v3.1.3/program-runtime/src/serialization.rs#L569
[the length of instruction data as a `u64`]: https://github.com/anza-xyz/agave/blob/v3.1.3/program-runtime/src/serialization.rs#L567
[the number of accounts as a `u64`]: https://github.com/anza-xyz/agave/blob/v3.1.3/program-runtime/src/serialization.rs#L531
[the `pinocchio` version of `sol_log_`]: https://github.com/anza-xyz/pinocchio/blob/pinocchio@v0.9.2/sdk/pinocchio/src/syscalls.rs#L42
[within the input buffer]: https://github.com/anza-xyz/agave/blob/v3.1.3/program-runtime/src/serialization.rs#L524
[`add64` (add to 64-bit register an immediate value) `add64_imm`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.ADD64_IMM.html
[`call` via `call_imm`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.CALL_IMM.html
[`exit`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.EXIT.html
[`jne_imm`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.JNE_IMM.html
[`jne` (jump if not equal) `jne_reg`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.JNE_REG.html
[`ldxdw` (load indexed double word) `ld_dw_reg`]: https://docs.rs/solana-sbpf/0.13.1/solana_sbpf/ebpf/constant.LD_DW_REG.html
[`sol_log_`]: https://github.com/anza-xyz/agave/blob/v3.1.3/syscalls/src/lib.rs#L345
