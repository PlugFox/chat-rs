# Performance Guidelines

## General Rules

- Never block the main thread. Use `spawn_blocking` for CPU-heavy work.
- Use `rayon` for CPU-bound parallelism on large collections.
- Batch database queries and use transactions.
- Wrap shared heavy resources in `Arc` — cloning is just an atomic increment.
- Profile before optimizing. Use `cargo flamegraph` or Tracy.
- Write `criterion` benchmarks for bulk data processing.
- Avoid heap allocation in hot paths: reuse `Vec` buffers, pool scratch memory.

## WebSocket Batching (Server)

Server can batch multiple events into a single WS frame (up to 20 messages or 16ms). Reduces per-frame overhead for burst delivery (SyncBatch).

## Client-Side Storage (chat_client_rs, separate repo)

- WAL mode SQLite enables concurrent reads with single writer.
- Read pool (r2d2, 2–4 connections) prevents read queries from serializing.
- Cursor-based pagination (WHERE id < ?, not OFFSET) — O(log n).
- No client-side FTS — search is server-only via PostgreSQL.


## Server

### Backpressure

Each WS session has a bounded send buffer (configurable, default 256 frames). On overflow → disconnect with `BufferOverflow` (3004, non-terminal).

### Rate Limiting

`governor` crate — GCRA algorithm, lock-free atomics.
- Connection level: per-IP (Tower middleware)
- Message level: per-user-per-chat (WS message loop)

Limits are configurable via `config.toml` and sent to clients in Welcome frame for client-side debouncing.
