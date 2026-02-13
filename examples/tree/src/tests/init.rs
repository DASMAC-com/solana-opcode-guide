use super::*;
use mollusk_svm::program;
use mollusk_svm::result::{Check, Config};
use pinocchio::sysvars::rent::Rent;
use solana_sdk::instruction::AccountMeta;
use tree_interface::{input_buffer, tree, TreeHeader};

const SIMD0194_EXEMPTION_THRESHOLD: f64 = 1.0;

/// Virtual address of the input buffer in the SVM memory map.
/// See `solana_sbpf::ebpf::MM_INPUT_START`.
const MM_INPUT_START: u64 = 0x400000000;

fn init_setup(
    program_language: ProgramLanguage,
) -> (TestSetup, Instruction, Vec<(Pubkey, Account)>) {
    let setup = setup_test(program_language);
    let (system_program_pubkey, system_program_account) =
        program::keyed_account_for_system_program();
    let (rent_sysvar_pubkey, rent_sysvar_account) =
        setup.mollusk.sysvars.keyed_account_for_rent_sysvar();

    let user_pubkey = Pubkey::new_unique();
    let tree_pubkey = Pubkey::new_unique();

    let instruction = Instruction::new_with_bytes(
        setup.program_id,
        &[],
        vec![
            AccountMeta::new(user_pubkey, true),
            AccountMeta::new(tree_pubkey, false),
            AccountMeta::new_readonly(system_program_pubkey, false),
            AccountMeta::new_readonly(rent_sysvar_pubkey, false),
        ],
    );

    let accounts = vec![
        (
            user_pubkey,
            Account::new(USER_LAMPORTS, 0, &system_program_pubkey),
        ),
        (tree_pubkey, Account::new(0, 0, &system_program_pubkey)),
        (system_program_pubkey, system_program_account),
        (rent_sysvar_pubkey, rent_sysvar_account),
    ];

    (setup, instruction, accounts)
}

fn pda_init_setup(
    program_language: ProgramLanguage,
) -> (TestSetup, Instruction, Vec<(Pubkey, Account)>) {
    let mut setup = setup_test(program_language);
    setup.mollusk.sysvars.rent.exemption_threshold = SIMD0194_EXEMPTION_THRESHOLD;
    let (system_program_pubkey, system_program_account) =
        program::keyed_account_for_system_program();
    let (rent_sysvar_pubkey, rent_sysvar_account) =
        setup.mollusk.sysvars.keyed_account_for_rent_sysvar();

    let user_pubkey = Pubkey::new_unique();
    let (tree_pubkey, _bump) = Pubkey::find_program_address(&[], &setup.program_id);

    let instruction = Instruction::new_with_bytes(
        setup.program_id,
        &[],
        vec![
            AccountMeta::new(user_pubkey, true),
            AccountMeta::new(tree_pubkey, false),
            AccountMeta::new_readonly(system_program_pubkey, false),
            AccountMeta::new_readonly(rent_sysvar_pubkey, false),
        ],
    );

    let accounts = vec![
        (
            user_pubkey,
            Account::new(USER_LAMPORTS, 0, &system_program_pubkey),
        ),
        (tree_pubkey, Account::new(0, 0, &system_program_pubkey)),
        (system_program_pubkey, system_program_account),
        (rent_sysvar_pubkey, rent_sysvar_account),
    ];

    (setup, instruction, accounts)
}

fn run_address_mismatch(
    lang: ProgramLanguage,
    account_index: usize,
    word_index: usize,
    word_size: usize,
    expected_error: error_codes::error,
) -> CaseResult {
    let (setup, mut instruction, mut accounts) = pda_init_setup(lang);

    let flip_index = (word_index * word_size) + (word_size - 1);
    accounts[account_index].0.as_mut()[flip_index] ^= 1;
    instruction.accounts[account_index].pubkey = accounts[account_index].0;

    check_error(&setup, &instruction, &accounts, expected_error)
}

#[derive(Clone, Copy)]
pub(super) enum InitCase {
    UserDataLen,
    TreeDuplicate,
    TreeDataLen,
    SystemProgramDuplicate,
    SystemProgramDataLen,
    RentDuplicate,
    RentAddressWord0,
    RentAddressWord1,
    RentAddressWord2,
    RentAddressWord3,
    RentAddressWord4,
    RentAddressWord5,
    RentAddressWord6,
    RentAddressWord7,
    InstructionData,
    PdaMismatchChunk0,
    PdaMismatchChunk1,
    PdaMismatchChunk2,
    PdaMismatchChunk3,
    UserInsufficientLamports,
    SystemProgramAddress,
    CreateAccountHappyPath,
}

