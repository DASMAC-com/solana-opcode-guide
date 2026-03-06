<!-- cspell:word reparented -->

<!-- cspell:word reparenting -->

# Insert-to-tree test specification

## Scope

Tests for the search, insertion, and rebalancing logic in the insert
instruction — everything after node allocation. The existing insert
tests cover input validation and the allocation/recycle paths; this
spec covers what happens once we have a node and need to place it in
the tree.

## Approach

### Pre-built tree states

Each rebalancing case requires a specific tree shape. Rather than
chaining multiple insert instructions, construct the tree layout
directly in account memory before processing the insert instruction.
This gives precise control over which case is triggered.

A helper builds the account data buffer by:

- Allocating space for `TreeHeader` + N existing `TreeNode`s + 1 free
  node on the stack.
- Setting all pointers as virtual addresses
  (`MM_INPUT_START + input_buffer::TREE_DATA_OFF + offset`).
- Setting `header.top` to the free stack node so the insert pops it
  instead of allocating (same pattern as `insert_skip_alloc_setup`).

### Fixed costs

Targeted case tests pop from the free stack, so they have zero fixed
costs. Multi-insert integration tests use the allocation path (CPI
transfer) for each insert. Their `fixed_costs()` must return
`CPI_BASE + SYSTEM_PROGRAM` per insert so the comparison table
subtracts the transfer CPI overhead and isolates the tree logic.

### Full-state assertion

Every targeted case test asserts the **entire** tree data account
after the insert instruction. Nothing is left unchecked.

**TreeHeader** (every field):

- `root` — pointer to the expected root node.
- `top` — pointer to the new free-stack top (null when the only
  free node was consumed).
- `next` — allocation pointer (unchanged by pop-from-stack inserts).

**Every TreeNode** (every field of every node in the buffer):

- `parent` — pointer to the expected parent (null for root).
- `child[L]`, `child[R]` — pointers to expected children (null for
  leaves).
- `key` — expected key.
- `value` — expected value (pre-set for existing nodes, from
  instruction data for the inserted node).
- `color` — expected color after rebalancing.

Each case below specifies the full expected state using compact
notation:

```text
Header: root=N0  top=null  next=<end>
N0: B key=10  parent=--  L=N1  R=N2
N1: R key=5   parent=N0  L=--  R=--   <- inserted
N2: B key=15  parent=N0  L=--  R=--
```

`--` means null. `<end>` is the address past the last node slot.
Node indices (N0, N1, ...) reflect memory layout order, not tree
position — N0 is always at offset `sizeof(TreeHeader)` in the
account data. Values are omitted from the notation for brevity;
pre-existing nodes keep their original values, and the inserted
node gets the instruction data value.

## Cases

### Search

Error cases — the tree account must be unchanged after the
instruction returns `KEY_EXISTS`.

| Case         | Setup               | Key | Expected           |
| ------------ | ------------------- | --- | ------------------ |
| Dup at root  | Root with key 10    | 10  | `KEY_EXISTS` error |
| Dup in left  | Root 10, L child 5  | 5   | `KEY_EXISTS` error |
| Dup in right | Root 10, R child 15 | 15  | `KEY_EXISTS` error |

### Insert to empty tree

```text
Before:
  Header: root=--  top=N0  next=<end>
  N0: (free stack node)

After insert key=42:
  Header: root=N0  top=--  next=<end>
  N0: R key=42  parent=--  L=--  R=--
```

### Case 1: parent is black

No rebalancing needed. Inserted node stays red under a black parent.

Left child variant (insert key=5):

```text
Before:
  Header: root=N0  top=N1  next=<end>
  N0: B key=10  parent=--  L=--  R=--

After:
  Header: root=N0  top=--  next=<end>
  N0: B key=10  parent=--  L=N1  R=--
  N1: R key=5   parent=N0  L=--  R=--   <- inserted
```

Right child variant (insert key=15): mirror of above, N1 at R of
N0.

### Case 4: parent is root and red

Parent is root (no grandparent). Recolor parent to black.

Left child variant (insert key=5):

