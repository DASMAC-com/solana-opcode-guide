# Counter

<!--@include: ./disclaimer.md-->

## Overview

1. Branch:
   1. If 2 accounts, increment
   1. If 3 accounts, initialize
1. Init:
   1. Error if not 3 accounts
   1. [`sol_try_find_program_address`]
   1. [`CreateAccount`]
   1. [`SIMD-0194`] took out 2x multiplier, then [`SIMD-0436`] made it lower
      value, but then superseded by [`SIMD-0437`] which hasn't landed
   1. So use [`DEFAULT_LAMPORTS_PER_BYTE`] and [`ACCOUNT_STORAGE_OVERHEAD`],
      yielding [`minimum_balance`]
   1. [Not yet activated] as of the time of this writing
   1. Testing framework [uses] the [soon-to-be-deprecated `Rent::default`]
1. Increment:
   1. [`sol_create_program_address`] using bump seed passed
   1. Error if not there
   1. Error if more than two accounts

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
