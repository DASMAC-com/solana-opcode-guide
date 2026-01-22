use mollusk_svm::program;
use mollusk_svm::result::Check;
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use std::fs;
use std::mem::{offset_of, size_of};
use test_utils::{setup_test, ProgramLanguage};

/// In an assembly file.
const LINE_LENGTH: usize = 75;

// Individual constant definition.
struct Constant {
    name: &'static str,
    value: u64,
    is_offset: bool,
    is_hex: bool,
    comment: Comment,
}

impl Constant {
    const OFFSET_SUFFIX: &str = "_OFF";

    fn create(
        name: &'static str,
        value: u64,
        is_offset: bool,
        is_hex: bool,
        comment: &'static str,
    ) -> Self {
        assert!(
            !name.ends_with(Self::OFFSET_SUFFIX),
            "Constant name must not end with {} (added automatically for offsets): {name}",
            Self::OFFSET_SUFFIX
        );
        if is_offset {
            assert!(
                value <= i16::MAX as u64,
                "Offset value must fit in i16: {name} = {value}"
            );
        }
        Self {
            name,
            value,
            is_offset,
            is_hex,
            comment: Comment::new(comment),
        }
    }

    fn new(name: &'static str, value: u64, comment: &'static str) -> Self {
        Self::create(name, value, false, false, comment)
    }

    fn new_hex(name: &'static str, value: u64, comment: &'static str) -> Self {
        Self::create(name, value, false, true, comment)
    }

    fn new_offset(name: &'static str, value: u64, comment: &'static str) -> Self {
        Self::create(name, value, true, false, comment)
    }

    fn asm_name(&self) -> String {
        if self.is_offset {
            format!("{}{}", self.name, Self::OFFSET_SUFFIX)
        } else {
            self.name.to_string()
        }
    }
}

// Error code definition.
struct ErrorCode {
    name: &'static str,
    comment: Comment,
}

impl ErrorCode {
    const PREFIX: &str = "E_";

    fn new(name: &'static str, comment: &'static str) -> Self {
        Self {
            name,
            comment: Comment::new(comment),
        }
    }

    fn asm_name(&self) -> String {
        format!("{}{}", Self::PREFIX, self.name)
    }
}

// Group of related constants.
enum ConstantGroup {
    // Standard group of constants with optional prefix.
    Standard {
        comment: Comment,
        constants: Vec<Constant>,
        prefix: Option<&'static str>,
    },
    // Error codes group where values are auto-incremented starting from 1.
    ErrorCodes {
        comment: Comment,
        codes: Vec<ErrorCode>,
    },
}

impl ConstantGroup {
    fn new(comment: &'static str) -> Self {
        Self::Standard {
            comment: Comment::new(comment),
            constants: Vec::new(),
            prefix: None,
        }
    }

    fn new_with_prefix(comment: &'static str, prefix: &'static str) -> Self {
        Self::Standard {
            comment: Comment::new(comment),
            constants: Vec::new(),
            prefix: Some(prefix),
        }
    }

    fn new_error_codes() -> Self {
        Self::ErrorCodes {
            comment: Comment::new("Error codes."),
            codes: Vec::new(),
        }
    }

    fn push(mut self, constant: Constant) -> Self {
        match &mut self {
            Self::Standard { constants, .. } => constants.push(constant),
            Self::ErrorCodes { .. } => panic!("Use push_error for error code groups"),
        }
        self
    }

    fn push_error(mut self, error: ErrorCode) -> Self {
        match &mut self {
            Self::Standard { .. } => panic!("Use push for standard groups"),
            Self::ErrorCodes { codes, .. } => codes.push(error),
        }
        self
    }

    fn comment(&self) -> &Comment {
        match self {
            Self::Standard { comment, .. } => comment,
            Self::ErrorCodes { comment, .. } => comment,
        }
    }

    fn prefix(&self) -> Option<&'static str> {
        match self {
            Self::Standard { prefix, .. } => *prefix,
            Self::ErrorCodes { .. } => Some(ErrorCode::PREFIX),
        }
    }
}

// Top-level container for all constant groups.
struct Constants {
    groups: Vec<ConstantGroup>,
}

impl Constants {
    fn new() -> Self {
        Self { groups: Vec::new() }
    }

    fn push(mut self, group: ConstantGroup) -> Self {
        self.groups.push(group);
        self
    }

