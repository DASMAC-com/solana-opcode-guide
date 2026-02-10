# Tree

<!-- @include: ./disclaimer.md -->

## Background

This example implements a [red-black tree][wikipedia tree page] in both
[SBPF assembly](../index.md) and Rust. It benchmarks various operations and code
paths side-by-side for a comprehensive breakdown of assembly vs Rust
performance.

## Entrypoint branching

::: code-group

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/tree/artifacts/snippets/asm/entrypoint-branching.txt{asm} [Assembly]

<<< ../../../examples/tree/artifacts/snippets/rs/entrypoint-branching.txt{rs} [Rust]

:::

<!-- @include: ../../../examples/tree/artifacts/tests/entrypoint_branching/result.txt{1,6} -->

## Initialize input checks

::: code-group

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/tree/artifacts/snippets/asm/initialize-input-checks.txt{asm} [Assembly]

<<< ../../../examples/tree/artifacts/snippets/rs/initialize-input-checks.txt{rs} [Rust]

:::

<!-- @include: ../../../examples/tree/artifacts/tests/initialize_input_checks/result.txt{1,13} -->

<!-- markdownlint-enable MD013 -->

## Initialize PDA checks

::: code-group

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/tree/artifacts/snippets/asm/initialize-pda-checks.txt{asm} [Assembly]

<<< ../../../examples/tree/artifacts/snippets/rs/initialize-pda-checks.txt{rs} [Rust]

:::

<!-- @include: ../../../examples/tree/artifacts/tests/initialize_pda_checks/result.txt{1,6} -->

<!-- markdownlint-enable MD013 -->

<!-- @include: ../../../examples/tree/artifacts/tests/initialize_pda_checks/result.txt{1,6} -->

## :white_check_mark: All tests

::: details `tests.rs`

<<< ../../../examples/tree/src/tests.rs{rs:line-numbers}

:::

[wikipedia tree page]: https://en.wikipedia.org/wiki/Red%E2%80%93black_tree