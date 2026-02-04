# Tree

<!-- @include: ./disclaimer.md -->

## Background

This example implements a [red-black tree][wikipedia tree page] in both
[SBPF assembly](../index.md) and Rust. It benchmarks various operations and code
paths side-by-side for a comprehensive breakdown of assembly vs Rust
performance.

## Input buffer checks

::: code-group

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/tree/artifacts/snippets/asm/check-input-buffer.txt{asm} [Assembly]

<<< ../../../examples/tree/artifacts/snippets/rs/check-input-buffer.txt{rs} [Rust]

:::

<!-- @include: ../../../examples/tree/artifacts/tests/fast_fails/result.txt{1,6} -->

<!-- markdownlint-enable MD013 -->

## :white_check_mark: All tests

::: details `tests.rs`

<<< ../../../examples/tree/src/tests.rs{rs:line-numbers}

:::

[wikipedia tree page]: https://en.wikipedia.org/wiki/Red%E2%80%93black_tree