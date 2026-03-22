/// Code generation orchestrator: parse → IR → generate → format.

pub(crate) mod dart;
pub(crate) mod ir;
pub(crate) mod parser;
pub(crate) mod typescript;

use std::fs;
use std::path::Path;
use std::process::{Command, ExitCode};

/// Run code generation from the workspace root.
pub(crate) fn run(workspace_root: &Path, check: bool) -> ExitCode {
    let protocol_src = workspace_root.join("crates/chat_protocol/src");

    eprintln!("Parsing protocol sources...");
    let ir = match parser::parse_protocol(&protocol_src) {
        Ok(ir) => ir,
        Err(e) => {
            eprintln!("Parse error: {e}");
            return ExitCode::FAILURE;
        }
    };

    eprintln!(
        "Parsed: {} enums, {} structs, {} bitflags, {} tagged enums, {} constants",
        ir.enums.len(),
        ir.structs.len(),
        ir.bitflags.len(),
        ir.tagged_enums.len(),
        ir.constants.len(),
    );

    if check {
        return run_check(workspace_root, &ir);
    }

    let dart_out = workspace_root.join("packages/chat_core_dart");
    let ts_out = workspace_root.join("packages/chat_core_ts");

    eprintln!("Generating Dart package...");
    if let Err(e) = dart::generate(&ir, &dart_out) {
        eprintln!("Dart generation error: {e}");
        return ExitCode::FAILURE;
    }

    eprintln!("Generating TypeScript package...");
    if let Err(e) = typescript::generate(&ir, &ts_out) {
        eprintln!("TypeScript generation error: {e}");
        return ExitCode::FAILURE;
    }

    format_dart(&dart_out);
    format_ts(&ts_out);

    eprintln!("Code generation complete.");
    ExitCode::SUCCESS
}

/// Run in check mode: generate to a temp dir, format, and compare with existing.
fn run_check(workspace_root: &Path, ir: &ir::ParsedModule) -> ExitCode {
    let tmp = std::env::temp_dir().join("chat_codegen_check");

    // Clean up any previous run
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).expect("failed to create temp dir");

    let tmp_dart = tmp.join("chat_core_dart");
    let tmp_ts = tmp.join("chat_core_ts");

    eprintln!("Generating to temp directory for comparison...");

    if let Err(e) = dart::generate(ir, &tmp_dart) {
        eprintln!("Dart generation error: {e}");
        let _ = fs::remove_dir_all(&tmp);
        return ExitCode::FAILURE;
    }

    if let Err(e) = typescript::generate(ir, &tmp_ts) {
        eprintln!("TypeScript generation error: {e}");
        let _ = fs::remove_dir_all(&tmp);
        return ExitCode::FAILURE;
    }

    // Format temp output so comparison is against formatted code.
    format_dart(&tmp_dart);
    format_ts(&tmp_ts);

    let existing_dart = workspace_root.join("packages/chat_core_dart");
    let existing_ts = workspace_root.join("packages/chat_core_ts");

    let mut diffs = Vec::new();

    compare_dirs(&tmp_dart, &existing_dart, &tmp_dart, &mut diffs);
    compare_dirs(&tmp_ts, &existing_ts, &tmp_ts, &mut diffs);

    let _ = fs::remove_dir_all(&tmp);

    if diffs.is_empty() {
        eprintln!("codegen --check: all generated files up to date.");
        ExitCode::SUCCESS
    } else {
        eprintln!("codegen --check: generated code is out of date!");
        for d in &diffs {
            eprintln!("  {d}");
        }
        eprintln!(
            "\n{} file(s) differ. Run `cargo xtask codegen` to update.",
            diffs.len()
        );
        ExitCode::FAILURE
    }
}

// ---------------------------------------------------------------------------
// Formatting
// ---------------------------------------------------------------------------

fn format_dart(dir: &Path) {
    eprintln!("Formatting Dart...");

    // Collect dirs that actually exist — dart format errors on missing paths.
    let subdirs: Vec<&str> = ["lib", "test", "benchmark"]
        .into_iter()
        .filter(|d| dir.join(d).exists())
        .collect();

    if subdirs.is_empty() {
        return;
    }

    let mut args = vec!["format", "-l", "80"];
    args.extend(subdirs);

    match Command::new("dart")
        .args(&args)
        .current_dir(dir)
        .stdout(std::process::Stdio::null())
        .status()
    {
        Ok(s) if !s.success() => eprintln!("Warning: dart format failed"),
        Err(_) => eprintln!("Warning: dart not found, skipping format"),
        _ => {}
    }
}

fn format_ts(dir: &Path) {
    eprintln!("Formatting TypeScript...");

    // Collect globs that match existing dirs.
    let mut globs = Vec::new();
    if dir.join("src").exists() {
        globs.push("src/**/*.ts".to_string());
    }
    if dir.join("tests").exists() {
        globs.push("tests/**/*.ts".to_string());
    }

    if globs.is_empty() {
        return;
    }

    let mut args = vec![
        "prettier".to_string(),
        "--write".to_string(),
        "--log-level".to_string(),
        "warn".to_string(),
    ];
    args.extend(globs);

    match Command::new("npx")
        .args(&args)
        .current_dir(dir)
        .status()
    {
        Ok(s) if !s.success() => eprintln!("Warning: prettier format failed"),
        Err(_) => eprintln!("Warning: npx not found, skipping format"),
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Directory comparison
// ---------------------------------------------------------------------------

/// Scaffolding files that are written once and may be hand-edited.
/// These are excluded from `--check` comparison.
const SCAFFOLDING: &[&str] = &[
    "pubspec.yaml",
    "package.json",
    "tsconfig.json",
    "vitest.config.ts",
    "error.dart",
    "reader.dart",
    "writer.dart",
    "error.ts",
    "reader.ts",
    "writer.ts",
    "_util.dart",
];

/// Recursively compare files in `gen_dir` against `existing_dir`.
/// `gen_root` is the top-level generated dir, used to compute relative paths.
/// Scaffolding files are skipped — they don't depend on IR.
fn compare_dirs(gen_dir: &Path, existing_dir: &Path, gen_root: &Path, diffs: &mut Vec<String>) {
    let entries = match fs::read_dir(gen_dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let existing = existing_dir.join(&name);

        if path.is_dir() {
            compare_dirs(&path, &existing, gen_root, diffs);
        } else if path.is_file() {
            // Skip scaffolding files — they are stable and may be hand-edited.
            if let Some(fname) = name.to_str() {
                if SCAFFOLDING.contains(&fname) {
                    continue;
                }
            }

            let rel = path.strip_prefix(gen_root).unwrap_or(&path);

            if !existing.exists() {
                diffs.push(format!("missing: {}", rel.display()));
                continue;
            }

            let gen_content = fs::read_to_string(&path).unwrap_or_default();
            let existing_content = fs::read_to_string(&existing).unwrap_or_default();

            if gen_content != existing_content {
                diffs.push(format!("differs: {}", rel.display()));
            }
        }
    }
}
