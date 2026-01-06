# Transfer

<!--@include: ./disclaimer.md-->

## :moneybag: Background

This example demonstrates how to transfer SOL ([Lamports]) between accounts
using SBPF assembly. This is a fundamental operation in Solana programs,
requiring proper account validation, ownership checks, and Lamport arithmetic.

A transfer operation requires three accounts:

| Account   | Description                    |
| --------- | ------------------------------ |
| Sender    | The account to transfer from   |
| Recipient | The account to transfer to     |
| System    | The System Program (for [CPI]) |

## :world_map: Account layout background

Accounts in the [input buffer](memo) are [serialized with the following offsets]
relative to the start of the account, assuming non-duplicate accounts without
any account data:

| Offset   | Length (bytes) | Description                |
| -------- | -------------- | -------------------------- |
| `0x0`    | 1              | [`NON_DUP_MARKER`]         |
| `0x50`   | 8              | Lamports balance           |
| `0x58`   | 8              | Length of account data     |
| `0x60`   | 10240          | Account data + padding     |
| `0x2860` | 8              | Account [rent epoch]       |

The account data padding length [is the sum of]:

1. [`MAX_PERMITTED_DATA_INCREASE`].
1. Additional padding to align the account data length [to an 8-byte boundary].

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

[cpi]: https://solana.com/docs/references/terminology#cross-program-invocation-cpi
[is the sum of]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/serialization.rs#L509-L511
[lamports]: https://solana.com/docs/references/terminology#lamport
[rent epoch]: https://solana.com/docs/core/accounts#account-structure
[serialized with the following offsets]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/serialization.rs#L530-L559
[to an 8-byte boundary]: https://docs.rs/solana-program-entrypoint/3.1.1/solana_program_entrypoint/constant.BPF_ALIGN_OF_U128.html
[`max_permitted_data_increase`]: https://docs.rs/solana-program-entrypoint/3.1.1/solana_program_entrypoint/constant.MAX_PERMITTED_DATA_INCREASE.html
[`non_dup_marker`]: https://docs.rs/solana-program-entrypoint/3.1.1/solana_program_entrypoint/constant.NON_DUP_MARKER.html