```text
Before:
  Header: root=N0  top=N1  next=<end>
  N0: R key=10  parent=--  L=--  R=--

After:
  Header: root=N0  top=--  next=<end>
  N0: B key=10  parent=--  L=N1  R=--   <- recolored B
  N1: R key=5   parent=N0  L=--  R=--   <- inserted
```

Right child variant (insert key=15): mirror of above.

### Case 2 + 3: red uncle, propagate to root

Parent and uncle are both red. Recolor parent, uncle, grandparent.
Grandparent is root, so the loop exits (case 3).

Left-left variant (insert key=1):

```text
Before:
  Header: root=N0  top=N3  next=<end>
  N0: B key=10  parent=--  L=N1  R=N2
  N1: R key=5   parent=N0  L=--  R=--
  N2: R key=15  parent=N0  L=--  R=--

After:
  Header: root=N0  top=--  next=<end>
  N0: R key=10  parent=--  L=N1  R=N2   <- recolored R
  N1: B key=5   parent=N0  L=N3  R=--   <- recolored B
  N2: B key=15  parent=N0  L=--  R=--   <- recolored B
  N3: R key=1   parent=N1  L=--  R=--   <- inserted
```

All four child positions trigger the same recolor path — only the
inserted node's position differs.

| Variant     | Insert key | Inserted at |
| ----------- | ---------- | ----------- |
| Left-left   | 1          | N1.L        |
| Left-right  | 7          | N1.R        |
| Right-left  | 12         | N2.L        |
| Right-right | 20         | N2.R        |

### Case 2 + 1: red uncle, propagate to black ancestor

Same recolor as case 2+3, but grandparent is not root — its parent
is black, so case 1 terminates.

Left-left variant (insert key=1):

```text
Before:
  Header: root=N0  top=N4  next=<end>
  N0: B key=20  parent=--  L=N1  R=--
  N1: B key=10  parent=N0  L=N2  R=N3
  N2: R key=5   parent=N1  L=--  R=--
  N3: R key=15  parent=N1  L=--  R=--

After:
  Header: root=N0  top=--  next=<end>
  N0: B key=20  parent=--  L=N1  R=--
  N1: R key=10  parent=N0  L=N2  R=N3   <- recolored R
  N2: B key=5   parent=N1  L=N4  R=--   <- recolored B
  N3: B key=15  parent=N1  L=--  R=--   <- recolored B
  N4: R key=1   parent=N2  L=--  R=--   <- inserted
```

Mirror variant: B(2) as root with B(10) as right child, insert
key=20. Same recolor pattern, mirrored.

### Case 6: black uncle, outer child — single rotation

Uncle is black or null. Node is an outer child (same direction as
parent relative to grandparent). Rotate grandparent, recolor.

Left-left, null uncle variant (insert key=1):

```text
Before:
  Header: root=N0  top=N2  next=<end>
  N0: B key=10  parent=--  L=N1  R=--
  N1: R key=5   parent=N0  L=--  R=--

After:
  Header: root=N1  top=--  next=<end>
  N0: R key=10  parent=N1  L=--  R=--   <- recolored R
  N1: B key=5   parent=--  L=N2  R=N0   <- recolored B, new root
  N2: R key=1   parent=N1  L=--  R=--   <- inserted
```

A non-null black uncle at the same depth as a null uncle is
impossible in a valid red-black tree. The parent is red (otherwise
no fixup triggers), so it contributes 0 to the black height. A
non-null black uncle contributes at least 1. These cannot balance
(RBT-4 violation). A non-null black uncle only appears after
case 2 propagation up the tree, which is covered by the case 2+6
tests below.

Right-right variants: mirror of above (insert key=20, parent is
R(15), rotation goes left).

### Case 5 + 6: black uncle, inner child — double rotation

Uncle is black or null. Node is an inner child (opposite direction
from parent relative to grandparent). Rotate parent first (case 5),
then fall through to case 6.

Left-right, null uncle variant (insert key=7):

