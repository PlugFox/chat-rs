# Chat SDK — Development Guide

## Quick Reference

- **Language**: Rust 2024 edition
- **Architecture**: Cargo workspace with 2 core crates + xtask (server + protocol only)

## Build & Run

```bash
cargo xtask check                    # Clippy + fmt + tests
cargo xtask fmt                      # Format workspace
cargo xtask test                     # Run all tests
cargo build --workspace              # Build everything
cargo run -p chat_server             # Run server (dev)
```

## Workspace Crates

| Crate           | Purpose                                                                     |
| --------------- | --------------------------------------------------------------------------- |
| `chat_protocol` | Shared types, codec, error codes — wire contract between clients and server |
| `chat_server`   | Server binary — axum + PostgreSQL + WS sessions                             |
| `xtask`         | Build automation (check, fmt, test)                                         |

> **Note:** The Rust client library (SQLite cache/storage) lives in a separate repository.
> Dart and TypeScript clients connect to the server directly via WebSocket.

## Rules for Development

- Always discuss architectural changes before implementing them.
- **Before touching any crate: read its `docs/` file first.** Use the Documentation table below. Only explore source if docs are insufficient.
- Keep crates focused — do not add unrelated functionality.
- Prefer small, incremental changes over large rewrites.
- Run `cargo check` after changes to verify compilation.
- When adding new functionality, update the corresponding `docs/` file.
- If docs were missing important details or out of sync — update them before finishing the task.
- No shell scripts — all automation via `xtask/` (Rust).
- `chat_protocol` uses `thiserror` for typed errors. `chat_server` uses `anyhow` for application errors.
- All IDs in wire protocol are `u32`. External string user IDs are mapped to internal `u32` on the server.

## Testing

- Write unit tests (`#[cfg(test)]`) for non-trivial pure logic.
- Cover integration scenarios.
- Think in corner cases: empty collections, zero/max values, missing values, race conditions.
- Use `proptest` for codec roundtrip testing and logic that must hold over ranges.
- Do not mock the database — use `:memory:` SQLite so real schema and queries are exercised.
- Property-based tests for all codec encode/decode paths.

## Performance

- Never block the main thread. Use `spawn_blocking` for CPU-heavy work.
- Use batch queries and transactions for database access.
- Wrap shared resources in `Arc` — cloning is just an atomic increment.
- Profile before optimizing. Use `cargo flamegraph`.
- Write `criterion` benchmarks for bulk data processing.
- Avoid heap allocation in hot paths.

## Documentation

| Topic                                 | File                                           |
| ------------------------------------- | ---------------------------------------------- |
| **Project goals & vision**            | [docs/goals.md](docs/goals.md)                 |
| **Roadmap & milestones**              | [docs/milestones.md](docs/milestones.md)       |
| Architecture & crate structure        | [docs/architecture.md](docs/architecture.md)   |
| WebSocket protocol & frames           | [docs/protocol.md](docs/protocol.md)           |
| User structure, flags & wire layout   | [docs/users.md](docs/users.md)                 |
| Chat structure, roles & wire layout   | [docs/chats.md](docs/chats.md)                 |
| Message structure & wire layout       | [docs/messages.md](docs/messages.md)           |
| Binary codec format                   | [docs/codec.md](docs/codec.md)                 |
| Error codes & disconnect codes        | [docs/error-codes.md](docs/error-codes.md)     |
| Database design (SQLite + PostgreSQL) | [docs/database.md](docs/database.md)           |
| Server architecture                   | [docs/server.md](docs/server.md)               |
| Client integration guide              | [docs/client.md](docs/client.md)               |
| Performance guidelines                | [docs/performance.md](docs/performance.md)     |
| Cross-platform notes                  | [docs/crossplatform.md](docs/crossplatform.md) |
| xtask automation                      | [docs/xtask.md](docs/xtask.md)                 |
| Future plans & client notes           | [docs/future-plans.md](docs/future-plans.md)   |