impl InitCase {
    pub(super) const CASES: &'static [Self] = &[
        Self::UserDataLen,
        Self::TreeDuplicate,
        Self::TreeDataLen,
        Self::SystemProgramDuplicate,
        Self::SystemProgramDataLen,
        Self::RentDuplicate,
        Self::RentAddressWord0,
        Self::RentAddressWord1,
        Self::RentAddressWord2,
        Self::RentAddressWord3,
        Self::RentAddressWord4,
        Self::RentAddressWord5,
        Self::RentAddressWord6,
        Self::RentAddressWord7,
        Self::InstructionData,
    ];

    pub(super) const PDA_CASES: &'static [Self] = &[
        Self::PdaMismatchChunk0,
        Self::PdaMismatchChunk1,
        Self::PdaMismatchChunk2,
        Self::PdaMismatchChunk3,
    ];

    pub(super) const CPI_CASES: &'static [Self] = &[
        Self::SystemProgramAddress,
        Self::UserInsufficientLamports,
        Self::CreateAccountHappyPath,
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
            Self::RentDuplicate => "Rent sysvar is duplicate",
            Self::RentAddressWord0 => "Rent address mismatch word 0",
            Self::RentAddressWord1 => "Rent address mismatch word 1",
            Self::RentAddressWord2 => "Rent address mismatch word 2",
            Self::RentAddressWord3 => "Rent address mismatch word 3",
            Self::RentAddressWord4 => "Rent address mismatch word 4",
            Self::RentAddressWord5 => "Rent address mismatch word 5",
            Self::RentAddressWord6 => "Rent address mismatch word 6",
            Self::RentAddressWord7 => "Rent address mismatch word 7",
            Self::InstructionData => "Non-empty instruction data",
            Self::PdaMismatchChunk0 => "PDA mismatch chunk 1",
            Self::PdaMismatchChunk1 => "PDA mismatch chunk 2",
            Self::PdaMismatchChunk2 => "PDA mismatch chunk 3",
            Self::PdaMismatchChunk3 => "PDA mismatch chunk 4",
            Self::UserInsufficientLamports => "User has insufficient Lamports",
            Self::SystemProgramAddress => "System Program is wrong address",
            Self::CreateAccountHappyPath => "CreateAccount happy path",
        }
    }

    fn fixed_costs(&self) -> u64 {
        match self {
            // Input checks - no syscalls.
            Self::UserDataLen
            | Self::TreeDuplicate
            | Self::TreeDataLen
            | Self::SystemProgramDuplicate
            | Self::SystemProgramDataLen
            | Self::RentDuplicate
            | Self::RentAddressWord0
            | Self::RentAddressWord1
            | Self::RentAddressWord2
            | Self::RentAddressWord3
            | Self::RentAddressWord4
            | Self::RentAddressWord5
            | Self::RentAddressWord6
            | Self::RentAddressWord7
            | Self::InstructionData => 0,
            // PDA checks - sol_try_find_program_address only.
            Self::PdaMismatchChunk0
            | Self::PdaMismatchChunk1
            | Self::PdaMismatchChunk2
            | Self::PdaMismatchChunk3 => fixed_costs::CREATE_PROGRAM_ADDRESS,
            // CPI with system program not found (never executes).
            Self::SystemProgramAddress => {
                fixed_costs::CREATE_PROGRAM_ADDRESS + fixed_costs::CPI_BASE
            }
            // CPI with system program executing.
            Self::UserInsufficientLamports | Self::CreateAccountHappyPath => {
                fixed_costs::CREATE_PROGRAM_ADDRESS
                    + fixed_costs::CPI_BASE
                    + fixed_costs::SYSTEM_PROGRAM
            }
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
            Self::RentDuplicate => {
                let (setup, mut instruction, mut accounts) = init_setup(lang);
                instruction.accounts[AccountIndex::RentSysvar as usize] =
                    instruction.accounts[AccountIndex::User as usize].clone();
                accounts[AccountIndex::RentSysvar as usize] =
                    accounts[AccountIndex::User as usize].clone();
                check_error(
                    &setup,
                    &instruction,
                    &accounts,
                    error_codes::error::RENT_DUPLICATE,
                )
            }
            Self::RentAddressWord0 => run_address_mismatch(
                lang,
                AccountIndex::RentSysvar as usize,
                0,
                size_of::<u32>(),
                error_codes::error::RENT_ADDRESS,
            ),
            Self::RentAddressWord1 => run_address_mismatch(
                lang,
                AccountIndex::RentSysvar as usize,
                1,
                size_of::<u32>(),
                error_codes::error::RENT_ADDRESS,
            ),
            Self::RentAddressWord2 => run_address_mismatch(
                lang,
                AccountIndex::RentSysvar as usize,
                2,
                size_of::<u32>(),
                error_codes::error::RENT_ADDRESS,
            ),
            Self::RentAddressWord3 => run_address_mismatch(
                lang,
                AccountIndex::RentSysvar as usize,
                3,
                size_of::<u32>(),
                error_codes::error::RENT_ADDRESS,
            ),
            Self::RentAddressWord4 => run_address_mismatch(
                lang,
                AccountIndex::RentSysvar as usize,
                4,
                size_of::<u32>(),
                error_codes::error::RENT_ADDRESS,
            ),
            Self::RentAddressWord5 => run_address_mismatch(
                lang,
                AccountIndex::RentSysvar as usize,
                5,
                size_of::<u32>(),
                error_codes::error::RENT_ADDRESS,
            ),
            Self::RentAddressWord6 => run_address_mismatch(
                lang,
                AccountIndex::RentSysvar as usize,
                6,
                size_of::<u32>(),
                error_codes::error::RENT_ADDRESS,
            ),
            Self::RentAddressWord7 => run_address_mismatch(
                lang,
                AccountIndex::RentSysvar as usize,
                7,
                size_of::<u32>(),
                error_codes::error::RENT_ADDRESS,
            ),
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
            Self::PdaMismatchChunk0 => run_address_mismatch(
                lang,
                AccountIndex::Tree as usize,
                0,
                size_of::<u64>(),
                error_codes::error::PDA_MISMATCH,
            ),
            Self::PdaMismatchChunk1 => run_address_mismatch(
                lang,
                AccountIndex::Tree as usize,
                1,
                size_of::<u64>(),
                error_codes::error::PDA_MISMATCH,
            ),
            Self::PdaMismatchChunk2 => run_address_mismatch(
                lang,
                AccountIndex::Tree as usize,
                2,
                size_of::<u64>(),
                error_codes::error::PDA_MISMATCH,
            ),
            Self::PdaMismatchChunk3 => run_address_mismatch(
                lang,
                AccountIndex::Tree as usize,
                3,
                size_of::<u64>(),
                error_codes::error::PDA_MISMATCH,
            ),
            Self::UserInsufficientLamports => {
                let (setup, instruction, mut accounts) = pda_init_setup(lang);
                accounts[AccountIndex::User as usize].1.lamports = 0;
                let result = setup.mollusk.process_instruction(&instruction, &accounts);
                // SystemError::ResultWithNegativeLamports.
                let expected = ProgramError::Custom(1);
                match &result.program_result {
                    MolluskResult::Failure(err) if *err == expected => CaseResult {
                        cu: result.compute_units_consumed,
                        error: None,
                    },
                    other => CaseResult {
                        cu: result.compute_units_consumed,
                        error: Some(format!("expected Failure({:?}), got {:?}", expected, other)),
                    },
                }
            }
            Self::SystemProgramAddress => {
                let (setup, mut instruction, mut accounts) = pda_init_setup(lang);
                let fake_pubkey = Pubkey::new_unique();
                accounts[AccountIndex::SystemProgram as usize].0 = fake_pubkey;
                instruction.accounts[AccountIndex::SystemProgram as usize].pubkey = fake_pubkey;
                let result = setup.mollusk.process_instruction(&instruction, &accounts);
                let expected = ProgramError::NotEnoughAccountKeys;
                match &result.program_result {
                    MolluskResult::Failure(err) if *err == expected => CaseResult {
                        cu: result.compute_units_consumed,
                        error: None,
                    },
                    other => CaseResult {
                        cu: result.compute_units_consumed,
                        error: Some(format!("expected Failure({:?}), got {:?}", expected, other)),
                    },
                }
            }
            Self::CreateAccountHappyPath => {
                let (setup, instruction, accounts) = pda_init_setup(lang);
                let result = setup.mollusk.process_instruction(&instruction, &accounts);
                match &result.program_result {
                    MolluskResult::Success => {
                        let tree = &result.resulting_accounts[AccountIndex::Tree as usize].1;
                        let rent_data = &accounts[AccountIndex::RentSysvar as usize].1.data;
                        let rent = Rent::from_bytes(rent_data).unwrap();
                        let expected_lamports =
                            rent.try_minimum_balance(cpi::TREE_DATA_LEN).unwrap();
                        let mut errors = Vec::new();
                        if tree.owner != setup.program_id {
                            errors.push(format!(
                                "owner: expected {:?}, got {:?}",
                                setup.program_id, tree.owner
                            ));
                        }
                        if tree.data.len() != cpi::TREE_DATA_LEN {
                            errors.push(format!(
                                "data len: expected {}, got {}",
                                cpi::TREE_DATA_LEN,
                                tree.data.len()
                            ));
                        }
                        if tree.lamports != expected_lamports {
                            errors.push(format!(
                                "lamports: expected {}, got {}",
                                expected_lamports, tree.lamports
                            ));
                        }
                        let expected_next = MM_INPUT_START
                            + input_buffer::TREE_DATA_OFF as u64
                            + size_of::<TreeHeader>() as u64;
                        let header = unsafe { &*(tree.data.as_ptr() as *const TreeHeader) };
                        let actual_next = header.next as u64;
                        if actual_next != expected_next {
                            errors.push(format!(
                                "next: expected {:#x}, got {:#x}",
                                expected_next, actual_next
                            ));
                        }
                        let config = Config {
                            panic: false,
                            verbose: false,
                        };
                        if !result.run_checks(&[Check::all_rent_exempt()], &config, &setup.mollusk)
                        {
                            errors.push("not all accounts are rent exempt".to_string());
                        }
                        CaseResult {
                            cu: result.compute_units_consumed,
                            error: if errors.is_empty() {
                                None
                            } else {
                                Some(errors.join("; "))
                            },
                        }
                    }
                    other => CaseResult {
                        cu: result.compute_units_consumed,
                        error: Some(format!("expected Success, got {:?}", other)),
                    },
                }
            }
        }
    }
}
