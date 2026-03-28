use std::path::Path;
use std::process::{Command, ExitCode, exit};

/// Detect changed files relative to a base branch and run only the relevant checks.
/// When `fix` is true, auto-fix what can be fixed (formatting, codegen) before checking.
pub(crate) fn ci(base: Option<&str>, fix: bool) -> ExitCode {
    let base = base.unwrap_or_else(|| detect_base_branch());
    let mode = if fix { "CI+FIX" } else { "CI" };

    eprintln!("=== {mode}: comparing against '{base}' ===\n");

    let changed = changed_files(base);
    if changed.is_empty() {
        eprintln!("No changed files detected — nothing to check.");
        return ExitCode::SUCCESS;
    }

    let scope = ChangeScope::from_files(&changed);

    eprintln!("Changed files ({}):", changed.len());
    for f in &changed {
        eprintln!("  {f}");
    }
    eprintln!();

    if !scope.rust && !scope.dart && !scope.typescript {
        eprintln!("No Rust/Dart/TypeScript changes detected — nothing to check.");
        return ExitCode::SUCCESS;
    }

    eprintln!(
        "Scope: rust={}, dart={}, typescript={}, codegen_check={}\n",
        scope.rust, scope.dart, scope.typescript, scope.codegen_check
    );

    let mut failed = false;

    if scope.rust {
        if !run_rust_checks(fix) {
            failed = true;
        }
    }

    if scope.codegen_check {
        if !run_codegen_check(fix) {
            failed = true;
        }
    }

    if scope.dart {
        if !run_dart_checks(fix) {
            failed = true;
        }
    }

    if scope.typescript {
        if !run_typescript_checks(fix) {
            failed = true;
        }
    }

    if failed {
        eprintln!("\n=== CI FAILED ===");
        ExitCode::FAILURE
    } else {
        eprintln!("\n=== CI PASSED ===");
        ExitCode::SUCCESS
    }
}

// ── Change detection ───────────────────────────────────────────────

struct ChangeScope {
    rust: bool,
    dart: bool,
    typescript: bool,
    /// Protocol source changed — generated code must be up to date.
    codegen_check: bool,
}

impl ChangeScope {
    fn from_files(files: &[String]) -> Self {
        let mut rust = false;
        let mut dart = false;
        let mut typescript = false;
        let mut codegen_check = false;

        for f in files {
            let path = Path::new(f);

            match path.extension().and_then(|e| e.to_str()) {
                Some("rs") => {
                    rust = true;
                    // If protocol source changed, codegen must be verified.
                    if f.starts_with("crates/chat_protocol/") {
                        codegen_check = true;
                    }
                }
                Some("dart") => dart = true,
                Some("ts") | Some("tsx") => typescript = true,
                _ => {}
            }

            // Cargo.toml / Cargo.lock changes → treat as Rust.
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == "Cargo.toml" || name == "Cargo.lock" {
                rust = true;
            }
            if name == "pubspec.yaml" || name == "pubspec.lock" || name == "analysis_options.yaml"
            {
                dart = true;
            }
            if name == "package.json"
                || name == "package-lock.json"
                || name == "tsconfig.json"
                || name == "vitest.config.ts"
            {
                typescript = true;
            }
        }

        Self {
            rust,
            dart,
            typescript,
            codegen_check,
        }
    }
}

/// Return the list of files that differ from the base branch,
/// including both committed and uncommitted (staged + unstaged) changes.
fn changed_files(base: &str) -> Vec<String> {
    let mut files = Vec::new();

    // Committed changes on this branch vs base.
    if let Some(output) = git(&["diff", "--name-only", &format!("{base}...HEAD")]) {
        for line in output.lines() {
            let l = line.trim();
            if !l.is_empty() {
                files.push(l.to_string());
            }
        }
    }

    // Uncommitted changes (staged + unstaged).
    if let Some(output) = git(&["diff", "--name-only", "HEAD"]) {
        for line in output.lines() {
            let l = line.trim();
            if !l.is_empty() && !files.contains(&l.to_string()) {
                files.push(l.to_string());
            }
        }
    }

    // Untracked files.
    if let Some(output) = git(&["ls-files", "--others", "--exclude-standard"]) {
        for line in output.lines() {
            let l = line.trim();
            if !l.is_empty() && !files.contains(&l.to_string()) {
                files.push(l.to_string());
            }
        }
    }

    files.sort();
    files
}

