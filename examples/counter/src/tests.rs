#[test]
fn test_constants() {
    use std::fs;

    // Comment type with validation.
    struct Comment(&'static str);

    impl Comment {
        fn new(text: &'static str) -> Self {
            assert!(!text.is_empty(), "Comment must not be empty");
            assert!(text.ends_with('.'), "Comment must end with '.': {text}");
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

        fn new(name: &'static str, value: u64, comment: &'static str) -> Self {
            assert!(
                !name.ends_with(Self::OFFSET_SUFFIX),
                "Non-offset constant name must not end with {}: {name}",
                Self::OFFSET_SUFFIX
            );
            Self {
                name,
                value,
                is_offset: false,
                is_hex: false,
                comment: Comment::new(comment),
            }
        }

        fn new_offset(name: &'static str, value: u64, comment: &'static str) -> Self {
            assert!(
                name.ends_with(Self::OFFSET_SUFFIX),
                "Offset constant name must end with {}: {name}",
                Self::OFFSET_SUFFIX
            );
            assert!(
                value <= i16::MAX as u64,
                "Offset value must fit in i16: {name} = {value}"
            );
            Self {
                name,
                value,
                is_offset: true,
                is_hex: false,
                comment: Comment::new(comment),
            }
        }
    }

    // Group of related constants.
    struct ConstantGroup {
        comment: Comment,
        constants: Vec<Constant>,
        prefix: Option<&'static str>,
    }

    impl ConstantGroup {
        fn new(comment: &'static str, constants: Vec<Constant>) -> Self {
            Self {
                comment: Comment::new(comment),
                constants,
                prefix: None,
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
            let mut output = String::new();
            for (i, group) in self.groups.iter().enumerate() {
                if i > 0 {
                    output.push('\n');
                }
                output.push_str(&format!("# {}\n", group.comment.as_str()));
                for constant in &group.constants {
                    let value = if constant.is_hex {
                        format!("0x{:x}", constant.value)
                    } else {
                        constant.value.to_string()
                    };
                    let name = match group.prefix {
                        Some(prefix) => format!("{}{}", prefix, constant.name),
                        None => constant.name.to_string(),
                    };
                    output.push_str(&format!(
                        "# {}\n.equ {}, {}\n",
                        constant.comment.as_str(),
                        name,
                        value
                    ));
                }
            }
            output
        }

        fn write_to_asm_file(&self, path: &str) {
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
    let constants = Constants::new(vec![ConstantGroup::new(
        "Miscellaneous constants.",
        vec![Constant::new_offset(
            "N_ACCOUNTS_OFF",
            0,
            "Number of accounts in virtual memory map.",
        )],
    )]);

    // Write to assembly file.
    let asm_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/counter/counter.s");
    constants.write_to_asm_file(asm_path);
}