    fn to_asm(&self) -> String {
        use std::collections::HashSet;

        // Check for duplicate prefixes.
        let mut seen_prefixes: HashSet<Option<&str>> = HashSet::new();
        for group in &self.groups {
            assert!(
                seen_prefixes.insert(group.prefix()),
                "Duplicate group prefix: {:?}",
                group.prefix()
            );
        }

        // Check for duplicate constant names (after applying prefix and suffix).
        let mut seen_names: HashSet<String> = HashSet::new();
        for group in &self.groups {
            match group {
                ConstantGroup::Standard {
                    constants, prefix, ..
                } => {
                    for constant in constants {
                        let name = match prefix {
                            Some(prefix) => format!("{}{}", prefix, constant.asm_name()),
                            None => constant.asm_name(),
                        };
                        assert!(
                            seen_names.insert(name.clone()),
                            "Duplicate constant name: {name}"
                        );
                    }
                }
                ConstantGroup::ErrorCodes { codes, .. } => {
                    for code in codes {
                        assert!(
                            seen_names.insert(code.asm_name()),
                            "Duplicate constant name: {}",
                            code.asm_name()
                        );
                    }
                }
            }
        }

        let mut output = String::new();
        for (i, group) in self.groups.iter().enumerate() {
            if i > 0 {
                output.push('\n');
            }
            output.push_str(&format!("# {}\n", group.comment().as_str()));
            output.push_str(&format!(
                "# {}\n",
                "-".repeat(group.comment().as_str().len())
            ));

            match group {
                ConstantGroup::Standard {
                    constants, prefix, ..
                } => {
                    for constant in constants {
                        let value = if constant.is_hex {
                            format!("0x{:x}", constant.value)
                        } else {
                            constant.value.to_string()
                        };
                        let name = match prefix {
                            Some(prefix) => format!("{}{}", prefix, constant.asm_name()),
                            None => constant.asm_name(),
                        };
                        // Try inline comment: ".equ NAME, VALUE # Comment."
                        let inline =
                            format!(".equ {}, {} # {}", name, value, constant.comment.as_str());
                        if inline.len() <= LINE_LENGTH {
                            output.push_str(&inline);
                            output.push('\n');
                        } else {
                            // Comment on separate line.
                            output.push_str(&format!(
                                "# {}\n.equ {}, {}\n",
                                constant.comment.as_str(),
                                name,
                                value
                            ));
                        }
                    }
                }
                ConstantGroup::ErrorCodes { codes, .. } => {
                    for (idx, code) in codes.iter().enumerate() {
                        let value = 1 + idx as u64; // Error codes start at 1.
                        let name = code.asm_name();
                        // Try inline comment: ".equ NAME, VALUE # Comment."
                        let inline =
                            format!(".equ {}, {} # {}", name, value, code.comment.as_str());
                        if inline.len() <= LINE_LENGTH {
                            output.push_str(&inline);
                            output.push('\n');
                        } else {
                            // Comment on separate line.
                            output.push_str(&format!(
                                "# {}\n.equ {}, {}\n",
                                code.comment.as_str(),
                                name,
                                value
                            ));
                        }
                    }
                }
            }
        }
        output
    }

    fn get(&self, name: &str) -> u64 {
        for group in &self.groups {
            match group {
                ConstantGroup::Standard {
                    constants, prefix, ..
                } => {
                    for constant in constants {
                        let full_name = match prefix {
                            Some(p) => format!("{}{}", p, constant.asm_name()),
                            None => constant.asm_name(),
                        };
                        if full_name == name {
                            return constant.value;
                        }
                    }
                }
                ConstantGroup::ErrorCodes { codes, .. } => {
                    for (idx, code) in codes.iter().enumerate() {
                        if code.asm_name() == name {
                            return (1 + idx) as u64;
                        }
                    }
                }
            }
        }
        panic!("Constant not found: {name}");
    }
}

// Comment type with validation.
struct Comment(&'static str);

impl Comment {
    const MAX_LENGTH: usize = LINE_LENGTH - 2; // Account for "# " prefix.

    fn new(text: &'static str) -> Self {
        assert!(!text.is_empty(), "Comment must not be empty");
        assert!(text.ends_with('.'), "Comment must end with '.': {text}");
        assert!(
            text.len() <= Self::MAX_LENGTH,
            "Comment must not exceed {} characters: {text}",
            Self::MAX_LENGTH
        );
        Self(text)
    }

    fn as_str(&self) -> &'static str {
        self.0
    }
}

