<!-- cspell:word inlines -->

<!-- cspell:word reparent -->

# Insert fixup rotation specification

## Scope

Assembly implementation of cases 5 and 6 in the insert fixup loop,
covering the four labels: `insert_fixup_case_5_dir_l`,
`insert_fixup_case_6_dir_l`, `insert_fixup_case_5_dir_r`,
`insert_fixup_case_6_dir_r`. Each label inlines the `rotate_subtree`
call from `program.rs` with the direction hardcoded.

## Conventions

- `dir_l`: parent is the LEFT child of grandparent (dir = LEFT).
- `dir_r`: parent is the RIGHT child of grandparent (dir = RIGHT).
- Case 5 rotates parent in dir, then reassigns node/parent.
- Case 6 rotates grandparent in opposite(dir), recolors, exits.

## Register contract

### Live registers at entry to case 5/6

| Reg | Value        | Source                               |
| --- | ------------ | ------------------------------------ |
| r1  | input buffer | entrypoint, never modified           |
| r2  | parent       | insert_search / case 5 reassignment  |
| r3  | grandparent  | insert_fixup_check_case_4 (non-null) |
| r9  | node         | insert_store_key_value_pair / case 5 |

### Scratch registers

Available for rotation temporaries. These do not need to be
preserved because cases 5/6 always exit with SUCCESS and never
reach `insert_fixup_case_2` or loop back to `insert_fixup_main`.

| Reg | Prior value     | Use in rotation                 |
| --- | --------------- | ------------------------------- |
| r4  | grandparent.key | great-grandparent (case 6 only) |
| r5  | parent.key      | free                            |
| r6  | parent.color    | new_root (case 5 only)          |
| r7  | uncle           | free                            |
| r8  | check scratch   | new_child / cmp scratch         |

## Control flow

```text
case_5_dir_l -> (fall through) -> case_6_dir_l -> exit
case_5_dir_r -> (fall through) -> case_6_dir_r -> exit

(or, skipping case 5:)
case_6_dir_l -> exit
case_6_dir_r -> exit
```

## Rotation algorithm

Reference (`program.rs:575`):

```rust
let parent = (*subtree).parent;
let new_root = (*subtree).child[opposite(direction)];
let new_child = (*new_root).child[direction];

(*subtree).child[opposite(direction)] = new_child;
if !new_child.is_null() { (*new_child).parent = subtree; }

(*new_root).child[direction] = subtree;
(*new_root).parent = parent;
(*subtree).parent = new_root;

if !parent.is_null() {
    (*parent).child[(subtree == (*parent).child[DIR_R]) as usize] = new_root;
} else {
    (*tree).root = new_root;
}
```

## Case 5, dir_l — rotate(parent, LEFT)

Subtree = parent (r2). Parent-of-subtree = grandparent (r3,
non-null per case 4 check). Direction = LEFT, opposite = RIGHT.

```text
r6 = parent.child[RIGHT]                  # new_root
r8 = r6.child[LEFT]                       # new_child

parent.child[RIGHT]  = r8                 # detach new_child
if r8 != null: r8.parent = parent         # reparent new_child

r6.child[LEFT]  = parent                  # new_root adopts subtree
r6.parent       = grandparent             # new_root links up
parent.parent   = r6                      # subtree links to new_root

grandparent.child[LEFT] = r6              # parent is LEFT child, hardcoded

# Rust post-rotation:
r9 = r2                                   # node = old parent
r2 = r6                                   # parent = new_root
# fall through to case_6_dir_l
```

## Case 6, dir_l — rotate(grandparent, RIGHT)

Subtree = grandparent (r3). Direction = RIGHT, opposite = LEFT.
New_root = grandparent.child[LEFT] = parent = r2 (already in
register).

```text
r4 = grandparent.parent                   # great-grandparent
r8 = r2.child[RIGHT]                      # new_child

grandparent.child[LEFT] = r8              # detach new_child
if r8 != null: r8.parent = grandparent    # reparent new_child

r2.child[RIGHT]     = grandparent         # parent adopts grandparent
r2.parent           = r4                  # parent links up
grandparent.parent  = r2                  # grandparent links to parent

if r4 != null:
    r8 = r4.child[RIGHT]
    if grandparent == r8:
        r4.child[RIGHT] = r2
    else:
        r4.child[LEFT]  = r2
else:
    tree.root = r2                        # [r1 + IB_TREE_DATA_ROOT_OFF]

# Recolor and exit:
parent.color      = BLACK
grandparent.color = RED
exit
```

## Case 5, dir_r — rotate(parent, RIGHT)

Mirror of case 5 dir_l. Subtree = parent (r2). Direction = RIGHT,
opposite = LEFT.

```text
r6 = parent.child[LEFT]                   # new_root
r8 = r6.child[RIGHT]                      # new_child

parent.child[LEFT]   = r8                 # detach new_child
if r8 != null: r8.parent = parent         # reparent new_child

r6.child[RIGHT] = parent                  # new_root adopts subtree
r6.parent        = grandparent            # new_root links up
parent.parent    = r6                     # subtree links to new_root

grandparent.child[RIGHT] = r6             # parent is RIGHT child, hardcoded

# Rust post-rotation:
r9 = r2                                   # node = old parent
r2 = r6                                   # parent = new_root
# fall through to case_6_dir_r
```

## Case 6, dir_r — rotate(grandparent, LEFT)

Mirror of case 6 dir_l. Subtree = grandparent (r3). Direction =
LEFT, opposite = RIGHT. New_root = grandparent.child[RIGHT] =
parent = r2.

```text
r4 = grandparent.parent                   # great-grandparent
r8 = r2.child[LEFT]                       # new_child

grandparent.child[RIGHT] = r8             # detach new_child
if r8 != null: r8.parent = grandparent    # reparent new_child

r2.child[LEFT]      = grandparent         # parent adopts grandparent
r2.parent           = r4                  # parent links up
grandparent.parent  = r2                  # grandparent links to parent

if r4 != null:
    r8 = r4.child[RIGHT]
    if grandparent == r8:
        r4.child[RIGHT] = r2
    else:
        r4.child[LEFT]  = r2
else:
    tree.root = r2                        # [r1 + IB_TREE_DATA_ROOT_OFF]

# Recolor and exit:
parent.color      = BLACK
grandparent.color = RED
exit
```

## Key observations

- Case 5 never needs a null/root check because grandparent is
  guaranteed non-null by the case 4 check. The direction of
  grandparent's child update is hardcoded (LEFT for dir_l, RIGHT
  for dir_r).
- Case 6 needs the full null check because great-grandparent can
  be null (grandparent may be root). The great-grandparent child
  update cannot be hardcoded by direction — it requires a pointer
  comparison.
- In case 6, new_root is always parent (r2), avoiding an extra
  load.
- The dir_l/dir_r variants are exact mirrors: every CHILD_L swaps
  with CHILD_R.
