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
