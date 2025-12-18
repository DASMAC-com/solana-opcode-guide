use mollusk_svm::result::Check;
use solana_sdk::account::AccountSharedData;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use test_utils::{setup_test, ProgramLanguage};

#[test]
fn test_asm() {
    let setup = setup_test(ProgramLanguage::Assembly);
}
