# Remove-to-tree test specification

## Scope

Tests for the remove instruction: input validation, search, BST
deletion, red-black rebalancing, node recycling, and return value
encoding. Mirrors the structure of the insert test specification.

## Shared test helpers

The insert tests define helpers (`NodeSpec`, `TreeSpec`,
`build_tree_account`, `assert_tree_account`, `node()`, `B`, `R`,
`node_vaddr`, `opt_vaddr`, `build_empty_tree`) that are equally
useful for remove tests. These should be extracted into a shared
module (e.g. `tests/common.rs`) imported by both `tests/insert.rs`
and `tests/remove.rs`.

Helpers that stay in `tests/insert.rs` (insert-specific):

- `insert_instruction_data`, `insert_setup`,
  `insert_skip_alloc_setup`, `insert_tree_setup`.
- `run_success`, `run_dup_error`.
- `InsertCase`, `MultiInsertCase`.

New helpers in `tests/remove.rs` (remove-specific):

- `remove_setup(lang, desc, remove_key)` -- build a pre-populated
  tree account and a `RemoveInstruction` with the given key. Uses
  two accounts (user + tree), no CPI accounts needed.
- `run_remove_success(lang, desc, remove_key, expected_value, expected_tree)` --
  execute the remove instruction, verify the
  return code encodes the expected value, then assert full tree
  state including the freed node slot.
- `run_remove_not_found(lang, desc, remove_key)` -- execute and
  verify `KEY_DOES_NOT_EXIST` error.

## Return value verification

On success the program returns `RemoveReturn` packed in `r0`:

```text
r0 = (value << 16) | REMOVE_STATUS_OK
```

The Solana runtime interprets any non-zero `r0` as
`ProgramError::Custom(r0)`. Tests must verify the result is
`ProgramError::Custom((value as u32) << 16 | REMOVE_STATUS_OK as u32)` where `REMOVE_STATUS_OK` equals `error::N_CODES`
(currently 15). This confirms both the value field (bits 16-31)
and the status field (bits 0-15) are encoded correctly.

## Freed node verification

When a node is removed it is cast to a `StackNode` and pushed onto
the free stack. The full-state assertion must verify:

- **`header.top`** points to the freed node slot.
- **Freed node `child[L]`** is null (zeroed by remove).
- **Freed node `child[R]`** is null (zeroed by remove).
- **Freed node `parent`** (offset 0, overlaps `StackNode.next`)
  equals the previous stack top (null if the stack was empty before
  removal).

The `key`, `value`, and `color` fields of the freed node are not
cleared by remove (insert overwrites them when the node is
recycled). Tests should still assert their values to confirm remove
does not clobber them unexpectedly -- they should retain whatever
values they had before removal.

The existing `assert_tree_account` already checks every field of
every node in the buffer, so including the freed node slot in the
expected `TreeSpec.nodes` list is sufficient. No new assertion
helper is needed; the freed node is simply a `NodeSpec` with null
children and `parent` set to the old stack top index.

## `build_tree_account` adjustment

The insert helper always appends one extra free slot after the
existing nodes (for the node to be inserted). For remove tests,
the tree should contain only the existing nodes with no extra free
slot -- there is nothing to insert, and the removed node itself
becomes the new free slot. A `build_tree_account_no_free` variant
(or a flag) is needed:

- `header.top = null` (no pre-existing free nodes).
- `header.next = null` (unused).
- Data length = `sizeof(TreeHeader) + N * sizeof(TreeNode)`.

After removal, the freed node's slot is already within the
existing data. `header.top` will point to the freed node, and
`StackNode.next` will be null (stack was empty).

If the tree already has free nodes on the stack before removal
(tested in multi-step scenarios), `header.top` is set to the
existing free node and the freed node's `StackNode.next` must
chain to it.

## Input check tests

Same checks as insert, using `RemoveInstruction` (discriminator 2,
3 bytes). Each test constructs a valid two-account remove setup,
mutates one property, and expects the corresponding error.

| Case              | Mutation                 | Expected error         |
| ----------------- | ------------------------ | ---------------------- |
| Data too short    | 1-byte instruction data  | `INSTRUCTION_DATA_LEN` |
| Data too long     | 4-byte instruction data  | `INSTRUCTION_DATA_LEN` |
| Too few accounts  | 1 account (user only)    | `N_ACCOUNTS`           |
| User has data     | User account with 1 byte | `USER_DATA_LEN`        |
| Tree is duplicate | Tree = copy of user      | `TREE_DUPLICATE`       |

