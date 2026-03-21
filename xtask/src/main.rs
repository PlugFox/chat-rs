// xtask is a build tool, not a service — eprintln/println are the correct output mechanism here.
#![allow(clippy::disallowed_macros)]

use std::env;
use std::process::{Command, ExitCode, exit};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let task = args.first().map(String::as_str);

    match task {
        Some("check") => check(),
        Some("fmt") => fmt(),
        Some("test") => test(),
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
  check    Run clippy + fmt check + tests on workspace
  fmt      Run rustfmt on workspace
  test     Run all tests"
    );
}

fn check() -> ExitCode {
    let steps: &[(&str, &[&str])] = &[
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
    ];

    for (label, cmd) in steps {
        eprintln!("\n=== {label} ===");
        let status = Command::new(cmd[0]).args(&cmd[1..]).status().unwrap_or_else(|e| {
            eprintln!("Failed to run {}: {e}", cmd[0]);
            exit(1);
        });
        if !status.success() {
            eprintln!("FAILED: {label}");
            return ExitCode::FAILURE;
        }
    }

    eprintln!("\n=== All checks passed ===");
    ExitCode::SUCCESS
}

fn fmt() -> ExitCode {
    let status = Command::new("cargo")
        .args(["fmt", "--all"])
        .status()
        .unwrap_or_else(|e| {
            eprintln!("Failed to run cargo fmt: {e}");
            exit(1);
        });
    if status.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn test() -> ExitCode {
    let status = Command::new("cargo")
        .args(["test", "--workspace"])
        .status()
        .unwrap_or_else(|e| {
            eprintln!("Failed to run cargo test: {e}");
            exit(1);
        });
    if status.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
