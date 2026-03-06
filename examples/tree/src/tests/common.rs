// cspell:word inorder
// cspell:word vaddr
use super::*;
use tree_interface::{input_buffer, tree, Color, StackNode, TreeHeader, TreeNode};

// ---------------------------------------------------------------------------
// Helpers: tree description types
// ---------------------------------------------------------------------------

pub(super) struct NodeSpec {
    pub key: u16,
    pub value: u16,
    pub color: u8,
    pub parent: Option<usize>,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

impl NodeSpec {
    pub fn val(mut self, v: u16) -> Self {
        self.value = v;
        self
    }
}

pub(super) struct TreeSpec<'a> {
    pub root: Option<usize>,
    pub top: Option<usize>,
    pub nodes: &'a [NodeSpec],
}

/// Compute the virtual address of node slot `i` in the tree account.
pub(super) fn node_vaddr(i: usize) -> u64 {
    MM_INPUT_START
        + input_buffer::TREE_DATA_OFF as u64
        + size_of::<TreeHeader>() as u64
        + (i as u64) * (size_of::<TreeNode>() as u64)
}

/// Convert an optional node index to a virtual address (0 for None).
pub(super) fn opt_vaddr(idx: Option<usize>) -> u64 {
    match idx {
        Some(i) => node_vaddr(i),
        None => 0,
    }
}

// ---------------------------------------------------------------------------
// Helper: build tree account data
// ---------------------------------------------------------------------------

/// Build tree account data with pre-existing nodes and one free StackNode.
///
/// Memory layout: TreeHeader | node[0] | node[1] | ... | node[N-1] | free_slot
///
/// - `header.root` → virtual address of `nodes[root]`, or null.
/// - `header.top`  → virtual address of the free slot (index = nodes.len()).
/// - `header.next` → 0 (unused in skip-alloc path).
pub(super) fn build_tree_account(desc: &TreeSpec, program_id: &Pubkey) -> (Pubkey, Account) {
    let n = desc.nodes.len();
    // N existing nodes + 1 free slot.
    let data_len = size_of::<TreeHeader>() + (n + 1) * size_of::<TreeNode>();
    let mut data = vec![0u8; data_len];

    // Write header.
    let header = data.as_mut_ptr() as *mut TreeHeader;
    unsafe {
        (*header).root = opt_vaddr(desc.root) as *mut TreeNode;
        (*header).top = node_vaddr(n) as *mut StackNode;
        (*header).next = core::ptr::null_mut();
    }

    // Write existing nodes.
    write_nodes(&mut data, desc.nodes);

    // Free slot is already zeroed (StackNode.next = null).

    let pubkey = Pubkey::new_unique();
    let mut account = Account::new(0, data_len, program_id);
    account.data = data;
    (pubkey, account)
}

/// Build tree account data with pre-existing nodes and no free slot.
///
/// Memory layout: TreeHeader | node[0] | node[1] | ... | node[N-1]
///
/// - `header.root` → virtual address of `nodes[root]`, or null.
/// - `header.top`  → null (no pre-existing free nodes).
/// - `header.next` → null (unused).
pub(super) fn build_tree_account_no_free(
    desc: &TreeSpec,
    program_id: &Pubkey,
) -> (Pubkey, Account) {
    let n = desc.nodes.len();
    let data_len = size_of::<TreeHeader>() + n * size_of::<TreeNode>();
    let mut data = vec![0u8; data_len];

    let header = data.as_mut_ptr() as *mut TreeHeader;
    unsafe {
        (*header).root = opt_vaddr(desc.root) as *mut TreeNode;
        (*header).top = core::ptr::null_mut();
        (*header).next = core::ptr::null_mut();
    }

    write_nodes(&mut data, desc.nodes);

    let pubkey = Pubkey::new_unique();
    let mut account = Account::new(0, data_len, program_id);
    account.data = data;
    (pubkey, account)
}

fn write_nodes(data: &mut [u8], nodes: &[NodeSpec]) {
    for (i, node) in nodes.iter().enumerate() {
        let offset = size_of::<TreeHeader>() + i * size_of::<TreeNode>();
        let ptr = unsafe { data.as_mut_ptr().add(offset) as *mut TreeNode };
        unsafe {
            (*ptr).parent = opt_vaddr(node.parent) as *mut TreeNode;
            (*ptr).child[tree::DIR_L] = opt_vaddr(node.left) as *mut TreeNode;
            (*ptr).child[tree::DIR_R] = opt_vaddr(node.right) as *mut TreeNode;
            (*ptr).key = node.key;
            (*ptr).value = node.value;
            (*ptr).color = core::mem::transmute(node.color);
        }
    }
}

