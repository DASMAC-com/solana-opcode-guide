use std::{fs, path::Path};

const ENTRYPOINT_START: &str = ".globl entrypoint";

fn main() {
    // Read in the assembly file and find the entrypoint marker.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let asm_path = Path::new(manifest_dir).join("src/tree/tree.s");
    let content = fs::read_to_string(&asm_path).unwrap();
    let marker_pos = content.find(ENTRYPOINT_START).unwrap();

    // Generate the constants and insert them before the entrypoint marker.
    let constants = format!(
        "{}\n{}",
        interface::memory_map::to_asm(),
        interface::ErrorCodes::to_asm(),
    );
    let new_content = format!("{}\n{}", constants, &content[marker_pos..]);
    if new_content != content {
        fs::write(&asm_path, new_content).unwrap();
    }
}
