# xtask — Build Automation

All automation is done via `xtask` (Rust). No shell scripts.

## Usage

```bash
cargo xtask <TASK>
```

## Available Tasks

| Task             | Description                                                        |
| ---------------- | ------------------------------------------------------------------ |
| `check`          | Run fmt check + clippy + tests on entire workspace                 |
| `fmt`            | Run rustfmt on workspace                                           |
| `test`           | Run all tests                                                      |
| `ci [BASE]`      | Smart CI — only check languages that changed vs BASE branch        |
| `codegen`        | Generate Dart & TypeScript packages from `chat_protocol`           |
| `codegen --check`| Verify generated code is up to date (CI mode)                      |

### `ci` — Smart CI

Detects changed files relative to a base branch and runs only the relevant checks:

- **Base branch**: defaults to `develop` (or `master` if the current branch is `develop`). Override with `cargo xtask ci main`.
- **Rust** (`.rs`, `Cargo.toml`, `Cargo.lock`): fmt check, clippy, tests.
- **Dart** (`.dart`, `pubspec.yaml`): format check, analyze, tests.
- **TypeScript** (`.ts`, `package.json`, `tsconfig.json`): tsc type check, vitest.
- **Codegen**: if `crates/chat_protocol/` sources changed, verifies generated Dart/TS is up to date.

#### `--fix` mode

`cargo xtask ci --fix` — auto-fix before checking:

| Language   | Fix actions                                                    |
| ---------- | -------------------------------------------------------------- |
| Rust       | `cargo fmt`, `clippy --fix --allow-dirty --allow-staged`       |
| Dart       | `dart format .`, `dart fix --apply`                            |
| TypeScript | `prettier --write`                                             |
| Codegen    | Regenerates Dart & TypeScript packages                         |

After fixing, the remaining checks (clippy, analyze, tsc, tests) still run to verify correctness.

## Planned Tasks

| Task           | Description                    |
| -------------- | ------------------------------ |
| `migrate`      | Run server database migrations |
| `docker-build` | Build server Docker image      |
