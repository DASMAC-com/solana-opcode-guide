use crate::constants::constants;
use mollusk_svm::program;
use mollusk_svm::result::Check;
use solana_rent::ACCOUNT_STORAGE_OVERHEAD;
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use std::mem::{offset_of, size_of};
use std::{fs, vec};
use test_utils::{setup_test, ProgramLanguage};

#[test]
fn test_asm_file_constants() {
    const GLOBAL_ENTRYPOINT: &str = ".global entrypoint";

    // Parse assembly file.
    let asm_path = setup_test(ProgramLanguage::Assembly)
        .asm_source_path
        .expect("Assembly source file not found");
    let content = fs::read_to_string(&asm_path).expect("Failed to read assembly file");
    let global_pos = content
        .find(GLOBAL_ENTRYPOINT)
        .expect("Could not find '.global entrypoint' in assembly file");

    // Overwrite assembly file with updated constants, asserting nothing changed.
    let after_global = &content[global_pos..];
    let new_content = format!("{}\n{}", constants().to_asm(), after_global);
    let changed = new_content != content;
    fs::write(&asm_path, new_content).expect("Failed to write assembly file");
    assert!(
        !changed,
        "Assembly file constants were out of date and have been updated. Please re-run the test."
    );
}

const USER_STARTING_LAMPORTS: u64 = 1_000_000;

enum Operation {
    Initialize,
    Increment,
}

#[repr(C, packed)]
struct CounterAccountData {
    counter: u64,
    bump_seed: u8,
}

#[repr(C, packed)]
struct CounterAccount {
    pubkey: Pubkey,
    owner: Pubkey,
    lamports: u64,
    data: CounterAccountData,
}

impl CounterAccount {
    fn check(&self) -> Check<'_> {
        Check::account(&self.pubkey)
            .data(self.data.as_bytes())
            .lamports(self.lamports)
            .space(size_of::<CounterAccountData>())
            .owner(&self.owner)
            .build()
    }
}

impl CounterAccountData {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
}

enum AccountIndex {
    User = 0,
    Pda = 1,
    SystemProgram = 2,
}

struct ComputeUnits {
    asm: u64,
    rs: u64,
}

#[derive(Clone, Copy)]
enum Case {
    // Initialize error cases (in ASM execution order).
    InitializeNoAccounts,
    InitializeTooManyAccounts,
    InitializeUserDataLen,
    InitializePdaDuplicate,
    InitializePdaDataLen,
    InitializeSystemProgramDuplicate,
    InitializeSystemProgramDataLen,
    InitializePdaMismatch,
    InitializeHappyPath,

    // Increment error cases (in ASM execution order).
    IncrementPdaDuplicate,
    IncrementPdaDataLen,
    IncrementNoInstructionData,
    IncrementUnableToDerivePda,
    IncrementPdaMismatch,
    IncrementHappyPath,
}

impl ComputeUnits {
    fn for_lang(&self, lang: &ProgramLanguage) -> u64 {
        match lang {
            ProgramLanguage::Assembly => self.asm,
            ProgramLanguage::Rust => self.rs,
        }
    }
}

