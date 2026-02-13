use super::*;
use mollusk_svm::program;
use solana_sdk::instruction::AccountMeta;
use tree_interface::{cpi, Instruction as TreeInstruction};

fn insert_setup(
    program_language: ProgramLanguage,
) -> (TestSetup, Instruction, Vec<(Pubkey, Account)>) {
    let setup = setup_test(program_language);
    let (system_program_pubkey, _) = program::keyed_account_for_system_program();

    let user_pubkey = Pubkey::new_unique();
    let tree_pubkey = Pubkey::new_unique();

    // Valid InsertInstruction: discriminator (1) + key (u16) + value (u16) = 5 bytes.
    let instruction_data: [u8; 5] = [
        TreeInstruction::Insert as u8,
        42, 0, // key
        1, 0,  // value
    ];

    let instruction = Instruction::new_with_bytes(
        setup.program_id,
        &instruction_data,
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
        (
            tree_pubkey,
            Account::new(0, cpi::TREE_DATA_LEN, &setup.program_id),
        ),
    ];

    (setup, instruction, accounts)
}

#[derive(Clone, Copy)]
pub(super) enum InsertCase {
    InstructionDataLenShort,
    InstructionDataLenLong,
    InsertHappyPath,
}

impl InsertCase {
    pub(super) const CASES: &'static [Self] = &[
        Self::InstructionDataLenShort,
        Self::InstructionDataLenLong,
        Self::InsertHappyPath,
    ];
}

impl TestCase for InsertCase {
    fn name(&self) -> &'static str {
        match self {
            Self::InstructionDataLenShort => "Instruction data too short",
            Self::InstructionDataLenLong => "Instruction data too long",
            Self::InsertHappyPath => "Insert happy path",
        }
    }

    fn run(&self, lang: ProgramLanguage) -> CaseResult {
        match self {
            Self::InstructionDataLenShort => {
                let (setup, mut instruction, accounts) = insert_setup(lang);
                // Correct discriminator but wrong length (1 byte instead of 5).
                instruction.data = vec![TreeInstruction::Insert as u8];
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::INSTRUCTION_DATA_LEN,
                )
            }
            Self::InstructionDataLenLong => {
                let (setup, mut instruction, accounts) = insert_setup(lang);
                // Correct discriminator but wrong length (6 bytes instead of 5).
                instruction.data = vec![TreeInstruction::Insert as u8, 0, 0, 0, 0, 0];
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::INSTRUCTION_DATA_LEN,
                )
            }
            Self::InsertHappyPath => {
                let (setup, instruction, accounts) = insert_setup(lang);
                let result = setup.mollusk.process_instruction(&instruction, &accounts);
                match &result.program_result {
                    MolluskResult::Success => CaseResult {
                        cu: result.compute_units_consumed,
                        error: None,
                    },
                    other => CaseResult {
                        cu: result.compute_units_consumed,
                        error: Some(format!("expected Success, got {:?}", other)),
                    },
                }
            }
        }
    }
}
