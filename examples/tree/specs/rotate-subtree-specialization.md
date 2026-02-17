# Rotate subtree specialization

## Scope

Inline the two `rotate_subtree` calls in the insert fixup case 5/6
block of `program.rs` with specialized logic that exploits
invariants known at each call site. This matches the assembly,
where each rotation is expanded inline with hardcoded directions.
The generic `rotate_subtree` is preserved for potential use by
delete or other operations.

## Convention

`dir` is "the direction of parent relative to grandparent",
computed by `direction(parent)`. All comments reference `dir` and
`opposite(dir)` rather than concrete LEFT/RIGHT.

## Case 5: rotate(parent, dir)

Replace `rotate_subtree(tree_header, parent, dir)` with:

```rust
// Case 5: rotate parent in dir.
//
// Grandparent is guaranteed non-null by the case 4 check, so
// no root-replacement path is needed. Parent is known to be
// grandparent.child[dir] from the direction() call, so the
// child pointer update is hardcoded without comparison.
{
    let new_root = (*parent).child[opposite(dir)];
    let new_child = (*new_root).child[dir];

    (*parent).child[opposite(dir)] = new_child;
    if !new_child.is_null() {
        (*new_child).parent = parent;
    }

    (*new_root).child[dir] = parent;
    (*new_root).parent = grandparent;
    (*parent).parent = new_root;

    (*grandparent).child[dir] = new_root;

    node = parent;
    parent = new_root;
}
```

Optimizations vs generic `rotate_subtree`:

- **No null check on grandparent**: Case 4 already checked and
  returned if grandparent were null.
- **No pointer comparison for child update**: Parent is
  `grandparent.child[dir]` by definition, so
  `grandparent.child[dir] = new_root` is correct without
  comparing `subtree == parent.child[DIR_R]`.
- **No `tree` parameter needed**: The root is never updated.

The `parent = (*grandparent).child[dir]` load from the original
code is replaced by `parent = new_root`, which is the same pointer
(the rotation placed new_root at `grandparent.child[dir]`).

## Case 6: rotate(grandparent, opposite(dir))

Replace `rotate_subtree(tree_header, grandparent, opposite(dir))`
with:

```rust
// Case 6: rotate grandparent in opposite(dir).
//
// The new root of this rotation is parent
// (= grandparent.child[dir]), which the caller already has,
// eliminating the generic version's load of
// subtree.child[opposite(direction)].
//
// Great-grandparent may be null (grandparent could be root),
// so the null check and root-replacement path are retained.
// Grandparent's position under great-grandparent is unrelated
// to dir, so the pointer comparison is also retained.
{
    let great_grandparent = (*grandparent).parent;
    let new_child = (*parent).child[opposite(dir)];

    (*grandparent).child[dir] = new_child;
    if !new_child.is_null() {
        (*new_child).parent = grandparent;
    }

    (*parent).child[opposite(dir)] = grandparent;
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
```

Optimizations vs generic `rotate_subtree`:

- **No new_root load**: The generic version loads
  `subtree.child[opposite(direction)]`. Here, new_root is
  `parent`, already in scope.
- **Great-grandparent null check retained**: Grandparent may be
  root.
- **Pointer comparison retained**: `dir` describes parent's
  position under grandparent, not grandparent's position under
  great-grandparent.

## Resulting caller

```rust
if uncle.is_null() || (*uncle).color == Color::Black {
    // Case 5.
    if node == (*parent).child[opposite(dir)] {
        // <case 5 rotation inlined here>
    }

    // Case 6.
    // <case 6 rotation inlined here>

    (*parent).color = Color::Black;
    (*grandparent).color = Color::Red;
    return SUCCESS;
}
```