```text
Before:
  Header: root=N0  top=N2  next=<end>
  N0: B key=10  parent=--  L=N1  R=--
  N1: R key=5   parent=N0  L=--  R=--

After:
  Header: root=N2  top=--  next=<end>
  N0: R key=10  parent=N2  L=--  R=--   <- recolored R
  N1: R key=5   parent=N2  L=--  R=--
  N2: B key=7   parent=--  L=N1  R=N0   <- inserted, new root
```

As with case 6 above, a non-null black uncle at the same depth
is impossible in a valid red-black tree (RBT-4). Case 2+5+6
tests cover the non-null black uncle scenario via propagation.

Right-left variants: mirror of above (insert key=12, parent is
R(15), double rotation goes right then left).

### Case 6: non-null great-grandparent

The existing case 6 tests have grandparent as root, so
great-grandparent is always null and the root-replacement path fires.
These variants place the rotation under a non-root grandparent to
cover the great-grandparent child pointer update branches in the
assembly (`insert_fixup_case_6_dir_l_left`,
`insert_fixup_case_6_dir_r_left`, and their fall-through
counterparts).

Left-left, GP is left child of GGP (insert key=1):

```text
Before:
  Header: root=N0  top=N4  next=<end>
  N0: B key=20  parent=--  L=N1  R=N3
  N1: B key=10  parent=N0  L=N2  R=--
  N2: R key=5   parent=N1  L=--  R=--
  N3: B key=25  parent=N0  L=--  R=--

After:
  Header: root=N0  top=--  next=<end>
  N0: B key=20  parent=--  L=N2  R=N3
  N1: R key=10  parent=N2  L=--  R=--   <- recolored R
  N2: B key=5   parent=N0  L=N4  R=N1   <- recolored B
  N3: B key=25  parent=N0  L=--  R=--
  N4: R key=1   parent=N2  L=--  R=--   <- inserted
```

Covers: GGP non-null, GP is left child of GGP (dir_l path,
`insert_fixup_case_6_dir_l_left`).

Left-left, GP is right child of GGP (insert key=10):

```text
Before:
  Header: root=N0  top=N4  next=<end>
  N0: B key=5   parent=--  L=N1  R=N2
  N1: B key=3   parent=N0  L=--  R=--
  N2: B key=20  parent=N0  L=N3  R=--
  N3: R key=15  parent=N2  L=--  R=--

After:
  Header: root=N0  top=--  next=<end>
  N0: B key=5   parent=--  L=N1  R=N3
  N1: B key=3   parent=N0  L=--  R=--
  N2: R key=20  parent=N3  L=--  R=--   <- recolored R
  N3: B key=15  parent=N0  L=N4  R=N2   <- recolored B
  N4: R key=10  parent=N3  L=--  R=--   <- inserted
```

Covers: GGP non-null, GP is right child of GGP (dir_l path,
fall-through at `jne r3, r8`).

Right-right, GP is right child of GGP (insert key=25):

```text
Before:
  Header: root=N0  top=N4  next=<end>
  N0: B key=5   parent=--  L=N1  R=N2
  N1: B key=3   parent=N0  L=--  R=--
  N2: B key=15  parent=N0  L=--  R=N3
  N3: R key=20  parent=N2  L=--  R=--

After:
  Header: root=N0  top=--  next=<end>
  N0: B key=5   parent=--  L=N1  R=N3
  N1: B key=3   parent=N0  L=--  R=--
  N2: R key=15  parent=N3  L=--  R=--   <- recolored R
  N3: B key=20  parent=N0  L=N2  R=N4   <- recolored B
  N4: R key=25  parent=N3  L=--  R=--   <- inserted
```

Covers: GGP non-null, GP is right child of GGP (dir_r path,
fall-through at `jne r3, r8`).

Right-right, GP is left child of GGP (insert key=17):

```text
Before:
  Header: root=N0  top=N4  next=<end>
  N0: B key=20  parent=--  L=N1  R=N3
  N1: B key=10  parent=N0  L=--  R=N2
  N2: R key=15  parent=N1  L=--  R=--
  N3: B key=25  parent=N0  L=--  R=--

After:
  Header: root=N0  top=--  next=<end>
  N0: B key=20  parent=--  L=N2  R=N3
  N1: R key=10  parent=N2  L=--  R=--   <- recolored R
  N2: B key=15  parent=N0  L=N1  R=N4   <- recolored B
  N3: B key=25  parent=N0  L=--  R=--
  N4: R key=17  parent=N2  L=--  R=--   <- inserted
```