impl Case {
    const fn get(self) -> ComputeUnits {
        match self {
            // Initialize
            Self::InitializeNoAccounts => ComputeUnits { asm: 5, rs: 6 },
            Self::InitializeTooManyAccounts => ComputeUnits { asm: 5, rs: 6 },
            Self::InitializeUserDataLen => ComputeUnits { asm: 7, rs: 9 },
            Self::InitializePdaDuplicate => ComputeUnits { asm: 9, rs: 16 },
            Self::InitializePdaDataLen => ComputeUnits { asm: 11, rs: 19 },
            Self::InitializeSystemProgramDuplicate => ComputeUnits { asm: 13, rs: 26 },
            Self::InitializeSystemProgramDataLen => ComputeUnits { asm: 15, rs: 29 },
            Self::InitializePdaMismatch => ComputeUnits {
                asm: 1543,
                rs: 1561,
            },
            Self::InitializeHappyPath => ComputeUnits {
                asm: 2834,
                rs: 2913,
            },

            // Increment
            Self::IncrementPdaDuplicate => ComputeUnits { asm: 10, rs: 16 },
            Self::IncrementPdaDataLen => ComputeUnits { asm: 12, rs: 19 },
            Self::IncrementNoInstructionData => ComputeUnits { asm: 14, rs: 26 },
            Self::IncrementUnableToDerivePda => ComputeUnits {
                asm: 1535,
                rs: 1558,
            },
            Self::IncrementPdaMismatch => ComputeUnits {
                asm: 1540,
                rs: 1565,
            },
            Self::IncrementHappyPath => ComputeUnits {
                asm: 1548,
                rs: 1575,
            },
        }
    }

    const fn name(&self) -> &'static str {
        match self {
            Self::InitializeNoAccounts => "No accounts",
            Self::InitializeTooManyAccounts => "Too many accounts",
            Self::InitializeUserDataLen => "User data len",
            Self::InitializePdaDuplicate => "PDA duplicate",
            Self::InitializePdaDataLen => "PDA data len",
            Self::InitializeSystemProgramDuplicate => "System program duplicate",
            Self::InitializeSystemProgramDataLen => "System program data len",
            Self::InitializePdaMismatch => "PDA mismatch",
            Self::InitializeHappyPath => "Happy path",
            Self::IncrementPdaDuplicate => "PDA duplicate",
            Self::IncrementPdaDataLen => "PDA data len",
            Self::IncrementNoInstructionData => "No instruction data",
            Self::IncrementUnableToDerivePda => "Unable to derive PDA",
            Self::IncrementPdaMismatch => "PDA mismatch",
            Self::IncrementHappyPath => "Happy path",
        }
    }

    const INITIALIZE_CASES: &'static [Case] = &[
        Case::InitializeNoAccounts,
        Case::InitializeTooManyAccounts,
        Case::InitializeUserDataLen,
        Case::InitializePdaDuplicate,
        Case::InitializePdaDataLen,
        Case::InitializeSystemProgramDuplicate,
        Case::InitializeSystemProgramDataLen,
        Case::InitializePdaMismatch,
        Case::InitializeHappyPath,
    ];

    const INCREMENT_CASES: &'static [Case] = &[
        Case::IncrementPdaDuplicate,
        Case::IncrementPdaDataLen,
        Case::IncrementNoInstructionData,
        Case::IncrementUnableToDerivePda,
        Case::IncrementPdaMismatch,
        Case::IncrementHappyPath,
    ];

    fn generate_markdown_table(title: &str, cases: &[Case]) -> String {
        let mut table = format!("### {}\n\n", title);
        table.push_str("| Case | ASM (CUs) | Rust (CUs) | Overhead | Overhead % |\n");
        table.push_str("|------|-----------|------------|----------|------------|\n");

        for case in cases {
            let cu = case.get();
            let overhead = cu.rs as i64 - cu.asm as i64;
            let overhead_pct = if cu.asm > 0 {
                (overhead as f64 / cu.asm as f64) * 100.0
            } else {
                0.0
            };
            table.push_str(&format!(
                "| {} | {} | {} | {:+} | {:+.1}% |\n",
                case.name(),
                cu.asm,
                cu.rs,
                overhead,
                overhead_pct
            ));
        }

        table
    }

    fn markdown_tables() -> String {
        let mut output = String::new();
        output.push_str(&Self::generate_markdown_table(
            "Initialize",
            Self::INITIALIZE_CASES,
        ));
        output.push('\n');
        output.push_str(&Self::generate_markdown_table(
            "Increment",
            Self::INCREMENT_CASES,
        ));
        output
    }
}

