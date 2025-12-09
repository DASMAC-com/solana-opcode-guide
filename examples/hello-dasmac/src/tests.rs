use mollusk_svm::result::Check;
use solana_sdk::instruction::Instruction;
use test_utils::{setup_test, ProgramLanguage};

#[test]
fn test_asm() {
    happy_path(ProgramLanguage::Assembly);
}

#[test]
fn test_rs() {
    happy_path(ProgramLanguage::Rust);
}

fn happy_path(program_language: ProgramLanguage) {
    let setup = setup_test!(program_language);

    // Invoke the program with an empty instruction and verify success.
    let result = setup.mollusk.process_and_validate_instruction(
        &Instruction::new_with_bytes(setup.program_id, &[], vec![]),
        &[],
        &[Check::success()],
    );
    assert!(!result.program_result.is_err());
}
