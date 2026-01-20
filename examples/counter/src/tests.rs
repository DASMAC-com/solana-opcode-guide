use std::collections::HashMap;
use std::fs;

/// Parses `.equ` constants from an assembly file.
/// Returns a map of constant name to value.
fn parse_asm_constants(path: &str) -> HashMap<String, u64> {
    let content = fs::read_to_string(path).expect("Failed to read assembly file");
    let mut constants = HashMap::new();

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with(".equ ") {
            // Format: .equ NAME, VALUE
            let rest = &line[5..]; // Skip ".equ "
            if let Some((name, value)) = rest.split_once(',') {
                let name = name.trim().to_string();
                let value = value.trim();
                if let Ok(v) = value.parse::<u64>() {
                    constants.insert(name, v);
                } else if let Some(hex) = value.strip_prefix("0x") {
                    if let Ok(v) = u64::from_str_radix(hex, 16) {
                        constants.insert(name, v as u64);
                    }
                }
            }
        }
    }

    constants
}

struct Constants {
    groups: Vec<ConstantGroup>,
}

struct ConstantGroup {
    comment: &'static str,
    constants: Vec<Constant>,
    prefix: Option<&'static str>,
}

struct Constant {
    name: &'static str,
    value: u64,
    is_offset: bool,
    is_hex: bool,
    comment: Option<&'static str>,
}

#[test]
fn test_constants() {
    // Define expected constants and their values.
    let expected: HashMap<&str, u64> = [("N_ACCOUNTS_OFFSET", 0)].into_iter().collect();

    // Parse the assembly file.
    let asm_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/counter/counter.s");
    let actual = parse_asm_constants(asm_path);

    // Verify all expected constants are present with correct values.
    for (name, expected_value) in &expected {
        let actual_value = actual
            .get(*name)
            .unwrap_or_else(|| panic!("Missing constant in assembly file: {name}"));
        assert_eq!(
            *actual_value, *expected_value,
            "Constant {name} has wrong value: expected {expected_value}, got {actual_value}"
        );
    }

    // Verify no extra constants are defined in the assembly file.
    for name in actual.keys() {
        assert!(
            expected.contains_key(name.as_str()),
            "Unexpected constant in assembly file: {name}. Add it to the expected constants or remove it from the assembly file."
        );
    }
}
