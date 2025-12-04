use mollusk_svm::{result::Check, Mollusk};
use solana_sdk::instruction::Instruction;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signer::Signer;

#[test]
fn asm() {
    let keypair =
        read_keypair_file("deploy/hello-dasmac-keypair.json").expect("Failed to read keypair file");
    let program_id = keypair.pubkey();

    let instruction = Instruction::new_with_bytes(program_id, &[], vec![]);
    let mollusk = Mollusk::new(&program_id, "deploy/hello-dasmac");
    let result = mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
    assert!(!result.program_result.is_err());
}

#[test]
fn rs() {
    let keypair =
        read_keypair_file("deploy/memo-keypair.json").expect("Failed to read keypair file");
    let program_id = keypair.pubkey();

    let instruction = Instruction::new_with_bytes(program_id, &[], vec![]);
    let mollusk = Mollusk::new(&program_id, "../target/deploy/hello_dasmac");
    let result = mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
    assert!(!result.program_result.is_err());
}