fn constants() -> Constants {
    #[repr(C)]
    struct SolInstruction {
        program_id_addr: u64,
        accounts_addr: u64,
        accounts_len: u64,
        data_addr: u64,
        data_len: u64,
    }

    #[repr(C)]
    struct SolAccountMeta {
        pubkey_addr: u64,
        is_writable: bool,
        is_signer: bool,
        pad: [u8; 6],
    }

    #[repr(C)]
    // Defined as bytes vectors so compiler doesn't align fields before end of struct during offset
    // calculations.
    struct CreateAccountInstructionData {
        variant: [u8; size_of::<u32>()],
        lamports: [u8; size_of::<u64>()],
        space: [u8; size_of::<u64>()],
        owner: [u8; size_of::<Pubkey>()],
        pad: [u8; 4],
    }

    #[repr(C)]
    struct SolSignerSeed {
        addr: u64,
        len: u64,
    }

    #[repr(C)]
    struct SolSignerSeeds {
        addr: u64,
        len: u64,
    }

    #[repr(C)]
    struct SolAccountInfo {
        key_addr: u64,
        lamports_addr: u64,
        data_len: u64,
        data_addr: u64,
        owner_addr: u64,
        rent_epoch: u64,
        is_signer: bool,
        is_writable: bool,
        executable: bool,
        pad: [u8; 5],
    }

    // Number of accounts for CPI create account instruction.
    const N_ACCOUNTS_CPI: usize = 2;
    // Number of signer seeds for PDA.
    const N_SIGNER_SEEDS_PDA: usize = 2;
    // Number of PDAs in CPI.
    const N_PDAS: usize = 1;

    #[repr(C)]
    struct StackFrameInit {
        instruction: SolInstruction,
        instruction_data: CreateAccountInstructionData,
        account_metas: [SolAccountMeta; N_ACCOUNTS_CPI],
        account_infos: [SolAccountInfo; N_ACCOUNTS_CPI],
        // User pubkey, then bump seed.
        signer_seeds: [SolSignerSeed; N_SIGNER_SEEDS_PDA],
        signers_seeds: [SolSignerSeeds; N_PDAS],
        pda: Pubkey,
    }

    #[repr(C)]
    struct MemoryMapInit {
        n_accounts: u64,
        user: StandardAccount, // Must be empty, or CreateAccount will fail.
        pda: StandardAccount,  // Must be empty, or CreateAccount will fail.
        system_program: SystemProgramAccount,
        program_id: Pubkey,
    }

    const MAX_PERMITTED_DATA_INCREASE: usize = 10240;

    #[allow(dead_code)]
    #[repr(C)]
    struct AccountLayout<const PADDED_DATA_SIZE: usize> {
        non_dup_marker: u8,
        is_signer: u8,
        is_writable: u8,
        is_executable: u8,
        original_data_len: [u8; 4],
        pubkey: [u8; 32],
        owner: [u8; 32],
        lamports: u64,
        data_len: u64,
        data_padded: [u8; PADDED_DATA_SIZE],
        rent_epoch: u64,
    }

    type StandardAccount = AccountLayout<MAX_PERMITTED_DATA_INCREASE>;
    type SystemProgramAccount = AccountLayout<{ MAX_PERMITTED_DATA_INCREASE + 16 }>;

    let constants = Constants::new()
        .push(
            ConstantGroup::new_error_codes()
                .push_error(ErrorCode::new("N_ACCOUNTS", "Invalid number of accounts."))
                .push_error(ErrorCode::new(
                    "USER_DATA_LEN",
                    "User data length is nonzero.",
                ))
                .push_error(ErrorCode::new(
                    "PDA_DATA_LEN",
                    "PDA data length is nonzero.",
                ))
                .push_error(ErrorCode::new(
                    "SYSTEM_PROGRAM_DATA_LEN",
                    "System Program data length is nonzero.",
                ))
                .push_error(ErrorCode::new(
                    "PDA_DUPLICATE",
                    "PDA is a duplicate account.",
                ))
                .push_error(ErrorCode::new(
                    "SYSTEM_PROGRAM_DUPLICATE",
                    "System Program is a duplicate account.",
                )),
        )
        .push(
            ConstantGroup::new("Input memory map layout.")
                .push(Constant::new_hex(
                    "NON_DUP_MARKER",
                    0xff,
                    "Flag that an account is not a duplicate.",
                ))
                .push(Constant::new("DATA_LEN_ZERO", 0, "Data length of zero."))
                .push(Constant::new(
                    "DATA_LEN_SYSTEM_PROGRAM",
                    "system_program".len() as u64,
                    "Data length of System Program.",
                ))
                .push(Constant::new(
                    "N_ACCOUNTS_INCREMENT",
                    2,
                    "Number of accounts for increment operation.",
                ))
                .push(Constant::new(
                    "N_ACCOUNTS_INIT",
                    3,
                    "Number of accounts for initialize operation.",
                ))
                .push(Constant::new_offset(
                    "N_ACCOUNTS",
                    0,
                    "Number of accounts in virtual memory map.",
                ))
                .push(Constant::new_offset(
                    "USER_DATA_LEN",
                    (offset_of!(MemoryMapInit, user) + offset_of!(StandardAccount, data_len))
                        as u64,
                    "User data length.",
                ))
                .push(Constant::new_offset(
                    "USER_PUBKEY",
                    (offset_of!(MemoryMapInit, user) + offset_of!(StandardAccount, pubkey)) as u64,
                    "User pubkey.",
                ))
                .push(Constant::new_offset(
                    "PDA_NON_DUP_MARKER",
                    (offset_of!(MemoryMapInit, pda) + offset_of!(StandardAccount, non_dup_marker))
                        as u64,
                    "PDA non-duplicate marker.",
                ))
                .push(Constant::new_offset(
                    "PDA_DATA_LEN",
                    (offset_of!(MemoryMapInit, pda) + offset_of!(StandardAccount, data_len)) as u64,
                    "PDA data length.",
                ))
                .push(Constant::new_offset(
                    "SYSTEM_PROGRAM_NON_DUP_MARKER",
                    (offset_of!(MemoryMapInit, system_program)
                        + offset_of!(SystemProgramAccount, non_dup_marker))
                        as u64,
                    "System Program non-duplicate marker.",
                ))
                .push(Constant::new_offset(
                    "SYSTEM_PROGRAM_DATA_LEN",
                    (offset_of!(MemoryMapInit, system_program)
                        + offset_of!(SystemProgramAccount, data_len)) as u64,
                    "System program data length.",
                )),
        );
    constants.push(
        ConstantGroup::new_with_prefix("Stack frame layout for initialize operation.", "STK_INIT_")
            .push(Constant::new_offset(
                "INSN",
                (size_of::<StackFrameInit>() - offset_of!(StackFrameInit, instruction)) as u64,
                "SolInstruction for CreateAccount CPI.",
            ))
            .push(Constant::new_offset(
                "SEED_1_ADDR",
                (size_of::<StackFrameInit>() - (offset_of!(StackFrameInit, signer_seeds))) as u64,
                "Pointer to user pubkey.",
            ))
            .push(Constant::new_offset(
                "SEED_1_LEN",
                (size_of::<StackFrameInit>()
                    - (offset_of!(StackFrameInit, signer_seeds) + offset_of!(SolSignerSeed, len)))
                    as u64,
                "Length of user pubkey.",
            ))
            .push(Constant::new_offset(
                "SEED_2_ADDR",
                (size_of::<StackFrameInit>()
                    - (offset_of!(StackFrameInit, signer_seeds) + size_of::<SolSignerSeed>()))
                    as u64,
                "Pointer to bump seed.",
            ))
            .push(Constant::new_offset(
                "SEED_2_LEN",
                (size_of::<StackFrameInit>()
                    - (offset_of!(StackFrameInit, signer_seeds)
                        + size_of::<SolSignerSeed>()
                        + offset_of!(SolSignerSeed, len))) as u64,
                "Length of bump seed.",
            ))
            .push(Constant::new_offset(
                "PDA",
                (size_of::<StackFrameInit>() - (offset_of!(StackFrameInit, pda))) as u64,
                "PDA.",
            )),
    )
}

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

