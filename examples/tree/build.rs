use std::{fs, path::Path};

fn main() {
    // Inject constants into ASM file.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let asm_path = Path::new(manifest_dir).join("src/tree/tree.s");

    const MARKER: &str = ".globl entrypoint";

    let content = fs::read_to_string(&asm_path).unwrap();
    let marker_pos = content.find(MARKER).unwrap();

    let constants = format!(
        "{}\n{}",
        interface::memory_map::to_asm(),
        interface::ErrorCodes::to_asm()
    );
    let new_content = format!("{}{}", constants, &content[marker_pos..]);

    if new_content != content {
        fs::write(&asm_path, new_content).unwrap();
    }
}
