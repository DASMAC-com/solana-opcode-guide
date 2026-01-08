# Transfer

<!--@include: ./disclaimer.md-->

## :moneybag: Background

This example demonstrates how to transfer SOL ([Lamports]) between accounts
using SBPF assembly. This is a fundamental operation in Solana programs,
requiring proper account validation, ownership checks, and Lamport arithmetic.

A transfer operation requires three accounts:

| Account   | Description                  |
| --------- | ---------------------------- |
| Sender    | The account to transfer from |
| Recipient | The account to transfer to   |
| System    | [System Program] (for [CPI]) |

## :world_map: Account layout background

Accounts in the [input buffer](memo) are [serialized with the following offsets]
relative to the start of the account, assuming non-duplicate accounts without
any account data:

<!-- markdownlint-disable MD013 -->

| Offset (bytes) | Length (bytes) | Description                                 |
| -------------- | -------------- | ------------------------------------------- |
| 0              | 1              | [`NON_DUP_MARKER`]                          |
| 1              | 1              | Is [signer]?                                |
| 2              | 1              | Is [writable]?                              |
| 3              | 1              | Is [executable][account structure]?         |
| 4              | 4              | Padding                                     |
| 8              | 32             | [Account pubkey]                            |
| 40             | 32             | Account [owner][account structure]          |
| 72             | 8              | [Lamports balance][account structure]       |
| 80             | 8              | Length of account data                      |
| 88             | 10240          | [Account data][account structure] + padding |
| 10328          | 8              | Account [rent epoch][account structure]     |

<!-- markdownlint-enable MD013 -->

The account data padding length [is the sum of]:

1. [`MAX_PERMITTED_DATA_INCREASE`].
1. Additional padding to align the account data length [to an 8-byte boundary].

Note however that the [System Program] is a [builtin], which means that its
[account data is its name], specifically `b"system_program"` (14 bytes) with two
extra bytes of padding to align to an 8-byte boundary. This means that the
`account data + padding` field of the System Program is actually 10256 bytes
long.

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

> [!note]
> The assembly file and testing framework in this example were adapted from an
> [`sbpf` example].

[account data is its name]: https://github.com/anza-xyz/agave/blob/v3.1.5/runtime/src/bank.rs#L5754
[account pubkey]: https://github.com/anza-xyz/agave/blob/v3.1.5/transaction-context/src/transaction_accounts.rs#L26
[account structure]: https://solana.com/docs/core/accounts#account-structure
[builtin]: https://github.com/anza-xyz/agave/blob/v3.1.5/builtins/src/lib.rs#L62-L68
[cpi]: https://solana.com/docs/references/terminology#cross-program-invocation-cpi
[is the sum of]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/serialization.rs#L509-L511
[lamports]: https://solana.com/docs/references/terminology#lamport
[serialized with the following offsets]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/serialization.rs#L530-L559
[signer]: https://github.com/anza-xyz/agave/blob/v3.1.5/transaction-context/src/lib.rs#L78-L79
[system program]: https://solana.com/docs/core/programs#the-system-program
[to an 8-byte boundary]: https://docs.rs/solana-program-entrypoint/3.1.1/solana_program_entrypoint/constant.BPF_ALIGN_OF_U128.html
[writable]: https://github.com/anza-xyz/agave/blob/v3.1.5/transaction-context/src/lib.rs#L80-L81
[`max_permitted_data_increase`]: https://docs.rs/solana-program-entrypoint/3.1.1/solana_program_entrypoint/constant.MAX_PERMITTED_DATA_INCREASE.html
[`non_dup_marker`]: https://docs.rs/solana-program-entrypoint/3.1.1/solana_program_entrypoint/constant.NON_DUP_MARKER.html
[`sbpf` example]: https://github.com/blueshift-gg/sbpf/blob/b7ac3d80da4400abff283fb0e68927c3c68a24d9/examples/sbpf-asm-cpi/src/sbpf-asm-cpi/sbpf-asm-cpi.s
