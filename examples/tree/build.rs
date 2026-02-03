use std::path::Path;

fn main() {
    // Inject constants into ASM file.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let asm_path = Path::new(manifest_dir).join("src/tree/tree.s");
    common::inject_asm(&asm_path);
}
