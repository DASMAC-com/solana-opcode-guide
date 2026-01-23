use solana_sdk::pubkey::Pubkey;
use std::mem::{offset_of, size_of};

/// In an assembly file, for viewable render on docs site.
const LINE_LENGTH: usize = 75;
/// Alignment for stack and account data.
const ALIGNMENT: usize = 8;

pub fn constants() -> Constants {
    // Number of accounts for CPI create account instruction.
    const N_ACCOUNTS_CPI: usize = 2;
    // Number of signer seeds for PDA.
    const N_SIGNER_SEEDS_PDA: usize = 2;
    // Number of PDAs in CPI.
    const N_PDAS: usize = 1;
    /// For an account during an instruction.
    const MAX_PERMITTED_DATA_INCREASE: usize = 10240;
    /// b"system_program" plus two bytes of padding.
    const SYSTEM_PROGRAM_DATA_LEN: usize = "system_program".len();
    const SYSTEM_PROGRAM_DATA_WITH_PAD_LEN: usize =
        SYSTEM_PROGRAM_DATA_LEN + (ALIGNMENT - SYSTEM_PROGRAM_DATA_LEN % ALIGNMENT) % ALIGNMENT;
    const ACCOUNT_STORAGE_OVERHEAD: usize = 128;

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
    // Defined as bytes vectors to prevent addition of inner padding during compilation.
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

    #[repr(C)]
    struct Rent {
        lamports_per_byte_year: u64,
        exemption_threshold: f64,
        burn_percent: u8,
        pad: [u8; 7],
    }

    #[repr(C)]
    struct StackFrameInit {
        system_program_pubkey: Pubkey, // Zero-initialized.
        instruction: SolInstruction,
        account_metas: [SolAccountMeta; N_ACCOUNTS_CPI],
        instruction_data: CreateAccountInstructionData,
        account_infos: [SolAccountInfo; N_ACCOUNTS_CPI],
        // User pubkey, then bump seed.
        signer_seeds: [SolSignerSeed; N_SIGNER_SEEDS_PDA],
        signers_seeds: [SolSignerSeeds; N_PDAS],
        pda: Pubkey,
        rent: Rent,
        memcmp_result: i32,
        pad: [u8; 4],
        bump_seed: u8,
    }

    #[repr(C)]
    struct PdaAccountData {
        counter: u64,
        bump_seed: u8,
    }

    #[repr(C)]
    struct MemoryMapInit {
        n_accounts: u64,
        user: StandardAccount, // Must be empty, or CreateAccount will fail.
        pda: StandardAccount,  // Reflects state before CreateAccount CPI.
        system_program: SystemProgramAccount,
        instruction_data_len: u64, // 0u64 for initialize operation.
        program_id: Pubkey,
    }

    #[allow(dead_code)]
    #[repr(C)]
    struct AccountLayout<const PADDED_DATA_SIZE: usize> {
        non_dup_marker: u8,
        is_signer: u8,
        is_writable: u8,
        is_executable: u8,
        original_data_len: [u8; 4],
        pubkey: [u8; size_of::<Pubkey>()],
        owner: [u8; size_of::<Pubkey>()],
        lamports: u64,
        data_len: u64,
        data_padded: [u8; PADDED_DATA_SIZE],
        rent_epoch: u64,
    }

    type StandardAccount = AccountLayout<MAX_PERMITTED_DATA_INCREASE>;
    type SystemProgramAccount =
        AccountLayout<{ MAX_PERMITTED_DATA_INCREASE + SYSTEM_PROGRAM_DATA_WITH_PAD_LEN }>;

    Constants::new()
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
                ))
                .push_error(ErrorCode::new(
                    "UNABLE_TO_DERIVE_PDA",
                    "Unable to derive PDA.",
                ))
                .push_error(ErrorCode::new(
                    "PDA_MISMATCH",
                    "Passed PDA does not match computed PDA.",
                )),
        )
        .push(
            ConstantGroup::new_with_prefix("Size of assorted types.", "SIZE_OF_")
                .push(Constant::new(
                    "PUBKEY",
                    size_of::<Pubkey>() as u64,
                    "Size of Pubkey.",
                ))
                .push(Constant::new("U8", size_of::<u8>() as u64, "Size of u8.")),
        )
        .push(
            ConstantGroup::new("Memory map layout.")
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
                    "PDA_PUBKEY",
                    (offset_of!(MemoryMapInit, pda) + offset_of!(StandardAccount, pubkey)) as u64,
                    "PDA pubkey.",
                ))
                .push(Constant::new_offset(
                    "PDA_DATA_LEN",
                    (offset_of!(MemoryMapInit, pda) + offset_of!(StandardAccount, data_len)) as u64,
                    "PDA data length.",
                ))
                .push(Constant::new(
                    "PDA_DATA_WITH_ACCOUNT_OVERHEAD",
                    (size_of::<u64>() + size_of::<u8>() + ACCOUNT_STORAGE_OVERHEAD) as u64,
                    "PDA account data length plus account overhead.",
                ))
                .push(Constant::new_offset(
                    "PDA_BUMP_SEED",
                    (offset_of!(MemoryMapInit, pda)
                        + offset_of!(StandardAccount, data_padded)
                        + offset_of!(PdaAccountData, bump_seed)) as u64,
                    "PDA bump seed.",
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
                ))
                .push(Constant::new_offset(
                    "PROGRAM_ID_INIT",
                    offset_of!(MemoryMapInit, program_id) as u64,
                    "Program ID during initialize operation.",
                )),
        )
        .push(
            ConstantGroup::new_with_prefix("CreateAccount instruction data.", "INIT_CPI_")
                .push(Constant::new(
                    "N_ACCOUNTS",
                    N_ACCOUNTS_CPI as u64,
                    "Number of accounts for CPI.",
                ))
                .push(Constant::new(
                    "INSN_DATA_LEN",
                    (size_of::<u32>() + size_of::<u64>() + size_of::<u64>() + size_of::<Pubkey>())
                        as u64,
                    "Length of instruction data.",
                ))
                .push(Constant::new("DISCRIMINATOR", 0, "Discriminator."))
                .push(Constant::new(
                    "ACCT_SIZE",
                    (size_of::<u64>() + size_of::<u8>()) as u64,
                    "Account size.",
                )),
        )
        .push(
            ConstantGroup::new_stack_layout(
                "Stack frame layout for initialize operation.",
                "STK_INIT_",
            )
            .push(Constant::new_offset(
                "SYSTEM_PROGRAM_PUBKEY",
                (size_of::<StackFrameInit>() - offset_of!(StackFrameInit, system_program_pubkey))
                    as u64,
                "System Program pubkey for CreateAccount CPI.",
            ))
            .push(Constant::new_offset(
                "INSN",
                (size_of::<StackFrameInit>() - offset_of!(StackFrameInit, instruction)) as u64,
                "SolInstruction for CreateAccount CPI.",
            ))
            .push(Constant::new_offset(
                "INSN_ACCOUNTS_ADDR",
                (size_of::<StackFrameInit>()
                    - (offset_of!(StackFrameInit, instruction)
                        + offset_of!(SolInstruction, accounts_addr))) as u64,
                "Accounts address in SolInstruction.",
            ))
            .push(Constant::new_offset(
                "INSN_ACCOUNTS_LEN",
                (size_of::<StackFrameInit>()
                    - (offset_of!(StackFrameInit, instruction)
                        + offset_of!(SolInstruction, accounts_len))) as u64,
                "Accounts length in SolInstruction.",
            ))
            .push(Constant::new_offset(
                "INSN_DATA_ADDR",
                (size_of::<StackFrameInit>()
                    - (offset_of!(StackFrameInit, instruction)
                        + offset_of!(SolInstruction, data_addr))) as u64,
                "Data address in SolInstruction.",
            ))
            .push(Constant::new_offset(
                "INSN_DATA_LEN",
                (size_of::<StackFrameInit>()
                    - (offset_of!(StackFrameInit, instruction)
                        + offset_of!(SolInstruction, data_len))) as u64,
                "Data length in SolInstruction.",
            ))
            .push(Constant::new_offset(
                "SYSTEM_PROGRAM_PUBKEY_TO_ACCOUNT_METAS",
                (offset_of!(StackFrameInit, account_metas)
                    - offset_of!(StackFrameInit, system_program_pubkey)) as u64,
                "Offset from System Program pubkey to account metas.",
            ))
            .push(Constant::new_offset(
                "ACCOUNT_METAS_TO_INSN_DATA",
                (offset_of!(StackFrameInit, instruction_data)
                    - offset_of!(StackFrameInit, account_metas)) as u64,
                "Offset from account metas to instruction data.",
            ))
            .push(Constant::new_maybe_unaligned_offset(
                "INSN_DATA_LAMPORTS",
                (size_of::<StackFrameInit>()
                    - (offset_of!(StackFrameInit, instruction_data)
                        + offset_of!(CreateAccountInstructionData, lamports)))
                    as u64,
                "Offset of lamports field inside CreateAccount instruction data.",
            ))
            .push(Constant::new_maybe_unaligned_offset(
                "INSN_DATA_SPACE",
                (size_of::<StackFrameInit>()
                    - (offset_of!(StackFrameInit, instruction_data)
                        + offset_of!(CreateAccountInstructionData, space))) as u64,
                "Offset of space field inside CreateAccount instruction data.",
            ))
            .push(Constant::new_maybe_unaligned_offset(
                "INSN_DATA_OWNER",
                (size_of::<StackFrameInit>()
                    - (offset_of!(StackFrameInit, instruction_data)
                        + offset_of!(CreateAccountInstructionData, owner))) as u64,
                "Offset of owner field inside CreateAccount instruction data.",
            ))
            .push(Constant::new_offset(
                "SEED_0_ADDR",
                (size_of::<StackFrameInit>() - (offset_of!(StackFrameInit, signer_seeds))) as u64,
                "Pointer to user pubkey.",
            ))
            .push(Constant::new_offset(
                "SEED_0_LEN",
                (size_of::<StackFrameInit>()
                    - (offset_of!(StackFrameInit, signer_seeds) + offset_of!(SolSignerSeed, len)))
                    as u64,
                "Length of user pubkey.",
            ))
            .push(Constant::new_offset(
                "SEED_1_ADDR",
                (size_of::<StackFrameInit>()
                    - (offset_of!(StackFrameInit, signer_seeds) + size_of::<SolSignerSeed>()))
                    as u64,
                "Pointer to bump seed.",
            ))
            .push(Constant::new_offset(
                "SEED_1_LEN",
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
            ))
            .push(Constant::new_offset(
                "RENT",
                (size_of::<StackFrameInit>() - (offset_of!(StackFrameInit, rent))) as u64,
                "Rent struct return.",
            ))
            .push(Constant::new_offset(
                "MEMCMP_RESULT",
                (size_of::<StackFrameInit>() - (offset_of!(StackFrameInit, memcmp_result))) as u64,
                "Compare result of sol_memcmp.",
            ))
            .push(Constant::new_offset(
                "BUMP_SEED",
                (size_of::<StackFrameInit>() - (offset_of!(StackFrameInit, bump_seed))) as u64,
                "Bump seed.",
            )),
        )
        .push(
            ConstantGroup::new("Assorted constants.")
                .push(Constant::new("NO_OFFSET", 0, "Offset of zero."))
                .push(Constant::new(
                    "SUCCESS",
                    0,
                    "Indicates successful operation.",
                ))
                .push(Constant::new(
                    "COMPARE_EQUAL",
                    0,
                    "Compare result indicating equality.",
                )),
        )
}

