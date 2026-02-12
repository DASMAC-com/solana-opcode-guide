mod entrypoint;
mod init;

use mollusk_svm::result::ProgramResult as MolluskResult;
use solana_sdk::account::Account;
use solana_sdk::instruction::Instruction;
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use test_utils::{setup_test, ProgramLanguage, TestSetup};
use tree_interface::{cpi, error_codes};

const USER_LAMPORTS: u64 = 1_000_000;

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

fn check_error(
    setup: &TestSetup,
    instruction: &Instruction,
    accounts: &[(Pubkey, Account)],
    expected_error: error_codes::error,
) -> CaseResult {
    let result = setup.mollusk.process_instruction(instruction, accounts);
    let expected = ProgramError::Custom(expected_error.into());
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

trait TestCase: Copy {
    fn name(&self) -> &'static str;
    fn run(&self, lang: ProgramLanguage) -> CaseResult;
    /// Returns the fixed syscall/CPI costs for this case.
    /// These costs are identical for both ASM and Rust implementations.
    fn fixed_costs(&self) -> u64 {
        0
    }
}

fn print_comparison_table<T: TestCase>(
    cases: &[T],
    allow_asm_failures: bool,
    allow_rust_failures: bool,
) {
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
            if allow_asm_failures {
                println!("  (ASM) {}: {}", case.name(), err);
            } else {
                failures.push(format!("  ASM {}: {}", case.name(), err));
            }
        }
        if let Some(err) = &rs.error {
            if allow_rust_failures {
                println!("  (Rust) {}: {}", case.name(), err);
            } else {
                failures.push(format!("  Rust {}: {}", case.name(), err));
            }
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
    print_comparison_table(entrypoint::EntrypointCase::CASES, false, false);
}

#[test]
fn test_initialize_input_checks() {
    print_comparison_table(init::InitCase::CASES, false, false);
}

#[test]
fn test_initialize_pda_checks() {
    print_comparison_table(init::InitCase::PDA_CASES, false, false);
}

#[test]
fn test_initialize_create_account() {
    print_comparison_table(init::InitCase::CPI_CASES, false, false);
}
