# Counter

<!-- @include: ./disclaimer.md -->

## Background

This example implements a simple on-chain counter program at a [PDA] account.
The program supports two operations: initializing a user's counter, and
incrementing a user's counter by a specified amount.

All constants for this example are derived programmatically in Rust, then
automatically inserted at the top of the assembly program file during a test:

::: details Constants

::: code-group

<<< ../../../examples/counter/src/constants.rs [Rust derivations]

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/counter/artifacts/tests/asm_file_constants/test.txt{rs} [ASM file insertion]

<!-- markdownlint-enable MD013 -->

<<< ../../../examples/counter/src/counter/counter.s{asm} [ASM program file]

:::

Importantly, this methodology strictly enforces [`i16` offset values] since, as
of the time of this writing,
[`sbpf` silently truncates offsets that are not `i16`].

## Entrypoint branching

The number of accounts acts as a discriminator for the two operations:

| Operation  | Number of accounts | Instruction data         |
| ---------- | ------------------ | ------------------------ |
| Initialize | 3                  | None                     |
| Increment  | 2                  | Increment amount (`u64`) |

| Account index | Description | Used for `initialize`? | Use for `increment`? |
| ------------- | ----------- | --------------------------------- | --------------------- |
| 0             | User's account          | Yes                   | Yes                   |
| 1             | Counter [PDA] account   | Yes                   | Yes                   |
| 2             | [System Program](transfer) account | Yes        | No                    |

Only the initialize operation requires the [System Program](transfer) account in
order to initialize the [PDA] account.  Hence the entrypoint first checks the
number of accounts passed in and branches accordingly, erroring out if the
number of accounts is unexpected.

<<< ../../../examples/counter/artifacts/snippets/asm/entrypoint.txt{asm}

## Initialize operation