## Search error tests

| Case           | Tree setup         | Key | Expected             |
| -------------- | ------------------ | --- | -------------------- |
| Empty tree     | Root is null       | 10  | `KEY_DOES_NOT_EXIST` |
| Not found (L)  | Root key=10        | 5   | `KEY_DOES_NOT_EXIST` |
| Not found (R)  | Root key=10        | 15  | `KEY_DOES_NOT_EXIST` |
| Not found deep | Root 10, L=5, R=15 | 12  | `KEY_DOES_NOT_EXIST` |

Search error tests must also verify the tree account data is
unchanged after the failed instruction.

## Simple removal cases

These cases do not trigger rebalancing.

### Simple case 2: remove root leaf

Single-node tree. Root becomes null, node is freed.

```text
Before:
  Header: root=N0  top=--  next=--
  N0: B key=10 val=10  parent=--  L=--  R=--

After remove key=10 (returns value=10):
  Header: root=--  top=N0  next=--
  N0: key=10 val=10 color=B  parent=--  L=--  R=--   <- freed
```

`parent` (= `StackNode.next`) is null because the stack was empty.

### Simple case 3: remove red leaf

Red leaf with a black parent. Detach the leaf.

Left child variant (remove key=5):

```text
Before:
  Header: root=N0  top=--  next=--
  N0: B key=10  parent=--  L=N1  R=--
  N1: R key=5   parent=N0  L=--  R=--

After:
  Header: root=N0  top=N1  next=--
  N0: B key=10  parent=--  L=--  R=--
  N1: key=5 color=R  parent=--  L=--  R=--   <- freed
```

Right child variant (remove key=15): mirror with N1 as right
child of N0.

### Simple case 1: node with one child

A node with exactly one child must be black, and the child must
be red (RB invariant). Replace the node with its child and
recolor the child black.

Right child variant (remove key=10):

```text
Before:
  Header: root=N0  top=--  next=--
  N0: B key=10  parent=--  L=--  R=N1
  N1: R key=15  parent=N0  L=--  R=--

After:
  Header: root=N1  top=N0  next=--
  N0: key=10 color=B  parent=--  L=--  R=--   <- freed
  N1: B key=15  parent=--  L=--  R=--   <- recolored B, new root
```

Left child variant (remove key=10, child is N1 at L):

```text
Before:
  Header: root=N0  top=--  next=--
  N0: B key=10  parent=--  L=N1  R=--
  N1: R key=5   parent=N0  L=--  R=--

After:
  Header: root=N1  top=N0  next=--
  N0: key=10 color=B  parent=--  L=--  R=--   <- freed
  N1: B key=5   parent=--  L=--  R=--   <- recolored B, new root
```

Non-root variant (remove key=15):

```text
Before:
  Header: root=N0  top=--  next=--
  N0: B key=10  parent=--  L=N1  R=N2
  N1: B key=5   parent=N0  L=--  R=--
  N2: B key=15  parent=N0  L=--  R=N3
  N3: R key=20  parent=N2  L=--  R=--

After:
  Header: root=N0  top=N2  next=--
  N0: B key=10  parent=--  L=N1  R=N3
  N1: B key=5   parent=N0  L=--  R=--
  N2: key=15 color=B  parent=--  L=--  R=--   <- freed
  N3: B key=20  parent=N0  L=--  R=--   <- recolored B
```

## Successor swap cases

When the node to delete has two children, copy key/value from
the in-order successor (leftmost in right subtree), then delete
the successor instead.

### Successor is immediate right child

```text
Before:
  Header: root=N0  top=--  next=--
  N0: B key=10  parent=--  L=N1  R=N2
  N1: B key=5   parent=N0  L=--  R=--
  N2: R key=15  parent=N0  L=--  R=--

After remove key=10 (returns value=10):
  Header: root=N0  top=N2  next=--
  N0: B key=15 val=15  parent=--  L=N1  R=--  <- copied from successor
  N1: B key=5   parent=N0  L=--  R=--
  N2: key=15 color=R  parent=--  L=--  R=--   <- freed (was successor)
```

The successor (N2, key=15) is a red leaf -- simple case 3 after
the swap.

### Successor with deep left descent

