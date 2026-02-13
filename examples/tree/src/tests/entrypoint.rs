use super::*;
use mollusk_svm::program;
use solana_sdk::instruction::AccountMeta;

#[derive(Clone, Copy)]
pub(super) enum EntrypointCase {
    InvalidDiscriminator,
}

impl EntrypointCase {
    pub(super) const CASES: &'static [Self] = &[Self::InvalidDiscriminator];
}

impl TestCase for EntrypointCase {
    fn name(&self) -> &'static str {
        match self {
            Self::InvalidDiscriminator => "Invalid instruction discriminator",
        }
    }

    fn run(&self, lang: ProgramLanguage) -> CaseResult {
        match self {
            Self::InvalidDiscriminator => {
                let setup = setup_test(lang);
                let (system_program_pubkey, _) = program::keyed_account_for_system_program();

                let user_pubkey = Pubkey::new_unique();
                let tree_pubkey = Pubkey::new_unique();

                let instruction = Instruction::new_with_bytes(
                    setup.program_id,
                    &[255], // Invalid discriminator.
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
