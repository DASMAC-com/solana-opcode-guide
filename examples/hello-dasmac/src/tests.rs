use mollusk_svm::{result::Check, Mollusk};
use solana_sdk::instruction::Instruction;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signer::Signer;

#[test]
fn asm() {
    let keypair = read_keypair_file(format!("deploy/{}-keypair.json", env!("CARGO_PKG_NAME")))
        .expect("Failed to read keypair file");
    let program_id = keypair.pubkey();

    let instruction = Instruction::new_with_bytes(program_id, &[], vec![]);
    let mollusk = Mollusk::new(&program_id, &format!("deploy/{}", env!("CARGO_PKG_NAME")));
    let result = mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
    assert!(!result.program_result.is_err());
}

#[test]
fn rs() {
    let keypair = read_keypair_file(format!("deploy/{}-keypair.json", env!("CARGO_PKG_NAME")))
        .expect("Failed to read keypair file");
    let program_id = keypair.pubkey();

    let instruction = Instruction::new_with_bytes(program_id, &[], vec![]);
    let mollusk = Mollusk::new(
        &program_id,
        &format!("../target/deploy/{}", env!("CARGO_PKG_NAME").replace('-', "_")),
    );
    let result = mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
    assert!(!result.program_result.is_err());
}
