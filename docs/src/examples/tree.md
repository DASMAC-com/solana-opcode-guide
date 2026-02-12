# Tree

<!-- @include: ./disclaimer.md -->

## Background

This example implements a [red-black tree][wikipedia tree page] in both
[SBPF assembly](../index.md) and Rust. Both implementations are compared
side-by-side with as much implementation parity as possible, using C-style Rust
(raw pointers, direct [syscalls](../indices/syscalls.md)) to minimize compiler
overhead.

## Build support

Constants, error codes, and C bindings are derived in a shared interface using
macros, then automatically inserted into the assembly program file at build
time.

::: details Interface

::: code-group

<<< ../../../examples/tree/interface/src/common.rs{rs:line-numbers}

<<< ../../../examples/tree/interface/src/asm.rs{rs:line-numbers}

<<< ../../../examples/tree/interface/src/bindings.rs{rs:line-numbers}

:::

::: details `build.rs`

<<< ../../../examples/tree/build.rs{rs:line-numbers}

:::

::: details Macros

<<< ../../../examples/tree/macros/src/lib.rs{rs:line-numbers}

:::

## Entrypoint branching

The Rust implementation does not use [`pinocchio`] for the entrypoint. Instead,
it uses C-style bindings with the [`SIMD-0321`] `r2` pointer.

::: details Implementations

::: code-group

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/tree/artifacts/snippets/asm/entrypoint-branching.txt{asm} [Assembly]

<<< ../../../examples/tree/artifacts/snippets/rs/entrypoint-branching.txt{rs} [Rust]

:::

::: details Benchmarking

<!-- @include: ../../../examples/tree/artifacts/tests/entrypoint_branching/result.txt{1,6} -->

:::

## Initialize

The initialize operation creates a tree [PDA] for the entire program, then
invokes a [`CreateAccount` CPI](counter#cpi-construction), with the same
[fixed costs as in the counter example](counter#compute-unit-analysis).

### Input checks

::: details Implementations

::: code-group

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/tree/artifacts/snippets/asm/initialize-input-checks.txt{asm} [Assembly]

<<< ../../../examples/tree/artifacts/snippets/rs/initialize-input-checks.txt{rs} [Rust]

:::

::: details Benchmarking

<!-- @include: ../../../examples/tree/artifacts/tests/initialize_input_checks/result.txt{1,17} -->

:::

<!-- markdownlint-enable MD013 -->

### PDA checks

::: details Implementations

::: code-group

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/tree/artifacts/snippets/asm/initialize-pda-checks.txt{asm} [Assembly]

<<< ../../../examples/tree/artifacts/snippets/rs/initialize-pda-checks.txt{rs} [Rust]

:::

::: details Benchmarking

<!-- @include: ../../../examples/tree/artifacts/tests/initialize_pda_checks/result.txt{1,6} -->

:::

<!-- markdownlint-enable MD013 -->

### Create account

The assembly implementation includes pointer walkthrough optimizations that are
not available in Rust, since the compiler enforces
[instruction-level parallelism][ilp].

::: details Implementations

::: code-group

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/tree/artifacts/snippets/asm/initialize-create-account.txt{asm} [Assembly]

<<< ../../../examples/tree/artifacts/snippets/rs/initialize-create-account.txt{rs} [Rust]

:::

::: details Benchmarking

<!-- @include: ../../../examples/tree/artifacts/tests/initialize_create_account/result.txt{1,5} -->

:::

<!-- markdownlint-enable MD013 -->

## :white_check_mark: All tests

::: details `tests.rs`

<<< ../../../examples/tree/src/tests.rs{rs:line-numbers}

:::

[ilp]: https://en.wikipedia.org/wiki/Instruction-level_parallelism
[compute unit]: https://solana.com/docs/references/terminology#compute-units
[pda]: https://solana.com/docs/core/pda
[wikipedia tree page]: https://en.wikipedia.org/wiki/Red%E2%80%93black_tree
[`pinocchio`]: https://github.com/anza-xyz/pinocchio
[`simd-0321`]: https://github.com/solana-foundation/solana-improvement-documents/blob/main/proposals/0321-vm-r2-instruction-data-pointer.md