fn happy_path_setup(
    program_language: ProgramLanguage,
    operation: Operation,
) -> (
    test_utils::TestSetup,
    Instruction,
    Vec<(Pubkey, Account)>,
    CounterAccount,
) {
    let setup = setup_test(program_language);
    let (system_program, system_account) = program::keyed_account_for_system_program();

    let user_pubkey = Pubkey::new_unique();
    let (pda_pubkey, bump_seed) =
        Pubkey::find_program_address(&[user_pubkey.as_ref()], &setup.program_id);

    let mut instruction = Instruction::new_with_bytes(
        setup.program_id,
        &[],
        vec![
            AccountMeta::new(user_pubkey, true),
            AccountMeta::new(pda_pubkey, false),
        ],
    );

    let mut accounts = vec![
        (
            instruction.accounts[AccountIndex::User as usize].pubkey,
            Account::new(USER_STARTING_LAMPORTS, 0, &system_program),
        ),
        (
            instruction.accounts[AccountIndex::Pda as usize].pubkey,
            Account::new(0, 0, &system_program),
        ),
    ];

    let counter_account = CounterAccount {
        pubkey: pda_pubkey,
        owner: setup.program_id,
        lamports: setup.mollusk.sysvars.rent.lamports_per_byte_year
            * ((size_of::<CounterAccountData>() as u64) + ACCOUNT_STORAGE_OVERHEAD),
        data: CounterAccountData {
            counter: 0,
            bump_seed,
        },
    };

    match operation {
        Operation::Initialize => {
            instruction
                .accounts
                .push(AccountMeta::new_readonly(system_program, false));
            accounts.push((system_program, system_account));
        }
        Operation::Increment => {
            let counter_account_info = &mut accounts[AccountIndex::Pda as usize].1;
            counter_account_info.lamports = counter_account.lamports;
            counter_account_info.data = counter_account.data.as_bytes().to_vec().clone();
            counter_account_info.owner = setup.program_id;
        }
    }

    (setup, instruction, accounts, counter_account)
}

fn test_no_accounts(lang: ProgramLanguage) {
    let cu = Case::InitializeNoAccounts.get().for_lang(&lang);
    let (setup, mut instruction, mut accounts, _) = happy_path_setup(lang, Operation::Initialize);

    instruction.accounts.clear();
    accounts.clear();

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[
            Check::err(ProgramError::Custom(constants().get("E_N_ACCOUNTS") as u32)),
            Check::compute_units(cu),
        ],
    );
}

fn test_too_many_accounts(lang: ProgramLanguage) {
    let cu = Case::InitializeTooManyAccounts.get().for_lang(&lang);
    let (setup, mut instruction, mut accounts, _) = happy_path_setup(lang, Operation::Initialize);

    instruction
        .accounts
        .push(AccountMeta::new_readonly(Pubkey::new_unique(), false));
    accounts.push((
        instruction.accounts.last().unwrap().pubkey,
        Account::default(),
    ));

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[
            Check::err(ProgramError::Custom(constants().get("E_N_ACCOUNTS") as u32)),
            Check::compute_units(cu),
        ],
    );
}

fn test_initialize_user_data_len(lang: ProgramLanguage) {
    let cu = Case::InitializeUserDataLen.get().for_lang(&lang);
    let (setup, instruction, mut accounts, _) = happy_path_setup(lang, Operation::Initialize);

    accounts[AccountIndex::User as usize].1.data = vec![1u8; 1];

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[
            Check::err(ProgramError::Custom(
                constants().get("E_USER_DATA_LEN") as u32
            )),
            Check::compute_units(cu),
        ],
    );
}

fn test_initialize_pda_duplicate(lang: ProgramLanguage) {
    let cu = Case::InitializePdaDuplicate.get().for_lang(&lang);
    let (setup, mut instruction, mut accounts, _) = happy_path_setup(lang, Operation::Initialize);

    instruction.accounts[AccountIndex::Pda as usize] =
        instruction.accounts[AccountIndex::User as usize].clone();
    accounts[AccountIndex::Pda as usize] = accounts[AccountIndex::User as usize].clone();

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[
            Check::err(ProgramError::Custom(
                constants().get("E_PDA_DUPLICATE") as u32
            )),
            Check::compute_units(cu),
        ],
    );
}

