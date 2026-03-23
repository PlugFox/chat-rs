# Chat SDK

Cross-platform chat platform. Server in Rust (axum + PostgreSQL). Dart/Flutter and TypeScript clients connect directly via WebSocket using the binary protocol defined in `chat_protocol`.

## Workspace

| Crate           | Description                                    |
| --------------- | ---------------------------------------------- |
| `chat_protocol` | Shared wire protocol types, codec, error codes |
| `chat_server`   | Server binary (axum + PostgreSQL + WebSocket)  |
| `xtask`         | Build automation                               |

> **Client library** (SQLite cache, outbox) — separate repository `chat_client_rs`.

## Quick Start

```bash
cargo xtask check   # fmt + clippy + tests
cargo build --workspace
```

## Code Generation

Dart and TypeScript client packages are generated from `chat_protocol` Rust types:

```bash
cargo xtask codegen          # generate Dart & TypeScript packages
cargo xtask codegen --check  # verify generated code is up to date (CI)
```

Generated packages live in `packages/chat_core_dart` and `packages/chat_core_ts`.

## Testing

### Rust

```bash
cargo xtask check            # fmt + clippy + tests (all-in-one)
cargo xtask test             # tests only
cargo test --workspace       # tests via cargo directly
```

### Dart

```bash
cd packages/chat_core_dart
dart test                    # run all tests (VM)
dart test -p node            # run all tests (JS/Node)

# coverage
dart test --coverage=coverage
dart run coverage:format_coverage --lcov --in=coverage --out=coverage/lcov.info --report-on=lib --check-ignore
lcov --summary coverage/lcov.info
```

## Documentation

See [CLAUDE.md](CLAUDE.md) for development guide and [SPEC.md](SPEC.md) for full specification.
