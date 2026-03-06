<!-- cspell:word cormen -->

<!-- cspell:word mehlhorn -->

<!-- cspell:word sedgewick -->

# Tree invariant specification

## Scope

Defines the structural invariants that every `TreeSpec` must
satisfy. These invariants are independent of the insert/remove
algorithm logic -- they follow directly from the definitions of a
binary search tree and a red-black tree. Test helpers in
`tests/common.rs` should verify these invariants on every
fabricated tree description (both before and after states) so that
specification errors surface as invariant violations rather than
silently producing incorrect test expectations.

## Binary search tree invariant

From Wikipedia:

> A binary search tree is a rooted binary tree in which nodes are
> arranged in strict total order in which the nodes with keys
> greater than any particular node A is stored on the right
> sub-trees to that node A and the nodes with keys equal to or
> less than A are stored on the left sub-trees to A, satisfying
> the binary search property.

### BST-1: ordering

For every node N in the tree:

- Every key in the left subtree of N is strictly less than N.key.
- Every key in the right subtree of N is strictly greater than
  N.key.

Note: the tree does not permit duplicate keys. Insert rejects
`KEY_EXISTS` and remove uses strict equality for lookup.

### BST-2: parent-child consistency

For every non-root node N:

- N.parent is non-null.
- N appears as exactly one of `N.parent.child[L]` or
  `N.parent.child[R]`.

For the root node:

- N.parent is null.
- `header.root` points to N.

For every node N with a non-null child C at position `child[d]`:

- C.parent equals N.

## Red-black tree invariants

From Wikipedia:

> In addition to the requirements imposed on a binary search tree
> the following must be satisfied by a red-black tree:
>
> 1. Every node is either red or black.
> 1. All null nodes are considered black.
> 1. A red node does not have a red child.
> 1. Every path from a given node to any of its leaf nodes (that
>    is, to any descendant null node) goes through the same number
>    of black nodes.
> 1. (Conclusion) If a node N has exactly one child, the child
>    must be red. If the child were black, its leaves would sit at
>    a different black depth than N's null node (which is
>    considered black by rule 2), violating requirement 4.

The invariants below restate these rules with labels used
throughout the test framework.

### RBT-1: valid coloring

Every node is either red (1) or black (0). No other values are
permitted. (Wikipedia rule 1.)

### RBT-2: null nodes are black

All null child pointers are considered black. This is a
definitional rule used by the other invariants and does not
require an explicit check -- null is implicitly black in the
checks below. (Wikipedia rule 2.)

### RBT-3: no red-red

A red node does not have a red child. Equivalently: if a node is
red, both of its children (if non-null) must be black. (Wikipedia
rule 3.)

### RBT-4: uniform black depth

Every path from a given node to any of its descendant null nodes
passes through the same number of black nodes. This count
(excluding the node itself, including null as black) is the
node's _black height_. (Wikipedia rule 4.)

### RBT-5: root is black (omitted)

From Wikipedia:

> Some authors, e.g. Cormen & al., claim "the root is black" as
> fifth requirement; but not Mehlhorn & Sanders or Sedgewick &
> Wayne. Since the root can always be changed from red to black,
> this rule has little effect on analysis. This article also omits
> it, because it slightly disturbs the recursive algorithms and
> proofs.

This invariant is **not checked** by `assert_invariants`. The
insert algorithm in this codebase follows the Wikipedia
formulation which omits the root-is-black rule. A red root is
valid: it does not violate any of the four core rules (RBT-1
through RBT-4) and does not affect the structural correctness of
the tree.

### RBT-C: one-child corollary

If a node N has exactly one child, that child must be red. If
the child were black, its null leaves would sit at a different
black depth than N's null child (which is black by RBT-2),
violating RBT-4. (Wikipedia rule 5 / conclusion.)

This is a derived property, not an independent axiom, but
checking it explicitly produces clearer error messages than
detecting the black-height mismatch.

## Free stack invariants

Freed nodes are linked through `StackNode.next` at offset 0
(overlapping `TreeNode.parent`). Free stack nodes are not part of
the tree and must not be visited during BST/RBT traversal.

### FS-1: stack top

If `header.top` is non-null, it points to a valid node slot
within the account data.

### FS-2: no overlap with tree

No node reachable from `header.root` via parent/child pointers
appears on the free stack, and vice versa. The set of tree nodes
and the set of free stack nodes are disjoint.

## Verification approach

### Where to check

Add an `assert_invariants(desc: &TreeSpec)` function in
`tests/common.rs`. This operates purely on the `TreeSpec`
description (node indices, not virtual addresses), making it
independent of account serialization.

Call `assert_invariants` on:

- Every `desc` (before state) passed to `run_remove_success`,
  `run_insert_success`, and similar helpers.
- Every `exp` (expected after state) passed to these helpers.
- Every intermediate tree state in multi-step tests.

This catches spec-level errors at test construction time,
before any program code executes.

### What to check

The function should verify, given a `TreeSpec`:

1. **BST-1 (ordering):** In-order traversal of the tree (starting
   from `root`, following `left`/`right` indices) yields strictly
   increasing keys.
1. **BST-2 (parent-child consistency):** For each node, verify
   that the parent index matches (root has no parent; non-root
   nodes appear in their parent's child list). For each child
   pointer, verify the child's parent points back.
1. **RBT-1 (valid coloring):** Every node's color is `B` or `R`.
1. **RBT-3 (no red-red):** If a node is red, its children (if
   present) are black.
1. **RBT-4 (uniform black depth):** Compute the black height of
   each node recursively. All paths from the root to null must
   yield the same count.
1. **RBT-C (one-child corollary):** If a node has exactly one
   child, that child is red.
1. **FS-1 (stack top):** If `top` is `Some(i)`, then `i` is a
   valid node index.
1. **FS-2 (disjoint sets):** Collect the set of node indices
   reachable from root. Collect the set of node indices reachable
   via the free stack (`top`, then following `parent` as
   `StackNode.next`). Verify the two sets are disjoint and their
   union covers all node indices in the buffer.

### Freed nodes

Freed nodes on the stack have null children (cleared by remove)
and use the `parent` field as `StackNode.next`. These nodes
must not be traversed as part of the BST/RBT checks. The
`TreeSpec` already distinguishes freed nodes implicitly: they
are reachable from `top` but not from `root`.

To identify freed nodes from a `TreeSpec`:

- Walk the free stack starting from `top`, following `parent` as
  the next pointer, until null.
- All remaining nodes (reachable from `root`) are tree nodes and
  must satisfy BST and RBT invariants.

### Error reporting

`assert_invariants` should return `Result<(), String>` with a
descriptive message identifying which invariant failed and which
node(s) are involved. Example:

```text
RBT-3 (no red-red): N2 (RED) has red child N4 at child[R]
RBT-4 (uniform black depth): root black height L=3, R=2
BST-1 (ordering): N3.key=15 in left subtree of N0.key=10
```

### Interaction with existing tests

The invariant checks run on `TreeSpec` descriptions, not on
program output. This makes them orthogonal to the program logic:

- If a test's **before** state fails invariants, the test setup
  is invalid and should be fixed in the test spec.
- If a test's **after** state fails invariants, the test
  expectation is wrong and should be traced against the reference
  algorithm.
- If the program produces output that passes `assert_tree_account`
  but the expected `TreeSpec` fails invariants, the spec has a
  bug.

This separation means invariant checking does not depend on or
validate program correctness -- it validates the test
specifications themselves.