fn test_initialize_pda_data_len(lang: ProgramLanguage) {
    let cu = Case::InitializePdaDataLen.get().for_lang(&lang);
    let (setup, instruction, mut accounts, _) = happy_path_setup(lang, Operation::Initialize);

    accounts[AccountIndex::Pda as usize].1.data = vec![1u8; 1];

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[
            Check::err(ProgramError::Custom(
                constants().get("E_PDA_DATA_LEN") as u32
            )),
            Check::compute_units(cu),
        ],
    );
}

fn test_initialize_system_program_duplicate(lang: ProgramLanguage) {
    let cu = Case::InitializeSystemProgramDuplicate.get().for_lang(&lang);
    let (setup, mut instruction, mut accounts, _) = happy_path_setup(lang, Operation::Initialize);

    instruction.accounts[AccountIndex::SystemProgram as usize] =
        instruction.accounts[AccountIndex::User as usize].clone();
    accounts[AccountIndex::SystemProgram as usize] = accounts[AccountIndex::User as usize].clone();

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[
            Check::err(ProgramError::Custom(
                constants().get("E_SYSTEM_PROGRAM_DUPLICATE") as u32,
            )),
            Check::compute_units(cu),
        ],
    );
}

fn test_initialize_system_program_data_len(lang: ProgramLanguage) {
    let cu = Case::InitializeSystemProgramDataLen.get().for_lang(&lang);
    let (setup, instruction, mut accounts, _) = happy_path_setup(lang, Operation::Initialize);

    accounts[AccountIndex::SystemProgram as usize].1.data = vec![];

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[
            Check::err(ProgramError::Custom(
                constants().get("E_SYSTEM_PROGRAM_DATA_LEN") as u32,
            )),
            Check::compute_units(cu),
        ],
    );
}

fn test_initialize_pda_mismatch(lang: ProgramLanguage) {
    let cu = Case::InitializePdaMismatch.get().for_lang(&lang);
    let (setup, instruction, accounts, _) = happy_path_setup(lang, Operation::Initialize);

    test_pda_mismatch_chunks(&setup, instruction, accounts, cu, None);
}

fn test_initialize_happy_path(lang: ProgramLanguage) {
    let cu = Case::InitializeHappyPath.get().for_lang(&lang);
    let (setup, instruction, accounts, counter_account) =
        happy_path_setup(lang, Operation::Initialize);

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[
            Check::success(),
            counter_account.check(),
            Check::compute_units(cu),
        ],
    );
}

fn test_increment_pda_duplicate(lang: ProgramLanguage) {
    let cu = Case::IncrementPdaDuplicate.get().for_lang(&lang);
    let (setup, mut instruction, mut accounts, _) = happy_path_setup(lang, Operation::Increment);

    instruction.accounts[AccountIndex::Pda as usize] =
        instruction.accounts[AccountIndex::User as usize].clone();
    accounts[AccountIndex::Pda as usize] = accounts[AccountIndex::User as usize].clone();

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[
            Check::err(ProgramError::Custom(
                constants().get("E_PDA_DUPLICATE") as u32
            )),
            Check::compute_units(cu),
        ],
    );
}

fn test_increment_pda_data_len(lang: ProgramLanguage) {
    let cu = Case::IncrementPdaDataLen.get().for_lang(&lang);
    let (setup, instruction, mut accounts, _) = happy_path_setup(lang, Operation::Increment);

    accounts[AccountIndex::Pda as usize].1.data = vec![1u8; 1];

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[
            Check::err(ProgramError::Custom(
                constants().get("E_PDA_DATA_LEN") as u32
            )),
            Check::compute_units(cu),
        ],
    );
}

