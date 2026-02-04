use interface::*;
use std::{collections::HashSet, fs, path::Path};

const ENTRYPOINT_START: &str = ".globl entrypoint";

macro_rules! asm_groups {
    ($($group:ident),* $(,)?) => {
        [$($group::to_asm()),*]
    };
}

fn main() {
    // Read in the assembly file and find the entrypoint marker.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let asm_path = Path::new(manifest_dir).join("src/tree/tree.s");
    let content = fs::read_to_string(&asm_path).unwrap();
    let marker_pos = content.find(ENTRYPOINT_START).unwrap();

    // Collect all constant groups.
    let groups = asm_groups![error_codes, input_buffer, misc];

    // Check for duplicate constant names.
    let mut seen = HashSet::new();
    for group in &groups {
        for line in group.lines() {
            if let Some(name) = line.strip_prefix(".equ ") {
                if let Some(name) = name.split(',').next() {
                    if !seen.insert(name.to_string()) {
                        panic!("Duplicate constant name: {}", name);
                    }
                }
            }
        }
    }

    // Generate the constants and insert them before the entrypoint marker.
    let constants = groups.join("\n");
    let new_content = format!("{}\n{}", constants, &content[marker_pos..]);
    if new_content != content {
        fs::write(&asm_path, new_content).unwrap();
    }
}
