# Insert fixup hardcoded branches

## Scope

Replace computed `dir`/`opposite(dir)` indexing in the insert fixup
loop of `program.rs` with explicit `dir_l`/`dir_r` branches that
use hardcoded `child[DIR_L]` and `child[DIR_R]` accesses. This
matches the assembly, where cases 5/6 are expanded into separate
`dir_l` and `dir_r` labels with constant offsets.

## Problem

The current Rust code computes `dir` at runtime via `direction()`
and indexes children with `child[dir]` / `child[opposite(dir)]`.
The compiler cannot resolve these to constant offsets, so every
child access compiles to a shift-add-load sequence (~3 instructions)
instead of a single load with a constant offset. This causes:

- ~14 extra ALU instructions for computed indexing.
- 3 stack spills + 3 reloads from register pressure.
- 2 extra instructions for `direction()` (pointer comparison +
  bool materialization vs. direct branch).

Total overhead: ~18 instructions (42%) vs. the hand-written
assembly on the case 5+6 critical path.

## Approach

### Direction check

Replace `direction()` + runtime `dir` variable with an early branch
on `parent == (*grandparent).child[DIR_L]`. Each arm has fully
hardcoded child accesses. The `direction()` and `opposite()`
functions become unused and are removed.

### Uncle computation

Move inside each branch with a hardcoded child index:

```rust
if parent == (*grandparent).child[tree::DIR_L] {
    let uncle = (*grandparent).child[tree::DIR_R];
    ...
} else {
    let uncle = (*grandparent).child[tree::DIR_L];
    ...
}
```

### Case 5 redundant load

The current code loads `(*parent).child[opposite(dir)]` twice:
once for the case 5 check and once as `new_root`. Hoist into a
local:

```rust
let pivot = (*parent).child[tree::DIR_R]; // dir_l variant
if node == pivot {
    // pivot is new_root — no second load
}
```

### Case 5/6 bodies

Identical to the current inline expansions from
`rotate-subtree-specialization.md`, but with every `child[dir]`
and `child[opposite(dir)]` replaced by `child[DIR_L]` or
`child[DIR_R]`.

### Case 2 and recolor

Case 2 (red uncle recolor) and the case 6 recolor + return are
shared across both branches. Case 2 is placed after the
`dir_l`/`dir_r` block using a shared `uncle` binding. The case
5/6 recolor remains inside the uncle-is-black guard (both
branches return SUCCESS).

## Resulting code

```rust
// ANCHOR: insert-fixup
// Get child direction, set at parent.
let child_dir = (key > (*parent).key) as usize;
(*parent).child[child_dir] = node;

// Main insert fixup.
loop {
    // Case 1.
    if (*parent).color == Color::Black {
        return SUCCESS;
    }

    let grandparent = (*parent).parent;
    if grandparent.is_null() {
        // Case 4.
        (*parent).color = Color::Black;
        return SUCCESS;
    }

    let uncle;
    if parent == (*grandparent).child[tree::DIR_L] {
        // dir_l: parent is left child of grandparent.
        uncle = (*grandparent).child[tree::DIR_R];
        if uncle.is_null() || (*uncle).color == Color::Black {
            // Case 5 dir_l: rotate parent LEFT.
            let pivot = (*parent).child[tree::DIR_R];
            if node == pivot {
                let new_root = pivot;
                let new_child = (*new_root).child[tree::DIR_L];

                (*parent).child[tree::DIR_R] = new_child;
                if !new_child.is_null() {
                    (*new_child).parent = parent;
                }

                (*new_root).child[tree::DIR_L] = parent;
                (*new_root).parent = grandparent;
                (*parent).parent = new_root;

                (*grandparent).child[tree::DIR_L] = new_root;

                node = parent;
                parent = new_root;
            }

            // Case 6 dir_l: rotate grandparent RIGHT.
            {
                let great_grandparent = (*grandparent).parent;
                let new_child = (*parent).child[tree::DIR_R];

                (*grandparent).child[tree::DIR_L] = new_child;
                if !new_child.is_null() {
                    (*new_child).parent = grandparent;
                }

                (*parent).child[tree::DIR_R] = grandparent;
                (*parent).parent = great_grandparent;
                (*grandparent).parent = parent;

                if !great_grandparent.is_null() {
                    let idx = (grandparent
                        == (*great_grandparent).child[tree::DIR_R])
                        as usize;
                    (*great_grandparent).child[idx] = parent;
                } else {
                    (*tree_header).root = parent;
                }
            }

            (*parent).color = Color::Black;
            (*grandparent).color = Color::Red;
            return SUCCESS;
        }
    } else {
        // dir_r: parent is right child of grandparent.
        uncle = (*grandparent).child[tree::DIR_L];
        if uncle.is_null() || (*uncle).color == Color::Black {
            // Case 5 dir_r: rotate parent RIGHT.
            let pivot = (*parent).child[tree::DIR_L];
            if node == pivot {
                let new_root = pivot;
                let new_child = (*new_root).child[tree::DIR_R];

                (*parent).child[tree::DIR_L] = new_child;
                if !new_child.is_null() {
                    (*new_child).parent = parent;
                }

                (*new_root).child[tree::DIR_R] = parent;
                (*new_root).parent = grandparent;
                (*parent).parent = new_root;

                (*grandparent).child[tree::DIR_R] = new_root;

                node = parent;
                parent = new_root;
            }

            // Case 6 dir_r: rotate grandparent LEFT.
            {
                let great_grandparent = (*grandparent).parent;
                let new_child = (*parent).child[tree::DIR_L];

                (*grandparent).child[tree::DIR_R] = new_child;
                if !new_child.is_null() {
                    (*new_child).parent = grandparent;
                }

                (*parent).child[tree::DIR_L] = grandparent;
                (*parent).parent = great_grandparent;
                (*grandparent).parent = parent;

                if !great_grandparent.is_null() {
                    let idx = (grandparent
                        == (*great_grandparent).child[tree::DIR_R])
                        as usize;
                    (*great_grandparent).child[idx] = parent;
                } else {
                    (*tree_header).root = parent;
                }
            }

            (*parent).color = Color::Black;
            (*grandparent).color = Color::Red;
            return SUCCESS;
        }
    }

    // Case 2.
    (*parent).color = Color::Black;
    (*uncle).color = Color::Black;
    (*grandparent).color = Color::Red;
    node = grandparent;

    parent = (*node).parent;
    if parent.is_null() {
        break;
    }
}
// Case 3.
SUCCESS
// ANCHOR_END: insert-fixup
```

## Diff from current code

The `dir_l` and `dir_r` blocks are exact mirrors: every
`child[DIR_L]` swaps with `child[DIR_R]`.

## Functions removed

- `direction()` — no longer called; the branch replaces it.
- `opposite()` — no longer called; hardcoded indices replace it.

Both are currently only used in the insert fixup loop. If a future
operation (e.g. delete) needs them, they can be re-added.

## Child direction assignment

The initial `let dir = if key > ... { DIR_R } else { DIR_L }`
before the loop is replaced with
`let child_dir = (key > (*parent).key) as usize` to use branchless
bool-to-index. This matches the existing search pattern in `program.rs:616`.

## Verification

After the change, rebuild and compare the disassembly to confirm
that all `child[]` accesses use constant offsets (+8 or +16) and
that no stack spills remain in the fixup loop.
