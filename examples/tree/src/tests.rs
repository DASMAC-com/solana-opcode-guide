mod entrypoint;
mod init;

use interface::error_codes;
use mollusk_svm::result::ProgramResult as MolluskResult;
use solana_sdk::account::Account;
use solana_sdk::instruction::Instruction;
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use test_utils::{setup_test, ProgramLanguage, TestSetup};

const USER_LAMPORTS: u64 = 1_000_000;

enum AccountIndex {
    User = 0,
    Tree = 1,
    SystemProgram = 2,
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
}

fn print_comparison_table<T: TestCase>(cases: &[T]) {
    let mut failures = Vec::new();

    println!("| Case | ASM (CUs) | Rust (CUs) | Overhead | Overhead % |");
    println!("|------|-----------|------------|----------|------------|");

    for case in cases {
        let asm = case.run(ProgramLanguage::Assembly);
        let rs = case.run(ProgramLanguage::Rust);
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
fn test_initialize_input_checks() {
    print_comparison_table(init::InitCase::CASES);
}
