use cargo_manifest::Manifest;
use regex::Regex;
use std::collections::HashSet;
use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Clone, Copy)]
enum DepKind {
    Regular,
    Dev,
}

const SBPF_ARCH_DISASSEMBLE: &str = "v2";
const SBPF_ARCH_DUMP: &str = "v4";
const SBPF_ARCH_TEST: &str = "v3";
const TOOLS_VERSION_DISASSEMBLE: &str = "1.52";
const TOOLS_VERSION_DUMP: &str = "1.51";
const TOOLS_VERSION_TEST: &str = "1.51";

fn main() {
    let mut utils_path: Option<PathBuf> = None;
    let mut program_dependencies = HashSet::<String>::new();
    let mut dev_dependencies = HashSet::<String>::new();
    let mut examples_keypair = None;

    let dir_paths = fs::read_dir(current_dir().expect("failed to get current directory"))
        .expect("failed to read examples directory")
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir());

    for path in dir_paths {
        let dir = path.file_name().expect("failed to get directory name");
        if dir == "target" {
            continue;
        } else if dir == "utils" {
            utils_path = Some(path.clone());
        } else {
            build_example(
                &path,
                &mut examples_keypair,
                &mut dev_dependencies,
                &mut program_dependencies,
            );
        }
    }

    check_dependencies(utils_path, program_dependencies, dev_dependencies);
}

fn check_dependencies(
    utils_path: Option<PathBuf>,
    program_dependencies: HashSet<String>,
    mut dev_dependencies: HashSet<String>,
) {
    // Parse dependencies from utils crates.
    let utils_path = utils_path.expect("missing utils directory");
    let test_utils_crate = &utils_path.clone().join("test-utils");
    let build_examples_crate = &utils_path.clone().join("build-examples");
    let mut build_dependencies = program_dependencies.clone();

    // Extend the dependency sets. Note all test utils are dev-dependencies.
    extend_dep_set(&mut dev_dependencies, test_utils_crate, DepKind::Regular);
    extend_dep_set(&mut dev_dependencies, build_examples_crate, DepKind::Dev);
    extend_dep_set(
        &mut build_dependencies,
        build_examples_crate,
        DepKind::Regular,
    );

    // Get program and build dependency crates manifests.
    let deps_path = utils_path.join("deps");
    let program_deps_crate = deps_path.join("program");
    let build_deps_crate = deps_path.join("build");

    // Verify manifest dependencies.
    verify_manifest_deps(&program_deps_crate, &program_dependencies, DepKind::Regular);
    verify_manifest_deps(&build_deps_crate, &dev_dependencies, DepKind::Dev);
    verify_manifest_deps(&build_deps_crate, &build_dependencies, DepKind::Regular);

    exit(0);
}

fn extend_dep_set(target: &mut HashSet<String>, crate_dir: &Path, kind: DepKind) {
    let manifest = crate_manifest(crate_dir);
    let dep_set = match kind {
        DepKind::Dev => &manifest.dev_dependencies,
        DepKind::Regular => &manifest.dependencies,
    };
    if let Some(dependencies) = dep_set {
        target.extend(dependencies.keys().cloned());
        for name in dependencies.keys() {
            target.insert(name.clone());
        }
    }
}

fn crate_manifest(crate_dir: &Path) -> Manifest {
    Manifest::from_path(crate_dir.join("Cargo.toml")).expect("failed to parse Cargo.toml")
}

fn verify_manifest_deps(crate_dir: &Path, expected_deps: &HashSet<String>, kind: DepKind) {
    let manifest = crate_manifest(crate_dir);
    let dep_set = match kind {
        DepKind::Dev => &manifest.dev_dependencies,
        DepKind::Regular => &manifest.dependencies,
    };
    let manifest_deps: HashSet<String> = match dep_set {
        Some(dependencies) => dependencies.keys().cloned().collect(),
        None => HashSet::new(),
    };

    let missing: Vec<_> = expected_deps.difference(&manifest_deps).collect();
    let extra: Vec<_> = manifest_deps.difference(expected_deps).collect();

    let section_str = match kind {
        DepKind::Dev => "[dev-dependencies]",
        DepKind::Regular => "[dependencies]",
    };

    if missing.is_empty() && extra.is_empty() {
        println!("{} match ({})", section_str, crate_dir.display());
    } else {
        if !missing.is_empty() {
            println!(
                "MISSING from {}: {:?} ({})",
                section_str,
                missing,
                crate_dir.display()
            );
        }
        if !extra.is_empty() {
            println!(
                "EXTRA in {}: {:?} ({})",
                section_str,
                extra,
                crate_dir.display()
            );
        }
        exit(1);
    }
}