const USER_STARTING_LAMPORTS: u64 = 10_000;

enum Operation {
    Initialize,
    Increment,
}

enum AccountIndex {
    User = 0,
    Pda = 1,
    SystemProgram = 2,
}
fn happy_path_setup(
    program_language: ProgramLanguage,
    operation: Operation,
) -> (
    test_utils::TestSetup,
    Instruction,
    Vec<(Pubkey, Account)>,
    Vec<Check<'static>>,
) {
    let setup = setup_test(program_language);
    let (system_program, system_account) = program::keyed_account_for_system_program();

    let mut instruction = Instruction::new_with_bytes(
        setup.program_id,
        &[],
        vec![
            AccountMeta::new(Pubkey::new_unique(), true),
            AccountMeta::new(Pubkey::new_unique(), false),
        ],
    );

    let mut accounts = vec![
        (
            instruction.accounts[AccountIndex::User as usize].pubkey,
            Account::new(USER_STARTING_LAMPORTS, 0, &system_program),
        ),
        (
            instruction.accounts[AccountIndex::Pda as usize].pubkey,
            Account::new(0, 0, &setup.program_id),
        ),
    ];

    let mut checks = vec![Check::success()];

    match operation {
        Operation::Initialize => {
            instruction
                .accounts
                .push(AccountMeta::new_readonly(system_program, false));
            accounts.push((system_program, system_account));
        }
        Operation::Increment => {
            // To be implemented.
        }
    }
    (setup, instruction, accounts, checks)
}

