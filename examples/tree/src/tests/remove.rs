use super::common::*;
use super::*;
use tree_interface::{
    input_buffer, InsertInstruction, Instruction as TreeInstruction,
    InstructionHeader, RemoveInstruction, StackNode, TreeHeader, TreeNode,
};

// ---------------------------------------------------------------------------
// Helpers: remove test setup
// ---------------------------------------------------------------------------

fn remove_setup(
    lang: ProgramLanguage,
    desc: &TreeSpec,
    remove_key: u16,
) -> (TestSetup, Instruction, Vec<(Pubkey, Account)>) {
    let setup = setup_test(lang);
    let (system_program_pubkey, _) = program::keyed_account_for_system_program();

    let user_pubkey = Pubkey::new_unique();
    let (tree_pubkey, tree_account) = build_tree_account_no_free(desc, &setup.program_id);

    let insn_data = RemoveInstruction {
        header: InstructionHeader {
            discriminator: TreeInstruction::Remove as u8,
        },
        key: remove_key,
    };

    let instruction = Instruction::new_with_bytes(
        setup.program_id,
        unsafe { as_bytes(&insn_data) },
        vec![
            AccountMeta::new(user_pubkey, true),
            AccountMeta::new(tree_pubkey, false),
        ],
    );

    let accounts = vec![
        (
            user_pubkey,
            Account::new(USER_LAMPORTS, 0, &system_program_pubkey),
        ),
        (tree_pubkey, tree_account),
    ];

    (setup, instruction, accounts)
}

/// A minimal two-account setup for input check tests.
fn remove_input_setup(
    lang: ProgramLanguage,
) -> (TestSetup, Instruction, Vec<(Pubkey, Account)>) {
    let desc = TreeSpec {
        root: Some(0),
        top: None,
        nodes: &[node(10, B, None, None, None)],
    };
    remove_setup(lang, &desc, 10)
}

// ---------------------------------------------------------------------------
// Helpers: remove runners
// ---------------------------------------------------------------------------

/// Execute a remove and verify success with full tree state.
///
/// On success, the program returns 0. Account changes are only persisted
/// by the SVM when the program returns 0.
fn run_remove_success(
    lang: ProgramLanguage,
    desc: &TreeSpec,
    remove_key: u16,
    expected: &TreeSpec,
) -> CaseResult {
    if let Err(e) = assert_invariants(desc) {
        return CaseResult { cu: 0, error: Some(format!("desc invariant: {}", e)) };
    }
    if let Err(e) = assert_invariants(expected) {
        return CaseResult { cu: 0, error: Some(format!("exp invariant: {}", e)) };
    }
    let (setup, instruction, accounts) = remove_setup(lang, desc, remove_key);
    let result = setup.mollusk.process_instruction(&instruction, &accounts);
    match &result.program_result {
        MolluskResult::Success => {
            let tree_data = &result.resulting_accounts[AccountIndex::Tree as usize]
                .1
                .data;
            match assert_tree_account(tree_data, expected) {
                Ok(()) => CaseResult {
                    cu: result.compute_units_consumed,
                    error: None,
                },
                Err(e) => CaseResult {
                    cu: result.compute_units_consumed,
                    error: Some(e),
                },
            }
        }
        other => CaseResult {
            cu: result.compute_units_consumed,
            error: Some(format!("expected Success, got {:?}", other)),
        },
    }
}

/// Execute a remove and verify KEY_DOES_NOT_EXIST error.
fn run_remove_not_found(
    lang: ProgramLanguage,
    desc: &TreeSpec,
    remove_key: u16,
) -> CaseResult {
    if let Err(e) = assert_invariants(desc) {
        return CaseResult { cu: 0, error: Some(format!("desc invariant: {}", e)) };
    }
    let (setup, instruction, accounts) = remove_setup(lang, desc, remove_key);
    check_error(
        &setup,
        &instruction,
        &accounts,
        error_codes::error::KEY_DOES_NOT_EXIST,
    )
}

// ---------------------------------------------------------------------------
// Helpers: multi-step operations
// ---------------------------------------------------------------------------

enum MultiStep {
    Insert { key: u16, value: u16 },
    Remove { key: u16 },
}

struct MultiStepCase<'a> {
    step: MultiStep,
    expected: TreeSpec<'a>,
}