fn run_command(tokens: &[&str], current_dir: &Path) {
    let (cmd, args) = tokens
        .split_first()
        .expect("command tokens cannot be empty");
    let status = std::process::Command::new(cmd)
        .args(args)
        .current_dir(current_dir)
        .status()
        .unwrap_or_else(|_| panic!("failed to run: {}", tokens.join(" ")));
    assert!(status.success(), "command failed: {}", tokens.join(" "));
}

fn remove_sbf_binary(path: &Path, package_name: &str) {
    let rs_package_name = package_name.replace("-", "_");
    let deploy_dir = path.parent().unwrap().join("target/deploy");
    let binary_path = deploy_dir.join(format!("{}.so", rs_package_name));
    if binary_path.exists() {
        fs::remove_file(&binary_path).expect("failed to remove .so binary");
    }
}

fn build_example(
    path: &Path,
    examples_keypair: &mut Option<String>,
    dev_dependencies: &mut HashSet<String>,
    program_dependencies: &mut HashSet<String>,
) {
    let dir = path.file_name().expect("failed to get directory name");
    let package_name = dir.to_str().unwrap();

    // Collect dependencies for build caching.
    let manifest = crate_manifest(path);
    assert!(
        manifest
            .package
            .clone()
            .expect("missing package section")
            .name
            == package_name,
        "directory name and package name do not match"
    );
    extend_dep_set(dev_dependencies, path, DepKind::Dev);
    extend_dep_set(program_dependencies, path, DepKind::Regular);

    // Verify keypair matches across all examples.
    let keypair_path = path.join(format!("deploy/{}-keypair.json", package_name));
    let keypair = fs::read_to_string(&keypair_path).expect("failed to read keypair file");
    if examples_keypair.is_none() {
        *examples_keypair = Some(keypair);
    } else {
        assert_eq!(
            examples_keypair.as_ref().unwrap(),
            &keypair,
            "example keypair does not match: {}",
            keypair_path.display()
        );
    }

    // Verify there is an assembly file at src/{package-name}/{package-name}.s
    let asm_path = path.join(format!("src/{}/{}.s", package_name, package_name));
    assert!(
        asm_path.exists(),
        "missing assembly file: {}",
        asm_path.display()
    );

    // Run build commands for ELF files to dump.
    run_command(&["sbpf", "build"], path);
    run_command(
        &[
            "cargo",
            "build-sbf",
            "--arch",
            SBPF_ARCH_DUMP,
            "--tools-version",
            TOOLS_VERSION_DUMP,
            "--dump",
        ],
        path,
    );

    // Dump the sbpf build.
    let asm_build_path = path.join(format!("deploy/{}.so", package_name));
    let dump_dir_path = path.join("dumps");
    let asm_dump_path = dump_dir_path.join("asm.txt");
    run_command(
        &[
            "dump.sh",
            asm_build_path.to_str().unwrap(),
            asm_dump_path.to_str().unwrap(),
        ],
        path,
    );

    // Move the cargo-build-sbf dump file.
    let rs_package_name = package_name.replace("-", "_");
    let rs_deploy_dir_path = path.parent().unwrap().join("target/deploy");
    let rs_dump_path_old = rs_deploy_dir_path.join(rs_package_name.clone() + "-dump.txt");
    let rs_dump_path = dump_dir_path.join("rs.txt");
    fs::rename(&rs_dump_path_old, &rs_dump_path).expect("failed to move cargo-build-sbf dump");

    // Clean metadata for the dump files.
    clean_dump_file_metadata(&asm_dump_path, package_name);
    clean_dump_file_metadata(&rs_dump_path, &package_name.replace("-", "_"));

    // Regenerate rust program for disassembly.
    remove_sbf_binary(path, package_name);
    run_command(
        &[
            "cargo",
            "build-sbf",
            "--arch",
            SBPF_ARCH_DISASSEMBLE,
            "--tools-version",
            TOOLS_VERSION_DISASSEMBLE,
        ],
        path,
    );
    let rs_build_path = rs_deploy_dir_path.join(rs_package_name + ".so");
    let rs_asm_path = dump_dir_path.join("rs.s");
    let disassemble_output = std::process::Command::new("sbpf")
        .args(["disassemble", rs_build_path.to_str().unwrap()])
        .output()
        .expect("failed to run: sbpf disassemble");
    assert!(
        disassemble_output.status.success(),
        "command failed: sbpf disassemble ({})",
        String::from_utf8_lossy(&disassemble_output.stderr)
    );
    fs::write(&rs_asm_path, disassemble_output.stdout).expect("failed to write rs.s");

    // Regenerate the testable version of the program now that dumps are done.
    remove_sbf_binary(path, package_name);
    run_command(
        &[
            "cargo",
            "build-sbf",
            "--arch",
            SBPF_ARCH_TEST,
            "--tools-version",
            TOOLS_VERSION_TEST,
        ],
        path,
    );

    // Run tests and save snippets.
    run_and_save_test_snippets(path, package_name);

    // Verify code snippets match source files.
    verify_code_snippets(path, package_name);
}

