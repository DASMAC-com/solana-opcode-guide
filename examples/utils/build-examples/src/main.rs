use cargo_manifest::Manifest;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

fn main() {
    let examples_dir = std::env::current_dir().expect("failed to get current directory");

    // Dependencies from example programs (for program-deps verification).
    let mut program_deps: BTreeSet<String> = BTreeSet::new();
    // Dependencies from build-examples (for build-deps verification only).
    let mut build_example_deps: BTreeSet<String> = BTreeSet::new();
    let mut all_dev_deps: BTreeSet<String> = BTreeSet::new();

    // Find all Cargo.toml files (excluding the root).
    let cargo_tomls = find_cargo_tomls(&examples_dir);

    // Store paths to special crates for later verification.
    let mut program_deps_path: Option<std::path::PathBuf> = None;
    let mut build_deps_path: Option<std::path::PathBuf> = None;

    for cargo_toml_path in &cargo_tomls {
        let manifest = Manifest::from_path(cargo_toml_path).expect("failed to parse Cargo.toml");

        // Skip workspace root Cargo.toml files (no package section).
        let Some(package) = &manifest.package else {
            continue;
        };

        // Track paths to special crates.
        if package.name == "program-deps" {
            program_deps_path = Some(cargo_toml_path.clone());
            continue;
        }
        if package.name == "build-deps" {
            build_deps_path = Some(cargo_toml_path.clone());
            continue;
        }
        if package.name == "test-utils" {
            continue;
        }

        let crate_name = &package.name;

        // Collect dependency names (build-examples deps tracked separately).
        if let Some(deps) = &manifest.dependencies {
            for name in deps.keys() {
                if package.name == "build-examples" {
                    build_example_deps.insert(name.clone());
                } else {
                    program_deps.insert(name.clone());
                }
            }
        }

        // Collect dev-dependency names.
        if let Some(deps) = &manifest.dev_dependencies {
            for name in deps.keys() {
                all_dev_deps.insert(name.clone());
            }
        }

        println!("Found: {} ({})", crate_name, cargo_toml_path.display());
    }

    println!("\n=== Program Dependencies ===");
    for dep in &program_deps {
        println!("  {}", dep);
    }

    println!("\n=== Build Example Dependencies ===");
    for dep in &build_example_deps {
        println!("  {}", dep);
    }

    println!("\n=== Dev Dependencies ===");
    for dep in &all_dev_deps {
        println!("  {}", dep);
    }

    // Verify program-deps has the same dependencies as collected program deps.
    let mut has_errors = false;
    if let Some(path) = program_deps_path {
        println!("\n=== Verifying program-deps ===");
        let manifest = Manifest::from_path(&path).expect("failed to parse program-deps Cargo.toml");
        let program_deps_crate: BTreeSet<String> = manifest
            .dependencies
            .as_ref()
            .map(|d| d.keys().cloned().collect())
            .unwrap_or_default();

        let missing: Vec<_> = program_deps.difference(&program_deps_crate).collect();
        let extra: Vec<_> = program_deps_crate.difference(&program_deps).collect();

        if missing.is_empty() && extra.is_empty() {
            println!("  OK: dependencies match");
        } else {
            has_errors = true;
            if !missing.is_empty() {
                println!("  MISSING from program-deps [dependencies]: {:?}", missing);
            }
            if !extra.is_empty() {
                println!("  EXTRA in program-deps [dependencies]: {:?}", extra);
            }
        }
    }

    // Verify build-deps has the same dependencies AND dev-dependencies.
    // build-deps should include both program deps and build-examples deps.
    if let Some(path) = build_deps_path {
        println!("\n=== Verifying build-deps ===");
        let manifest = Manifest::from_path(&path).expect("failed to parse build-deps Cargo.toml");

        // All deps for build-deps = program deps + build-examples deps.
        let all_build_deps: BTreeSet<String> =
            program_deps.union(&build_example_deps).cloned().collect();

        // Check dependencies.
        let build_deps_crate: BTreeSet<String> = manifest
            .dependencies
            .as_ref()
            .map(|d| d.keys().cloned().collect())
            .unwrap_or_default();

        let missing: Vec<_> = all_build_deps.difference(&build_deps_crate).collect();
        let extra: Vec<_> = build_deps_crate.difference(&all_build_deps).collect();

        if missing.is_empty() && extra.is_empty() {
            println!("  OK: [dependencies] match");
        } else {
            has_errors = true;
            if !missing.is_empty() {
                println!("  MISSING from build-deps [dependencies]: {:?}", missing);
            }
            if !extra.is_empty() {
                println!("  EXTRA in build-deps [dependencies]: {:?}", extra);
            }
        }

        // Check dev-dependencies.
        let build_deps_dev_deps: BTreeSet<String> = manifest
            .dev_dependencies
            .as_ref()
            .map(|d| d.keys().cloned().collect())
            .unwrap_or_default();

        let missing: Vec<_> = all_dev_deps.difference(&build_deps_dev_deps).collect();
        let extra: Vec<_> = build_deps_dev_deps.difference(&all_dev_deps).collect();

        if missing.is_empty() && extra.is_empty() {
            println!("  OK: [dev-dependencies] match");
        } else {
            has_errors = true;
            if !missing.is_empty() {
                println!(
                    "  MISSING from build-deps [dev-dependencies]: {:?}",
                    missing
                );
            }
            if !extra.is_empty() {
                println!("  EXTRA in build-deps [dev-dependencies]: {:?}", extra);
            }
        }
    }

    if has_errors {
        std::process::exit(1);
    }
}

fn find_cargo_tomls(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut results = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            // Skip target directory.
            if path.file_name().map(|n| n == "target").unwrap_or(false) {
                continue;
            }

            if path.is_dir() {
                // Check for Cargo.toml in this directory.
                let cargo_toml = path.join("Cargo.toml");
                if cargo_toml.exists() {
                    results.push(cargo_toml);
                }
                // Recurse into subdirectories.
                results.extend(find_cargo_tomls(&path));
            }
        }
    }

    results
}