// Individual constant definition.
struct Constant {
    name: &'static str,
    value: u64,
    is_offset: bool,
    may_be_unaligned: bool,
    is_hex: bool,
    comment: Comment,
}

impl Constant {
    const OFFSET_SUFFIX: &str = "_OFF";

    fn create(
        name: &'static str,
        value: u64,
        is_offset: bool,
        may_be_unaligned: bool,
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
            may_be_unaligned,
            is_hex,
            comment: Comment::new(comment),
        }
    }

    fn new(name: &'static str, value: u64, comment: &'static str) -> Self {
        Self::create(name, value, false, false, false, comment)
    }

    fn new_hex(name: &'static str, value: u64, comment: &'static str) -> Self {
        Self::create(name, value, false, false, true, comment)
    }

    fn new_offset(name: &'static str, value: u64, comment: &'static str) -> Self {
        Self::create(name, value, true, false, false, comment)
    }

    fn new_maybe_unaligned_offset(name: &'static str, value: u64, comment: &'static str) -> Self {
        Self::create(name, value, true, true, false, comment)
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
        is_stack: bool,
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
            is_stack: false,
        }
    }

    fn new_with_prefix(comment: &'static str, prefix: &'static str) -> Self {
        Self::Standard {
            comment: Comment::new(comment),
            constants: Vec::new(),
            prefix: Some(prefix),
            is_stack: false,
        }
    }

    fn new_stack_layout(comment: &'static str, prefix: &'static str) -> Self {
        Self::Standard {
            comment: Comment::new(comment),
            constants: Vec::new(),
            prefix: Some(prefix),
            is_stack: true,
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
            Self::Standard {
                constants,
                is_stack,
                ..
            } => {
                if *is_stack {
                    assert!(
                        constant.is_offset,
                        "Stack layout group must only contain offsets: {}",
                        constant.name
                    );
                    if !constant.may_be_unaligned {
                        assert!(
                            constant.value.is_multiple_of(ALIGNMENT as u64),
                            "Stack offset must be {}-byte aligned: {} = {}",
                            ALIGNMENT,
                            constant.name,
                            constant.value
                        );
                    }
                }
                constants.push(constant);
            }
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
pub struct Constants {
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

    pub fn to_asm(&self) -> String {
        use std::collections::HashSet;

        // Check for duplicate prefixes.
        let mut seen_prefixes: HashSet<&str> = HashSet::new();
        for group in &self.groups {
            if let Some(prefix) = group.prefix() {
                assert!(
                    seen_prefixes.insert(prefix),
                    "Duplicate group prefix: {prefix:?}",
                );
            }
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

    pub fn get(&self, name: &str) -> u64 {
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
