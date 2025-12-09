use cargo_manifest::Manifest;
use std::collections::HashSet;
use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

const SBPF_ARCH_DUMP: &str = "v4";
const SBPF_ARCH_TEST: &str = "v3";
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

    // Get dependencies and dev dependencies from all example crates.
    for path in dir_paths {
        let dir = path.file_name().expect("failed to get directory name");
        if dir == "target" {
            continue;
        } else if dir == "utils" {
            utils_path = Some(path.clone());
        } else {
            process_example(
                &path,
                &mut examples_keypair,
                &mut dev_dependencies,
                &mut program_dependencies,
            );
        }
    }

    // Parse dependencies from non-example crates.
    let utils_path = utils_path.expect("missing utils directory");
    let test_utils_crate = &utils_path.clone().join("test-utils");
    let build_examples_crate = &utils_path.clone().join("build-examples");
    let mut build_dependencies = program_dependencies.clone();

    // All test utils dependencies are dev dependencies.
    extend_dep_set(&mut dev_dependencies, test_utils_crate, false);
    extend_dep_set(&mut dev_dependencies, build_examples_crate, true);
    extend_dep_set(&mut build_dependencies, build_examples_crate, false);

    // Verify program dependencies.
    let deps_path = utils_path.join("deps");
    let program_deps_crate = deps_path.join("program");
    let build_deps_crate = deps_path.join("build");

    // Verify manifest dependencies.
    verify_manifest_deps(&program_deps_crate, &program_dependencies, false);
    verify_manifest_deps(&build_deps_crate, &dev_dependencies, true);
    verify_manifest_deps(&build_deps_crate, &build_dependencies, false);

    exit(0);
}

fn extend_dep_set(target: &mut HashSet<String>, crate_dir: &Path, dev_deps: bool) {
    let manifest = crate_manifest(crate_dir);
    let dep_set = if dev_deps {
        &manifest.dev_dependencies
    } else {
        &manifest.dependencies
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

fn verify_manifest_deps(crate_dir: &Path, expected_deps: &HashSet<String>, dev_deps: bool) {
    let manifest = crate_manifest(crate_dir);
    let dep_set = if dev_deps {
        &manifest.dev_dependencies
    } else {
        &manifest.dependencies
    };
    let manifest_deps: HashSet<String> = match dep_set {
        Some(dependencies) => dependencies.keys().cloned().collect(),
        None => HashSet::new(),
    };

    let missing: Vec<_> = expected_deps.difference(&manifest_deps).collect();
    let extra: Vec<_> = manifest_deps.difference(expected_deps).collect();

    if missing.is_empty() && extra.is_empty() {
        println!(
            "{} match ({})",
            dependency_section_str(&dev_deps),
            crate_dir.display()
        );
    } else {
        if !missing.is_empty() {
            println!(
                "MISSING from {}: {:?} ({})",
                dependency_section_str(&dev_deps),
                missing,
                crate_dir.display()
            );
        }
        if !extra.is_empty() {
            println!(
                "EXTRA in {}: {:?} ({})",
                dependency_section_str(&dev_deps),
                extra,
                crate_dir.display()
            );
        }
        exit(1);
    }
}

fn dependency_section_str(dev_deps: &bool) -> &str {
    if *dev_deps {
        "[dev-dependencies]"
    } else {
        "[dependencies]"
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

fn process_example(
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
    extend_dep_set(dev_dependencies, path, true);
    extend_dep_set(program_dependencies, path, false);

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

    // Run build commands.
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
}
