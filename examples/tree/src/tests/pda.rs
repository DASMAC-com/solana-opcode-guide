use super::*;
use mollusk_svm::program;
use solana_sdk::instruction::AccountMeta;

fn pda_init_setup(
    program_language: ProgramLanguage,
) -> (TestSetup, Instruction, Vec<(Pubkey, Account)>) {
    let setup = setup_test(program_language);
    let (system_program_pubkey, system_program_account) =
        program::keyed_account_for_system_program();

    let user_pubkey = Pubkey::new_unique();
    // Derive PDA with no seeds (matching CPI_N_SEEDS_TRY_FIND_PDA = 0).
    let (tree_pubkey, _bump) = Pubkey::find_program_address(&[], &setup.program_id);

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

/// Test PDA mismatch detection in each 8-byte chunk of the 32-byte pubkey.
pub(super) fn test_pda_mismatch_chunks(lang: ProgramLanguage) {
    const FINAL_BIT: usize = size_of::<u64>() - 1;

    let (setup, instruction, accounts) = pda_init_setup(lang);

    for chunk in 0..size_of::<Pubkey>() / size_of::<u64>() {
        let mut instruction = instruction.clone();
        let mut accounts = accounts.clone();

        // Flip the last bit of the chunk to create a mismatch.
        let flip_index = (chunk * size_of::<u64>()) + FINAL_BIT;
        accounts[AccountIndex::Tree as usize].0.as_mut()[flip_index] ^= 1;
        instruction.accounts[AccountIndex::Tree as usize].pubkey =
            accounts[AccountIndex::Tree as usize].0;

        let result = check_error(
            &setup,
            &instruction,
            &accounts,
            error_codes::error::PDA_MISMATCH,
        );
        assert!(
            result.error.is_none(),
            "PDA mismatch chunk {}: {}",
            chunk,
            result.error.unwrap()
        );
    }
}
