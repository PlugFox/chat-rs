# Architecture

## Overview

Chat SDK — кроссплатформенная чат-платформа. Клиенты (Dart/Flutter, TypeScript) подключаются к серверу напрямую через WebSocket. Rust обеспечивает сервер (axum + PostgreSQL). Локальный кэш и хранение сообщений на клиенте — в отдельном репозитории (`chat_client_rs`).

```
Flutter (Dart)                     TypeScript / React
    │                                     │
    │ WebSocket (напрямую)                │ WebSocket (напрямую)
    │                                     │
    └──────────────────┬──────────────────┘
                       │
                  chat_server
              (axum + PostgreSQL)
```

### Опциональный локальный кэш

Dart/Flutter может использовать `chat_client_rs` (отдельный репозиторий) через FFI для персистентного локального кэша сообщений (SQLite). TypeScript на Web хранит кэш в IndexedDB или не использует персистентность.

```
Flutter (Dart)
    │
    ├── WebSocket → chat_server    (транспорт, события)
    │
    └── FFI → chat_client_rs       (локальный кэш, outbox, SQLite)
              [отдельный репозиторий]
```

## Workspace Crates (этот репозиторий)

| Crate           | Purpose                                                                     |
| --------------- | --------------------------------------------------------------------------- |
| `chat_protocol` | Shared types, codec, error codes — wire contract between clients and server |
| `chat_server`   | Server binary — axum, PostgreSQL, WS sessions                               |
| `xtask`         | Build automation (check, fmt, test, codegen)                                |

> **Rust client library** — отдельный репозиторий `chat_client_rs`. Охватывает только кэш и хранение сообщений (SQLite), без WS-транспорта.

## Data Flow — Server-Centric

```
Client (Dart / TS)
    │
    │  WS frames (binary, chat_protocol codec)
    ▼
chat_server
    │
    ├── PostgreSQL (persistent storage)
    ├── DashMap / Redis (sessions, pub-sub)
    └── fan-out → other connected clients
```

Клиент отправляет команды (SendMessage, ReadReceipt, etc.) через WS фреймы. Сервер сохраняет в PostgreSQL и рассылает события всем подписанным сессиям.

## Crate Development Order

1. **`chat_protocol`** — first crate, defines the wire contract
2. **`chat_server`** — server

## Tech Stack (Rust — Server)

| Component         | Crate                              | Purpose                                     |
| ----------------- | ---------------------------------- | ------------------------------------------- |
| Async runtime     | `tokio` (full)                     | Single runtime                              |
| WebSocket         | `tokio-tungstenite` + `rustls`     | No OpenSSL, identical on all platforms      |
| Serialization     | `serde` + `serde_json`             | JSON fields (extra, config)                 |
| Errors (protocol) | `thiserror`                        | Typed errors in `chat_protocol`             |
| Errors (app)      | `anyhow`                           | Application-level errors in server          |
| Bitflags          | `bitflags`                         | Rich text styles, permissions               |
| Cancellation      | `tokio-util` (`CancellationToken`) | Background task shutdown                    |
| Web framework     | `axum` (ws)                        | Server HTTP + WebSocket                     |
| PostgreSQL        | `sqlx` (postgres)                  | Server DB with compile-time checked queries |
| JWT               | `jsonwebtoken`                     | Token verification                          |
| Rate limiting     | `governor`                         | GCRA, keyed, lock-free                      |
| Presence tracking | `roaring`                          | In-memory online bitmap; Redis impl planned |

## Why `rustls` Instead of OpenSSL

- No binary dependencies, simpler app store review
- Public security audit
- Identical behavior on all native platforms
- Standard TLS encryption declaration for App Store / Google Play
