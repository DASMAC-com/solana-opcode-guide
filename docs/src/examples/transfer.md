# Transfer

<!--@include: ./disclaimer.md-->

## :moneybag: Background

This example demonstrates how to transfer SOL ([Lamports]) between accounts
using SBPF assembly. This is a fundamental operation in Solana programs,
requiring proper account validation, ownership checks, and Lamport arithmetic.

A transfer operation requires three accounts:

| Account   | Description                                 |
| --------- | ------------------------------------------- |
| Sender    | The account to transfer from                |
| Recipient | The account to transfer to                  |
| System    | [System Program] (for [CPI](#transfer-cpi)) |

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
`account data + padding` portion of the System Program is actually 10256 bytes
long.

## :shield: Account validation

Assembly offsets are validated in Rust using struct operations:

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/transfer/artifacts/snippets/asm/offsets.txt{1-11 asm} [Assembly]

<!-- markdownlint-enable MD013 -->

::: details `test_offsets`

<<< ../../../examples/transfer/artifacts/tests/offsets/test.txt{rs}

:::

Due to the account layout order, account layout input validation takes place in
a specific sequence:

<<< ../../../examples/transfer/artifacts/snippets/asm/accounts.txt{4-30 asm}

## :outbox_tray: Transfer CPI {#transfer-cpi}

The [System Program] is responsible for transferring Lamports between accounts,
and is invoked internally in this example using a [CPI] via the
[`sol_invoke_signed_c` syscall], which accepts the following parameters:

| Register | Description                  |
| -------- | ---------------------------- |
| `r1`     | [Instruction] pointer        |
| `r2`     | [Account info] array pointer |
| `r3`     | [Account info] array length  |
| `r4`     | [Signer seed] array pointer  |
| `r5`     | [Signer seed] array length   |

1. The [instruction] contains the following layout:

    | Offset (bytes) | Length (bytes) | Description                          |
    | -------------- | -------------- | ------------------------------------ |
    | 0              | 8              | Program ID ([System Program] pubkey) |
    | 8              | 8              | [Account metadata] array pointer     |
    | 16             | 8              | [Account metadata] array length      |
    | 24             | 8              | [Transfer instruction data] pointer  |
    | 32             | 8              | [Transfer instruction data] length   |

   1. The [transfer instruction data] is [encoded via `bincode`], which uses
    [`u32` enum variants] such that the transfer instruction data has the following
    layout:

        | Offset (bytes) | Length (bytes) | Description                               |
        | -------------- | -------------- | ----------------------------------------- |
        | 0              | 4              | Transfer instruction [enum variant] (`2`) |
        | 8              | 8              | Amount of Lamports to send                |

In this example, no signer seeds are required due to the lack of a [PDA signer].

## :white_check_mark: All tests

::: details `tests.rs`

<<< ../../../examples/transfer/src/tests.rs

:::

> [!note]
> The assembly file and testing framework in this example were adapted from an
> [`sbpf` example].

[account data is its name]: https://github.com/anza-xyz/agave/blob/v3.1.5/runtime/src/bank.rs#L5754
[account info]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/cpi.rs#L90-L103
[account metadata]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/cpi.rs#L81-L88
[account pubkey]: https://github.com/anza-xyz/agave/blob/v3.1.5/transaction-context/src/transaction_accounts.rs#L26
[account structure]: https://solana.com/docs/core/accounts#account-structure
[builtin]: https://github.com/anza-xyz/agave/blob/v3.1.5/builtins/src/lib.rs#L62-L68
[cpi]: https://solana.com/docs/core/cpi
[encoded via `bincode`]: https://github.com/anza-xyz/solana-sdk/blob/sdk@v3.0.0/system-interface/src/instruction.rs#L822
[enum variant]: https://github.com/anza-xyz/solana-sdk/blob/sdk@v3.0.0/system-interface/src/instruction.rs#L82
[instruction]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/cpi.rs#L70-L79
[is the sum of]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/serialization.rs#L509-L511
[lamports]: https://solana.com/docs/references/terminology#lamport
[pda signer]: https://solana.com/docs/core/cpi#cpis-with-pda-signers
[serialized with the following offsets]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/serialization.rs#L530-L559
[signer]: https://github.com/anza-xyz/agave/blob/v3.1.5/transaction-context/src/lib.rs#L78-L79
[signer seed]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/cpi.rs#L105-L111
[system program]: https://solana.com/docs/core/programs#the-system-program
[to an 8-byte boundary]: https://docs.rs/solana-program-entrypoint/3.1.1/solana_program_entrypoint/constant.BPF_ALIGN_OF_U128.html
[transfer instruction data]: https://docs.rs/solana-system-interface/latest/solana_system_interface/instruction/enum.SystemInstruction.html#variant.Transfer
[writable]: https://github.com/anza-xyz/agave/blob/v3.1.5/transaction-context/src/lib.rs#L80-L81
[`max_permitted_data_increase`]: https://docs.rs/solana-program-entrypoint/3.1.1/solana_program_entrypoint/constant.MAX_PERMITTED_DATA_INCREASE.html
[`non_dup_marker`]: https://docs.rs/solana-program-entrypoint/3.1.1/solana_program_entrypoint/constant.NON_DUP_MARKER.html
[`sbpf` example]: https://github.com/blueshift-gg/sbpf/blob/b7ac3d80da4400abff283fb0e68927c3c68a24d9/examples/sbpf-asm-cpi/src/sbpf-asm-cpi/sbpf-asm-cpi.s
[`sol_invoke_signed_c` syscall]: https://github.com/anza-xyz/solana-sdk/blob/sdk@v3.0.0/define-syscall/src/definitions.rs#L6
[`u32` enum variants]: https://sr.ht/~stygianentity/bincode/#why-does-bincode-not-respect-coderepru8code
