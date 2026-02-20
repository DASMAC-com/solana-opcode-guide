mod common;
mod entrypoint;
mod init;
mod insert;
mod remove;

use mollusk_svm::program;
use mollusk_svm::result::{Check, Config, ProgramResult as MolluskResult};
use pinocchio::sysvars::rent::Rent;
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use test_utils::{setup_test, ProgramLanguage, TestSetup};
use tree_interface::{cpi, error_codes};

const USER_LAMPORTS: u64 = 1_000_000;

/// Virtual address of the input buffer in the SVM memory map.
/// See `solana_sbpf::ebpf::MM_INPUT_START`.
const MM_INPUT_START: u64 = 0x400000000;

/// Rent exemption threshold per SIMD-0194.
const SIMD0194_EXEMPTION_THRESHOLD: f64 = 1.0;

/// Set up a test with SIMD-0194 rent exemption threshold.
fn setup_test_with_rent(lang: ProgramLanguage) -> TestSetup {
    let mut setup = setup_test(lang);
    setup.mollusk.sysvars.rent.exemption_threshold = SIMD0194_EXEMPTION_THRESHOLD;
    setup
}

/// Cast a sized value to its raw byte representation.
unsafe fn as_bytes<T: Sized>(val: &T) -> &[u8] {
    core::slice::from_raw_parts(val as *const T as *const u8, size_of::<T>())
}

/// Fixed costs for syscalls and CPI operations.
mod fixed_costs {
    /// Cost for sol_try_find_program_address syscall.
    pub const CREATE_PROGRAM_ADDRESS: u64 = 1500;
    /// CPI base invocation cost (SIMD-0339).
    pub const CPI_BASE: u64 = 946;
    /// System Program operation cost.
    pub const SYSTEM_PROGRAM: u64 = 150;
}

enum AccountIndex {
    User = 0,
    Tree = 1,
    SystemProgram = 2,
    RentSysvar = 3,
}

struct CaseResult {
    cu: u64,
    error: Option<String>,
}

fn check_result(
    setup: &TestSetup,
    instruction: &Instruction,
    accounts: &[(Pubkey, Account)],
    expected: ProgramError,
) -> CaseResult {
    let result = setup.mollusk.process_instruction(instruction, accounts);
    match &result.program_result {
        MolluskResult::Failure(err) if *err == expected => CaseResult {
            cu: result.compute_units_consumed,
            error: None,
        },
        other => CaseResult {
            cu: result.compute_units_consumed,
            error: Some(format!("expected Failure({:?}), got {:?}", expected, other)),
        },
    }
}

fn flip_account_address(
    instruction: &mut Instruction,
    accounts: &mut [(Pubkey, Account)],
    account_index: usize,
    chunk_index: usize,
    chunk_size: usize,
) {
    let flip_index = (chunk_index * chunk_size) + chunk_size - 1;
    accounts[account_index].0.as_mut()[flip_index] ^= 1;
    instruction.accounts[account_index].pubkey = accounts[account_index].0;
}

fn check_error(
    setup: &TestSetup,
    instruction: &Instruction,
    accounts: &[(Pubkey, Account)],
    expected_error: error_codes::error,
) -> CaseResult {
    check_result(
        setup,
        instruction,
        accounts,
        ProgramError::Custom(expected_error.into()),
    )
}

trait TestCase: Copy {
    fn name(&self) -> &'static str;
    fn run(&self, lang: ProgramLanguage) -> CaseResult;
    /// Returns the fixed syscall/CPI costs for this case.
    /// These costs are identical for both ASM and Rust implementations.
    fn fixed_costs(&self) -> u64 {
        0
    }
}

fn print_comparison_table<T: TestCase>(cases: &[T]) {
    let mut failures = Vec::new();
    let has_fixed_costs = cases.iter().any(|c| c.fixed_costs() > 0);

    if has_fixed_costs {
        println!("| Test case | Fixed CU costs | ASM (net CUs) | Rust (net CUs) | Overhead | Overhead % |");
        println!("|-----------|----------------|---------------|----------------|----------|------------|");
    } else {
        println!("| Test case | ASM (CUs) | Rust (CUs) | Overhead | Overhead % |");
        println!("|-----------|-----------|------------|----------|------------|");
    }

    for case in cases {
        let asm = case.run(ProgramLanguage::Assembly);
        let rs = case.run(ProgramLanguage::Rust);
        let fixed = case.fixed_costs();

        if has_fixed_costs {
            let asm_net = asm.cu.saturating_sub(fixed);
            let rs_net = rs.cu.saturating_sub(fixed);
            let overhead = rs_net as i64 - asm_net as i64;
            let overhead_pct = if asm_net > 0 {
                format!("{:+.1}%", (overhead as f64 / asm_net as f64) * 100.0)
            } else {
                "N/A".to_string()
            };
            println!(
                "| {} | {} | {} | {} | {:+} | {} |",
                case.name(),
                fixed,
                asm_net,
                rs_net,
                overhead,
                overhead_pct
            );
        } else {
            let overhead = rs.cu as i64 - asm.cu as i64;
            let overhead_pct = if asm.cu > 0 {
                (overhead as f64 / asm.cu as f64) * 100.0
            } else {
                0.0
            };
            println!(
                "| {} | {} | {} | {:+} | {:+.1}% |",
                case.name(),
                asm.cu,
                rs.cu,
                overhead,
                overhead_pct
            );
        }

        if let Some(err) = &asm.error {
            failures.push(format!("  ASM {}: {}", case.name(), err));
        }
        if let Some(err) = &rs.error {
            failures.push(format!("  Rust {}: {}", case.name(), err));
        }
    }

    assert!(
        failures.is_empty(),
        "\nFailed cases:\n{}",
        failures.join("\n")
    );
}

#[test]
fn test_entrypoint_branching() {
    print_comparison_table(entrypoint::EntrypointCase::CASES);
}

#[test]
fn test_insert_input_checks() {
    print_comparison_table(insert::InsertCase::INPUT_CASES);
}

#[test]
fn test_insert_alloc_checks() {
    print_comparison_table(insert::InsertCase::ALLOC_CHECK_CASES);
}

#[test]
fn test_insert_alloc() {
    print_comparison_table(insert::InsertCase::ALLOC_CASES);
}

#[test]
fn test_initialize_input_checks() {
    print_comparison_table(init::InitCase::CASES);
}

#[test]
fn test_initialize_pda_checks() {
    print_comparison_table(init::InitCase::PDA_CASES);
}

#[test]
fn test_initialize_create_account() {
    print_comparison_table(init::InitCase::CPI_CASES);
}

#[test]
fn test_insert_search() {
    print_comparison_table(insert::InsertCase::SEARCH_CASES);
}

#[test]
fn test_insert_to_tree() {
    print_comparison_table(insert::InsertCase::TREE_CASES);
}

#[test]
fn test_multi_insert() {
    print_comparison_table(insert::MultiInsertCase::CASES);
}

#[test]
fn test_remove_input_checks() {
    print_comparison_table(remove::RemoveCase::INPUT_CASES);
}

#[test]
fn test_remove_search() {
    print_comparison_table(remove::RemoveCase::SEARCH_CASES);
}

#[test]
fn test_remove_simple() {
    print_comparison_table(remove::RemoveCase::SIMPLE_CASES);
}

// #[test]
// fn test_remove_rebalance() {
//     print_comparison_table(remove::RemoveCase::REBALANCE_CASES);
// }

// #[test]
// fn test_multi_remove() {
//     print_comparison_table(remove::MultiRemoveCase::CASES);
// }
