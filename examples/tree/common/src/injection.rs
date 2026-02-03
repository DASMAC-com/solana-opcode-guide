use crate::{memory_map, ErrorCodes};
use std::fs;
use std::path::Path;

/// Inject generated constants into the ASM file.
///
/// This replaces everything before `.globl entrypoint` with the generated constants.
pub fn inject_asm(asm_path: &Path) {
    const GLOBAL_ENTRYPOINT: &str = ".globl entrypoint";

    let content = fs::read_to_string(asm_path).expect("Failed to read assembly file");

    let global_pos = content
        .find(GLOBAL_ENTRYPOINT)
        .expect("Could not find '.globl entrypoint' in assembly file");

    let after_global = &content[global_pos..];
    let constants = std::format!("{}\n{}", memory_map::to_asm(), ErrorCodes::to_asm());
    let new_content = std::format!("{}{}", constants, after_global);

    if new_content != content {
        fs::write(asm_path, new_content).expect("Failed to write assembly file");
    }
}
