use test_utils::{setup_test, ProgramLanguage};

#[test]
fn test_constants() {
    use std::fs;

    const LINE_LENGTH: usize = 75;

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
        fn new(name: &'static str, comment: &'static str) -> Self {
            Self {
                name,
                comment: Comment::new(comment),
            }
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

        fn new_error_codes(comment: &'static str) -> Self {
            Self::ErrorCodes {
                comment: Comment::new(comment),
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

        const ERROR_CODE_PREFIX: &str = "E_";

        fn prefix(&self) -> Option<&'static str> {
            match self {
                Self::Standard { prefix, .. } => *prefix,
                Self::ErrorCodes { .. } => Some(Self::ERROR_CODE_PREFIX),
            }
        }
    }

    // Top-level container for all constant groups.
    struct Constants {
        groups: Vec<ConstantGroup>,
    }

    impl Constants {
        const GLOBAL_ENTRYPOINT: &str = ".global entrypoint";

        fn new(groups: Vec<ConstantGroup>) -> Self {
            Self { groups }
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
                            let name = format!("{}{}", ConstantGroup::ERROR_CODE_PREFIX, code.name);
                            assert!(
                                seen_names.insert(name.clone()),
                                "Duplicate constant name: {name}"
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
                            let name = format!("{}{}", ConstantGroup::ERROR_CODE_PREFIX, code.name);
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

        fn write_to_asm_file(&self, path: &std::path::Path) {
            let content = fs::read_to_string(path).expect("Failed to read assembly file");
            let global_pos = content
                .find(Self::GLOBAL_ENTRYPOINT)
                .expect("Could not find '.global entrypoint' in assembly file");
            let after_global = &content[global_pos..];
            let new_content = format!("{}\n{}", self.to_asm(), after_global);
            fs::write(path, new_content).expect("Failed to write assembly file");
        }
    }

    // Define constants.
    let constants = Constants::new(vec![ConstantGroup::new("Input memory map account layout.")
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
        ))]);

    // Write to assembly file.
    let setup = setup_test(ProgramLanguage::Assembly);
    let asm_path = setup
        .asm_source_path
        .expect("Assembly source file not found");
    constants.write_to_asm_file(&asm_path);
}
