use std::process::{Command, ExitCode, exit};

pub(crate) fn dev(args: &[String]) -> ExitCode {
    let subcmd = args.first().map(String::as_str);

    match subcmd {
        Some("up") => compose_up(),
        Some("down") => compose_down(),
        Some("reset") => compose_reset(),
        Some("status") => compose_status(),
        Some("psql") => psql(&args[1..]),
        Some("logs") => compose_logs(),
        Some("help") | None => {
            print_dev_help();
            ExitCode::SUCCESS
        }
        Some(unknown) => {
            eprintln!("Unknown dev subcommand: {unknown}");
            print_dev_help();
            ExitCode::FAILURE
        }
    }
}

fn compose_up() -> ExitCode {
    eprintln!("Starting dev services...");
    let status = docker_compose(&["up", "-d", "--wait"]);
    if status {
        eprintln!("\nPostgreSQL is ready at: postgres://chat:chat@localhost/chat_db");
        eprintln!("Run server with:        cargo run -p chat_server");
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn compose_down() -> ExitCode {
    eprintln!("Stopping dev services...");
    if docker_compose(&["down"]) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn compose_reset() -> ExitCode {
    eprintln!("Resetting dev services (removing volumes)...");
    if docker_compose(&["down", "-v"]) && docker_compose(&["up", "-d", "--wait"]) {
        eprintln!("\nPostgreSQL reset and ready at: postgres://chat:chat@localhost/chat_db");
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn compose_status() -> ExitCode {
    if docker_compose(&["ps"]) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn compose_logs() -> ExitCode {
    if docker_compose(&["logs", "--tail=50", "-f"]) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn psql(extra_args: &[String]) -> ExitCode {
    let mut cmd = Command::new("docker");
    cmd.args(["compose", "exec", "postgres", "psql", "-U", "chat", "-d", "chat_db"]);
    for arg in extra_args {
        cmd.arg(arg);
    }
    let status = cmd.status().unwrap_or_else(|e| {
        eprintln!("Failed to run docker compose exec: {e}");
        exit(1);
    });
    if status.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn docker_compose(args: &[&str]) -> bool {
    Command::new("docker")
        .arg("compose")
        .args(args)
        .status()
        .unwrap_or_else(|e| {
            eprintln!("Failed to run docker compose: {e}");
            eprintln!("Is Docker installed and running?");
            exit(1);
        })
        .success()
}

fn print_dev_help() {
    eprintln!(
        "\
Usage: cargo xtask dev <COMMAND>

Commands:
  up       Start dev services (PostgreSQL)
  down     Stop dev services
  reset    Stop, remove volumes, and restart (fresh DB)
  status   Show running services
  psql     Open psql shell to dev database
  logs     Follow service logs"
    );
}
