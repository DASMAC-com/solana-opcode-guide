use mollusk_svm::result::Check;
use solana_sdk::instruction::Instruction;
use test_utils::{setup_test, ProgramLanguage};

#[test]
fn asm() {
    let setup = setup_test(env!("CARGO_PKG_NAME"), ProgramLanguage::Assembly);
    let program_id = setup.program_id;
    let mollusk = setup.mollusk;

    let instruction = Instruction::new_with_bytes(program_id, &[], vec![]);
    let result = mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
    assert!(!result.program_result.is_err());
}

#[test]
fn rs() {
    let setup = setup_test(env!("CARGO_PKG_NAME"), ProgramLanguage::Rust);
    let program_id = setup.program_id;
    let mollusk = setup.mollusk;

    let instruction = Instruction::new_with_bytes(program_id, &[], vec![]);
    let result = mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
    assert!(!result.program_result.is_err());
}