fn detect_base_branch() -> &'static str {
    let current = git(&["rev-parse", "--abbrev-ref", "HEAD"]).unwrap_or_default();
    let current = current.trim();
    if current == "develop" { "master" } else { "develop" }
}

fn git(args: &[&str]) -> Option<String> {
    Command::new("git")
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
}

// ── Runners ────────────────────────────────────────────────────────

fn run_rust_checks(fix: bool) -> bool {
    if fix {
        // Fix first, then verify.
        run_steps(
            "Rust",
            &[
                ("Formatting", &["cargo", "fmt", "--all"]),
                (
                    "Clippy auto-fix",
                    &[
                        "cargo",
                        "clippy",
                        "--workspace",
                        "--all-targets",
                        "--fix",
                        "--allow-dirty",
                        "--allow-staged",
                        "--",
                        "-D",
                        "warnings",
                    ],
                ),
                ("Running tests", &["cargo", "test", "--workspace"]),
            ],
        )
    } else {
        run_steps(
            "Rust",
            &[
                ("Checking formatting", &["cargo", "fmt", "--all", "--check"]),
                (
                    "Running clippy",
                    &[
                        "cargo",
                        "clippy",
                        "--workspace",
                        "--all-targets",
                        "--",
                        "-D",
                        "warnings",
                    ],
                ),
                ("Running tests", &["cargo", "test", "--workspace"]),
            ],
        )
    }
}

fn run_codegen_check(fix: bool) -> bool {
    if fix {
        // Regenerate, then verify.
        run_steps(
            "Codegen",
            &[("Regenerating Dart & TypeScript", &["cargo", "xtask", "codegen"])],
        )
    } else {
        run_steps(
            "Codegen",
            &[(
                "Verifying generated code is up to date",
                &["cargo", "xtask", "codegen", "--check"],
            )],
        )
    }
}

fn run_dart_checks(fix: bool) -> bool {
    let dir = "packages/chat_core_dart";
    if fix {
        run_steps_in(
            "Dart",
            dir,
            &[
                ("Formatting", &["dart", "format", "."]),
                ("Fix lints", &["dart", "fix", "--apply"]),
                ("Analyzing", &["dart", "analyze", "--fatal-infos"]),
                ("Running tests", &["dart", "test"]),
            ],
        )
    } else {
        run_steps_in(
            "Dart",
            dir,
            &[
                (
                    "Checking formatting",
                    &["dart", "format", "--set-exit-if-changed", "."],
                ),
                ("Analyzing", &["dart", "analyze", "--fatal-infos"]),
                ("Running tests", &["dart", "test"]),
            ],
        )
    }
}

fn run_typescript_checks(fix: bool) -> bool {
    let dir = "packages/chat_core_ts";

    // Ensure deps are installed.
    let node_modules = Path::new(dir).join("node_modules");
    if !node_modules.exists() {
        eprintln!("\n--- TypeScript: installing dependencies ---");
        let status = Command::new("npm")
            .args(["install"])
            .current_dir(dir)
            .status()
            .unwrap_or_else(|e| {
                eprintln!("Failed to run npm install: {e}");
                exit(1);
            });
        if !status.success() {
            eprintln!("FAILED: npm install");
            return false;
        }
    }

    if fix {
        run_steps_in(
            "TypeScript",
            dir,
            &[
                ("Formatting", &["npx", "prettier", "--write", "src/", "tests/"]),
                ("Type checking", &["npx", "tsc", "--noEmit"]),
                ("Running tests", &["npx", "vitest", "run"]),
            ],
        )
    } else {
        run_steps_in(
            "TypeScript",
            dir,
            &[
                ("Type checking", &["npx", "tsc", "--noEmit"]),
                ("Running tests", &["npx", "vitest", "run"]),
            ],
        )
    }
}

// ── Helpers ────────────────────────────────────────────────────────

fn run_steps(group: &str, steps: &[(&str, &[&str])]) -> bool {
    run_steps_in(group, ".", steps)
}

fn run_steps_in(group: &str, dir: &str, steps: &[(&str, &[&str])]) -> bool {
    for (label, cmd) in steps {
        eprintln!("\n--- {group}: {label} ---");
        let status = Command::new(cmd[0])
            .args(&cmd[1..])
            .current_dir(dir)
            .status()
            .unwrap_or_else(|e| {
                eprintln!("Failed to run {}: {e}", cmd[0]);
                exit(1);
            });
        if !status.success() {
            eprintln!("FAILED: {group}: {label}");
            return false;
        }
    }
    true
}