#[test]
fn test_asm_no_accounts() {
    let (setup, mut instruction, mut accounts, _checks) =
        happy_path_setup(ProgramLanguage::Assembly, Operation::Initialize);

    instruction.accounts.clear();
    accounts.clear();

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            constants().get("E_N_ACCOUNTS") as u32,
        ))],
    );
}

#[test]
fn test_asm_too_many_accounts() {
    let (setup, mut instruction, mut accounts, _checks) =
        happy_path_setup(ProgramLanguage::Assembly, Operation::Initialize);

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
        &[Check::err(ProgramError::Custom(
            constants().get("E_N_ACCOUNTS") as u32,
        ))],
    );
}

#[test]
fn test_asm_initialize_user_data_len() {
    let (setup, instruction, mut accounts, _checks) =
        happy_path_setup(ProgramLanguage::Assembly, Operation::Initialize);

    accounts[AccountIndex::User as usize].1.data = vec![1u8; 1];

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            constants().get("E_USER_DATA_LEN") as u32,
        ))],
    );
}

#[test]
fn test_asm_initialize_pda_data_len() {
    let (setup, instruction, mut accounts, _checks) =
        happy_path_setup(ProgramLanguage::Assembly, Operation::Initialize);

    accounts[AccountIndex::Pda as usize].1.data = vec![1u8; 1];

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            constants().get("E_PDA_DATA_LEN") as u32,
        ))],
    );
}

#[test]
fn test_asm_initialize_system_program_data_len() {
    let (setup, instruction, mut accounts, _checks) =
        happy_path_setup(ProgramLanguage::Assembly, Operation::Initialize);

    accounts[AccountIndex::SystemProgram as usize].1.data = vec![];

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            constants().get("E_SYSTEM_PROGRAM_DATA_LEN") as u32,
        ))],
    );
}

#[test]
fn test_asm_pda_duplicate() {
    let (setup, mut instruction, mut accounts, _checks) =
        happy_path_setup(ProgramLanguage::Assembly, Operation::Initialize);

    instruction.accounts[AccountIndex::Pda as usize] =
        instruction.accounts[AccountIndex::User as usize].clone();
    accounts[AccountIndex::Pda as usize] = accounts[AccountIndex::User as usize].clone();

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            constants().get("E_PDA_DUPLICATE") as u32,
        ))],
    );
}

#[test]
fn test_asm_system_program_duplicate() {
    let (setup, mut instruction, mut accounts, _checks) =
        happy_path_setup(ProgramLanguage::Assembly, Operation::Initialize);

    instruction.accounts[AccountIndex::SystemProgram as usize] =
        instruction.accounts[AccountIndex::User as usize].clone();
    accounts[AccountIndex::SystemProgram as usize] = accounts[AccountIndex::User as usize].clone();

    setup.mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            constants().get("E_SYSTEM_PROGRAM_DUPLICATE") as u32,
        ))],
    );
}
