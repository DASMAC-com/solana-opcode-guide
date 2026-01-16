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

Accounts in the [input buffer](memo) are [serialized] and [deserialized] with
the following offsets relative to the start of the account, assuming
non-duplicate accounts without any account data:

<!-- markdownlint-disable MD013 -->

| Offset (bytes) | Length (bytes) | Description                              |
| -------------- | -------------- | ---------------------------------------- |
| 0              | 1              | [`NON_DUP_MARKER`]                       |
| 1              | 1              | Is [signer]?                             |
| 2              | 1              | Is [writable]?                           |
| 3              | 1              | Is [executable][account structure]?      |
| 4              | 4              | [Original account data length]           |
| 8              | 32             | [Account pubkey]                         |
| 40             | 32             | [Account owner][account structure]       |
| 72             | 8              | [Lamports balance][account structure]    |
| 80             | 8              | [Account data][account structure] length |
| 88             | 0              | [Account data][account structure] (none) |
| 88             | 10240          | Account data padding                     |
| 10328          | 8              | Account [rent epoch][account structure]  |

<!-- markdownlint-enable MD013 -->

The account data padding length [is the sum of]:

1. [`MAX_PERMITTED_DATA_INCREASE`].
1. Additional padding to align the account data length [to an 8-byte boundary].

Note however that the [System Program] is a [builtin], which means that its
[account data is its name], specifically `b"system_program"` (14 bytes). This
means that the System Program has the following:

| Offset (bytes) | Length (bytes) | Description                             |
| -------------- | -------------- | --------------------------------------- |
| 88             | 14             | [Account data][account structure]       |
| 102            | 10242          | Account data padding                    |
| 10344          | 8              | Account [rent epoch][account structure] |

## :shield: Input validation

Input offsets are validated in Rust using struct operations:

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/transfer/artifacts/snippets/asm/input-offsets.txt{1-22 asm} [Assembly]

<!-- markdownlint-enable MD013 -->

::: details `test_input_offsets`

<<< ../../../examples/transfer/artifacts/tests/input_offsets/test.txt{rs}

:::

Due to the account layout order, account layout validation takes place in
a specific sequence, before the final Lamport balance check:

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/transfer/artifacts/snippets/asm/input-validation.txt{4-30 asm}

<!-- markdownlint-enable MD013 -->

## :outbox_tray: Transfer CPI layout {#transfer-cpi}

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

The [instruction] layout is as follows:

> | Offset (bytes) | Length (bytes) | Description                            |
> | -------------- | -------------- | -------------------------------------- |
> | 0              | 8              | Program ID ([System Program] pointer ) |
> | 8              | 8              | [Account metadata] array pointer       |
> | 16             | 8              | [Account metadata] array length        |
> | 24             | 8              | [Transfer instruction data] pointer    |
> | 32             | 8              | [Transfer instruction data] length     |
>
> Each element in the [account metadata] array has the following layout:
>
> > | Offset (bytes) | Length (bytes) | Description              |
> > | -------------- | -------------- | ------------------------ |
> > | 0              | 8              | [Account pubkey] pointer |
> > | 8              | 1              | Is [writable]?           |
> > | 9              | 1              | Is [signer]?             |
> > | 10             | 6              | [C-style array padding]  |
>
> The [transfer instruction data] is [encoded via `bincode`], which uses
> [`u32` enum variants] such that the transfer instruction data layout is as
> follows:
>
> <!-- markdownlint-disable MD013 -->
>
> > | Offset (bytes) | Length (bytes) | Description                               |
> > | -------------- | -------------- | ----------------------------------------- |
> > | 0              | 4              | Transfer instruction [enum variant] (`2`) |
> > | 8              | 8              | Amount of Lamports to send                |

<!-- markdownlint-enable MD013 -->

Each [account info] element has the following layout:

<!-- markdownlint-disable MD013 -->

> | Offset (bytes) | Length (bytes) | Description                                   |
> | -------------- | -------------- | --------------------------------------------- |
> | 0              | 8              | [Account pubkey] pointer                      |
> | 8              | 8              | [Lamports balance][account structure] pointer |
> | 16             | 8              | [Account data][account structure] length      |
> | 24             | 8              | [Account data][account structure] pointer     |
> | 32             | 8              | [Account owner][account structure] pointer    |
> | 40             | 8              | [Account rent epoch][account structure]       |
> | 48             | 1              | Is [signer]?                                  |
> | 49             | 1              | Is [writable]?                                |
> | 50             | 1              | Is [executable][account structure]?           |
> | 51             | 5              | [C-style array padding]                       |

<!-- markdownlint-enable MD013 -->

In this example, no signer seeds are required due to the lack of a [PDA signer].

Since the data required by the CPI is too wide to fit in one of the
[64-bit general purpose registers][isa], it must be allocated within a
[stack frame], which is [4096 bytes] wide and [pointed to by `r10`][isa].
Moreover, since [CPI processor checks] rely on [inner alignment checks], any
data allocated on the stack must be aligned to at least an 8-byte boundary since
the largest primitive data type used across the [instruction],
[account metadata], and [account info] data structures is a `u64` pointer:

| `r10` Offset | Length | Description                                        |
| ------------ | ------ | -------------------------------------------------- |
| 200          | 40     | [Instruction]                                      |
| 160          | 16     | Encoded [transfer instruction data] (with padding) |
| 144          | 32     | [Account metadata] array (2 accounts)              |
| 112          | 112    | [Account info] array (2 accounts)                  |

CPI offsets are validated in Rust using struct operations:

