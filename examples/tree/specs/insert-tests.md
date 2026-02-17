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

Left-left, black uncle variant (insert key=1):

```text
Before:
  Header: root=N0  top=N3  next=<end>
  N0: B key=10  parent=--  L=N1  R=N2
  N1: R key=5   parent=N0  L=--  R=--
  N2: B key=15  parent=N0  L=--  R=--

After:
  Header: root=N1  top=--  next=<end>
  N0: R key=10  parent=N1  L=--  R=N2   <- recolored R
  N1: B key=5   parent=--  L=N3  R=N0   <- recolored B, new root
  N2: B key=15  parent=N0  L=--  R=--
  N3: R key=1   parent=N1  L=--  R=--   <- inserted
```

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

Left-right, black uncle variant (insert key=7):

```text
Before:
  Header: root=N0  top=N3  next=<end>
  N0: B key=10  parent=--  L=N1  R=N2
  N1: R key=5   parent=N0  L=--  R=--
  N2: B key=15  parent=N0  L=--  R=--

After:
  Header: root=N3  top=--  next=<end>
  N0: R key=10  parent=N3  L=--  R=N2   <- recolored R
  N1: R key=5   parent=N3  L=--  R=--
  N2: B key=15  parent=N0  L=--  R=--
  N3: B key=7   parent=--  L=N1  R=N0   <- inserted, new root
```

Right-left variants: mirror of above (insert key=12, parent is
R(15), double rotation goes right then left).

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
