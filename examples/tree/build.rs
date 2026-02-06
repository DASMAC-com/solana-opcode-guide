use interface::*;
use std::{collections::HashSet, fs, path::Path};

const CONSTANTS_ANCHOR_START: &str = "# ANCHOR: constants";
const CONSTANTS_ANCHOR_END: &str = "# ANCHOR_END: constants";

macro_rules! asm_groups {
    ($($group:ident),* $(,)?) => {
        [$($group::to_asm()),*]
    };
}

fn main() {
    // Collect all constant groups.
    let groups = asm_groups![
        error_codes,
        sizes,
        data,
        pubkey_chunk,
        input_buffer,
        init_stack_frame,
        cpi
    ];

    // Read in the assembly file.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let manifest_path = Path::new(manifest_dir);
    let asm_path = manifest_path.join("src/tree/tree.s");
    let content = fs::read_to_string(&asm_path).unwrap();

    // Find the constants anchor region.
    let anchor_start = content
        .find(CONSTANTS_ANCHOR_START)
        .expect("missing '# ANCHOR: constants' in assembly file");
    let anchor_end = content
        .find(CONSTANTS_ANCHOR_END)
        .expect("missing '# ANCHOR_END: constants' in assembly file");
    assert!(
        anchor_start < anchor_end,
        "ANCHOR: constants must come before ANCHOR_END: constants"
    );

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

    // Generate the constants and insert inside the anchor region.
    let constants = groups.join("\n");
    let constants = constants.trim();
    let before_anchor = &content[..anchor_start + CONSTANTS_ANCHOR_START.len()];
    let after_anchor = &content[anchor_end..];
    let new_content = format!("{}\n{}\n{}", before_anchor, constants, after_anchor);
    if new_content != content {
        fs::write(&asm_path, &new_content).unwrap();
    }

    // Extract ANCHOR snippets from source files (use new_content for asm since it's canonical).
    extract_snippets(&new_content, manifest_path, "asm");

    let rs_path = manifest_path.join("src/program.rs");
    let rs_content = fs::read_to_string(&rs_path).unwrap();
    extract_snippets(&rs_content, manifest_path, "rs");
}

/// Extract ANCHOR snippets from source content and write to artifacts/snippets/{kind}/.
fn extract_snippets(content: &str, manifest_dir: &Path, kind: &str) {
    let snippets_dir = manifest_dir.join(format!("artifacts/snippets/{}", kind));

    // Wipe existing snippets directory.
    if snippets_dir.exists() {
        fs::remove_dir_all(&snippets_dir).unwrap();
    }

    let mut current_anchor: Option<String> = None;
    let mut current_lines: Vec<&str> = Vec::new();
    let mut snippets: Vec<(String, String)> = Vec::new();
    let mut seen_names: HashSet<String> = HashSet::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Check for ANCHOR: start tag (supports # or // comments).
        if let Some(rest) = trimmed
            .strip_prefix("# ANCHOR:")
            .or(trimmed.strip_prefix("// ANCHOR:"))
        {
            let name = rest.trim().to_string();
            assert!(
                current_anchor.is_none(),
                "Nested ANCHOR not allowed: found '{}' while inside '{}'",
                name,
                current_anchor.unwrap()
            );
            assert!(
                seen_names.insert(name.clone()),
                "Duplicate ANCHOR name: '{}'",
                name
            );
            current_anchor = Some(name);
            current_lines.clear();
            continue;
        }

        // Check for ANCHOR_END: end tag.
        if let Some(rest) = trimmed
            .strip_prefix("# ANCHOR_END:")
            .or(trimmed.strip_prefix("// ANCHOR_END:"))
        {
            let name = rest.trim();
            if let Some(ref anchor_name) = current_anchor {
                assert_eq!(
                    anchor_name, name,
                    "ANCHOR_END mismatch: expected '{}', found '{}'",
                    anchor_name, name
                );
                let snippet_content = current_lines.join("\n");
                snippets.push((anchor_name.clone(), snippet_content));
                current_anchor = None;
                current_lines.clear();
            } else {
                panic!("ANCHOR_END '{}' without matching ANCHOR", name);
            }
            continue;
        }

        // Collect lines inside an anchor.
        if current_anchor.is_some() {
            current_lines.push(line);
        }
    }

    assert!(
        current_anchor.is_none(),
        "Unclosed ANCHOR: '{}'",
        current_anchor.unwrap()
    );

    // Write snippets to files.
    if !snippets.is_empty() {
        fs::create_dir_all(&snippets_dir).unwrap();
        for (name, content) in snippets {
            let snippet_path = snippets_dir.join(format!("{}.txt", name));
            fs::write(&snippet_path, content).unwrap();
        }
    }
}
