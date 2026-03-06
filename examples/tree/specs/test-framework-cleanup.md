<!-- cspell:word pubkeys -->

<!-- cspell:word stdlib -->

<!-- cspell:word specced -->

# Test framework cleanup specification

## Scope

Recommendations for reducing duplication, improving consistency,
and simplifying the test harness across `tests.rs`, `init.rs`,
`insert.rs`, and `entrypoint.rs`.

## 1. Unify error-checking helpers

### Problem

Three distinct error-match patterns are copy-pasted throughout the
test modules:

- **`check_error`** (shared) handles `ProgramError::Custom(code)`.
- **Inline match for stdlib errors** (`InvalidRealloc`,
  `NotEnoughAccountKeys`, `Custom(1)`) is duplicated in
  `AllocMaxDataLen`, `UserInsufficientLamports`, and
  `SystemProgramAddress`.
- **Success-with-validation** is duplicated in `AllocHappyPath`
  and `CreateAccountHappyPath`.

### Proposal

Generalize `check_error` into `check_result` that accepts a
`ProgramError` directly:

```rust
fn check_result(
    setup: &TestSetup,
    instruction: &Instruction,
    accounts: &[(Pubkey, Account)],
    expected: ProgramError,
) -> CaseResult { ... }
```

The existing `check_error` becomes a thin wrapper:

```rust
fn check_error(..., code: error_codes::error) -> CaseResult {
    check_result(..., ProgramError::Custom(code.into()))
}
```

This eliminates the inline match blocks in `AllocMaxDataLen`,
`UserInsufficientLamports`, and `SystemProgramAddress`.

## 2. Merge `NodeDesc` and `ExpectedNode`

### Problem

`NodeDesc` and `ExpectedNode` are structurally identical. The only
behavioral difference is that `node()` sets `value = key` while
`expected()` takes an explicit value. Two types for the same shape
adds noise.

### Proposal

Single type `NodeSpec` with one constructor and a builder method:

```rust
struct NodeSpec {
    key: u16,
    value: u16,
    color: u8,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

fn node(key, color, parent, left, right) -> NodeSpec {
    NodeSpec { key, value: key, color, parent, left, right }
}
```

For expected nodes where value differs from key (the inserted node),
use a builder:

```rust
impl NodeSpec {
    fn val(mut self, v: u16) -> Self { self.value = v; self }
}
```

Usage:

```rust
// Pre-existing node: value = key (default).
node(10, B, None, Some(1), None)
// Inserted node: value = 1.
node(42, R, Some(0), None, None).val(1)
```

`TreeDesc` and `ExpectedTree` merge into a single `TreeSpec` with
an optional `top` field (`TreeDesc` never sets it explicitly —
`build_tree_account` computes it; `ExpectedTree` specifies it for
assertion). Making `top` `Option<Option<usize>>` or always
computing it from context keeps one type.

## 3. Reduce `fixed_costs` match verbosity

### Problem

`InitCase::fixed_costs()` enumerates all 17 zero-cost variants
explicitly. Adding a new input-check variant requires updating the
match arm. `InsertCase::fixed_costs()` uses `_ => 0` — simpler
and forward-compatible.

### Proposal

All `fixed_costs()` implementations should use `_ => 0` as the
default arm. Only list the non-zero cases explicitly.

## 4. Fix `PdaMismatchChunk` display name indexing

### Problem

Variant names are `PdaMismatchChunk0..3` but display names are
"PDA mismatch chunk 1..4". The off-by-one makes the name not match
the variant.

### Proposal

Display as "PDA mismatch chunk 0..3" to match the variant names.

## 5. Remove unused `allow_asm_failures` / `allow_rust_failures`

### Problem

Both parameters are always `false` in every call site. They add two
booleans to every `print_comparison_table` call that are never
exercised.

### Proposal

Remove both parameters. If per-case failure tolerance is ever
needed, add it as a `TestCase` trait method (`fn allow_failure`)
so the table function doesn't need configuration.

## 6. Factor common setup into builders

### Problem

