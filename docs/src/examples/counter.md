# Counter

<!--@include: ./disclaimer.md-->

## Overview

1. Branch:
   1. If 2 accounts, increment
   1. If 3 accounts, initialize
1. Init:
   1. Error if not 3 accounts
   1. [`sol_try_find_program_address`] returns address
   1. [`CreateAccount`]
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
1. Address compare
    1. [`sol_memcmp`] is [subject to metering] that charges the larger of a
       [10 CU base cost], and a [per-byte cost of 250 CUs], with
       [`r4` set to 0 if both regions are equal]

[`sol_try_find_program_address`] implements [the following returns]:

| Register | Success Value                          | Failure Value                     |
|----------|----------------------------------------|-----------------------------------|
| `r0`     | 0 | 1 |
| `r4` | Pointer to [PDA] | Unchanged |
| `r5` | Pointer to [bump seed][pda] | Unchanged |

[`sol_create_program_address`] implements [the following returns][create_pda_returns]:

| Register | Success Value                          | Failure Value                     |
|----------|----------------------------------------|-----------------------------------|
| `r0`     | 0 | 1 |
| `r4` | Pointer to [PDA] | Unchanged |

[pda]: https://solana.com/docs/core/pda
[create_pda_returns]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/lib.rs#L798-L834
[the following returns]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/lib.rs#L836-L886
[`r4` set to 0 if both regions are equal]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/mem_ops.rs#L162-L173
[per-byte cost of 250 CUs]: https://github.com/anza-xyz/agave/blob/v3.1.6/program-runtime/src/execution_budget.rs#L205
[10 CU base cost]: https://github.com/anza-xyz/agave/blob/v3.1.6/program-runtime/src/execution_budget.rs#L222
[subject to metering]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/mem_ops.rs#L3-L10
[`sol_memcmp`]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/mem_ops.rs#L67-L111
[not yet activated]: https://github.com/anza-xyz/agave/wiki/Feature-Gate-Tracker-Schedule
[soon-to-be-deprecated `rent::default`]: https://github.com/anza-xyz/solana-sdk/blob/rent@v3.1.0/rent/src/lib.rs#L108-L114
[uses]: https://github.com/anza-xyz/mollusk/blob/0.10.0/harness/src/sysvar.rs#L37
[`account_storage_overhead`]: https://docs.rs/solana-rent/3.1.0/solana_rent/constant.ACCOUNT_STORAGE_OVERHEAD.html
[`createaccount`]: https://github.com/anza-xyz/solana-sdk/blob/sdk@v3.0.0/system-interface/src/instruction.rs#L88-L97
[`default_lamports_per_byte`]: https://docs.rs/solana-rent/3.1.0/solana_rent/constant.DEFAULT_LAMPORTS_PER_BYTE.html
[`minimum_balance`]: https://docs.rs/solana-rent/3.1.0/solana_rent/struct.Rent.html#method.minimum_balance
[`simd-0194`]: https://github.com/solana-foundation/solana-improvement-documents/blob/main/proposals/0194-deprecate-rent-exemption-threshold.md
[`simd-0436`]: https://github.com/solana-foundation/solana-improvement-documents/blob/main/proposals/0436-reduce-rent-exempt-minimum-by-2x.md
[`simd-0437`]: https://github.com/solana-foundation/solana-improvement-documents/pull/437
[`sol_create_program_address`]: https://github.com/anza-xyz/agave/blob/v3.1.6/platform-tools-sdk/sbf/c/inc/sol/inc/pubkey.inc#L64-L72
[`sol_try_find_program_address`]: https://github.com/anza-xyz/agave/blob/v3.1.6/platform-tools-sdk/sbf/c/inc/sol/inc/pubkey.inc#L74-L83