fn run_multi_step(lang: ProgramLanguage, n_slots: usize, steps: &[MultiStepCase]) -> CaseResult {
    let setup = setup_test(lang);
    let (system_program_pubkey, _) = program::keyed_account_for_system_program();

    let user_pubkey = Pubkey::new_unique();
    let (tree_pubkey, mut tree_account) = build_empty_tree(n_slots, &setup.program_id);

    let mut total_cu = 0u64;

    for (i, step) in steps.iter().enumerate() {
        if let Err(e) = assert_invariants(&step.expected) {
            let step_desc = match &step.step {
                MultiStep::Insert { key, .. } => format!("insert key={}", key),
                MultiStep::Remove { key, .. } => format!("remove key={}", key),
            };
            return CaseResult {
                cu: 0,
                error: Some(format!("step {} ({}) exp invariant: {}", i, step_desc, e)),
            };
        }

        let insn_bytes: Vec<u8> = match &step.step {
            MultiStep::Insert { key, value } => {
                let insn = InsertInstruction {
                    header: InstructionHeader {
                        discriminator: TreeInstruction::Insert as u8,
                    },
                    key: *key,
                    value: *value,
                };
                unsafe { as_bytes(&insn) }.to_vec()
            }
            MultiStep::Remove { key } => {
                let insn = RemoveInstruction {
                    header: InstructionHeader {
                        discriminator: TreeInstruction::Remove as u8,
                    },
                    key: *key,
                };
                unsafe { as_bytes(&insn) }.to_vec()
            }
        };

        let instruction = Instruction::new_with_bytes(
            setup.program_id,
            &insn_bytes,
            vec![
                AccountMeta::new(user_pubkey, true),
                AccountMeta::new(tree_pubkey, false),
            ],
        );

        let accounts = vec![
            (
                user_pubkey,
                Account::new(USER_LAMPORTS, 0, &system_program_pubkey),
            ),
            (tree_pubkey, tree_account.clone()),
        ];

        let result = setup.mollusk.process_instruction(&instruction, &accounts);
        total_cu += result.compute_units_consumed;

        // Both insert and remove return 0 on success.
        if result.program_result != MolluskResult::Success {
            let step_desc = match &step.step {
                MultiStep::Insert { key, .. } => format!("insert key={}", key),
                MultiStep::Remove { key, .. } => format!("remove key={}", key),
            };
            return CaseResult {
                cu: total_cu,
                error: Some(format!(
                    "step {} ({}): expected Success, got {:?}",
                    i, step_desc, result.program_result
                )),
            };
        }

        tree_account = result.resulting_accounts[AccountIndex::Tree as usize].1.clone();
        if let Err(e) = assert_tree_account(&tree_account.data, &step.expected) {
            let step_desc = match &step.step {
                MultiStep::Insert { key, .. } => format!("insert key={}", key),
                MultiStep::Remove { key, .. } => format!("remove key={}", key),
            };
            return CaseResult {
                cu: total_cu,
                error: Some(format!("step {} ({}): {}", i, step_desc, e)),
            };
        }
    }

    CaseResult {
        cu: total_cu,
        error: None,
    }
}

// ---------------------------------------------------------------------------
// Test case enum
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub(super) enum RemoveCase {
    // Input validation (cases 1-5).
    InputDataShort,
    InputDataLong,
    InputNAccounts,
    InputUserDataLen,
    InputTreeDuplicate,
    // Search errors (cases 6-9).
    SearchEmptyTree,
    SearchNotFoundLeft,
    SearchNotFoundRight,
    SearchNotFoundDeep,
    // Simple removal (cases 10-15).
    SimpleRootLeaf,
    SimpleRedLeafL,
    SimpleRedLeafR,
    SimpleOneChildRootR,
    SimpleOneChildRootL,
    SimpleOneChildNonRoot,
    // Successor swap (cases 16-18).
    SuccessorImmediate,
    SuccessorDeep,
    SuccessorWithChild,
    // Rebalancing (cases 19-42).
    Case4L,
    Case4R,
    Case6L,
    Case6R,
    Case56L,
    Case56R,
    Case3Then4L,
    Case3Then4R,
    Case3Then6L,
    Case3Then6R,
    Case3Then56L,
    Case3Then56R,
    Case2PropL,
    Case2PropR,
    Case2Then4,
    Case2Then6,
    Case6NewChildL,
    Case6NewChildR,
    Case6ParentRootL,
    Case6ParentRootR,
    Case6ParentGgpL,
    Case6ParentGgpR,
    Case3ParentRootL,
    Case3ParentRootR,
}

impl RemoveCase {
    pub(super) const INPUT_CASES: &'static [Self] = &[
        Self::InputDataShort,
        Self::InputDataLong,
        Self::InputNAccounts,
        Self::InputUserDataLen,
        Self::InputTreeDuplicate,
    ];

    pub(super) const SEARCH_CASES: &'static [Self] = &[
        Self::SearchEmptyTree,
        Self::SearchNotFoundLeft,
        Self::SearchNotFoundRight,
        Self::SearchNotFoundDeep,
    ];

    pub(super) const SIMPLE_CASES: &'static [Self] = &[
        Self::SimpleRootLeaf,
        Self::SimpleRedLeafL,
        Self::SimpleRedLeafR,
        Self::SimpleOneChildRootR,
        Self::SimpleOneChildRootL,
        Self::SimpleOneChildNonRoot,
    ];

    pub(super) const SUCCESSOR_CASES: &'static [Self] = &[
        Self::SuccessorImmediate,
        Self::SuccessorDeep,
        Self::SuccessorWithChild,
    ];

    pub(super) const REBALANCE_CASES: &'static [Self] = &[
        Self::Case4L,
        Self::Case4R,
        Self::Case6L,
        Self::Case6R,
        Self::Case56L,
        Self::Case56R,
        Self::Case3Then4L,
        Self::Case3Then4R,
        Self::Case3Then6L,
        Self::Case3Then6R,
        Self::Case3Then56L,
        Self::Case3Then56R,
        Self::Case2PropL,
        Self::Case2PropR,
        Self::Case2Then4,
        Self::Case2Then6,
        Self::Case6NewChildL,
        Self::Case6NewChildR,
        Self::Case6ParentRootL,
        Self::Case6ParentRootR,
        Self::Case6ParentGgpL,
        Self::Case6ParentGgpR,
        Self::Case3ParentRootL,
        Self::Case3ParentRootR,
    ];
}

