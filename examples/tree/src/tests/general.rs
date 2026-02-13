use super::*;
use mollusk_svm::program;
use solana_sdk::instruction::AccountMeta;

fn general_setup(
    program_language: ProgramLanguage,
) -> (TestSetup, Instruction, Vec<(Pubkey, Account)>) {
    let setup = setup_test(program_language);
    let (system_program_pubkey, _) = program::keyed_account_for_system_program();

    let user_pubkey = Pubkey::new_unique();
    let tree_pubkey = Pubkey::new_unique();

    let instruction = Instruction::new_with_bytes(
        setup.program_id,
        &[],
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
        (tree_pubkey, Account::default()),
    ];

    (setup, instruction, accounts)
}

#[derive(Clone, Copy)]
pub(super) enum GeneralCase {
    UserDataLen,
    TreeDuplicate,
    InstructionDiscriminator,
}

impl GeneralCase {
    pub(super) const CASES: &'static [Self] = &[
        Self::UserDataLen,
        Self::TreeDuplicate,
        Self::InstructionDiscriminator,
    ];
}

impl TestCase for GeneralCase {
    fn name(&self) -> &'static str {
        match self {
            Self::UserDataLen => "User has nonzero data length",
            Self::TreeDuplicate => "Tree account is duplicate",
            Self::InstructionDiscriminator => "Invalid instruction discriminator",
        }
    }

    fn run(&self, lang: ProgramLanguage) -> CaseResult {
        match self {
            Self::UserDataLen => {
                let (setup, instruction, mut accounts) = general_setup(lang);
                accounts[AccountIndex::User as usize].1.data = vec![1u8; 1];
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::USER_DATA_LEN,
                )
            }
            Self::TreeDuplicate => {
                let (setup, mut instruction, mut accounts) = general_setup(lang);
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
            Self::InstructionDiscriminator => {
                let (setup, mut instruction, accounts) = general_setup(lang);
                instruction.data = vec![255]; // Invalid discriminator.
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::INSTRUCTION_DISCRIMINATOR,
                )
            }
        }
    }
}
