# Counter

<!-- @include: ./disclaimer.md -->

## Background

This example implements a simple on-chain counter program at a [PDA] account.
The program supports two operations: initializing a user's counter, and
incrementing a user's counter by a specified amount. The counter [PDA] account
stores the following data:

| Size (bytes) | Description             |
| ------------ | ----------------------- |
| 8            | Counter value (`u64`)   |
| 1            | [Bump seed][pda] (`u8`) |

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

Importantly, this methodology strictly enforces
[8-byte aligned stack offsets](transfer#transfer-cpi), as well as
[`i16` offset values] since, as of the time of this writing,
[`sbpf` silently truncates offsets that are not `i16`].

## Entrypoint branching

The number of accounts acts as a discriminator for the two operations:

| Operation  | Number of accounts | Instruction data         |
| ---------- | ------------------ | ------------------------ |
| Initialize | 3                  | None                     |
| Increment  | 2                  | Increment amount (`u64`) |

<!-- markdownlint-disable MD013 -->

| Account index | Description                        | Used for `initialize`? | Use for `increment`? |
| ------------- | ---------------------------------- | ---------------------- | -------------------- |
| 0             | User's account                     | Yes                    | Yes                  |
| 1             | Counter [PDA] account              | Yes                    | Yes                  |
| 2             | [System Program](transfer) account | Yes                    | No                   |

<!-- markdownlint-enable MD013 -->

Only the initialize operation requires the [System Program](transfer) account in
order to initialize the [PDA] account. Hence the entrypoint first checks the
number of accounts passed in and branches accordingly, erroring out if the
number of accounts is unexpected.

<<< ../../../examples/counter/artifacts/snippets/asm/entrypoint.txt{asm}

## Initialize operation

### Layout background

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

The initialize operation stack contains the same allocated regions as the
[transfer example](transfer#transfer-cpi) plus the following additional regions,
described below:

| Size (bytes) | Description                                                   |
| ------------ | ------------------------------------------------------------- |
| 16           | [`SolSignerSeed`] for user's [pubkey]                         |
| 16           | [`SolSignerSeed`] for bump seed                               |
| 16           | [`SolSignerSeeds`] for [CPI](transfer#transfer-cpi)           |
| 32           | [PDA] from [`sol_try_find_program_address`] (`r4`)            |
| 24           | [`Rent`] from [`sol_get_rent_sysvar`] (`r1`)                  |
| 4            | `i32` from [`sol_memcmp`] (`r4`)                              |
| 4            | Padding to maintain 8-byte alignment                          |
| 1            | [Bump seed][pda] from [`sol_try_find_program_address`] (`r5`) |

### Signer seeds

Unlike in the [transfer CPI](transfer#transfer-cpi), the [`CreateAccount`]
instruction [CPI](transfer#transfer-cpi) requires [signer seeds][pda-seeds]:

| Register | Description                                       |
| -------- | ------------------------------------------------- |
| `r4`     | Pointer to array of [`SolSignerSeeds`]            |
| `r5`     | Number of elements in array (`1` in this example) |

There is only one [PDA signer][pda-seeds], such that the single
[`SolSignerSeeds`] points to the following array of two [`SolSignerSeed`]
structures:

| Index | Description          |
| ----- | -------------------- |
| 0     | User's [pubkey]      |
| 1     | [PDA bump seed][pda] |

Hence after checking the input memory map, the [`SolSignerSeed`] structures are
populated on the [stack](transfer#transfer-cpi):

<<< ../../../examples/counter/artifacts/snippets/asm/init-seeds.txt{asm}

### PDA checks {pda-checks}

The [PDA] and [bump seed][pda] are then computed by
[`sol_try_find_program_address`], whose [implementation] similarly relies on a
[`SolSignerSeed`] array, in this case containing a single [`SolSignerSeed`] for
the user's [pubkey]:

| Register | Description                                                    |
| -------- | -------------------------------------------------------------- |
| `r0`     | Return code: set to `0` on success, `1` on fail                |
| `r1`     | Pointer to array of [`SolSignerSeed`]                          |
| `r2`     | Number of elements in [`SolSignerSeed`] array (1 in this case) |
| `r3`     | [PDA] owning program ID (counter program ID)                   |
| `r4`     | Pointer to fill with [PDA] ([unchanged] on error)              |
| `r5`     | Pointer to fill with [bump seed][pda] ([unchanged] on error)   |

Notably, the [bump seed][pda] is temporarily stored on the
[stack](transfer#transfer-cpi) instead of directly in the passed [PDA] account
data, since the [`CreateAccount`] instruction [CPI processor exit routine] later
[overwrites data] on account creation.

<<< ../../../examples/counter/artifacts/snippets/asm/init-find-pda.txt{asm}

The computed [PDA] is then compared against the passed [PDA] account's
[pubkey] using [`sol_memcmp`], which is [subject to metering] that charges the
larger of a [10 CU base cost], and a [per-byte cost of 250 CUs]. The
[inner compare function] compare result is `0i32` only if the two regions are
equal:

| Register | Description                                 |
| -------- | ------------------------------------------- |
| `r0`     | Always returns 0                            |
| `r1`     | Pointer to first region                     |
| `r2`     | Pointer to second region                    |
| `r3`     | Number of bytes to compare                  |
| `r4`     | Pointer to fill with compare result (`i32`) |

<<< ../../../examples/counter/artifacts/snippets/asm/init-pda-compare.txt{asm}

### Minimum balance

The testing framework in this example [uses] the
[soon-to-be-deprecated `Rent::default`] implementation, so the assembly program
relies on [`sol_get_rent_sysvar`] which has a [return value] of [`Rent`],
written to the pointer passed [in `r1`][`sol_get_rent_sysvar`]. The resulting
[`minimum_balance`] is then computed as product of:

1. [`Rent.lamports_per_byte_year`][`rent`] ([`DEFAULT_LAMPORTS_PER_BYTE_YEAR`])
1. [PDA] account data length (`9`) plus [`ACCOUNT_STORAGE_OVERHEAD`]

> [!note]
> As of the time of this writing, [rent] is under active development:
> [`SIMD-0194`], which has [not yet activated], is superseded by [`SIMD-0436`],
> which is in turn superseded by [`SIMD-0437`].

This resulting minimum balance is directly stored in the [`CreateAccount`]
instruction data buffer on the [stack](transfer#transfer-cpi):

<<< ../../../examples/counter/artifacts/snippets/asm/min-balance.txt{asm}

### CPI construction

As in the [transfer CPI](transfer#transfer-cpi), the [`CreateAccount`]
instruction and associated account information regions are populated, this time
with an additional optimization: the [deprecated `rent_epoch` field][rent] is
ignored, since the [internal CPI `CallerAccount` structure][`calleraccount`]
does not include it, hence it is unprocessed by [`update_callee_account`].

Notably, the [`CreateAccount`] instruction data owner program ID field is
populated via [`sol_memcpy`], which has the same
[CU cost as `sol_memcmp`](#pda-checks) but no compare value return:

::: details Optimized instruction and account region setup

<<< ../../../examples/counter/artifacts/snippets/asm/cpi-setup.txt{asm}

:::

Unlike in the [transfer CPI](transfer#transfer-cpi), this example additionally
populates a [`SolSignerSeeds`] region on the [stack](transfer#transfer-cpi)
since there is a [PDA signer][pda-seeds]:

<<< ../../../examples/counter/artifacts/snippets/asm/seeded-cpi.txt{asm}

### Bump seed storage

Finally, the [bump seed][pda] computed earlier by
[`sol_try_find_program_address`] is stored in the last byte of the [PDA] account
data:

<<< ../../../examples/counter/artifacts/snippets/asm/store-seed.txt{asm}

## Increment operation

The increment operation starts by checking the user's account data length,
padding as needed to
[ensure 8-byte alignment](transfer#account-layout-background). Notably, since
[`i32` immediates] are
[cast to `i64` by the interpreter][`i32` interpretation], then
[cast to `u64` by `AND64_IMM`][`and64_imm`], the VM's use of Rust
[sign extension] enables the following concise padding calculation, guaranteed
not to overflow given that [`MAX_PERMITTED_DATA_LENGTH`] is much less than
$2 ^ {64} - 1$:

<<< ../../../examples/counter/artifacts/snippets/asm/user-data-len.txt{asm}

This algorithm is verified with a simple test:

<<< ../../../examples/counter/artifacts/tests/pad_masking/test.txt{rs}

[`create_program_address`] limits seeds to [`MAX_SEED_LEN`] each. So there is
one [signer seeds] array pointing an array of two [signer seed] structures,
one containing the user's [pubkey] and one containing the bump seed.

[`sol_create_program_address`] implements
[the following returns][create_pda_returns]:

| Register | Success                          | Failure     |
| -------- | -------------------------------- | ----------- |
| `r0`     | 0                                | 1           |
| `r4`     | Passed pointer filled with [PDA] | [Unchanged] |

[10 cu base cost]: https://github.com/anza-xyz/agave/blob/v3.1.6/program-runtime/src/execution_budget.rs#L222
[cpi processor exit routine]: https://github.com/anza-xyz/agave/blob/v3.1.6/program-runtime/src/cpi.rs#L907-L921
[create_pda_returns]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/lib.rs#L798-L834
[implementation]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/lib.rs#L836-L886
[inner compare function]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/mem_ops.rs#L162-L173
[internally disallows account data]: https://github.com/anza-xyz/agave/blob/v3.1.6/programs/system/src/system_processor.rs#L189-L192
[not yet activated]: https://github.com/anza-xyz/agave/wiki/Feature-Gate-Tracker-Schedule
[overwrites data]: https://github.com/anza-xyz/agave/blob/v3.1.6/program-runtime/src/cpi.rs#L1248
[pda]: https://solana.com/docs/core/pda
[pda-seeds]: https://solana.com/docs/core/cpi#cpis-with-pda-signers
[per-byte cost of 250 cus]: https://github.com/anza-xyz/agave/blob/v3.1.6/program-runtime/src/execution_budget.rs#L205
[program id serialization]: https://github.com/anza-xyz/agave/blob/v3.1.6/program-runtime/src/serialization.rs#L569
[pubkey]: https://solana.com/docs/core/accounts#public-key
[rent]: https://solana.com/docs/core/accounts#account-structure
[return value]: https://github.com/anza-xyz/agave/blob/v3.1.6/program-runtime/src/sysvar_cache.rs#L156-L158
[sign extension]: https://en.wikipedia.org/wiki/Sign_extension
[signer seed]: https://github.com/anza-xyz/agave/blob/v3.1.6/platform-tools-sdk/sbf/c/inc/sol/pubkey.h#L56-L62
[signer seeds]: https://github.com/anza-xyz/agave/blob/v3.1.6/platform-tools-sdk/sbf/c/inc/sol/pubkey.h#L64-L71
[soon-to-be-deprecated `rent::default`]: https://github.com/anza-xyz/solana-sdk/blob/rent@v3.1.0/rent/src/lib.rs#L108-L114
[subject to metering]: https://github.com/anza-xyz/agave/blob/v3.1.6/syscalls/src/mem_ops.rs#L3-L10
[unchanged]: https://github.com/anza-xyz/sbpf/blob/v0.14.0/src/interpreter.rs#L606-L612
[uses]: https://github.com/anza-xyz/mollusk/blob/0.10.0/harness/src/sysvar.rs#L37
[`account_storage_overhead`]: https://docs.rs/solana-rent/3.1.0/solana_rent/constant.ACCOUNT_STORAGE_OVERHEAD.html
[`and64_imm`]: https://github.com/anza-xyz/sbpf/blob/v0.14.0/src/interpreter.rs#L371
[`calleraccount`]: https://docs.rs/solana-program-runtime/3.1.7/solana_program_runtime/cpi/struct.CallerAccount.html
[`createaccount`]: https://github.com/anza-xyz/solana-sdk/blob/sdk@v3.0.0/system-interface/src/instruction.rs#L88-L97
[`create_account`]: https://github.com/anza-xyz/agave/blob/v3.1.6/programs/system/src/system_processor.rs#L146-L179
[`create_program_address`]: https://docs.rs/solana-address/2.0.0/solana_address/struct.Address.html#method.create_program_address
[`default_lamports_per_byte_year`]: https://docs.rs/solana-rent/3.0.0/solana_rent/constant.DEFAULT_LAMPORTS_PER_BYTE_YEAR.html
[`i16` offset values]: https://github.com/anza-xyz/sbpf/blob/v0.14.1/doc/bytecode.md?plain=1#L45
[`i32` immediates]: https://github.com/anza-xyz/sbpf/blob/v0.14.0/doc/bytecode.md#instruction-layout
[`i32` interpretation]: https://github.com/anza-xyz/sbpf/blob/v0.14.0/src/ebpf.rs#L682
[`max_permitted_data_length`]: https://docs.rs/solana-system-interface/3.0.0/solana_system_interface/constant.MAX_PERMITTED_DATA_LENGTH.html
[`max_seed_len`]: https://docs.rs/solana-address/2.0.0/solana_address/constant.MAX_SEED_LEN.html
[`minimum_balance`]: https://docs.rs/solana-rent/3.1.0/solana_rent/struct.Rent.html#method.minimum_balance
[`rent`]: https://docs.rs/solana-rent/3.1.0/solana_rent/struct.Rent.html
[`sbpf` silently truncates offsets that are not `i16`]: https://github.com/blueshift-gg/sbpf/issues/97
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
[`update_callee_account`]: https://github.com/anza-xyz/agave/blob/v3.1.7/program-runtime/src/cpi.rs#L1145-L1215