```text
Before:
  Header: root=N0  top=--  next=--
  N0: B key=10  parent=--  L=N1  R=N2
  N1: B key=5   parent=N0  L=--  R=--
  N2: B key=20  parent=N0  L=N3  R=N4
  N3: R key=15  parent=N2  L=--  R=--
  N4: R key=25  parent=N2  L=--  R=--

After remove key=10 (returns value=10):
  Header: root=N0  top=N3  next=--
  N0: B key=15 val=15  parent=--  L=N1  R=N2  <- copied from successor
  N1: B key=5   parent=N0  L=--  R=--
  N2: B key=20  parent=N0  L=--  R=N4
  N3: key=15 color=R  parent=--  L=--  R=--   <- freed (was successor)
  N4: R key=25  parent=N2  L=--  R=--
```

The successor (N3, key=15) is a red leaf -- simple case 3 after
the swap.

### Successor with right child

The successor has no left child but may have a right child.
This triggers simple case 1 (one-child node) after the swap.

```text
Before:
  Header: root=N0  top=--  next=--
  N0: B key=10  parent=--  L=N1  R=N2
  N1: B key=5   parent=N0  L=--  R=--
  N2: B key=15  parent=N0  L=--  R=N3
  N3: R key=20  parent=N2  L=--  R=--

After remove key=10 (returns value=10):
  Header: root=N0  top=N2  next=--
  N0: B key=15 val=15  parent=--  L=N1  R=N3  <- copied from successor
  N1: B key=5   parent=N0  L=--  R=--
  N2: key=15 color=B  parent=--  L=--  R=--   <- freed (was successor)
  N3: B key=20  parent=N0  L=--  R=--   <- recolored B
```

Successor N2 has one child (N3). Simple case 1: replace N2 with
N3, recolor N3 black.

## Rebalancing cases

Entry condition: a black leaf was removed from `parent.child[dir]`.
The rebalancing loop executes with `(parent, dir)`.

Cases follow the Wikipedia algorithm numbering. Each case lists
both direction variants (dir_l where the removed node was a left
child, dir_r where it was a right child) unless noted otherwise.

### Case 4: red parent, black sibling, black nephews

Recolor sibling red and parent black. No rotation.

Dir_l variant (remove key=5):

```text
Before:
  Header: root=N0  top=--  next=--
  N0: R key=10  parent=--  L=N1  R=N2
  N1: B key=5   parent=N0  L=--  R=--
  N2: B key=15  parent=N0  L=--  R=--

After:
  Header: root=N0  top=N1  next=--
  N0: B key=10  parent=--  L=--  R=N2   <- recolored B
  N1: key=5 color=B  parent=--  L=--  R=--   <- freed
  N2: R key=15  parent=N0  L=--  R=--   <- recolored R
```

Dir_r variant (remove key=15): mirror of above.

### Case 6: black sibling, distant nephew red

Single rotation at parent. Sibling takes parent's color, parent
and distant nephew become black.

Dir_l variant (remove key=5):

```text
Before:
  Header: root=N0  top=--  next=--
  N0: B key=10  parent=--  L=N1  R=N2
  N1: B key=5   parent=N0  L=--  R=--
  N2: B key=15  parent=N0  L=--  R=N3
  N3: R key=20  parent=N2  L=--  R=--

After:
  Header: root=N2  top=N1  next=--
  N0: B key=10  parent=N2  L=--  R=--   <- recolored B
  N1: key=5 color=B  parent=--  L=--  R=--   <- freed
  N2: B key=15  parent=--  L=N0  R=N3   <- new root
  N3: B key=20  parent=N2  L=--  R=--   <- recolored B
```

Dir_r variant (remove key=20):

```text
Before:
  Header: root=N0  top=--  next=--
  N0: B key=10  parent=--  L=N1  R=N2
  N1: B key=5   parent=N0  L=N3  R=--
  N2: B key=20  parent=N0  L=--  R=--
  N3: R key=3   parent=N1  L=--  R=--

After:
  Header: root=N1  top=N2  next=--
  N0: B key=10  parent=N1  L=--  R=--   <- recolored B
  N1: B key=5   parent=--  L=N3  R=N0   <- new root
  N2: key=20 color=B  parent=--  L=--  R=--   <- freed
  N3: B key=3   parent=N1  L=--  R=--   <- recolored B
```

### Case 6: non-null new_child in rotation

The rotation transfers sibling's child on the `dir` side to
parent. When that child is non-null, it must be reparented.