Covers: GGP non-null, GP is left child of GGP (dir_r path,
`insert_fixup_case_6_dir_r_left`).

### Case 2 + 6: non-null new_child in rotation

The existing case 6 tests have null `new_child` in the rotation
(parent has no child on the transferred side). These variants use
case 2 propagation to reach case 6 with a populated subtree, so the
`new_child` pointer is non-null and the reparenting branch fires
(`insert_fixup_case_6_dir_l_skip` / `dir_r_skip` fall-through).

Dir_l variant (insert key=1):

```text
Before:
  Header: root=N0  top=N7  next=<end>
  N0: B key=20  parent=--  L=N1  R=N3
  N1: R key=10  parent=N0  L=N2  R=N6
  N2: B key=5   parent=N1  L=N4  R=N5
  N3: B key=25  parent=N0  L=--  R=--
  N4: R key=3   parent=N2  L=--  R=--
  N5: R key=7   parent=N2  L=--  R=--
  N6: B key=15  parent=N1  L=--  R=--

After:
  Header: root=N1  top=--  next=<end>
  N0: R key=20  parent=N1  L=N6  R=N3   <- recolored R
  N1: B key=10  parent=--  L=N2  R=N0   <- recolored B, new root
  N2: R key=5   parent=N1  L=N4  R=N5   <- recolored R (case 2)
  N3: B key=25  parent=N0  L=--  R=--
  N4: B key=3   parent=N2  L=N7  R=--   <- recolored B (case 2)
  N5: B key=7   parent=N2  L=--  R=--   <- recolored B (case 2)
  N6: B key=15  parent=N0  L=--  R=--   <- reparented
  N7: R key=1   parent=N4  L=--  R=--   <- inserted
```

Path: insert at N4.L → case 2 (uncle N5 red) recolors
N4/N5/N2 → node=N2, parent=N1 → case 6 dir_l rotates N0
right with `new_child = N6` (non-null).

Dir_r variant (insert key=30):

```text
Before:
  Header: root=N0  top=N7  next=<end>
  N0: B key=5   parent=--  L=N1  R=N2
  N1: B key=3   parent=N0  L=--  R=--
  N2: R key=15  parent=N0  L=N3  R=N4
  N3: B key=10  parent=N2  L=--  R=--
  N4: B key=20  parent=N2  L=N5  R=N6
  N5: R key=17  parent=N4  L=--  R=--
  N6: R key=25  parent=N4  L=--  R=--

After:
  Header: root=N2  top=--  next=<end>
  N0: R key=5   parent=N2  L=N1  R=N3   <- recolored R
  N1: B key=3   parent=N0  L=--  R=--
  N2: B key=15  parent=--  L=N0  R=N4   <- recolored B, new root
  N3: B key=10  parent=N0  L=--  R=--   <- reparented
  N4: R key=20  parent=N2  L=N5  R=N6   <- recolored R (case 2)
  N5: B key=17  parent=N4  L=--  R=--   <- recolored B (case 2)
  N6: B key=25  parent=N4  L=--  R=N7   <- recolored B (case 2)
  N7: R key=30  parent=N6  L=--  R=--   <- inserted
```

Path: insert at N6.R → case 2 (uncle N5 red) recolors
N5/N6/N4 → node=N4, parent=N2 → case 6 dir_r rotates N0
left with `new_child = N3` (non-null).

### Case 2 + 5 + 6: non-null new_child in rotations

The existing case 5+6 tests have null `new_child` in both rotations.
These variants use case 2 propagation to reach case 5 with a
populated subtree, producing non-null `new_child` in both the case 5
and case 6 rotations (`insert_fixup_case_5_dir_l_skip` /
`dir_r_skip` and `insert_fixup_case_6_dir_l_skip` / `dir_r_skip`
fall-through).

Dir_l variant (insert key=11):

