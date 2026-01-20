use mollusk_svm::program;
use mollusk_svm::result::Check;
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use std::fs;
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
        pub program_id_addr: u64,
        pub accounts_addr: u64,
        pub accounts_len: u64,
        pub data_addr: u64,
        pub data_len: u64,
    }

    #[repr(C)]
    struct SolAccountMeta {
        pub pubkey_addr: u64,
        pub is_writable: bool,
        pub is_signer: bool,
    }

    #[repr(C)]
    struct CreateAccountInstructionData {
        variant: u32,
        lamports: u64,
        space: u64,
        owner: Pubkey,
    }

    #[repr(C)]
    struct SolSignerSeed {
        pub addr: u64,
        pub len: u64,
    }

    #[repr(C)]
    struct SolSignerSeeds {
        pub addr: u64,
        pub len: u64,
    }

    #[repr(C)]
    struct SolAccountInfo {
        pub key_addr: u64,
        pub lamports_addr: u64,
        pub data_len: u64,
        pub data_addr: u64,
        pub owner_addr: u64,
        pub rent_epoch: u64,
        pub is_signer: bool,
        pub is_writable: bool,
        pub executable: bool,
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
        bump_seed: u8,
    }

    Constants::new()
        .push(
            ConstantGroup::new_error_codes()
                .push_error(ErrorCode::new("N_ACCOUNTS", "Invalid number of accounts.")),
        )
        .push(
            ConstantGroup::new("Input memory map account layout.")
                .push(Constant::new_offset(
                    "N_ACCOUNTS",
                    0,
                    "Number of accounts in virtual memory map.",
                ))
                .push(Constant::new_hex(
                    "NON_DUP_MARKER",
                    0xff,
                    "Flag that an account is not a duplicate.",
                ))
                .push(Constant::new(
                    "N_ACCOUNTS_INCREMENT",
                    2,
                    "Number of accounts for increment operation.",
                ))
                .push(Constant::new(
                    "N_ACCOUNTS_INIT",
                    3,
                    "Number of accounts for init operation.",
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

#[test]
fn test_asm_expected_failures() {
    let setup = setup_test(ProgramLanguage::Assembly);
    let (system_program, system_account) = program::keyed_account_for_system_program();

    // Check no accounts.
    setup.mollusk.process_and_validate_instruction(
        &Instruction::new_with_bytes(setup.program_id, &[], vec![]),
        &[],
        &[Check::err(ProgramError::Custom(
            constants().get("E_N_ACCOUNTS") as u32,
        ))],
    );

    // Check too many accounts.
    let n_accounts: usize = 4;
    let account_metas = vec![AccountMeta::new_readonly(Pubkey::new_unique(), false); n_accounts];
    let account_infos =
        vec![(account_metas[0].pubkey, Account::new(0, 0, &system_program),); n_accounts];
    setup.mollusk.process_and_validate_instruction(
        &Instruction::new_with_bytes(setup.program_id, &[], account_metas),
        &account_infos,
        &[Check::err(ProgramError::Custom(
            constants().get("E_N_ACCOUNTS") as u32,
        ))],
    );
}
