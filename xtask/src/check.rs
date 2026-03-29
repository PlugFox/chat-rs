use std::process::{Command, ExitCode, exit};

pub(crate) fn check() -> ExitCode {
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

pub(crate) fn fmt() -> ExitCode {
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

pub(crate) fn test() -> ExitCode {
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