Dir_l variant (remove key=3):

```text
Before:
  Header: root=N0  top=--  next=--
  N0: B key=10  parent=--  L=N1  R=N2
  N1: B key=5   parent=N0  L=N4  R=--
  N2: B key=20  parent=N0  L=N3  R=N5
  N3: R key=15  parent=N2  L=--  R=--
  N4: B key=3   parent=N1  L=--  R=--
  N5: R key=25  parent=N2  L=--  R=--

After remove key=5 (successor swap: N1 gets key=3 from N4,
then delete N4 which is a black leaf):
  Header: root=N0  top=N4  next=--
  N0: B key=10  parent=N2  L=N1  R=N3   <- reparented
  N1: B key=3   parent=N0  L=--  R=--   <- copied from successor
  N2: B key=20  parent=--  L=N0  R=N5   <- new root
  N3: B key=15  parent=N0  L=--  R=--   <- reparented, recolored B
  N4: key=3 color=B  parent=--  L=--  R=--   <- freed
  N5: B key=25  parent=N2  L=--  R=--   <- recolored B
```

Wait -- this case is getting complex because it also involves a
successor swap. A cleaner test isolates the rotation by removing
a black leaf directly. Concrete before/after states for non-null
new_child cases will be determined during implementation when
exact tree shapes are constructed.

### Case 5 + 6: close nephew red, distant nephew black

Rotate sibling away from dir (case 5), then rotate parent toward
dir (case 6).

Dir_l variant (remove key=5):

```text
Before:
  Header: root=N0  top=--  next=--
  N0: B key=10  parent=--  L=N1  R=N2
  N1: B key=5   parent=N0  L=--  R=--
  N2: B key=20  parent=N0  L=N3  R=--
  N3: R key=15  parent=N2  L=--  R=--

After:
  Header: root=N3  top=N1  next=--
  N0: B key=10  parent=N3  L=--  R=--   <- recolored B
  N1: key=5 color=B  parent=--  L=--  R=--   <- freed
  N2: R key=20  parent=N3  L=--  R=--   <- recolored R
  N3: B key=15  parent=--  L=N0  R=N2   <- new root
```

Dir_r variant (remove key=20):

```text
Before:
  Header: root=N0  top=--  next=--
  N0: B key=10  parent=--  L=N1  R=N2
  N1: B key=5   parent=N0  L=--  R=N3
  N2: B key=20  parent=N0  L=--  R=--
  N3: R key=7   parent=N1  L=--  R=--

After:
  Header: root=N3  top=N2  next=--
  N0: B key=10  parent=N3  L=--  R=--   <- recolored B
  N1: R key=5   parent=N3  L=--  R=--   <- recolored R
  N2: key=20 color=B  parent=--  L=--  R=--   <- freed
  N3: B key=7   parent=--  L=N1  R=N0   <- new root
```

### Case 3 + case 4: red sibling, then recolor

Red sibling is rotated, making the old close_nephew the new
sibling. If the new sibling has two black/null nephews, recolor
(case 4).

Dir_l variant (remove key=3):

```text
Before:
  Header: root=N0  top=--  next=--
  N0: B key=10  parent=--  L=N1  R=N2
  N1: B key=5   parent=N0  L=N4  R=--
  N2: R key=20  parent=N0  L=N3  R=N5
  N3: B key=15  parent=N2  L=--  R=--
  N4: B key=3   parent=N1  L=--  R=--

After remove key=5 (successor swap from N4, delete N4 as black
leaf, dir=L relative to N1):
  Header: root=N2  top=N4  next=--
  N0: B key=10  parent=N2  L=N1  R=N3   <- reparented
  N1: B key=3   parent=N0  L=--  R=--   <- swapped key/val
  N2: B key=20  parent=--  L=N0  R=N5   <- recolored B, new root
  N3: R key=15  parent=N0  L=--  R=--   <- recolored R (case 4)
  N4: key=3 color=B  parent=--  L=--  R=--   <- freed
  N5: B key=25  parent=N2  L=--  R=--
```

These trees are moderately complex. Exact before/after states
should be verified by hand or with a reference implementation
during test construction. The spec lists the required paths; the
implementation fills in precise node layouts.

### Case 3 + case 6: red sibling, then distant nephew red

After the case 3 rotation, the new sibling's distant nephew is
red. Jump directly to case 6.

Dir_l and dir_r variants needed.

