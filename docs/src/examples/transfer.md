# Transfer

<!--@include: ./disclaimer.md-->

## :moneybag: Background

The transfer example demonstrates how to transfer SOL ([lamports]) between
accounts using SBPF assembly. This is a fundamental operation in Solana
programs, requiring proper account validation, ownership checks, and lamport
arithmetic.

A transfer operation requires three accounts:

| Account | Description                    |
| ------- | ------------------------------ |
| Source  | The account to transfer from   |
| Destination | The account to transfer to |
| System  | The System Program (for CPI)   |

## :shield: Input validation

The program first validates that exactly 3 accounts are passed by checking
the [number of accounts in the input buffer](memo).

::: details Full program

<<< ../../../examples/transfer/src/transfer/transfer.s{asm}

:::

## :white_check_mark: All tests

::: details `tests.rs`

<<< ../../../examples/transfer/src/tests.rs{rs:line-numbers}

:::

[lamports]: https://solana.com/docs/references/terminology#lamport