`insert_setup`, `insert_skip_alloc_setup`, `insert_max_data_setup`,
`init_setup`, and `pda_init_setup` share the same boilerplate:

1. Create `TestSetup`.
1. Get system program and rent sysvar keyed accounts.
1. Create user and tree pubkeys.
1. Build instruction with account metas.
1. Assemble `(Pubkey, Account)` vec.

Steps 2-5 are nearly identical across all five functions. The
differences are:

- Whether rent is configured (SIMD-0194 threshold).
- Tree pubkey derivation (unique vs PDA).
- Which accounts are included (2 vs 4).
- Tree account initial state (empty, header-only, pre-built, max
  data length).
- Instruction data (init discriminator vs insert struct).

### Proposal

Introduce a `SetupBuilder` that handles the common parts:

```rust
struct SetupBuilder {
    setup: TestSetup,
    user: (Pubkey, Account),
    tree: (Pubkey, Account),
    system_program: Option<(Pubkey, Account)>,
    rent_sysvar: Option<(Pubkey, Account)>,
}
```

Methods on the builder configure the tree account, account list,
and instruction, then finalize into the existing
`(TestSetup, Instruction, Vec<(Pubkey, Account)>)` tuple. Each
existing setup function becomes a few builder calls.

Weigh this against the cost of added indirection — if the builder
doesn't clearly reduce total lines, keep the current explicit
functions but extract just the shared preamble (getting
system_program/rent keyed accounts) into a helper.

## 7. Share imports via parent module

### Problem

Each test module imports `mollusk_svm::program`, `AccountMeta`,
`Rent`, `Check`, `Config` independently. These are used by both
`init.rs` and `insert.rs`.

### Proposal

Re-export the common types from `tests.rs` so that `use super::*`
covers them:

```rust
// tests.rs
use mollusk_svm::program;
use mollusk_svm::result::{Check, Config};
use pinocchio::sysvars::rent::Rent;
use solana_sdk::instruction::AccountMeta;
```

The child modules already use `use super::*` and would pick these
up without additional imports.

## 8. Unify address-mismatch helpers

### Problem

`run_address_mismatch` (init) and `insert_alloc_address_mismatch`
(insert) do the same bit-flip logic with different setup functions
and word sizes. The core pattern — flip a byte in a pubkey chunk
to trigger a mismatch — is identical.

### Proposal

Extract the bit-flip operation into a shared helper in `tests.rs`:

```rust
fn flip_account_address(
    instruction: &mut Instruction,
    accounts: &mut [(Pubkey, Account)],
    account_index: usize,
    chunk_index: usize,
    chunk_size: usize,
) {
    let flip_index = (chunk_index * chunk_size) + chunk_size - 1;
    accounts[account_index].0.as_mut()[flip_index] ^= 1;
    instruction.accounts[account_index].pubkey =
        accounts[account_index].0;
}
```

The module-specific helpers become: call setup, call
`flip_account_address`, call `check_error`.

## 9. Implement multi-insert integration tests

### Problem

The insert-tests spec (section "Multi-insert integration tests")
describes five sequential-insert test sequences. These are not yet
implemented.

### Proposal

Implement them as specified, using the existing `build_tree_account`
and `assert_tree_account` infrastructure. Each test processes
multiple insert instructions in sequence, feeding resulting account
state forward.

## Priority

Recommendations ordered by impact-to-effort ratio:

1. **Fix PDA mismatch display names** — trivial, correctness.
1. **Remove `allow_*_failures` params** — trivial, noise
   reduction.
1. **Unify `check_error` → `check_result`** — small change,
   removes three inline match blocks.
1. **Default `_ => 0` for `fixed_costs`** — one-line fix.
1. **Share imports via parent** — small, removes duplicate
   imports.
1. **Merge `NodeDesc`/`ExpectedNode`** — moderate, cleaner API.
1. **Extract address-flip helper** — moderate, deduplicates.
1. **Factor setup builders** — larger refactor, weigh benefit.
1. **Multi-insert integration tests** — new feature, already
   specced.
