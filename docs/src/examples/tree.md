# Tree

<!-- @include: ./disclaimer.md -->

## Background

This example implements a [red-black tree][wikipedia tree page] in both
[SBPF assembly](../index.md) and Rust. It benchmarks various operations and code
paths side-by-side for a comprehensive breakdown of assembly vs Rust
performance.

## :white_check_mark: All tests

::: details `tests.rs`

<<< ../../../examples/tree/src/tests.rs{rs:line-numbers}

:::

[wikipedia tree page]: https://en.wikipedia.org/wiki/Red%E2%80%93black_tree