### Case 3 + case 5 + case 6: red sibling, then double rotation

After the case 3 rotation, the new sibling's close nephew is red
and distant nephew is black. Case 5 rotates, then case 6 rotates.

Dir_l and dir_r variants needed.

### Case 2: propagation

Black sibling, both nephews black, black parent. Recolor sibling
red and propagate upward with `node = parent`.

Dir_l variant (remove key=3):

```text
Before:
  Header: root=N0  top=--  next=--
  N0: B key=10  parent=--  L=N1  R=N2
  N1: B key=5   parent=N0  L=N3  R=--
  N2: B key=15  parent=N0  L=--  R=--
  N3: B key=3   parent=N1  L=--  R=--

After remove key=5 (successor swap from N3, delete N3 as black
leaf):
  Header: root=N0  top=N3  next=--
  N0: B key=10  parent=--  L=N1  R=N2
  N1: B key=3   parent=N0  L=--  R=--   <- swapped key/val
  N2: R key=15  parent=N0  L=--  R=--   <- recolored R (case 2)
  N3: key=3 color=B  parent=--  L=--  R=--   <- freed
```

Wait -- after case 2 recolors sibling and propagates to N0 (the
root), the while condition `parent = node->parent` is null, so
the loop exits. But N0 is already black, so the tree is valid
with a shorter black height. The resulting tree has N2 as red,
which combined with case 2's recolor of the sibling means the
tree stays balanced.

Exact states for case 2 propagation chains (case 2 → case 4,
case 2 → case 6, etc.) require larger trees and should be
verified during implementation.

### Case 2 + case 4: propagate then red parent

Case 2 propagates upward, reaching a red parent. Case 4 recolors
and terminates.

### Case 2 + case 6: propagate then rotation

Case 2 propagates upward, reaching a position where the distant
nephew is red. Case 6 rotates and terminates.

## Case 6: parent null check variants

Case 6 rotates parent. If parent is the tree root, the rotation
must update `tree.root`. If parent has a parent, the
great-grandparent's child pointer must be updated (left or right
depending on direction).

| Variant               | Parent position         | Covers              |
| --------------------- | ----------------------- | ------------------- |
| Parent is root        | Parent has no parent    | Root replacement    |
| Parent is left child  | Parent is GGP's L child | GGP.child[L] update |
| Parent is right child | Parent is GGP's R child | GGP.child[R] update |

Each variant needs dir_l and dir_r sub-variants.

### Case 3: parent null check

Case 3 rotates parent. Same root vs. non-root variants as
case 6.

## Test case summary

### Input checks

| #   | Case              |
| --- | ----------------- |
| 1   | Data too short    |
| 2   | Data too long     |
| 3   | Too few accounts  |
| 4   | User has data     |
| 5   | Tree is duplicate |

### Search errors

| #   | Case              |
| --- | ----------------- |
| 6   | Empty tree        |
| 7   | Not found (left)  |
| 8   | Not found (right) |
| 9   | Not found (deep)  |

### Simple removal

| #   | Case                              | Direction |
| --- | --------------------------------- | --------- |
| 10  | Root leaf (simple case 2)         | --        |
| 11  | Red leaf (simple case 3)          | L         |
| 12  | Red leaf (simple case 3)          | R         |
| 13  | One child at root (simple case 1) | R child   |
| 14  | One child at root (simple case 1) | L child   |
| 15  | One child non-root (sc 1)         | R child   |

### Successor swap

| #   | Case                       |
| --- | -------------------------- |
| 16  | Immediate right child      |
| 17  | Deep left descent          |
| 18  | Successor with right child |

### Rebalancing (black leaf removal)

| #   | Path                          | Dir |
| --- | ----------------------------- | --- |
| 19  | Case 4                        | L   |
| 20  | Case 4                        | R   |
| 21  | Case 6                        | L   |
| 22  | Case 6                        | R   |
| 23  | Case 5 + 6                    | L   |
| 24  | Case 5 + 6                    | R   |
| 25  | Case 3 → 4                    | L   |
| 26  | Case 3 → 4                    | R   |
| 27  | Case 3 → 6                    | L   |
| 28  | Case 3 → 6                    | R   |
| 29  | Case 3 → 5 → 6                | L   |
| 30  | Case 3 → 5 → 6                | R   |
| 31  | Case 2 (propagate to root)    | L   |
| 32  | Case 2 (propagate to root)    | R   |
| 33  | Case 2 → 4                    | --  |
| 34  | Case 2 → 6                    | --  |
| 35  | Case 6 non-null new_child     | L   |
| 36  | Case 6 non-null new_child     | R   |
| 37  | Case 6 parent=root            | L   |
| 38  | Case 6 parent=root            | R   |
| 39  | Case 6 parent=GGP left child  | L   |
| 40  | Case 6 parent=GGP right child | R   |
| 41  | Case 3 parent=root            | L   |
| 42  | Case 3 parent=root            | R   |

