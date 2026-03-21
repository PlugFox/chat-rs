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

## Documentation

See [CLAUDE.md](CLAUDE.md) for development guide and [SPEC.md](SPEC.md) for full specification.