::: details `test_cpi_offsets`

<<< ../../../examples/transfer/artifacts/tests/cpi_offsets/test.txt{rs}

:::

## :wrench: Optimized CPI construction

CPI data regions are first allocated on the stack using the calculated offsets:

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/transfer/artifacts/snippets/asm/stack-allocations.txt{3-11 asm}

<!-- markdownlint-enable MD013 -->

Instruction data is then populated, leveraging
[zero-initialized stack memory] to encode the [System Program] pubkey rather
than load it from the passed account:

> [!tip]
> The [System Program] pubkey is `111111...` in [base58], which is all zeros in
> binary.

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/transfer/artifacts/snippets/asm/instruction-allocation.txt{3-22 asm}

<!-- markdownlint-enable MD013 -->

[Account information](#account-layout-background) is then copied into the
[account metadata] and [account info] arrays, with optimizations that leverage
the zero-initialized stack memory and known offsets:

::: details Account transcription

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/transfer/artifacts/snippets/asm/account-population.txt{3-110 asm}

<!-- markdownlint-enable MD013 -->

:::

Finally, the CPI is invoked, leveraging the
[zero-initialized `r5` memory](memo#error-checking) for another optimization
since no [signer seeds][pda signer] are required:

<<< ../../../examples/transfer/artifacts/snippets/asm/invoke-cpi.txt{3-13 asm}

::: details Full program

<<< ../../../examples/transfer/src/transfer/transfer.s{asm}

:::

## :white_check_mark: All tests

::: details `tests.rs`

<<< ../../../examples/transfer/src/tests.rs

:::

> [!note]
> The assembly file and testing framework in this example were adapted from an
> [`sbpf` example].

[4096 bytes]: https://docs.rs/solana-program-runtime/3.1.6/solana_program_runtime/execution_budget/constant.STACK_FRAME_SIZE.html
[account data is its name]: https://github.com/anza-xyz/agave/blob/v3.1.5/runtime/src/bank.rs#L5754
[account info]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/cpi.rs#L90-L103
[account metadata]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/cpi.rs#L81-L88
[account pubkey]: https://github.com/anza-xyz/agave/blob/v3.1.5/transaction-context/src/transaction_accounts.rs#L26
[account structure]: https://solana.com/docs/core/accounts#account-structure
[base58]: https://solana.com/docs/core/accounts#account-address
[builtin]: https://github.com/anza-xyz/agave/blob/v3.1.5/builtins/src/lib.rs#L62-L68
[c-style array padding]: https://doc.rust-lang.org/reference/type-layout.html#reprc-unions
[cpi]: https://solana.com/docs/core/cpi
[cpi processor checks]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/cpi.rs#L829-L854
[deserialized]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/serialization.rs#L597-L659
[encoded via `bincode`]: https://github.com/anza-xyz/solana-sdk/blob/sdk@v3.0.0/system-interface/src/instruction.rs#L822
[enum variant]: https://github.com/anza-xyz/solana-sdk/blob/sdk@v3.0.0/system-interface/src/instruction.rs#L82
[inner alignment checks]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/memory.rs#L39-L56
[instruction]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/cpi.rs#L70-L79
[is the sum of]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/serialization.rs#L509-L511
[isa]: https://github.com/anza-xyz/sbpf/blob/v0.13.1/doc/bytecode.md#registers
[lamports]: https://solana.com/docs/references/terminology#lamport
[original account data length]: https://github.com/anza-xyz/agave/blob/v3.1.6/program-runtime/src/cpi.rs#L231-L235
[pda signer]: https://solana.com/docs/core/cpi#cpis-with-pda-signers
[serialized]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/serialization.rs#L530-L559
[signer]: https://github.com/anza-xyz/agave/blob/v3.1.5/transaction-context/src/lib.rs#L78-L79
[signer seed]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/cpi.rs#L105-L111
[stack frame]: https://en.wikipedia.org/wiki/Call_stack#Stack_and_frame_pointers
[system program]: https://solana.com/docs/core/programs#the-system-program
[to an 8-byte boundary]: https://docs.rs/solana-program-entrypoint/3.1.1/solana_program_entrypoint/constant.BPF_ALIGN_OF_U128.html
[transfer instruction data]: https://docs.rs/solana-system-interface/latest/solana_system_interface/instruction/enum.SystemInstruction.html#variant.Transfer
[writable]: https://github.com/anza-xyz/agave/blob/v3.1.5/transaction-context/src/lib.rs#L80-L81
[zero-initialized stack memory]: https://github.com/anza-xyz/agave/blob/v3.1.5/program-runtime/src/mem_pool.rs#L68-L70
[`max_permitted_data_increase`]: https://docs.rs/solana-program-entrypoint/3.1.1/solana_program_entrypoint/constant.MAX_PERMITTED_DATA_INCREASE.html
[`non_dup_marker`]: https://docs.rs/solana-program-entrypoint/3.1.1/solana_program_entrypoint/constant.NON_DUP_MARKER.html
[`sbpf` example]: https://github.com/blueshift-gg/sbpf/blob/b7ac3d80da4400abff283fb0e68927c3c68a24d9/examples/sbpf-asm-cpi/src/sbpf-asm-cpi/sbpf-asm-cpi.s
[`sol_invoke_signed_c` syscall]: https://github.com/anza-xyz/solana-sdk/blob/sdk@v3.0.0/define-syscall/src/definitions.rs#L6
[`u32` enum variants]: https://sr.ht/~stygianentity/bincode/#why-does-bincode-not-respect-coderepru8code