### Multi-step integration

| #   | Case                              |
| --- | --------------------------------- |
| 43  | Insert 3, remove 1 (minimal)      |
| 44  | Insert 7, remove all (full cycle) |
| 45  | Insert-remove-insert (recycling)  |

## Multi-step integration tests

Sequential operations that verify insert and remove interact
correctly. These use `build_empty_tree` to pre-allocate free
slots, then chain insert and remove instructions, asserting full
tree state after each step.

| Test       | Sequence               | Purpose          |
| ---------- | ---------------------- | ---------------- |
| Minimal    | Insert 10,5,15; rm 5   | Basic remove     |
| Full cycle | Insert 7 nodes; rm all | All nodes freed  |
| Recycle    | Insert 3; rm 1; insert | Reuse from stack |

The recycle test is critical: after removing a node, the free
stack must correctly provide it for the next insert. This
validates the `StackNode.next` chain and `header.top` updates
across both operations.

## Coverage notes

The test list above covers all paths through the Wikipedia
rebalancing algorithm, both direction variants, successor swap
edge cases, and the `rotate_subtree` null/non-null checks. As
optimizations are introduced (inlined rotations, hardcoded
direction branches), additional test cases may be needed to
ensure full branch coverage of the new code paths. Each
optimization should reference which existing tests cover it and
add new cases for any branches not yet exercised.

## Reference algorithm

The verbatim Wikipedia remove-rebalancing algorithm (source of
truth for case numbering and control flow):

```c
void remove(Tree* tree, Node* node) {
    Node* parent = node->parent;

    Node* sibling;
    Node* close_nephew;
    Node* distant_nephew;

    Direction dir = direction(node);

    parent->child[dir] = NULL;
    goto start_balance;

    do {
        dir = direction(node);
start_balance:
        sibling = parent->child[1 - dir];
        distant_nephew = sibling->child[1 - dir];
        close_nephew = sibling->child[dir];
        if (sibling->color == RED) {
            // Case #3
            rotate_subtree(tree, parent, dir);
            parent->color = RED;
            sibling->color = BLACK;
            sibling = close_nephew;

            distant_nephew = sibling->child[1 - dir];
            if (distant_nephew
                && distant_nephew->color == RED) {
                goto case_6;
            }
            close_nephew = sibling->child[dir];
            if (close_nephew
                && close_nephew->color == RED) {
                goto case_5;
            }

            // Case #4
            sibling->color = RED;
            parent->color = BLACK;
            return;
        }

        if (distant_nephew
            && distant_nephew->color == RED) {
            goto case_6;
        }

        if (close_nephew
            && close_nephew->color == RED) {
            goto case_5;
        }

        if (!parent) {
            // Case #1
            return;
        }

        if (parent->color == RED) {
            // Case #4
            sibling->color = RED;
            parent->color = BLACK;
            return;
        }

        // Case #2
        sibling->color = RED;
        node = parent;

    } while (parent = node->parent);

case_5:

    rotate_subtree(tree, sibling, 1 - dir);
    sibling->color = RED;
    close_nephew->color = BLACK;
    distant_nephew = sibling;
    sibling = close_nephew;

case_6:

    rotate_subtree(tree, parent, dir);
    sibling->color = parent->color;
    parent->color = BLACK;
    distant_nephew->color = BLACK;
    return;
}
```

The test cases map to paths through this algorithm:

- **Case 2**: the `sibling->color = RED; node = parent` path,
  loop continues.
- **Case 3**: `sibling->color == RED` branch, then falls through
  to check nephews of the new sibling.
- **Case 4**: `parent->color == RED` recolor (both direct and
  after case 3).
- **Case 5**: `close_nephew` is red, `goto case_5` from either
  the case 3 block or the main block.
- **Case 6**: `distant_nephew` is red, `goto case_6` from either
  the case 3 block or the main block.
