# Server Architecture

## Stack

| Component     | Crate                    | Purpose                             |
| ------------- | ------------------------ | ----------------------------------- |
| Web framework | `axum` (ws)              | HTTP + WebSocket                    |
| Async runtime | `tokio` (full)           | Single runtime                      |
| Database      | PostgreSQL + `sqlx`      | Async, compile-time checked queries |
| Cache/Pub-Sub | Redis (`deadpool-redis`) | Pro tier: clustering, presence      |
| JWT           | `jsonwebtoken`           | Token verification                  |
| Rate limiting | `governor`               | GCRA, keyed, lock-free              |
| Config        | `toml` + `serde`         | Server configuration file           |
| Protocol      | `chat_protocol`          | Shared types with client            |

## Directory Structure

```
crates/chat_server/
├── src/
│   ├── main.rs           ← entry point, configuration
│   ├── config.rs         ← TOML config, ServerLimits
│   ├── app.rs            ← axum Router, middleware
│   ├── ws/
│   │   ├── mod.rs        ← WebSocket upgrade handler
│   │   ├── session.rs    ← DeviceSession, subscriptions
│   │   └── dispatch.rs   ← frame dispatching
│   ├── handlers/
│   │   ├── auth.rs       ← POST /auth/token, POST /auth/verify
│   │   ├── bot.rs        ← Bot API endpoints
│   │   └── upload.rs     ← Media upload (chunked)
│   ├── services/
│   │   ├── message.rs    ← message business logic
│   │   ├── chat.rs       ← chat creation/management
│   │   ├── delivery.rs   ← delivery: WS + push + webhook
│   │   ├── presence.rs   ← online statuses
│   │   └── outbox.rs     ← incoming outbox command processing
│   ├── db/
│   │   ├── mod.rs        ← connection pool (sqlx::PgPool)
│   │   ├── queries/      ← SQL queries (compile-time checked)
│   │   └── models.rs     ← server models
│   ├── cluster/
│   │   ├── mod.rs        ← ClusterBackend trait
│   │   ├── single.rs     ← SingleNodeBackend (Free)
│   │   └── redis.rs      ← RedisBackend (Pro)
│   └── webhook/
│       ├── sender.rs     ← webhook event delivery
│       └── interceptor.rs← synchronous interceptor
├── migrations/
│   ├── 001_initial.sql
│   └── ...
└── config.example.toml
```

## Authentication

Two modes:
1. **Pre-issued Chat Token** (production) — backend issues JWT via server API
2. **Token Exchange** (dev) — client token verified via webhook

JWT payload uses internal `user_id: i64` (not string external_id).

## Rate Limiting

Two levels:
- **Connection level**: connections per IP (Tower middleware)
- **Message level**: messages per user per chat (in WS message loop)

Both use `governor` crate (GCRA algorithm).

## Tiers

### Free — Single Node
- In-memory `DashMap` for sessions, `tokio::broadcast` for pub/sub
- Webhooks, Bot API, Interceptors disabled

### Pro — Multi Node (Redis)
- Redis Pub/Sub for cross-node message delivery
- Redis for presence with TTL + heartbeat
- Redis for distributed rate limiting
- Redis for message chunk caching

Abstracted via `ClusterBackend` trait.

### Redis Chunk Caching

Completed message chunks (64 messages) are cacheable in Redis because message IDs
are sequential and messages are never physically deleted — chunk boundaries are stable.

Cache key: `chat:{chat_id}:chunk:{chunk_id}`

Value: serialized list of messages + `max_updated_at`. TTL ~1h or LRU eviction.

**On any message mutation in a chunk:**
```
DEL chat:{chat_id}:chunk:{message_id >> 6}
```

Almost all mutations happen in the last (and occasionally second-to-last) chunk,
so cache invalidation is rare and surgical. Completed chunks (all 64 slots filled)
are effectively immutable — only edits/deletes can invalidate them.

See [protocol.md — Message Chunks](protocol.md#message-chunks) for chunk layout.

## Graceful Shutdown

On `SIGTERM`:
1. Stop accepting new connections
2. Send `DisconnectCode::ServerShutdown` (3000) to all clients
3. Wait for in-flight messages (10s timeout)
4. Flush pending webhooks
5. Close all WS connections
6. Exit

## Configuration

See `config.example.toml` for full reference. Key sections:
- `[server]` — host, port, buffer sizes
- `[database]` — PostgreSQL connection
- `[auth]` — JWT secret, exchange URL
- `[limits]` — message/frame/extra size limits
- `[rate_limits]` — per-IP, per-chat, per-user limits
- `[cluster]` — Redis for Pro tier
- `[webhooks]` — webhook configuration
- `[storage]` — S3 media storage
- `[license]` — license key (empty = Free)