```text
Before:
  Header: root=N0  top=N7  next=<end>
  N0: B key=20  parent=--  L=N1  R=N4
  N1: R key=10  parent=N0  L=N2  R=N3
  N2: B key=5   parent=N1  L=--  R=--
  N3: B key=15  parent=N1  L=N5  R=N6
  N4: B key=25  parent=N0  L=--  R=--
  N5: R key=12  parent=N3  L=--  R=--
  N6: R key=17  parent=N3  L=--  R=--

After:
  Header: root=N3  top=--  next=<end>
  N0: R key=20  parent=N3  L=N6  R=N4   <- recolored R
  N1: R key=10  parent=N3  L=N2  R=N5   <- reparented
  N2: B key=5   parent=N1  L=--  R=--
  N3: B key=15  parent=--  L=N1  R=N0   <- recolored B, new root
  N4: B key=25  parent=N0  L=--  R=--
  N5: B key=12  parent=N1  L=N7  R=--   <- recolored B, reparented
  N6: B key=17  parent=N0  L=--  R=--   <- recolored B, reparented
  N7: R key=11  parent=N5  L=--  R=--   <- inserted
```

Path: insert at N5.L → case 2 (uncle N6 red) recolors
N5/N6/N3 → node=N3, parent=N1 → case 5 dir_l rotates N1
left with `new_child = N5` (non-null) → case 6 dir_l rotates
N0 right with `new_child = N6` (non-null).

Dir_r variant (insert key=18):

```text
Before:
  Header: root=N0  top=N7  next=<end>
  N0: B key=10  parent=--  L=N1  R=N2
  N1: B key=5   parent=N0  L=--  R=--
  N2: R key=20  parent=N0  L=N3  R=N4
  N3: B key=15  parent=N2  L=N5  R=N6
  N4: B key=25  parent=N2  L=--  R=--
  N5: R key=12  parent=N3  L=--  R=--
  N6: R key=17  parent=N3  L=--  R=--

After:
  Header: root=N3  top=--  next=<end>
  N0: R key=10  parent=N3  L=N1  R=N5   <- recolored R
  N1: B key=5   parent=N0  L=--  R=--
  N2: R key=20  parent=N3  L=N6  R=N4   <- reparented
  N3: B key=15  parent=--  L=N0  R=N2   <- recolored B, new root
  N4: B key=25  parent=N2  L=--  R=--
  N5: B key=12  parent=N0  L=--  R=--   <- recolored B, reparented
  N6: B key=17  parent=N2  L=--  R=N7   <- recolored B, reparented
  N7: R key=18  parent=N6  L=--  R=--   <- inserted
```

Path: insert at N6.R → case 2 (uncle N5 red) recolors
N5/N6/N3 → node=N3, parent=N2 → case 5 dir_r rotates N2
right with `new_child = N6` (non-null) → case 6 dir_r rotates
N0 left with `new_child = N5` (non-null).

## Multi-insert integration tests

A handful of sequential-insert tests to validate that chained
insertions produce correct trees. Process multiple insert instructions
in sequence, feeding each resulting account state into the next.

| Test        | Sequence                | Purpose          |
| ----------- | ----------------------- | ---------------- |
| 3-node      | 10, 5, 15               | Minimal balanced |
| Left-skew   | 10, 5, 1                | Right rotation   |
| Right-skew  | 10, 15, 20              | Left rotation    |
| Zigzag      | 10, 5, 7                | Double rotation  |
| 7-node full | 10, 5, 15, 3, 7, 12, 20 | Multiple rounds  |

## Multi-insert verification

Multi-insert integration tests also assert full state. After each
insert in the sequence, the expected complete tree layout is
specified. The test feeds the resulting account data from one
instruction into the next and asserts the full state at each step.

## Test helpers needed

- `build_tree(nodes: &[NodeSpec]) -> Vec<u8>` — Serialize a tree
  layout into account data with correct virtual address pointers
  and a free stack node for the new insertion.
- `assert_tree(account_data: &[u8], expected: &TreeSpec)` — Assert
  every field of the header and every field of every node against
  the expected specification. Panics with a diff on mismatch.