fn test_increment_no_instruction_data(lang: ProgramLanguage) {
    let cu = Case::IncrementNoInstructionData.get().for_lang(&lang);
    let (setup, instruction, accounts, _) = happy_path_setup(lang, Operation::Increment);

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[
            Check::err(ProgramError::Custom(
                constants().get("E_INVALID_INSTRUCTION_DATA_LEN") as u32,
            )),
            Check::compute_units(cu),
        ],
    );
}

fn test_increment_unable_to_derive_pda(lang: ProgramLanguage) {
    let cu = Case::IncrementUnableToDerivePda.get().for_lang(&lang);
    let (setup, mut instruction, mut accounts, _) = happy_path_setup(lang, Operation::Increment);

    instruction.data = 1u64.to_le_bytes().to_vec();

    // Find a user pubkey whose PDA bump is < u8::MAX, so bump + 1 is guaranteed to fail since
    // find_program_address already rejected it.
    let mut user_pubkey = accounts[AccountIndex::User as usize].0;
    let (mut pda_pubkey, mut bump_seed) =
        Pubkey::find_program_address(&[user_pubkey.as_ref()], &setup.program_id);
    while bump_seed == u8::MAX {
        user_pubkey = Pubkey::new_unique();
        (pda_pubkey, bump_seed) =
            Pubkey::find_program_address(&[user_pubkey.as_ref()], &setup.program_id);
    }

    // Update account keys and set bump seed + 1 in PDA account data.
    instruction.accounts[AccountIndex::User as usize].pubkey = user_pubkey;
    instruction.accounts[AccountIndex::Pda as usize].pubkey = pda_pubkey;
    accounts[AccountIndex::User as usize].0 = user_pubkey;
    accounts[AccountIndex::Pda as usize].0 = pda_pubkey;
    accounts[AccountIndex::Pda as usize].1.data[offset_of!(CounterAccountData, bump_seed)] =
        bump_seed + 1;

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[
            Check::err(ProgramError::Custom(
                constants().get("E_UNABLE_TO_DERIVE_PDA") as u32,
            )),
            Check::compute_units(cu),
        ],
    );
}

fn test_increment_pda_mismatch(lang: ProgramLanguage) {
    let cu = Case::IncrementPdaMismatch.get().for_lang(&lang);
    let (setup, instruction, accounts, _) = happy_path_setup(lang, Operation::Increment);

    test_pda_mismatch_chunks(
        &setup,
        instruction,
        accounts,
        cu,
        Some(1u64.to_le_bytes().to_vec()),
    );
}

/// Helper for testing PDA mismatch detection in each 8-byte chunk of the 32-byte pubkey.
fn test_pda_mismatch_chunks(
    setup: &test_utils::TestSetup,
    instruction: Instruction,
    accounts: Vec<(Pubkey, Account)>,
    base_cu: u64,
    instruction_data: Option<Vec<u8>>,
) {
    const CHUNK_INCREMENT: [u64; size_of::<Pubkey>() / size_of::<u64>()] = [0, 3, 6, 9];
    const FINAL_BIT: usize = size_of::<u64>() - 1;

    for (chunk, &increment) in CHUNK_INCREMENT.iter().enumerate() {
        let mut instruction = instruction.clone();
        let mut accounts = accounts.clone();

        if let Some(ref data) = instruction_data {
            instruction.data = data.clone();
        }

        // Flip the last bit of the chunk to create a mismatch.
        let flip_index = (chunk * size_of::<u64>()) + FINAL_BIT;
        accounts[AccountIndex::Pda as usize].0.as_mut()[flip_index] ^= 1;
        instruction.accounts[AccountIndex::Pda as usize].pubkey =
            accounts[AccountIndex::Pda as usize].0;

        setup.mollusk.process_and_validate_instruction(
            &instruction,
            &accounts,
            &[
                Check::err(ProgramError::Custom(
                    constants().get("E_PDA_MISMATCH") as u32
                )),
                Check::compute_units(base_cu + increment),
            ],
        );
    }
}

