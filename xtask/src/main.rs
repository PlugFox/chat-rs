// xtask is a build tool, not a service — eprintln/println are the correct output mechanism here.
#![allow(clippy::disallowed_macros)]

mod check;
mod ci;
mod codegen;

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let task = args.first().map(String::as_str);

    match task {
        Some("check") => check::check(),
        Some("fmt") => check::fmt(),
        Some("test") => check::test(),
        Some("ci") => {
            let fix = args.iter().any(|a| a == "--fix");
            let base = args[1..].iter().find(|a| !a.starts_with('-')).map(String::as_str);
            ci::ci(base, fix)
        }
        Some("codegen") => {
            let workspace_root = workspace_root();
            let is_check = args.iter().any(|a| a == "--check");
            codegen::run(&workspace_root, is_check)
        }
        Some("help") | None => {
            print_help();
            ExitCode::SUCCESS
        }
        Some(unknown) => {
            eprintln!("Unknown task: {unknown}");
            print_help();
            ExitCode::FAILURE
        }
    }
}

fn print_help() {
    eprintln!(
        "\
Usage: cargo xtask <TASK>

Tasks:
  check       Run clippy + fmt check + tests on workspace
  fmt         Run rustfmt on workspace
  test        Run all tests
  ci [BASE]   Smart CI — detect changed files vs BASE branch and run
              only the relevant checks (Rust/Dart/TypeScript).
              BASE defaults to 'develop' (or 'master' if on develop).
              --fix  Auto-fix formatting, lints, and regenerate code.
  codegen     Generate Dart & TypeScript packages from chat_protocol
              --check  Verify generated code is up to date (CI mode)"
    );
}

/// Locate the workspace root (parent of the xtask directory).
fn workspace_root() -> std::path::PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    std::path::Path::new(manifest_dir)
        .parent()
        .expect("xtask should be in a subdirectory of the workspace root")
        .to_owned()
}
