use super::*;
use mollusk_svm::program;
use solana_sdk::instruction::AccountMeta;

fn init_setup(
    program_language: ProgramLanguage,
) -> (TestSetup, Instruction, Vec<(Pubkey, Account)>) {
    let setup = setup_test(program_language);
    let (system_program_pubkey, system_program_account) =
        program::keyed_account_for_system_program();

    let user_pubkey = Pubkey::new_unique();
    let tree_pubkey = Pubkey::new_unique();

    let instruction = Instruction::new_with_bytes(
        setup.program_id,
        &[],
        vec![
            AccountMeta::new(user_pubkey, true),
            AccountMeta::new(tree_pubkey, false),
            AccountMeta::new_readonly(system_program_pubkey, false),
        ],
    );

    let accounts = vec![
        (
            user_pubkey,
            Account::new(USER_LAMPORTS, 0, &system_program_pubkey),
        ),
        (tree_pubkey, Account::new(0, 0, &system_program_pubkey)),
        (system_program_pubkey, system_program_account),
    ];

    (setup, instruction, accounts)
}

#[derive(Clone, Copy)]
pub(super) enum InitCase {
    UserDataLen,
    TreeDuplicate,
    TreeDataLen,
    SystemProgramDuplicate,
    SystemProgramDataLen,
    InstructionData,
}

impl InitCase {
    pub(super) const CASES: &'static [Self] = &[
        Self::UserDataLen,
        Self::TreeDuplicate,
        Self::TreeDataLen,
        Self::SystemProgramDuplicate,
        Self::SystemProgramDataLen,
        Self::InstructionData,
    ];
}

impl TestCase for InitCase {
    fn name(&self) -> &'static str {
        match self {
            Self::UserDataLen => "User has nonzero data length",
            Self::TreeDuplicate => "Tree account is duplicate",
            Self::TreeDataLen => "Tree has nonzero data length",
            Self::SystemProgramDuplicate => "System program is duplicate",
            Self::SystemProgramDataLen => "System program wrong data length",
            Self::InstructionData => "Non-empty instruction data",
        }
    }

    fn run(&self, lang: ProgramLanguage) -> CaseResult {
        match self {
            Self::UserDataLen => {
                let (setup, instruction, mut accounts) = init_setup(lang);
                accounts[AccountIndex::User as usize].1.data = vec![1u8; 1];
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::USER_DATA_LEN,
                )
            }
            Self::TreeDuplicate => {
                let (setup, mut instruction, mut accounts) = init_setup(lang);
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
            Self::TreeDataLen => {
                let (setup, instruction, mut accounts) = init_setup(lang);
                accounts[AccountIndex::Tree as usize].1.data = vec![1u8; 1];
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::TREE_DATA_LEN,
                )
            }
            Self::SystemProgramDuplicate => {
                let (setup, mut instruction, mut accounts) = init_setup(lang);
                instruction.accounts[AccountIndex::SystemProgram as usize] =
                    instruction.accounts[AccountIndex::User as usize].clone();
                accounts[AccountIndex::SystemProgram as usize] =
                    accounts[AccountIndex::User as usize].clone();
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::SYSTEM_PROGRAM_DUPLICATE,
                )
            }
            Self::SystemProgramDataLen => {
                let (setup, instruction, mut accounts) = init_setup(lang);
                accounts[AccountIndex::SystemProgram as usize].1.data = vec![];
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::SYSTEM_PROGRAM_DATA_LEN,
                )
            }
            Self::InstructionData => {
                let (setup, mut instruction, accounts) = init_setup(lang);
                instruction.data = vec![1u8; 1];
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::INSTRUCTION_DATA,
                )
            }
        }
    }
}