// ---------------------------------------------------------------------------
// Helper: assert tree account (full state)
// ---------------------------------------------------------------------------

/// Assert every field of the tree account data against expected state.
/// Returns Ok(()) on match, Err(description) on mismatch.
pub(super) fn assert_tree_account(data: &[u8], expected: &TreeSpec) -> Result<(), String> {
    let mut errors = Vec::new();
    let n = expected.nodes.len();

    // Check data length (at least enough for the expected nodes).
    let min_len = size_of::<TreeHeader>() + n * size_of::<TreeNode>();
    if data.len() < min_len {
        errors.push(format!(
            "data len: expected at least {}, got {}",
            min_len,
            data.len()
        ));
    }

    // Check header.
    let header = data.as_ptr() as *const TreeHeader;
    unsafe {
        let root_addr = (*header).root as u64;
        let expected_root = opt_vaddr(expected.root);
        if root_addr != expected_root {
            errors.push(format!(
                "header.root: expected {:#x}, got {:#x}",
                expected_root, root_addr
            ));
        }

        let top_addr = (*header).top as u64;
        let expected_top = opt_vaddr(expected.top);
        if top_addr != expected_top {
            errors.push(format!(
                "header.top: expected {:#x}, got {:#x}",
                expected_top, top_addr
            ));
        }

        let next_addr = (*header).next as u64;
        if next_addr != 0 {
            errors.push(format!("header.next: expected 0x0, got {:#x}", next_addr));
        }
    }

    // Check each node.
    for i in 0..n {
        let offset = size_of::<TreeHeader>() + i * size_of::<TreeNode>();
        if offset + size_of::<TreeNode>() > data.len() {
            errors.push(format!("N{}: out of bounds", i));
            continue;
        }
        let ptr = unsafe { data.as_ptr().add(offset) as *const TreeNode };
        let exp = &expected.nodes[i];
        let label = format!("N{}", i);

        unsafe {
            let parent_addr = core::ptr::read_unaligned(core::ptr::addr_of!((*ptr).parent)) as u64;
            let expected_parent = opt_vaddr(exp.parent);
            if parent_addr != expected_parent {
                errors.push(format!(
                    "{}.parent: expected {:#x}, got {:#x}",
                    label, expected_parent, parent_addr
                ));
            }

            let left_addr =
                core::ptr::read_unaligned(core::ptr::addr_of!((*ptr).child[tree::DIR_L])) as u64;
            let expected_left = opt_vaddr(exp.left);
            if left_addr != expected_left {
                errors.push(format!(
                    "{}.L: expected {:#x}, got {:#x}",
                    label, expected_left, left_addr
                ));
            }

            let right_addr =
                core::ptr::read_unaligned(core::ptr::addr_of!((*ptr).child[tree::DIR_R])) as u64;
            let expected_right = opt_vaddr(exp.right);
            if right_addr != expected_right {
                errors.push(format!(
                    "{}.R: expected {:#x}, got {:#x}",
                    label, expected_right, right_addr
                ));
            }

            let key = core::ptr::read_unaligned(core::ptr::addr_of!((*ptr).key));
            if key != exp.key {
                errors.push(format!("{}.key: expected {}, got {}", label, exp.key, key));
            }

            let value = core::ptr::read_unaligned(core::ptr::addr_of!((*ptr).value));
            if value != exp.value {
                errors.push(format!(
                    "{}.value: expected {}, got {}",
                    label, exp.value, value
                ));
            }

            let color = core::ptr::read_unaligned(core::ptr::addr_of!((*ptr).color)) as u8;
            if color != exp.color {
                let color_name = |c: u8| if c == 0 { "B" } else { "R" };
                errors.push(format!(
                    "{}.color: expected {}, got {}",
                    label,
                    color_name(exp.color),
                    color_name(color)
                ));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("; "))
    }
}

// ---------------------------------------------------------------------------
// Invariant checker (operates on TreeSpec, not account data)
// ---------------------------------------------------------------------------

/// Check all structural invariants of a `TreeSpec`.
///
/// Verifies BST ordering, parent-child consistency, RBT coloring rules,
/// and free stack validity. Returns `Ok(())` if all invariants hold,
/// or `Err` with descriptions of all violations found.
///
/// See `specs/tree-invariants.md` for the full specification.
pub(super) fn assert_invariants(desc: &TreeSpec) -> Result<(), String> {
    let mut errors = Vec::new();
    let n = desc.nodes.len();

    // --- Collect tree nodes (reachable from root via child pointers) ---
    let mut tree_set = vec![false; n];
    if let Some(root) = desc.root {
        if root >= n {
            errors.push(format!("root index {} out of bounds (n={})", root, n));
        } else {
            collect_reachable(desc.nodes, root, &mut tree_set, &mut errors);
        }
    }

    // --- Collect free stack nodes ---
    let mut stack_set = vec![false; n];
    {
        let mut cur = desc.top;
        while let Some(i) = cur {
            if i >= n {
                break; // Slot beyond described nodes (pre-allocated).
            }
            if stack_set[i] {
                errors.push(format!("FS: cycle in free stack at N{}", i));
                break;
            }
            stack_set[i] = true;
            cur = desc.nodes[i].parent; // parent field = StackNode.next
        }
    }

    // --- FS-2: disjoint sets and coverage ---
    for i in 0..n {
        if tree_set[i] && stack_set[i] {
            errors.push(format!(
                "FS-2 (disjoint): N{} in both tree and free stack",
                i
            ));
        }
        if !tree_set[i] && !stack_set[i] {
            errors.push(format!(
                "FS-2 (coverage): N{} neither in tree nor on free stack",
                i
            ));
        }
    }

    // --- BST and RBT checks (only on tree nodes) ---
    if let Some(root) = desc.root {
        if root < n {
            // BST-2: root has no parent.
            if desc.nodes[root].parent.is_some() {
                errors.push(format!(
                    "BST-2: root N{} has parent={:?}",
                    root, desc.nodes[root].parent
                ));
            }
        }

        for i in 0..n {
            if !tree_set[i] {
                continue;
            }
            let nd = &desc.nodes[i];

            // RBT-1: valid coloring.
            if nd.color != B && nd.color != R {
                errors.push(format!("RBT-1 (valid coloring): N{} color={}", i, nd.color));
            }

            // BST-2: parent-child consistency (non-root).
            if i != root {
                match nd.parent {
                    None => errors.push(format!("BST-2: non-root N{} has no parent", i)),
                    Some(p) => {
                        if p >= n {
                            errors.push(format!("BST-2: N{}.parent={} out of bounds", i, p));
                        } else {
                            let par = &desc.nodes[p];
                            if par.left != Some(i) && par.right != Some(i) {
                                errors.push(format!(
                                    "BST-2: N{}.parent=N{} but N{} has L={:?} R={:?}",
                                    i, p, p, par.left, par.right
                                ));
                            }
                        }
                    }
                }
            }

            // BST-2: children point back.
            if let Some(l) = nd.left {
                if l < n && desc.nodes[l].parent != Some(i) {
                    errors.push(format!(
                        "BST-2: N{}.L=N{} but N{}.parent={:?}",
                        i, l, l, desc.nodes[l].parent
                    ));
                }
            }
            if let Some(r) = nd.right {
                if r < n && desc.nodes[r].parent != Some(i) {
                    errors.push(format!(
                        "BST-2: N{}.R=N{} but N{}.parent={:?}",
                        i, r, r, desc.nodes[r].parent
                    ));
                }
            }

            // RBT-3: no red-red.
            if nd.color == R {
                if let Some(l) = nd.left {
                    if l < n && desc.nodes[l].color == R {
                        errors.push(format!(
                            "RBT-3 (no red-red): N{} (RED) has red child N{} at L",
                            i, l
                        ));
                    }
                }
                if let Some(r) = nd.right {
                    if r < n && desc.nodes[r].color == R {
                        errors.push(format!(
                            "RBT-3 (no red-red): N{} (RED) has red child N{} at R",
                            i, r
                        ));
                    }
                }
            }

            // RBT-C: one-child corollary.
            let has_l = nd.left.is_some();
            let has_r = nd.right.is_some();
            if has_l != has_r {
                let child = nd.left.or(nd.right).unwrap();
                if child < n && desc.nodes[child].color != R {
                    errors.push(format!(
                        "RBT-C (one-child): N{} has one child N{} which is BLACK",
                        i, child
                    ));
                }
            }
        }

        // BST-1: in-order traversal yields strictly increasing keys.
        if root < n {
            let mut keys = Vec::new();
            inorder_keys(desc.nodes, root, &mut keys);
            for w in keys.windows(2) {
                if w[0] >= w[1] {
                    errors.push(format!(
                        "BST-1 (ordering): keys not strictly increasing: {} >= {}",
                        w[0], w[1]
                    ));
                    break;
                }
            }

            // RBT-4: uniform black depth.
            if let Err(e) = black_height(desc.nodes, root) {
                errors.push(e);
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("; "))
    }
}

/// Walk tree from `idx` via child pointers, marking visited nodes.
fn collect_reachable(
    nodes: &[NodeSpec],
    idx: usize,
    visited: &mut Vec<bool>,
    errors: &mut Vec<String>,
) {
    if idx >= nodes.len() {
        errors.push(format!(
            "child index {} out of bounds (n={})",
            idx,
            nodes.len()
        ));
        return;
    }
    if visited[idx] {
        errors.push(format!("cycle in tree at N{}", idx));
        return;
    }
    visited[idx] = true;
    if let Some(l) = nodes[idx].left {
        collect_reachable(nodes, l, visited, errors);
    }
    if let Some(r) = nodes[idx].right {
        collect_reachable(nodes, r, visited, errors);
    }
}

/// Collect keys via in-order traversal.
fn inorder_keys(nodes: &[NodeSpec], idx: usize, keys: &mut Vec<u16>) {
    if let Some(l) = nodes[idx].left {
        if l < nodes.len() {
            inorder_keys(nodes, l, keys);
        }
    }
    keys.push(nodes[idx].key);
    if let Some(r) = nodes[idx].right {
        if r < nodes.len() {
            inorder_keys(nodes, r, keys);
        }
    }
}

/// Compute the black height of a subtree. Returns `Err` if any node has
/// unequal left/right black heights (RBT-4 violation).
fn black_height(nodes: &[NodeSpec], idx: usize) -> Result<usize, String> {
    let nd = &nodes[idx];
    let lbh = match nd.left {
        Some(l) if l < nodes.len() => black_height(nodes, l)?,
        _ => 0,
    };
    let rbh = match nd.right {
        Some(r) if r < nodes.len() => black_height(nodes, r)?,
        _ => 0,
    };
    if lbh != rbh {
        return Err(format!(
            "RBT-4 (uniform black depth): N{} (key={}) L black height={}, R black height={}",
            idx, nd.key, lbh, rbh
        ));
    }
    Ok(lbh + if nd.color == B { 1 } else { 0 })
}

// ---------------------------------------------------------------------------
// Shorthand constructors
// ---------------------------------------------------------------------------

pub(super) const B: u8 = Color::Black as u8;
pub(super) const R: u8 = Color::Red as u8;

pub(super) fn node(
    key: u16,
    color: u8,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
) -> NodeSpec {
    NodeSpec {
        key,
        value: key,
        color,
        parent,
        left,
        right,
    }
}

// ---------------------------------------------------------------------------
// Helper: build empty tree with pre-allocated free slots
// ---------------------------------------------------------------------------

/// Build an empty tree account with `n` pre-allocated free slots.
pub(super) fn build_empty_tree(n: usize, program_id: &Pubkey) -> (Pubkey, Account) {
    let data_len = size_of::<TreeHeader>() + n * size_of::<TreeNode>();
    let mut data = vec![0u8; data_len];

    let header = data.as_mut_ptr() as *mut TreeHeader;
    unsafe {
        (*header).root = core::ptr::null_mut();
        (*header).top = if n > 0 {
            node_vaddr(0) as *mut StackNode
        } else {
            core::ptr::null_mut()
        };
        (*header).next = core::ptr::null_mut();
    }

    // Link free slots into a singly-linked list.
    for i in 0..n {
        let offset = size_of::<TreeHeader>() + i * size_of::<TreeNode>();
        let slot = unsafe { data.as_mut_ptr().add(offset) as *mut StackNode };
        unsafe {
            (*slot).next = if i + 1 < n {
                node_vaddr(i + 1) as *mut StackNode
            } else {
                core::ptr::null_mut()
            };
        }
    }

    let pubkey = Pubkey::new_unique();
    let mut account = Account::new(0, data_len, program_id);
    account.data = data;
    (pubkey, account)
}