Like in the [transfer example](transfer), the initialize operation uses a
[System Program CPI](transfer#transfer-cpi) but with [`CreateAccount`]
instruction data:

| Size (bytes) | Description                                            |
| ------------ | ------------------------------------------------------ |
| 4            | [Enum variant](transfer#transfer-cpi) (`0`)            |
| 8            | Lamports to transfer to new account                    |
| 8            | Bytes to allocate for new account                      |
| 32           | Owner program ID for new account (the counter program) |

Notably, [`create_account`] calls [`transfer`], which
[internally disallows account data] such that the entire [memory map](memo) is
statically sized for the initialize operation, including the
[Program ID serialization] at end of the [input buffer](memo). Hence the initial
memory map checks at the start of the initialize operation:

<<< ../../../examples/counter/artifacts/snippets/asm/init-map-checks.txt{asm}

Unlike in the [transfer CPI](transfer#transfer-cpi), the [`CreateAccount`]
instruction [CPI](transfer#transfer-cpi) requires [signer seeds][pda-seeds]:

| Register | Description |
| -------- | ------------------------------------------------------ |
| `r4`     | Pointer to array of [`SolSignerSeeds`]                 |
| `r5`     | Number of elements in array (`1` in this example)      |

There is only one [PDA signer][pda-seeds], such that the single
[`SolSignerSeeds`] points to the following array of two [`SolSignerSeed`]
structures:

| Index | Description                          |
| ----- | ------------------------------------ |
| 0     | User's [pubkey]                        |
| 1     | [PDA bump seed][pda]                           |

Hence after checking the input memory map, the [`SolSignerSeed`] structures are
populated on the [stack](transfer#transfer-cpi), which includes the same
allocated regions as the [transfer example](transfer#transfer-cpi) plus the
following additional regions:

| Size (bytes) | Description                                                  |
| ------------ | ------------------------------------------------------------ |
| 16           | [`SolSignerSeed`] for user's [pubkey]                          |
| 16           | [`SolSignerSeed`] for bump seed                              |
| 16           | [`SolSignerSeeds`] for [CPI](#transfer)                      |
| 32           | [PDA] from [`sol_try_find_program_address`] (`r4`)           |

<<< ../../../examples/counter/artifacts/snippets/asm/init-seeds.txt{asm}

The [PDA] and [bump seed][pda] are computed by [`sol_try_find_program_address`],
whose [implementation] similarly relies on a [`SolSignerSeed`] array, in this
case containing a single [`SolSignerSeed`] for the user's [pubkey]:

| Register | Description                                                      |
| -------- | ---------------------------------------------------------------- |
| `r0`     | Return code: set to `0` on success, `1` on fail                  |
| `r1`     | Pointer to array of [`SolSignerSeed`]  |
| `r2`     | Number of elements in [`SolSignerSeed`] array (1 in this case)                |
| `r3`     | [PDA] owning program ID (counter program ID)                     |
| `r4`     | Pointer filled with [PDA] ([unchanged] on error)                 |
| `r5`     | Pointer filled with [bump seed][pda] ([unchanged] on error)      |

## Increment operation

## Links

1. Init:
   1. [`SIMD-0194`] took out 2x multiplier, then [`SIMD-0436`] made it lower
      value, but then superseded by [`SIMD-0437`] which hasn't landed
   1. So use [`DEFAULT_LAMPORTS_PER_BYTE`] and [`ACCOUNT_STORAGE_OVERHEAD`],
      yielding [`minimum_balance`]
   1. [Not yet activated] as of the time of this writing
   1. Testing framework [uses] the [soon-to-be-deprecated `Rent::default`]
1. Increment:
   1. [`sol_create_program_address`]
   1. Error if not there
   1. Error if more than two accounts
1. Address compare/copy
   1. [`sol_memcmp`] is [subject to metering] that charges the larger of a
      [10 CU base cost], and a [per-byte cost of 250 CUs], with
      [`r4` set to 0 if both regions are equal]
   1. [`sol_memcpy`] is same but no return value.

[`create_program_address`] limits seeds to [`MAX_SEED_LEN`] each. So there is
one [signer seeds] array pointing an array of two [signer seed] structures,
one containing the user's [pubkey] and one containing the bump seed.

[`sol_create_program_address`] implements
[the following returns][create_pda_returns]:

| Register | Success                          | Failure     |
| -------- | -------------------------------- | ----------- |
| `r0`     | 0                                | 1           |
| `r4`     | Passed pointer filled with [PDA] | [Unchanged] |

[`sol_get_rent_sysvar`] has a [return value] of pointer-to-[`Rent`] struct
[in `r1`][`sol_get_rent_sysvar`].

[10 cu base cost]: https://github.com/anza-xyz/agave/blob/v3.1.6/program-runtime/src/execution_budget.rs#L222
[implementation]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/lib.rs#L836-L886
[create_pda_returns]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/lib.rs#L798-L834
[internally disallows account data]: https://github.com/anza-xyz/agave/blob/v3.1.6/programs/system/src/system_processor.rs#L189-L192
[not yet activated]: https://github.com/anza-xyz/agave/wiki/Feature-Gate-Tracker-Schedule
[pda]: https://solana.com/docs/core/pda
[per-byte cost of 250 cus]: https://github.com/anza-xyz/agave/blob/v3.1.6/program-runtime/src/execution_budget.rs#L205
[program id serialization]: https://github.com/anza-xyz/agave/blob/v3.1.6/program-runtime/src/serialization.rs#L569
[return value]: https://github.com/anza-xyz/agave/blob/v3.1.6/program-runtime/src/sysvar_cache.rs#L156-L158
[signer seed]: https://github.com/anza-xyz/agave/blob/v3.1.6/platform-tools-sdk/sbf/c/inc/sol/pubkey.h#L56-L62
[signer seeds]: https://github.com/anza-xyz/agave/blob/v3.1.6/platform-tools-sdk/sbf/c/inc/sol/pubkey.h#L64-L71
[soon-to-be-deprecated `rent::default`]: https://github.com/anza-xyz/solana-sdk/blob/rent@v3.1.0/rent/src/lib.rs#L108-L114
[subject to metering]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/mem_ops.rs#L3-L10
[unchanged]: https://github.com/anza-xyz/sbpf/blob/v0.14.0/src/interpreter.rs#L606-L612
[uses]: https://github.com/anza-xyz/mollusk/blob/0.10.0/harness/src/sysvar.rs#L37
[`account_storage_overhead`]: https://docs.rs/solana-rent/3.1.0/solana_rent/constant.ACCOUNT_STORAGE_OVERHEAD.html
[`createaccount`]: https://github.com/anza-xyz/solana-sdk/blob/sdk@v3.0.0/system-interface/src/instruction.rs#L88-L97
[`create_account`]: https://github.com/anza-xyz/agave/blob/v3.1.6/programs/system/src/system_processor.rs#L146-L179
[`create_program_address`]: https://docs.rs/solana-address/2.0.0/solana_address/struct.Address.html#method.create_program_address
[`default_lamports_per_byte`]: https://docs.rs/solana-rent/3.1.0/solana_rent/constant.DEFAULT_LAMPORTS_PER_BYTE.html
[`max_seed_len`]: https://docs.rs/solana-address/2.0.0/solana_address/constant.MAX_SEED_LEN.html
[`minimum_balance`]: https://docs.rs/solana-rent/3.1.0/solana_rent/struct.Rent.html#method.minimum_balance
[`r4` set to 0 if both regions are equal]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/mem_ops.rs#L162-L173
[`rent`]: https://docs.rs/solana-rent/3.1.0/solana_rent/struct.Rent.html
[`simd-0194`]: https://github.com/solana-foundation/solana-improvement-documents/blob/main/proposals/0194-deprecate-rent-exemption-threshold.md
[`simd-0436`]: https://github.com/solana-foundation/solana-improvement-documents/blob/main/proposals/0436-reduce-rent-exempt-minimum-by-2x.md
[`simd-0437`]: https://github.com/solana-foundation/solana-improvement-documents/pull/437
[`solsignerseeds`]: https://github.com/anza-xyz/agave/blob/v3.1.6/platform-tools-sdk/sbf/c/inc/sol/inc/pubkey.inc#L55-L62
[`solsignerseed`]: https://github.com/anza-xyz/agave/blob/v3.1.6/platform-tools-sdk/sbf/c/inc/sol/inc/pubkey.inc#L47-L53
[`sol_create_program_address`]: https://github.com/anza-xyz/agave/blob/v3.1.6/platform-tools-sdk/sbf/c/inc/sol/inc/pubkey.inc#L64-L72
[`sol_get_rent_sysvar`]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/sysvar.rs#L135-L155
[`sol_memcmp`]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/mem_ops.rs#L67-L111
[`sol_memcpy`]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/mem_ops.rs#L26-L47
[`sol_try_find_program_address`]: https://github.com/anza-xyz/agave/blob/v3.1.6/platform-tools-sdk/sbf/c/inc/sol/inc/pubkey.inc#L74-L83
[`transfer`]: https://github.com/anza-xyz/agave/blob/v3.1.6/programs/system/src/system_processor.rs#L210-L233
[`i16` offset values]: https://github.com/anza-xyz/sbpf/blob/v0.14.1/doc/bytecode.md?plain=1#L45
[`sbpf` silently truncates offsets that are not `i16`]: https://github.com/blueshift-gg/sbpf/issues/97
[pda-seeds]: https://solana.com/docs/core/cpi#cpis-with-pda-signers
[pubkey]: https://solana.com/docs/core/accounts#public-key