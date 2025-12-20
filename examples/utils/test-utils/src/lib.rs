use mollusk_svm::Mollusk;
use solana_sdk::account::{Account, AccountSharedData};
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Keypair};
use solana_sdk::signer::Signer;

pub enum ProgramLanguage {
    Assembly,
    Rust,
}

pub struct TestSetup {
    pub keypair: Keypair,
    pub program_id: Pubkey,
    pub mollusk: Mollusk,
}

/// Sets up a test environment for the given program language.
///
/// The package name is inferred from the current working directory, which works because tests run
/// from the package root and package directories match their names.
///
/// ```ignore
/// let setup = setup_test(ProgramLanguage::Assembly);
/// ```
pub fn setup_test(implementation: ProgramLanguage) -> TestSetup {
    let package_name = std::env::current_dir()
        .expect("Failed to get current directory")
        .file_name()
        .expect("Failed to get directory name")
        .to_str()
        .expect("Directory name is not valid UTF-8")
        .to_string();

    let keypair = read_keypair_file(format!("deploy/{}-keypair.json", package_name))
        .expect("Failed to read keypair file");
    let program_id = keypair.pubkey();

    let program_path = match implementation {
        ProgramLanguage::Assembly => format!("deploy/{}", package_name),
        ProgramLanguage::Rust => {
            let binary_name = package_name.replace('-', "_");
            format!("../target/deploy/{}", binary_name)
        }
    };

    let mollusk = Mollusk::new(&program_id, &program_path);

    TestSetup {
        keypair,
        program_id,
        mollusk,
    }
}

pub fn single_mock_account() -> ((Pubkey, Account), Vec<AccountMeta>) {
    let mock_account_pubkey = Pubkey::new_unique();
    let mock_account_data = AccountSharedData::default().into();
    let accounts = vec![AccountMeta::new(mock_account_pubkey, false)];
    ((mock_account_pubkey, mock_account_data), accounts)
}
