# Tree

<!-- @include: ./disclaimer.md -->

## Background

This example implements a [red-black tree][wikipedia tree page] in both
[SBPF assembly](../index.md) and Rust. Both implementations are compared
side-by-side with as much implementation parity as possible, using C-style Rust
(raw pointers, direct [syscalls](../indices/syscalls.md)) to minimize compiler
overhead.

::: details Core data structures

<<< ../../../examples/tree/artifacts/snippets/interface/tree-defs-common.txt{rs}

:::

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
it uses C-style bindings with the [`SIMD-0321`] `r2` pointer. Note that the Rust
implementation already introduces overhead at this point in program flow due to
greedy [tail call optimizations][tail call].

::: details Implementations

::: code-group

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/tree/artifacts/snippets/asm/entrypoint-branching.txt{asm} [Assembly]

<<< ../../../examples/tree/artifacts/snippets/rs/entrypoint-branching.txt{rs} [Rust]

:::

::: details Benchmarking

<!-- @include: ../../../examples/tree/artifacts/tests/entrypoint_branching/result.txt{1,3} -->

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

<!-- @include: ../../../examples/tree/artifacts/tests/initialize_input_checks/result.txt{1,19} -->

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

## Insert

### Input checks

::: details Implementations

::: code-group

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/tree/artifacts/snippets/asm/insert-input-checks.txt{asm} [Assembly]

<<< ../../../examples/tree/artifacts/snippets/rs/insert-input-checks.txt{rs} [Rust]

:::

::: details Benchmarking

<!-- @include: ../../../examples/tree/artifacts/tests/insert_input_checks/result.txt{1,7} -->

:::

<!-- markdownlint-enable MD013 -->

### Allocate

::: details Implementations

::: code-group

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/tree/artifacts/snippets/asm/insert-allocate.txt{asm} [Assembly]

<<< ../../../examples/tree/artifacts/snippets/rs/insert-allocate.txt{rs} [Rust]

:::

::: details Benchmarking

<!-- @include: ../../../examples/tree/artifacts/tests/insert_alloc_checks/result.txt{1,10} -->

<!-- @include: ../../../examples/tree/artifacts/tests/insert_alloc/result.txt{1,4} -->
:::

<!-- markdownlint-enable MD013 -->

### Search

::: details Implementations

::: code-group

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/tree/artifacts/snippets/asm/insert-search.txt{asm} [Assembly]

<<< ../../../examples/tree/artifacts/snippets/rs/insert-search.txt{rs} [Rust]

:::

::: details Benchmarking

<!-- @include: ../../../examples/tree/artifacts/tests/insert_search/result.txt{1,5} -->

:::

<!-- markdownlint-enable MD013 -->

### Insert fixup

<!-- markdownlint-disable MD013 -->

::: details Case 1

::: code-group

<<< ../../../examples/tree/artifacts/snippets/asm/insert-fixup-case-1.txt{asm} [Assembly]

<<< ../../../examples/tree/artifacts/snippets/rs/insert-fixup-case-1.txt{rs} [Rust]

:::

::: details Case 4

::: code-group

<<< ../../../examples/tree/artifacts/snippets/asm/insert-fixup-case-4.txt{asm} [Assembly]

<<< ../../../examples/tree/artifacts/snippets/rs/insert-fixup-case-4.txt{rs} [Rust]

:::

::: details Cases 5 and 6 (left)

::: code-group

<<< ../../../examples/tree/artifacts/snippets/asm/insert-fixup-case-5-6-dir-l.txt{asm} [Assembly]

<<< ../../../examples/tree/artifacts/snippets/rs/insert-fixup-case-5-6-dir-l.txt{rs} [Rust]

:::

::: details Cases 5 and 6 (right)

::: code-group

<<< ../../../examples/tree/artifacts/snippets/asm/insert-fixup-case-5-6-dir-r.txt{asm} [Assembly]

<<< ../../../examples/tree/artifacts/snippets/rs/insert-fixup-case-5-6-dir-r.txt{rs} [Rust]

:::

::: details Cases 2 and 3

::: code-group

<<< ../../../examples/tree/artifacts/snippets/asm/insert-fixup-case-2-3.txt{asm} [Assembly]

<<< ../../../examples/tree/artifacts/snippets/rs/insert-fixup-case-2-3.txt{rs} [Rust]

:::

::: details Benchmarking

<!-- @include: ../../../examples/tree/artifacts/tests/insert_to_tree/result.txt{1,25} -->

<!-- @include: ../../../examples/tree/artifacts/tests/multi_insert/result.txt{1,7} -->

:::

<!-- markdownlint-enable MD013 -->

## Remove

### Input checks

::: details Implementations

::: code-group

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/tree/artifacts/snippets/asm/remove-input-checks.txt{asm} [Assembly]

<<< ../../../examples/tree/artifacts/snippets/rs/remove-input-checks.txt{rs} [Rust]

:::

::: details Benchmarking

<!-- @include: ../../../examples/tree/artifacts/tests/remove_input_checks/result.txt{1,7} -->

:::

<!-- markdownlint-enable MD013 -->

### Search

::: details Implementations

::: code-group

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/tree/artifacts/snippets/asm/remove-search.txt{asm} [Assembly]

<<< ../../../examples/tree/artifacts/snippets/rs/remove-search.txt{rs} [Rust]

:::

::: details Benchmarking

<!-- @include: ../../../examples/tree/artifacts/tests/remove_search/result.txt{1,6} -->

:::

<!-- markdownlint-enable MD013 -->

[ilp]: https://en.wikipedia.org/wiki/Instruction-level_parallelism
[pda]: https://solana.com/docs/core/pda
[tail call]: https://en.wikipedia.org/wiki/Tail_call
[wikipedia tree page]: https://en.wikipedia.org/wiki/Red%E2%80%93black_tree
[`pinocchio`]: https://github.com/anza-xyz/pinocchio
[`simd-0321`]: https://github.com/solana-foundation/solana-improvement-documents/blob/main/proposals/0321-vm-r2-instruction-data-pointer.md