struct IncrementTestCase {
    user_account_data_length: u64,
    starting_counter: u64,
    increment: u64,
}

const INCREMENT_TEST_CASES: &[IncrementTestCase] = &[
    // Aligned user data lengths.
    IncrementTestCase {
        user_account_data_length: 0,
        starting_counter: 0,
        increment: 1,
    },
    IncrementTestCase {
        user_account_data_length: 0,
        starting_counter: 0,
        increment: u64::MAX,
    },
    IncrementTestCase {
        user_account_data_length: 0,
        starting_counter: u64::MAX,
        increment: 1,
    },
    IncrementTestCase {
        user_account_data_length: 0,
        starting_counter: u64::MAX,
        increment: u64::MAX,
    },
    IncrementTestCase {
        user_account_data_length: 8,
        starting_counter: 0,
        increment: 1,
    },
    IncrementTestCase {
        user_account_data_length: 16,
        starting_counter: 1,
        increment: 1,
    },
    IncrementTestCase {
        user_account_data_length: 128,
        starting_counter: 100,
        increment: 200,
    },
    // Unaligned user data lengths.
    IncrementTestCase {
        user_account_data_length: 1,
        starting_counter: 0,
        increment: 1,
    },
    IncrementTestCase {
        user_account_data_length: 7,
        starting_counter: 1,
        increment: u64::MAX - 1,
    },
    IncrementTestCase {
        user_account_data_length: 9,
        starting_counter: 100,
        increment: 200,
    },
    IncrementTestCase {
        user_account_data_length: 15,
        starting_counter: u64::MAX,
        increment: 1,
    },
    IncrementTestCase {
        user_account_data_length: 100,
        starting_counter: u64::MAX,
        increment: u64::MAX,
    },
];

fn test_increment_happy_path(lang: ProgramLanguage) {
    let cu = Case::IncrementHappyPath.get().for_lang(&lang);
    for tc in INCREMENT_TEST_CASES {
        let (setup, mut instruction, mut accounts, mut counter_account) =
            happy_path_setup(lang, Operation::Increment);

        instruction.data = tc.increment.to_le_bytes().to_vec();

        accounts[AccountIndex::User as usize].1.data =
            vec![0u8; tc.user_account_data_length as usize];

        accounts[AccountIndex::Pda as usize].1.data[..size_of::<u64>()]
            .copy_from_slice(&tc.starting_counter.to_le_bytes());

        counter_account.data.counter = tc.starting_counter.wrapping_add(tc.increment);

        setup.mollusk.process_and_validate_instruction(
            &instruction,
            &accounts,
            &[
                Check::success(),
                counter_account.check(),
                Check::compute_units(cu),
            ],
        );
    }
}

#[test]
fn test_asm_no_accounts() {
    test_no_accounts(ProgramLanguage::Assembly);
}

#[test]
fn test_rs_no_accounts() {
    test_no_accounts(ProgramLanguage::Rust);
}

#[test]
fn test_asm_too_many_accounts() {
    test_too_many_accounts(ProgramLanguage::Assembly);
}

#[test]
fn test_rs_too_many_accounts() {
    test_too_many_accounts(ProgramLanguage::Rust);
}

#[test]
fn test_asm_initialize_user_data_len() {
    test_initialize_user_data_len(ProgramLanguage::Assembly);
}

#[test]
fn test_rs_initialize_user_data_len() {
    test_initialize_user_data_len(ProgramLanguage::Rust);
}

#[test]
fn test_asm_initialize_pda_duplicate() {
    test_initialize_pda_duplicate(ProgramLanguage::Assembly);
}

#[test]
fn test_rs_initialize_pda_duplicate() {
    test_initialize_pda_duplicate(ProgramLanguage::Rust);
}

#[test]
fn test_asm_initialize_pda_data_len() {
    test_initialize_pda_data_len(ProgramLanguage::Assembly);
}

