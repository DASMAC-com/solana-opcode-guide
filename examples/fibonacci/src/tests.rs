use std::vec;

use fib_rs::Fib;
use mollusk_svm::result::Check;
use solana_sdk::instruction::Instruction;
use solana_sdk::program_error::ProgramError;
use test_utils::{setup_test, single_mock_account, ProgramLanguage};

const MAX_FIB_INDEX_U32: u8 = 47;
const E_PASSED_ACCOUNT: u32 = u32::MAX;
const E_INDEX_TOO_BIG: u32 = u32::MAX - 1;

#[test]
fn test_asm() {
    let setup = setup_test(ProgramLanguage::Assembly);

    // Verify failure for passing an account.
    let (account, accounts) = single_mock_account();
    setup.mollusk.process_and_validate_instruction(
        &Instruction::new_with_bytes(setup.program_id, &[], accounts.clone()),
        &[account],
        &[Check::err(ProgramError::Custom(E_PASSED_ACCOUNT))],
    );

    // Verify failure for index too big.
    setup.mollusk.process_and_validate_instruction(
        &Instruction::new_with_bytes(setup.program_id, &[MAX_FIB_INDEX_U32 + 1], vec![]),
        &[],
        &[Check::err(ProgramError::Custom(E_INDEX_TOO_BIG))],
    );

    // Initialize a vector with 48 slots, one for each Fibonacci number from F(0) to F(47), along
    // with their compute unit consumption.
    let mut fib_numbers: Vec<(u32, u64)> = vec![(0, 0); (MAX_FIB_INDEX_U32 + 1) as usize];

    // For F(0) = 0, the program result should be considered a success.
    fib_numbers[0] = (
        0,
        setup
            .mollusk
            .process_and_validate_instruction(
                &Instruction::new_with_bytes(setup.program_id, &[0], vec![]),
                &[],
                &[Check::success()],
            )
            .compute_units_consumed,
    );

    // For F(1) onwards, verify correct Fibonacci numbers are returned as a custom error.
    for n in 1..=MAX_FIB_INDEX_U32 {
        let expected_fib: u32 = Fib::single(n.into()).try_into().unwrap();
        fib_numbers[n as usize] = (
            expected_fib,
            setup
                .mollusk
                .process_and_validate_instruction(
                    &Instruction::new_with_bytes(setup.program_id, &[n], vec![]),
                    &[],
                    &[Check::err(ProgramError::Custom(expected_fib))],
                )
                .compute_units_consumed,
        );
    }

    // Pretty print the Fibonacci numbers along with their compute unit consumption.
    println!("Fibonacci numbers and compute units:");
    for n in 0..=MAX_FIB_INDEX_U32 {
        println!(
            "{:<7}{:<10} (Compute Units: {})",
            format!("F({}):", n),
            fib_numbers[n as usize].0,
            fib_numbers[n as usize].1
        );
    }
}

/// Verify the index of the maximum Fibonacci number that fits in a u32, while allowing space for
/// two error codes that may be returned by the Fibonacci program.
#[test]
fn verify_max_fib_u32() {
    assert!(Fib::single(MAX_FIB_INDEX_U32.into()) <= E_INDEX_TOO_BIG.into());
    assert!(Fib::single((MAX_FIB_INDEX_U32 + 1).into()) > u32::MAX.into());
}
