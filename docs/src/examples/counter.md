# Counter

<!-- @include: ./disclaimer.md -->

## Background

This example implements a simple on-chain counter program at a [PDA] account.
The program supports two operations: initializing a user's counter, and
incrementing a user's counter by a specified amount.

## Entrypoint branching

The number of accounts acts as a discriminator for the two operations:

| Operation  | Number of accounts | Instruction data         |
| ---------- | ------------------ | ------------------------ |
| Initialize | 3                  | None                     |
| Increment  | 2                  | Increment amount (`u64`) |

Both operations require the user's account followed by the counter [PDA]
account, but only the initialize operation also requires the
[System Program](transfer) account in order to initialize the [PDA] account.
Hence the entrypoint first checks the number of accounts passed in and branches
accordingly, erroring out if the number of accounts is unexpected.

<<< ../../../examples/counter/artifacts/snippets/asm/entrypoint.txt{asm}

## Init operation

## Increment operation

## Links

1. Init:
   1. [`sol_try_find_program_address`] returns address
   1. [`CreateAccount`] is like from the transfer example, but uses different
      args.
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
   1. [Program ID serialization] at end of [input buffer](memo)

[`create_program_address`] limits seeds to [`MAX_SEED_LEN`] each. So there is
one [signer seeds] array pointing an array of two [signer seed] structures,
one containing the owner's pubkey and one containing the bump seed.

[`sol_try_find_program_address`] implements [the following returns]:

| Register | Success                     | Failure     |
| -------- | --------------------------- | ----------- |
| `r0`     | 0                           | 1           |
| `r4`     | Pointer to [PDA]            | [Unchanged] |
| `r5`     | Pointer to [bump seed][pda] | [Unchanged] |

[`sol_create_program_address`] implements
[the following returns][create_pda_returns]:

| Register | Success          | Failure     |
| -------- | ---------------- | ----------- |
| `r0`     | 0                | 1           |
| `r4`     | Pointer to [PDA] | [Unchanged] |

[`sol_get_rent_sysvar`] has a [return value] of pointer-to-[`Rent`] struct [in `r1`][`sol_get_rent_sysvar`].

[10 cu base cost]: https://github.com/anza-xyz/agave/blob/v3.1.6/program-runtime/src/execution_budget.rs#L222
[create_pda_returns]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/lib.rs#L798-L834
[not yet activated]: https://github.com/anza-xyz/agave/wiki/Feature-Gate-Tracker-Schedule
[pda]: https://solana.com/docs/core/pda
[per-byte cost of 250 cus]: https://github.com/anza-xyz/agave/blob/v3.1.6/program-runtime/src/execution_budget.rs#L205
[program id serialization]: https://github.com/anza-xyz/agave/blob/v3.1.6/program-runtime/src/serialization.rs#L569
[return value]: https://github.com/anza-xyz/agave/blob/v3.1.6/program-runtime/src/sysvar_cache.rs#L156-L158
[signer seed]: https://github.com/anza-xyz/agave/blob/v3.1.6/platform-tools-sdk/sbf/c/inc/sol/pubkey.h#L56-L62
[signer seeds]: https://github.com/anza-xyz/agave/blob/v3.1.6/platform-tools-sdk/sbf/c/inc/sol/pubkey.h#L64-L71
[soon-to-be-deprecated `rent::default`]: https://github.com/anza-xyz/solana-sdk/blob/rent@v3.1.0/rent/src/lib.rs#L108-L114
[subject to metering]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/mem_ops.rs#L3-L10
[the following returns]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/lib.rs#L836-L886
[unchanged]: https://github.com/anza-xyz/sbpf/blob/v0.14.0/src/interpreter.rs#L606-L612
[uses]: https://github.com/anza-xyz/mollusk/blob/0.10.0/harness/src/sysvar.rs#L37
[`account_storage_overhead`]: https://docs.rs/solana-rent/3.1.0/solana_rent/constant.ACCOUNT_STORAGE_OVERHEAD.html
[`createaccount`]: https://github.com/anza-xyz/solana-sdk/blob/sdk@v3.0.0/system-interface/src/instruction.rs#L88-L97
[`create_program_address`]: https://docs.rs/solana-address/2.0.0/solana_address/struct.Address.html#method.create_program_address
[`default_lamports_per_byte`]: https://docs.rs/solana-rent/3.1.0/solana_rent/constant.DEFAULT_LAMPORTS_PER_BYTE.html
[`max_seed_len`]: https://docs.rs/solana-address/2.0.0/solana_address/constant.MAX_SEED_LEN.html
[`minimum_balance`]: https://docs.rs/solana-rent/3.1.0/solana_rent/struct.Rent.html#method.minimum_balance
[`r4` set to 0 if both regions are equal]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/mem_ops.rs#L162-L173
[`rent`]: https://docs.rs/solana-rent/3.1.0/solana_rent/struct.Rent.html
[`simd-0194`]: https://github.com/solana-foundation/solana-improvement-documents/blob/main/proposals/0194-deprecate-rent-exemption-threshold.md
[`simd-0436`]: https://github.com/solana-foundation/solana-improvement-documents/blob/main/proposals/0436-reduce-rent-exempt-minimum-by-2x.md
[`simd-0437`]: https://github.com/solana-foundation/solana-improvement-documents/pull/437
[`sol_create_program_address`]: https://github.com/anza-xyz/agave/blob/v3.1.6/platform-tools-sdk/sbf/c/inc/sol/inc/pubkey.inc#L64-L72
[`sol_get_rent_sysvar`]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/sysvar.rs#L135-L155
[`sol_memcmp`]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/mem_ops.rs#L67-L111
[`sol_memcpy`]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/mem_ops.rs#L26-L47
[`sol_try_find_program_address`]: https://github.com/anza-xyz/agave/blob/v3.1.6/platform-tools-sdk/sbf/c/inc/sol/inc/pubkey.inc#L74-L83