#[test]
fn test_rs_initialize_pda_data_len() {
    test_initialize_pda_data_len(ProgramLanguage::Rust);
}

#[test]
fn test_asm_initialize_system_program_duplicate() {
    test_initialize_system_program_duplicate(ProgramLanguage::Assembly);
}

#[test]
fn test_rs_initialize_system_program_duplicate() {
    test_initialize_system_program_duplicate(ProgramLanguage::Rust);
}

#[test]
fn test_asm_initialize_system_program_data_len() {
    test_initialize_system_program_data_len(ProgramLanguage::Assembly);
}

#[test]
fn test_rs_initialize_system_program_data_len() {
    test_initialize_system_program_data_len(ProgramLanguage::Rust);
}

#[test]
fn test_asm_initialize_pda_mismatch() {
    test_initialize_pda_mismatch(ProgramLanguage::Assembly);
}

#[test]
fn test_rs_initialize_pda_mismatch() {
    test_initialize_pda_mismatch(ProgramLanguage::Rust);
}

#[test]
fn test_asm_initialize_happy_path() {
    test_initialize_happy_path(ProgramLanguage::Assembly);
}

#[test]
fn test_rs_initialize_happy_path() {
    test_initialize_happy_path(ProgramLanguage::Rust);
}

#[test]
fn test_pad_masking() {
    let increment = 7;
    let mask_immediate = -8i32; // Assembly immediate.
    let mask = (mask_immediate as i64) as u64; // VM interpretation.
    let hex = 0xffff_ffff_ffff_fff8u64;
    let binary = 0b1111111111111111111111111111111111111111111111111111111111111000u64;
    assert_eq!(mask, hex);
    assert_eq!(mask, u64::MAX - 7u64);
    assert_eq!(mask, binary);

    let padded_data_len = |data_len: u64| -> u64 { (data_len + increment) & mask };

    assert_eq!(padded_data_len(0), 0);
    assert_eq!(padded_data_len(1), 8);
    assert_eq!(padded_data_len(8), 8);
    assert_eq!(padded_data_len(9), 16);
    assert_eq!(padded_data_len(15), 16);
}

#[test]
fn test_asm_increment_pda_duplicate() {
    test_increment_pda_duplicate(ProgramLanguage::Assembly);
}

#[test]
fn test_rs_increment_pda_duplicate() {
    test_increment_pda_duplicate(ProgramLanguage::Rust);
}

#[test]
fn test_asm_increment_pda_data_len() {
    test_increment_pda_data_len(ProgramLanguage::Assembly);
}

#[test]
fn test_rs_increment_pda_data_len() {
    test_increment_pda_data_len(ProgramLanguage::Rust);
}

#[test]
fn test_asm_increment_no_instruction_data() {
    test_increment_no_instruction_data(ProgramLanguage::Assembly);
}

#[test]
fn test_rs_increment_no_instruction_data() {
    test_increment_no_instruction_data(ProgramLanguage::Rust);
}

#[test]
fn test_asm_increment_unable_to_derive_pda() {
    test_increment_unable_to_derive_pda(ProgramLanguage::Assembly);
}

#[test]
fn test_rs_increment_unable_to_derive_pda() {
    test_increment_unable_to_derive_pda(ProgramLanguage::Rust);
}

#[test]
fn test_asm_increment_pda_mismatch() {
    test_increment_pda_mismatch(ProgramLanguage::Assembly);
}

#[test]
fn test_rs_increment_pda_mismatch() {
    test_increment_pda_mismatch(ProgramLanguage::Rust);
}

#[test]
fn test_asm_increment_happy_path() {
    test_increment_happy_path(ProgramLanguage::Assembly);
}

#[test]
fn test_rs_increment_happy_path() {
    test_increment_happy_path(ProgramLanguage::Rust);
}

#[test]
fn test_print_compute_units_tables() {
    println!("\n{}", Case::markdown_tables());
}