/// For any lines containing "{package_name}.so", replace the start of the line up until the match
/// with just "{package_name}.so". This removes any build-path-specific metadata.
fn clean_dump_file_metadata(dump_path: &Path, package_name: &str) {
    let dump_contents = fs::read_to_string(dump_path).expect("failed to read dump file");
    let modified_contents = dump_contents
        .lines()
        .map(|line| {
            if let Some(index) = line.find(format!("{}.so", package_name).as_str()) {
                format!(
                    "{}.so{}",
                    package_name,
                    &line[index + format!("{}.so", package_name).len()..]
                )
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");
    fs::write(dump_path, modified_contents).expect("failed to write modified dump file");
}

/// Discover test functions in the tests.rs file and return (test_name, test_code) pairs.
fn discover_tests(tests_path: &Path) -> Vec<(String, String)> {
    let contents = fs::read_to_string(tests_path).expect("failed to read tests.rs");

    // Match #[test] followed by fn test_name(...) { ... }
    // Find matching braces to extract the full function body.
    let test_attr_re = Regex::new(r"#\[test\]\s*\n\s*fn\s+(\w+)").unwrap();

    let mut tests = Vec::new();

    for cap in test_attr_re.captures_iter(&contents) {
        let test_name = cap.get(1).unwrap().as_str().to_string();
        let match_start = cap.get(0).unwrap().start();

        // Find the opening brace of the function.
        let rest = &contents[match_start..];
        if let Some(brace_start) = rest.find('{') {
            // Count braces to find the matching closing brace.
            let mut brace_count = 0;
            let mut end_pos = 0;
            for (i, c) in rest[brace_start..].char_indices() {
                match c {
                    '{' => brace_count += 1,
                    '}' => {
                        brace_count -= 1;
                        if brace_count == 0 {
                            end_pos = brace_start + i + 1;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if end_pos > 0 {
                let test_code = rest[..end_pos].to_string();
                tests.push((test_name, test_code));
            }
        }
    }

    tests
}

/// Run tests and save snippets for each test.
fn run_and_save_test_snippets(path: &Path, package_name: &str) {
    let tests_path = path.join("src/tests.rs");
    assert!(
        tests_path.exists(),
        "missing tests.rs file: {}",
        tests_path.display()
    );

    let tests = discover_tests(&tests_path);
    let snippets_dir = path.join("snippets");

    for (test_name, test_code) in tests {
        // Create the snippet directory.
        let test_dir = snippets_dir.join(&test_name);
        fs::create_dir_all(&test_dir).expect("failed to create snippet directory");

        // Save the test code.
        let test_file = test_dir.join("test.txt");
        fs::write(&test_file, &test_code).expect("failed to write test.txt");

        // Run the test and capture output.
        let full_test_name = format!("tests::{}", test_name);
        let test_output = std::process::Command::new("cargo")
            .args([
                "test",
                "--package",
                package_name,
                "--lib",
                "--",
                &full_test_name,
                "--exact",
                "--nocapture",
            ])
            .current_dir(path)
            .output()
            .expect("failed to run cargo test");
        assert!(
            test_output.status.success(),
            "test failed: {}",
            full_test_name
        );

        // Combine stdout and stderr for the result.
        let mut result = String::from_utf8_lossy(&test_output.stdout).to_string();
        if !test_output.stderr.is_empty() {
            result.push_str(&String::from_utf8_lossy(&test_output.stderr));
        }

        // Clean the test output for readability.
        let result = clean_test_output(&result);

        let result_file = test_dir.join("result.txt");
        fs::write(&result_file, result).expect("failed to write result.txt");
    }
}

/// Clean test output for readability by:
/// - Removing "running N tests" and summary lines
/// - Removing cargo build output lines
/// - Simplifying timestamps to "[ ... DEBUG ... ]"
/// - Truncating the program ID to "DASMAC..."
fn clean_test_output(output: &str) -> String {
    let timestamp_re = Regex::new(r"\[\d{4}-\d{2}-\d{2}T[\d:.]+Z DEBUG [^\]]+\]").unwrap();
    let program_id_re = Regex::new(r"DASMACWxk3nD3fhGGS5XvCgkKvqyZQbU2rJSMyW3Co1z").unwrap();

    output
        .lines()
        .filter(|line| {
            // Keep test result lines and DEBUG log lines.
            (line.starts_with("test ") && line.contains("...")) || line.contains("DEBUG")
        })
        .map(|line| {
            let line = timestamp_re.replace_all(line, "[ ... DEBUG ... ]");
            let line = program_id_re.replace_all(&line, "DASMAC...");
            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Verify that code snippets in snippets/asm and snippets/rs match the source files.
fn verify_code_snippets(path: &Path, package_name: &str) {
    let snippets_dir = path.join("snippets");

    // Check asm snippets against the .s source file.
    let asm_snippets_dir = snippets_dir.join("asm");
    if asm_snippets_dir.exists() {
        let asm_source_path = path.join(format!("src/{}/{}.s", package_name, package_name));
        let asm_source = fs::read_to_string(&asm_source_path)
            .unwrap_or_else(|_| panic!("failed to read {}", asm_source_path.display()));
        verify_snippets_in_source(&asm_snippets_dir, &asm_source, "asm");
    }

    // Check rs snippets against the program.rs source file.
    let rs_snippets_dir = snippets_dir.join("rs");
    if rs_snippets_dir.exists() {
        let rs_source_path = path.join("src/program.rs");
        let rs_source = fs::read_to_string(&rs_source_path)
            .unwrap_or_else(|_| panic!("failed to read {}", rs_source_path.display()));
        verify_snippets_in_source(&rs_snippets_dir, &rs_source, "rs");
    }
}

/// Verify all .txt snippets in a directory exist in the source content.
fn verify_snippets_in_source(snippets_dir: &Path, source: &str, snippet_type: &str) {
    let entries = fs::read_dir(snippets_dir).unwrap_or_else(|_| {
        panic!(
            "failed to read snippets directory: {}",
            snippets_dir.display()
        )
    });

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "txt") {
            let snippet_name = path.file_name().unwrap().to_str().unwrap();
            let snippet_content = fs::read_to_string(&path)
                .unwrap_or_else(|_| panic!("failed to read snippet: {}", path.display()));

            // Normalize whitespace for comparison (trim trailing whitespace from each line).
            let normalized_snippet: String = snippet_content
                .lines()
                .map(|line| line.trim_end())
                .collect::<Vec<_>>()
                .join("\n");

            let normalized_source: String = source
                .lines()
                .map(|line| line.trim_end())
                .collect::<Vec<_>>()
                .join("\n");

            assert!(
                normalized_source.contains(&normalized_snippet),
                "snippet '{}' in snippets/{} not found in source file:\n---\n{}\n---",
                snippet_name,
                snippet_type,
                snippet_content
            );
        }
    }
}
