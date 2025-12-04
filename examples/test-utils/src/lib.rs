use mollusk_svm::Mollusk;
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

#[doc(hidden)]
pub fn setup_test_impl(package_name: &str, implementation: ProgramLanguage) -> TestSetup {
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

#[macro_export]
macro_rules! setup_test {
    ($language:expr) => {
        $crate::setup_test_impl(env!("CARGO_PKG_NAME"), $language)
    };
}
