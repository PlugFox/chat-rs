# xtask — Build Automation

All automation is done via `xtask` (Rust). No shell scripts.

## Usage

```bash
cargo xtask <TASK>
```

## Available Tasks

| Task    | Description                                        |
| ------- | -------------------------------------------------- |
| `check` | Run fmt check + clippy + tests on entire workspace |
| `fmt`   | Run rustfmt on workspace                           |
| `test`  | Run all tests                                      |

## Planned Tasks

| Task                  | Description                                                 |
| --------------------- | ----------------------------------------------------------- |
| `codegen`             | Generate Dart/TypeScript types from `chat_protocol` structs |
| `codegen --lang dart` | Generate Dart types only                                    |
| `codegen --lang ts`   | Generate TypeScript types only                              |
| `migrate`             | Run server database migrations                              |
| `docker-build`        | Build server Docker image                                   |