impl TestCase for RemoveCase {
    fn name(&self) -> &'static str {
        match self {
            Self::InputDataShort => "Data too short",
            Self::InputDataLong => "Data too long",
            Self::InputNAccounts => "Too few accounts",
            Self::InputUserDataLen => "User has data",
            Self::InputTreeDuplicate => "Tree is duplicate",
            Self::SearchEmptyTree => "Empty tree",
            Self::SearchNotFoundLeft => "Not found (left)",
            Self::SearchNotFoundRight => "Not found (right)",
            Self::SearchNotFoundDeep => "Not found (deep)",
            Self::SimpleRootLeaf => "Root leaf (sc 2)",
            Self::SimpleRedLeafL => "Red leaf L (sc 3)",
            Self::SimpleRedLeafR => "Red leaf R (sc 3)",
            Self::SimpleOneChildRootR => "One child root R (sc 1)",
            Self::SimpleOneChildRootL => "One child root L (sc 1)",
            Self::SimpleOneChildNonRoot => "One child non-root (sc 1)",
            Self::SuccessorImmediate => "Successor immediate R",
            Self::SuccessorDeep => "Successor deep L descent",
            Self::SuccessorWithChild => "Successor with R child",
            Self::Case4L => "Case 4 dir_l",
            Self::Case4R => "Case 4 dir_r",
            Self::Case6L => "Case 6 dir_l",
            Self::Case6R => "Case 6 dir_r",
            Self::Case56L => "Case 5+6 dir_l",
            Self::Case56R => "Case 5+6 dir_r",
            Self::Case3Then4L => "Case 3->4 dir_l",
            Self::Case3Then4R => "Case 3->4 dir_r",
            Self::Case3Then6L => "Case 3->6 dir_l",
            Self::Case3Then6R => "Case 3->6 dir_r",
            Self::Case3Then56L => "Case 3->5->6 dir_l",
            Self::Case3Then56R => "Case 3->5->6 dir_r",
            Self::Case2PropL => "Case 2 propagate L",
            Self::Case2PropR => "Case 2 propagate R",
            Self::Case2Then4 => "Case 2->4",
            Self::Case2Then6 => "Case 2->6",
            Self::Case6NewChildL => "Case 6 new_child L",
            Self::Case6NewChildR => "Case 6 new_child R",
            Self::Case6ParentRootL => "Case 6 parent=root L",
            Self::Case6ParentRootR => "Case 6 parent=root R",
            Self::Case6ParentGgpL => "Case 6 parent=GGP L",
            Self::Case6ParentGgpR => "Case 6 parent=GGP R",
            Self::Case3ParentRootL => "Case 3 parent=root L",
            Self::Case3ParentRootR => "Case 3 parent=root R",
        }
    }

    fn run(&self, lang: ProgramLanguage) -> CaseResult {
        match self {
            // ----- Input validation -----
            Self::InputDataShort => {
                let (setup, mut instruction, accounts) = remove_input_setup(lang);
                instruction.data = vec![TreeInstruction::Remove as u8];
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::INSTRUCTION_DATA_LEN,
                )
            }
            Self::InputDataLong => {
                let (setup, mut instruction, accounts) = remove_input_setup(lang);
                instruction.data = vec![TreeInstruction::Remove as u8, 0, 0, 0];
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::INSTRUCTION_DATA_LEN,
                )
            }
            Self::InputNAccounts => {
                let (setup, mut instruction, mut accounts) = remove_input_setup(lang);
                instruction.accounts.pop();
                accounts.pop();
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::N_ACCOUNTS,
                )
            }
            Self::InputUserDataLen => {
                let (setup, instruction, mut accounts) = remove_input_setup(lang);
                accounts[AccountIndex::User as usize].1.data = vec![1u8; 1];
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::USER_DATA_LEN,
                )
            }
            Self::InputTreeDuplicate => {
                let (setup, mut instruction, mut accounts) = remove_input_setup(lang);
                instruction.accounts[AccountIndex::Tree as usize] =
                    instruction.accounts[AccountIndex::User as usize].clone();
                accounts[AccountIndex::Tree as usize] =
                    accounts[AccountIndex::User as usize].clone();
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::TREE_DUPLICATE,
                )
            }

            // ----- Search errors -----

            Self::SearchEmptyTree => {
                let desc = TreeSpec {
                    root: None,
                    top: None,
                    nodes: &[],
                };
                run_remove_not_found(lang, &desc, 10)
            }
            Self::SearchNotFoundLeft => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[node(10, B, None, None, None)],
                };
                run_remove_not_found(lang, &desc, 5)
            }
            Self::SearchNotFoundRight => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[node(10, B, None, None, None)],
                };
                run_remove_not_found(lang, &desc, 15)
            }
            Self::SearchNotFoundDeep => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(15, B, Some(0), None, None),
                    ],
                };
                run_remove_not_found(lang, &desc, 12)
            }

            // ----- Simple removal -----

            // Simple case 2: remove root leaf.
            Self::SimpleRootLeaf => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[node(10, B, None, None, None)],
                };
                let exp = TreeSpec {
                    root: None,
                    top: Some(0),
                    nodes: &[node(10, B, None, None, None)],
                };
                run_remove_success(lang, &desc, 10, &exp)
            }

            // Simple case 3: remove red leaf (left child).
            Self::SimpleRedLeafL => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), None),
                        node(5, R, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: Some(1),
                    nodes: &[
                        node(10, B, None, None, None),
                        node(5, R, None, None, None),
                    ],
                };
                run_remove_success(lang, &desc, 5, &exp)
            }

            // Simple case 3: remove red leaf (right child).
            Self::SimpleRedLeafR => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, None, Some(1)),
                        node(15, R, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: Some(1),
                    nodes: &[
                        node(10, B, None, None, None),
                        node(15, R, None, None, None),
                    ],
                };
                run_remove_success(lang, &desc, 15, &exp)
            }

            // Simple case 1: one child at root (right child).
            Self::SimpleOneChildRootR => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, None, Some(1)),
                        node(15, R, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(1),
                    top: Some(0),
                    nodes: &[
                        node(10, B, None, None, None),
                        node(15, B, None, None, None),
                    ],
                };
                run_remove_success(lang, &desc, 10, &exp)
            }

            // Simple case 1: one child at root (left child).
            Self::SimpleOneChildRootL => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), None),
                        node(5, R, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(1),
                    top: Some(0),
                    nodes: &[
                        node(10, B, None, None, None),
                        node(5, B, None, None, None),
                    ],
                };
                run_remove_success(lang, &desc, 10, &exp)
            }

            // Simple case 1: one child non-root (right child).
            Self::SimpleOneChildNonRoot => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(15, B, Some(0), None, Some(3)),
                        node(20, R, Some(2), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: Some(2),
                    nodes: &[
                        node(10, B, None, Some(1), Some(3)),
                        node(5, B, Some(0), None, None),
                        node(15, B, None, None, None),
                        node(20, B, Some(0), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 15, &exp)
            }

            // ----- Successor swap -----

            // Successor is immediate right child.
            // B(10) with R(5) left, R(15) right. Remove 10: swap with
            // successor N2(15), then delete N2 (red leaf, simple case 3).
            Self::SuccessorImmediate => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, R, Some(0), None, None),
                        node(15, R, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: Some(2),
                    nodes: &[
                        node(15, B, None, Some(1), None).val(15),
                        node(5, R, Some(0), None, None),
                        node(15, R, None, None, None),
                    ],
                };
                run_remove_success(lang, &desc, 10, &exp)
            }

            // Successor with deep left descent.
            Self::SuccessorDeep => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(20, B, Some(0), Some(3), Some(4)),
                        node(15, R, Some(2), None, None),
                        node(25, R, Some(2), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: Some(3),
                    nodes: &[
                        node(15, B, None, Some(1), Some(2)).val(15),
                        node(5, B, Some(0), None, None),
                        node(20, B, Some(0), None, Some(4)),
                        node(15, R, None, None, None),
                        node(25, R, Some(2), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 10, &exp)
            }

            // Successor with right child.
            Self::SuccessorWithChild => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(15, B, Some(0), None, Some(3)),
                        node(20, R, Some(2), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: Some(2),
                    nodes: &[
                        node(15, B, None, Some(1), Some(3)).val(15),
                        node(5, B, Some(0), None, None),
                        node(15, B, None, None, None),
                        node(20, B, Some(0), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 10, &exp)
            }

            // ----- Rebalancing -----

            // Case 4 dir_l: red parent, black sibling, black nephews.
            Self::Case4L => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, R, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(15, B, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: Some(1),
                    nodes: &[
                        node(10, B, None, None, Some(2)),
                        node(5, B, None, None, None),
                        node(15, R, Some(0), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 5, &exp)
            }

            // Case 4 dir_r.
            Self::Case4R => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, R, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(15, B, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: Some(2),
                    nodes: &[
                        node(10, B, None, Some(1), None),
                        node(5, R, Some(0), None, None),
                        node(15, B, None, None, None),
                    ],
                };
                run_remove_success(lang, &desc, 15, &exp)
            }

            // Case 6 dir_l: black sibling, distant nephew red.
            Self::Case6L => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(15, B, Some(0), None, Some(3)),
                        node(20, R, Some(2), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(2),
                    top: Some(1),
                    nodes: &[
                        node(10, B, Some(2), None, None),
                        node(5, B, None, None, None),
                        node(15, B, None, Some(0), Some(3)),
                        node(20, B, Some(2), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 5, &exp)
            }

            // Case 6 dir_r.
            Self::Case6R => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), Some(3), None),
                        node(20, B, Some(0), None, None),
                        node(3, R, Some(1), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(1),
                    top: Some(2),
                    nodes: &[
                        node(10, B, Some(1), None, None),
                        node(5, B, None, Some(3), Some(0)),
                        node(20, B, None, None, None),
                        node(3, B, Some(1), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 20, &exp)
            }

            // Case 5+6 dir_l.
            Self::Case56L => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(20, B, Some(0), Some(3), None),
                        node(15, R, Some(2), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(3),
                    top: Some(1),
                    nodes: &[
                        node(10, B, Some(3), None, None),
                        node(5, B, None, None, None),
                        node(20, B, Some(3), None, None),
                        node(15, B, None, Some(0), Some(2)),
                    ],
                };
                run_remove_success(lang, &desc, 5, &exp)
            }

            // Case 5+6 dir_r.
            Self::Case56R => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, Some(3)),
                        node(20, B, Some(0), None, None),
                        node(7, R, Some(1), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(3),
                    top: Some(2),
                    nodes: &[
                        node(10, B, Some(3), None, None),
                        node(5, B, Some(3), None, None),
                        node(20, B, None, None, None),
                        node(7, B, None, Some(1), Some(0)),
                    ],
                };
                run_remove_success(lang, &desc, 20, &exp)
            }

            // Case 3 -> 4 dir_l.
            Self::Case3Then4L => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(20, R, Some(0), Some(3), Some(4)),
                        node(15, B, Some(2), None, None),
                        node(25, B, Some(2), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(2),
                    top: Some(1),
                    nodes: &[
                        node(10, B, Some(2), None, Some(3)),
                        node(5, B, None, None, None),
                        node(20, B, None, Some(0), Some(4)),
                        node(15, R, Some(0), None, None),
                        node(25, B, Some(2), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 5, &exp)
            }

            // Case 3 -> 4 dir_r.
            Self::Case3Then4R => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, R, Some(0), Some(3), Some(4)),
                        node(20, B, Some(0), None, None),
                        node(3, B, Some(1), None, None),
                        node(7, B, Some(1), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(1),
                    top: Some(2),
                    nodes: &[
                        node(10, B, Some(1), Some(4), None),
                        node(5, B, None, Some(3), Some(0)),
                        node(20, B, None, None, None),
                        node(3, B, Some(1), None, None),
                        node(7, R, Some(0), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 20, &exp)
            }

            // Case 3 -> 6 dir_l.
            Self::Case3Then6L => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(20, R, Some(0), Some(3), Some(5)),
                        node(15, B, Some(2), None, Some(4)),
                        node(17, R, Some(3), None, None),
                        node(25, B, Some(2), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(2),
                    top: Some(1),
                    nodes: &[
                        node(10, B, Some(3), None, None),
                        node(5, B, None, None, None),
                        node(20, B, None, Some(3), Some(5)),
                        node(15, R, Some(2), Some(0), Some(4)),
                        node(17, B, Some(3), None, None),
                        node(25, B, Some(2), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 5, &exp)
            }

            // Case 3 -> 6 dir_r.
            Self::Case3Then6R => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, R, Some(0), Some(4), Some(3)),
                        node(20, B, Some(0), None, None),
                        node(7, B, Some(1), Some(5), None),
                        node(3, B, Some(1), None, None),
                        node(6, R, Some(3), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(1),
                    top: Some(2),
                    nodes: &[
                        node(10, B, Some(3), None, None),
                        node(5, B, None, Some(4), Some(3)),
                        node(20, B, None, None, None),
                        node(7, R, Some(1), Some(5), Some(0)),
                        node(3, B, Some(1), None, None),
                        node(6, B, Some(3), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 20, &exp)
            }

            // Case 3 -> 5 -> 6 dir_l.
            Self::Case3Then56L => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(20, R, Some(0), Some(3), Some(5)),
                        node(15, B, Some(2), Some(4), None),
                        node(13, R, Some(3), None, None),
                        node(25, B, Some(2), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(2),
                    top: Some(1),
                    nodes: &[
                        node(10, B, Some(4), None, None),
                        node(5, B, None, None, None),
                        node(20, B, None, Some(4), Some(5)),
                        node(15, B, Some(4), None, None),
                        node(13, R, Some(2), Some(0), Some(3)),
                        node(25, B, Some(2), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 5, &exp)
            }

            // Case 3 -> 5 -> 6 dir_r.
            Self::Case3Then56R => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, R, Some(0), Some(4), Some(3)),
                        node(20, B, Some(0), None, None),
                        node(7, B, Some(1), None, Some(5)),
                        node(3, B, Some(1), None, None),
                        node(8, R, Some(3), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(1),
                    top: Some(2),
                    nodes: &[
                        node(10, B, Some(5), None, None),
                        node(5, B, None, Some(4), Some(5)),
                        node(20, B, None, None, None),
                        node(7, B, Some(5), None, None),
                        node(3, B, Some(1), None, None),
                        node(8, R, Some(1), Some(3), Some(0)),
                    ],
                };
                run_remove_success(lang, &desc, 20, &exp)
            }

            // Case 2: propagate to root (dir_l).
            Self::Case2PropL => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(15, B, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: Some(1),
                    nodes: &[
                        node(10, B, None, None, Some(2)),
                        node(5, B, None, None, None),
                        node(15, R, Some(0), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 5, &exp)
            }

            // Case 2: propagate to root (dir_r).
            Self::Case2PropR => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(15, B, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: Some(2),
                    nodes: &[
                        node(10, B, None, Some(1), None),
                        node(5, R, Some(0), None, None),
                        node(15, B, None, None, None),
                    ],
                };
                run_remove_success(lang, &desc, 15, &exp)
            }

            // Case 2 -> 4: propagate then red parent.
            Self::Case2Then4 => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(20, B, None, Some(1), Some(4)),
                        node(10, R, Some(0), Some(2), Some(3)),
                        node(5, B, Some(1), None, None),
                        node(15, B, Some(1), None, None),
                        node(25, B, Some(0), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: Some(2),
                    nodes: &[
                        node(20, B, None, Some(1), Some(4)),
                        node(10, B, Some(0), None, Some(3)),
                        node(5, B, None, None, None),
                        node(15, R, Some(1), None, None),
                        node(25, B, Some(0), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 5, &exp)
            }

            // Case 2 -> 6: propagate then distant nephew red.
            // 9-node tree with bh=3 at root. Remove B(5) leaf:
            // case 2 at B(10), propagate to B(20), then case 6
            // (distant nephew R(35) is red).
            Self::Case2Then6 => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(20, B, None, Some(1), Some(4)),
                        node(10, B, Some(0), Some(2), Some(3)),
                        node(5, B, Some(1), None, None),
                        node(15, B, Some(1), None, None),
                        node(30, B, Some(0), Some(5), Some(6)),
                        node(25, B, Some(4), None, None),
                        node(35, R, Some(4), Some(7), Some(8)),
                        node(33, B, Some(6), None, None),
                        node(40, B, Some(6), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(4),
                    top: Some(2),
                    nodes: &[
                        node(20, B, Some(4), Some(1), Some(5)),
                        node(10, B, Some(0), None, Some(3)),
                        node(5, B, None, None, None),
                        node(15, R, Some(1), None, None),
                        node(30, B, None, Some(0), Some(6)),
                        node(25, B, Some(0), None, None),
                        node(35, B, Some(4), Some(7), Some(8)),
                        node(33, B, Some(6), None, None),
                        node(40, B, Some(6), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 5, &exp)
            }

            // Case 6 with non-null new_child dir_l.
            // B(10) root, B(5) left, B(20) right with R(15)/R(25).
            // Remove B(5): case 6 rotates left; new_child R(15) is
            // reparented from B(20).child[L] to B(10).child[R].
            Self::Case6NewChildL => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(20, B, Some(0), Some(3), Some(4)),
                        node(15, R, Some(2), None, None),
                        node(25, R, Some(2), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(2),
                    top: Some(1),
                    nodes: &[
                        node(10, B, Some(2), None, Some(3)),
                        node(5, B, None, None, None),
                        node(20, B, None, Some(0), Some(4)),
                        node(15, R, Some(0), None, None),
                        node(25, B, Some(2), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 5, &exp)
            }

            // Case 6 with non-null new_child dir_r.
            // B(10) root, B(5) left with R(3)/R(7), B(20) right.
            // Remove B(20): case 6 rotates right; new_child R(7) is
            // reparented from B(5).child[R] to B(10).child[L].
            Self::Case6NewChildR => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), Some(3), Some(4)),
                        node(20, B, Some(0), None, None),
                        node(3, R, Some(1), None, None),
                        node(7, R, Some(1), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(1),
                    top: Some(2),
                    nodes: &[
                        node(10, B, Some(1), Some(4), None),
                        node(5, B, None, Some(3), Some(0)),
                        node(20, B, None, None, None),
                        node(3, B, Some(1), None, None),
                        node(7, R, Some(0), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 20, &exp)
            }

            // Case 6 parent=root dir_l (same tree as Case6L).
            Self::Case6ParentRootL => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(15, B, Some(0), None, Some(3)),
                        node(20, R, Some(2), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(2),
                    top: Some(1),
                    nodes: &[
                        node(10, B, Some(2), None, None),
                        node(5, B, None, None, None),
                        node(15, B, None, Some(0), Some(3)),
                        node(20, B, Some(2), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 5, &exp)
            }

            // Case 6 parent=root dir_r.
            Self::Case6ParentRootR => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), Some(3), None),
                        node(20, B, Some(0), None, None),
                        node(3, R, Some(1), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(1),
                    top: Some(2),
                    nodes: &[
                        node(10, B, Some(1), None, None),
                        node(5, B, None, Some(3), Some(0)),
                        node(20, B, None, None, None),
                        node(3, B, Some(1), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 20, &exp)
            }

            // Case 6 parent=GGP left child dir_l.
            // 8-node tree, bh=3 at root. N1(B(10)) is left child of
            // root B(30). Remove B(5) from N1's left: case 6 rotates
            // N1 left, N3(B(20)) takes N1's place. GGP N0 updates
            // child[L] = N3.
            Self::Case6ParentGgpL => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(30, B, None, Some(1), Some(5)),
                        node(10, B, Some(0), Some(2), Some(3)),
                        node(5, B, Some(1), None, None),
                        node(20, B, Some(1), None, Some(4)),
                        node(25, R, Some(3), None, None),
                        node(40, B, Some(0), Some(6), Some(7)),
                        node(35, B, Some(5), None, None),
                        node(45, B, Some(5), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: Some(2),
                    nodes: &[
                        node(30, B, None, Some(3), Some(5)),
                        node(10, B, Some(3), None, None),
                        node(5, B, None, None, None),
                        node(20, B, Some(0), Some(1), Some(4)),
                        node(25, B, Some(3), None, None),
                        node(40, B, Some(0), Some(6), Some(7)),
                        node(35, B, Some(5), None, None),
                        node(45, B, Some(5), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 5, &exp)
            }

            // Case 6 parent=GGP right child dir_r.
            // 8-node tree, bh=3 at root. N1(B(20)) is right child of
            // root B(5). Remove B(25) from N1's right: case 6 rotates
            // N1 right, N2(B(10)) takes N1's place. GGP N0 updates
            // child[R] = N2.
            Self::Case6ParentGgpR => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(5, B, None, Some(5), Some(1)),
                        node(20, B, Some(0), Some(2), Some(3)),
                        node(10, B, Some(1), Some(4), None),
                        node(25, B, Some(1), None, None),
                        node(7, R, Some(2), None, None),
                        node(2, B, Some(0), Some(6), Some(7)),
                        node(1, B, Some(5), None, None),
                        node(3, B, Some(5), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(0),
                    top: Some(3),
                    nodes: &[
                        node(5, B, None, Some(5), Some(2)),
                        node(20, B, Some(2), None, None),
                        node(10, B, Some(0), Some(4), Some(1)),
                        node(25, B, None, None, None),
                        node(7, B, Some(2), None, None),
                        node(2, B, Some(0), Some(6), Some(7)),
                        node(1, B, Some(5), None, None),
                        node(3, B, Some(5), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 25, &exp)
            }

            // Case 3 parent=root dir_l.
            Self::Case3ParentRootL => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, B, Some(0), None, None),
                        node(20, R, Some(0), Some(3), Some(4)),
                        node(15, B, Some(2), None, None),
                        node(25, B, Some(2), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(2),
                    top: Some(1),
                    nodes: &[
                        node(10, B, Some(2), None, Some(3)),
                        node(5, B, None, None, None),
                        node(20, B, None, Some(0), Some(4)),
                        node(15, R, Some(0), None, None),
                        node(25, B, Some(2), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 5, &exp)
            }

            // Case 3 parent=root dir_r.
            Self::Case3ParentRootR => {
                let desc = TreeSpec {
                    root: Some(0),
                    top: None,
                    nodes: &[
                        node(10, B, None, Some(1), Some(2)),
                        node(5, R, Some(0), Some(3), Some(4)),
                        node(20, B, Some(0), None, None),
                        node(3, B, Some(1), None, None),
                        node(7, B, Some(1), None, None),
                    ],
                };
                let exp = TreeSpec {
                    root: Some(1),
                    top: Some(2),
                    nodes: &[
                        node(10, B, Some(1), Some(4), None),
                        node(5, B, None, Some(3), Some(0)),
                        node(20, B, None, None, None),
                        node(3, B, Some(1), None, None),
                        node(7, R, Some(0), None, None),
                    ],
                };
                run_remove_success(lang, &desc, 20, &exp)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Multi-step integration tests
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub(super) enum MultiRemoveCase {
    /// Insert 10,5,15; remove 5.
    Minimal,
    /// Insert 7 nodes; remove all.
    FullCycle,
    /// Insert 3; remove 1; insert 1.
    Recycle,
}

impl MultiRemoveCase {
    pub(super) const CASES: &'static [Self] = &[
        Self::Minimal,
        Self::FullCycle,
        Self::Recycle,
    ];
}

impl TestCase for MultiRemoveCase {
    fn name(&self) -> &'static str {
        match self {
            Self::Minimal => "Insert 3, remove 1",
            Self::FullCycle => "Insert 7, remove all",
            Self::Recycle => "Insert-remove-insert (recycle)",
        }
    }

    fn run(&self, lang: ProgramLanguage) -> CaseResult {
        match self {
            Self::Minimal => run_multi_step(lang, 3, &[
                MultiStepCase {
                    step: MultiStep::Insert { key: 10, value: 1 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: Some(1),
                        nodes: &[node(10, R, None, None, None).val(1)],
                    },
                },
                MultiStepCase {
                    step: MultiStep::Insert { key: 5, value: 1 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: Some(2),
                        nodes: &[
                            node(10, B, None, Some(1), None).val(1),
                            node(5, R, Some(0), None, None).val(1),
                        ],
                    },
                },
                MultiStepCase {
                    step: MultiStep::Insert { key: 15, value: 1 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: None,
                        nodes: &[
                            node(10, B, None, Some(1), Some(2)).val(1),
                            node(5, R, Some(0), None, None).val(1),
                            node(15, R, Some(0), None, None).val(1),
                        ],
                    },
                },
                MultiStepCase {
                    step: MultiStep::Remove { key: 5 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: Some(1),
                        nodes: &[
                            node(10, B, None, None, Some(2)).val(1),
                            // Freed node: key/value/color retained, children nulled,
                            // parent (= StackNode.next) = null (stack was empty).
                            node(5, R, None, None, None).val(1),
                            node(15, R, Some(0), None, None).val(1),
                        ],
                    },
                },
            ]),

            Self::FullCycle => run_multi_step(lang, 7, &[
                // Insert 10,5,15,3,7,12,20.
                MultiStepCase {
                    step: MultiStep::Insert { key: 10, value: 1 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: Some(1),
                        nodes: &[node(10, R, None, None, None).val(1)],
                    },
                },
                MultiStepCase {
                    step: MultiStep::Insert { key: 5, value: 1 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: Some(2),
                        nodes: &[
                            node(10, B, None, Some(1), None).val(1),
                            node(5, R, Some(0), None, None).val(1),
                        ],
                    },
                },
                MultiStepCase {
                    step: MultiStep::Insert { key: 15, value: 1 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: Some(3),
                        nodes: &[
                            node(10, B, None, Some(1), Some(2)).val(1),
                            node(5, R, Some(0), None, None).val(1),
                            node(15, R, Some(0), None, None).val(1),
                        ],
                    },
                },
                MultiStepCase {
                    step: MultiStep::Insert { key: 3, value: 1 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: Some(4),
                        nodes: &[
                            node(10, R, None, Some(1), Some(2)).val(1),
                            node(5, B, Some(0), Some(3), None).val(1),
                            node(15, B, Some(0), None, None).val(1),
                            node(3, R, Some(1), None, None).val(1),
                        ],
                    },
                },
                MultiStepCase {
                    step: MultiStep::Insert { key: 7, value: 1 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: Some(5),
                        nodes: &[
                            node(10, R, None, Some(1), Some(2)).val(1),
                            node(5, B, Some(0), Some(3), Some(4)).val(1),
                            node(15, B, Some(0), None, None).val(1),
                            node(3, R, Some(1), None, None).val(1),
                            node(7, R, Some(1), None, None).val(1),
                        ],
                    },
                },
                MultiStepCase {
                    step: MultiStep::Insert { key: 12, value: 1 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: Some(6),
                        nodes: &[
                            node(10, R, None, Some(1), Some(2)).val(1),
                            node(5, B, Some(0), Some(3), Some(4)).val(1),
                            node(15, B, Some(0), Some(5), None).val(1),
                            node(3, R, Some(1), None, None).val(1),
                            node(7, R, Some(1), None, None).val(1),
                            node(12, R, Some(2), None, None).val(1),
                        ],
                    },
                },
                MultiStepCase {
                    step: MultiStep::Insert { key: 20, value: 1 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: None,
                        nodes: &[
                            node(10, R, None, Some(1), Some(2)).val(1),
                            node(5, B, Some(0), Some(3), Some(4)).val(1),
                            node(15, B, Some(0), Some(5), Some(6)).val(1),
                            node(3, R, Some(1), None, None).val(1),
                            node(7, R, Some(1), None, None).val(1),
                            node(12, R, Some(2), None, None).val(1),
                            node(20, R, Some(2), None, None).val(1),
                        ],
                    },
                },
                // Remove all: 3, 20, 7, 12, 5, 15, 10.
                // Removing leaves first to simplify expected states.
                MultiStepCase {
                    step: MultiStep::Remove { key: 3 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: Some(3),
                        nodes: &[
                            node(10, R, None, Some(1), Some(2)).val(1),
                            node(5, B, Some(0), None, Some(4)).val(1),
                            node(15, B, Some(0), Some(5), Some(6)).val(1),
                            node(3, R, None, None, None).val(1),
                            node(7, R, Some(1), None, None).val(1),
                            node(12, R, Some(2), None, None).val(1),
                            node(20, R, Some(2), None, None).val(1),
                        ],
                    },
                },
                MultiStepCase {
                    step: MultiStep::Remove { key: 20 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: Some(6),
                        nodes: &[
                            node(10, R, None, Some(1), Some(2)).val(1),
                            node(5, B, Some(0), None, Some(4)).val(1),
                            node(15, B, Some(0), Some(5), None).val(1),
                            node(3, R, None, None, None).val(1),
                            node(7, R, Some(1), None, None).val(1),
                            node(12, R, Some(2), None, None).val(1),
                            node(20, R, Some(3), None, None).val(1),
                        ],
                    },
                },
                MultiStepCase {
                    step: MultiStep::Remove { key: 7 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: Some(4),
                        nodes: &[
                            node(10, R, None, Some(1), Some(2)).val(1),
                            node(5, B, Some(0), None, None).val(1),
                            node(15, B, Some(0), Some(5), None).val(1),
                            node(3, R, None, None, None).val(1),
                            node(7, R, Some(6), None, None).val(1),
                            node(12, R, Some(2), None, None).val(1),
                            node(20, R, Some(3), None, None).val(1),
                        ],
                    },
                },
                MultiStepCase {
                    step: MultiStep::Remove { key: 12 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: Some(5),
                        nodes: &[
                            node(10, R, None, Some(1), Some(2)).val(1),
                            node(5, B, Some(0), None, None).val(1),
                            node(15, B, Some(0), None, None).val(1),
                            node(3, R, None, None, None).val(1),
                            node(7, R, Some(6), None, None).val(1),
                            node(12, R, Some(4), None, None).val(1),
                            node(20, R, Some(3), None, None).val(1),
                        ],
                    },
                },
                MultiStepCase {
                    step: MultiStep::Remove { key: 5 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: Some(1),
                        nodes: &[
                            node(10, B, None, None, Some(2)).val(1),
                            node(5, B, Some(5), None, None).val(1),
                            node(15, R, Some(0), None, None).val(1),
                            node(3, R, None, None, None).val(1),
                            node(7, R, Some(6), None, None).val(1),
                            node(12, R, Some(4), None, None).val(1),
                            node(20, R, Some(3), None, None).val(1),
                        ],
                    },
                },
                MultiStepCase {
                    step: MultiStep::Remove { key: 15 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: Some(2),
                        nodes: &[
                            node(10, B, None, None, None).val(1),
                            node(5, B, Some(5), None, None).val(1),
                            node(15, R, Some(1), None, None).val(1),
                            node(3, R, None, None, None).val(1),
                            node(7, R, Some(6), None, None).val(1),
                            node(12, R, Some(4), None, None).val(1),
                            node(20, R, Some(3), None, None).val(1),
                        ],
                    },
                },
                MultiStepCase {
                    step: MultiStep::Remove { key: 10 },
                    expected: TreeSpec {
                        root: None,
                        top: Some(0),
                        nodes: &[
                            node(10, B, Some(2), None, None).val(1),
                            node(5, B, Some(5), None, None).val(1),
                            node(15, R, Some(1), None, None).val(1),
                            node(3, R, None, None, None).val(1),
                            node(7, R, Some(6), None, None).val(1),
                            node(12, R, Some(4), None, None).val(1),
                            node(20, R, Some(3), None, None).val(1),
                        ],
                    },
                },
            ]),

            Self::Recycle => run_multi_step(lang, 3, &[
                // Insert 10,5,15.
                MultiStepCase {
                    step: MultiStep::Insert { key: 10, value: 10 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: Some(1),
                        nodes: &[node(10, R, None, None, None)],
                    },
                },
                MultiStepCase {
                    step: MultiStep::Insert { key: 5, value: 5 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: Some(2),
                        nodes: &[
                            node(10, B, None, Some(1), None),
                            node(5, R, Some(0), None, None),
                        ],
                    },
                },
                MultiStepCase {
                    step: MultiStep::Insert { key: 15, value: 15 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: None,
                        nodes: &[
                            node(10, B, None, Some(1), Some(2)),
                            node(5, R, Some(0), None, None),
                            node(15, R, Some(0), None, None),
                        ],
                    },
                },
                // Remove 5: red leaf removal, N1 freed onto stack.
                MultiStepCase {
                    step: MultiStep::Remove { key: 5 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: Some(1),
                        nodes: &[
                            node(10, B, None, None, Some(2)),
                            node(5, R, None, None, None),
                            node(15, R, Some(0), None, None),
                        ],
                    },
                },
                // Insert 7: pops N1 from stack, reuses slot.
                MultiStepCase {
                    step: MultiStep::Insert { key: 7, value: 7 },
                    expected: TreeSpec {
                        root: Some(0),
                        top: None,
                        nodes: &[
                            node(10, B, None, Some(1), Some(2)),
                            node(7, R, Some(0), None, None),
                            node(15, R, Some(0), None, None),
                        ],
                    },
                },
            ]),
        }
    }
}
