use super::*;
use solana_sdk::instruction::AccountMeta;

#[derive(Clone, Copy)]
pub(super) enum EntrypointCase {
    NoAccounts,
    OneAccount,
    ThreeAccounts,
    FiveAccounts,
}

impl EntrypointCase {
    pub(super) const CASES: &'static [Self] = &[
        Self::NoAccounts,
        Self::OneAccount,
        Self::ThreeAccounts,
        Self::FiveAccounts,
    ];

    const fn n_accounts(&self) -> usize {
        match self {
            Self::NoAccounts => 0,
            Self::OneAccount => 1,
            Self::ThreeAccounts => 3,
            Self::FiveAccounts => 5,
        }
    }
}

impl TestCase for EntrypointCase {
    fn name(&self) -> &'static str {
        match self {
            Self::NoAccounts => "No accounts",
            Self::OneAccount => "One account",
            Self::ThreeAccounts => "Three accounts",
            Self::FiveAccounts => "Five accounts",
        }
    }

    fn run(&self, lang: ProgramLanguage) -> CaseResult {
        let setup = setup_test(lang);

        let account_metas: Vec<AccountMeta> = (0..self.n_accounts())
            .map(|_| AccountMeta::new(Pubkey::new_unique(), false))
            .collect();
        let accounts: Vec<(Pubkey, Account)> = account_metas
            .iter()
            .map(|meta| (meta.pubkey, Account::default()))
            .collect();

        let instruction = Instruction::new_with_bytes(setup.program_id, &[], account_metas);
        check_error(
            &setup,
            &instruction,
            &accounts,
            error_codes::error::N_ACCOUNTS,
        )
    }
}
