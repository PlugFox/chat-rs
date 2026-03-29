# Server Architecture

## Stack

| Component     | Crate                    | Purpose                             |
| ------------- | ------------------------ | ----------------------------------- |
| Web framework | `axum` (ws)              | HTTP + WebSocket                    |
| Async runtime | `tokio` (full)           | Single runtime                      |
| Database      | PostgreSQL + `sqlx`      | Async, runtime-checked queries      |
| Cache/Pub-Sub | Redis (`deadpool-redis`) | Pro tier: clustering, presence      |
| JWT           | `jsonwebtoken`           | Token verification (HS256)          |
| Rate limiting | `governor`               | GCRA, keyed, lock-free              |
| Config        | `toml` + `serde`         | Server configuration file           |
| Protocol      | `chat_protocol`          | Shared types with client            |
| Sessions      | `dashmap`                | Lock-free concurrent session map    |
| Sync          | `parking_lot`            | Lightweight mutex for subscriptions |

## Directory Structure

```
crates/chat_server/
├── src/
│   ├── lib.rs            ← library root (re-exports for tests)
│   ├── main.rs           ← entry point, signal handling
│   ├── config.rs         ← TOML config parsing, validation, defaults
│   ├── app.rs            ← axum Router, middleware (trace, timeout, CORS)
│   ├── state.rs          ← AppState, SessionHandle
│   ├── ws/
│   │   ├── mod.rs        ← WebSocket upgrade handler
│   │   ├── session.rs    ← frame encoding helpers (Ack, Error, event)
│   │   └── dispatch.rs   ← frame loop, dispatch by FramePayload
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── auth.rs       ← Hello → JWT verify → Welcome
│   │   ├── message.rs    ← SendMessage → validate → insert → ack → fan-out
│   │   └── subscribe.rs  ← Subscribe/Unsubscribe channel management
│   └── db/
│       ├── mod.rs        ← connection pool (sqlx::PgPool), migrations
│       └── queries.rs    ← SQL queries (runtime-checked)
├── migrations/
│   └── 001_initial.sql   ← full schema (10 tables)
├── tests/
│   ├── common/
│   │   └── mod.rs        ← TestServer + TestClient
│   └── integration.rs    ← 4 integration tests
└── config/
    ├── config.example.toml
    └── config.dev.toml
```

### Planned modules (future milestones)

```
│   ├── handlers/
│   │   ├── bot.rs        ← Bot API endpoints (M7)
│   │   └── upload.rs     ← Media upload (M3)
│   ├── services/
│   │   ├── message.rs    ← message business logic (M2+)
│   │   ├── chat.rs       ← chat creation/management (M6)
│   │   ├── delivery.rs   ← delivery: WS + push + webhook (M4/M7)
│   │   ├── presence.rs   ← online statuses (M2)
│   │   └── outbox.rs     ← incoming outbox command processing (M2)
│   ├── cluster/
│   │   ├── mod.rs        ← ClusterBackend trait (M8)
│   │   ├── single.rs     ← SingleNodeBackend (Free)
│   │   └── redis.rs      ← RedisBackend (Pro)
│   └── webhook/
│       ├── sender.rs     ← webhook event delivery (M7)
│       └── interceptor.rs← synchronous interceptor (M7)
```

## Authentication

JWT-based with `sub` claim containing the external user ID (string). Server maps `external_id` → internal `u32` on Hello.

Claims format:
```json
{
  "sub": "external_user_id",
  "exp": 1234567890
}
```

Algorithm: HS256 with shared secret from `config.auth.jwt_secret`.

## Session Management

Each WebSocket connection is represented by a `SessionHandle`:
- `user_id: u32` — internal user ID
- `device_id: Uuid` — client device identifier
- `session_id: u32` — transient per-connection ID
- `subscriptions: Mutex<HashSet<String>>` — subscribed channels
- `sender: mpsc::Sender<Vec<u8>>` — bounded outbound frame buffer
- `event_seq: AtomicU32` — monotonic counter for server-push events

Sessions are stored in `DashMap<(u32, Uuid), Arc<SessionHandle>>`. Duplicate sessions (same user + device) are replaced.

## Rate Limiting

Two levels (planned):
- **Connection level**: connections per IP (Tower middleware)
- **Message level**: messages per user per chat (in WS message loop)

Both use `governor` crate (GCRA algorithm).

## Tiers

### Free — Single Node
- In-memory `DashMap` for sessions, channel-based fan-out
- Webhooks, Bot API, Interceptors disabled

### Pro — Multi Node (Redis)
- Redis Pub/Sub for cross-node message delivery
- Redis for presence with TTL + heartbeat
- Redis for distributed rate limiting
- Redis for message chunk caching

Abstracted via `ClusterBackend` trait (M8).

### Redis Chunk Caching

Completed message chunks (64 messages) are cacheable in Redis because message IDs
are sequential and messages are never physically deleted — chunk boundaries are stable.

Cache key: `chat:{chat_id}:chunk:{chunk_id}`

Value: serialized list of messages + `max_updated_at`. TTL ~1h or LRU eviction.

**On any message mutation in a chunk:**
```
DEL chat:{chat_id}:chunk:{message_id >> 6}
```

See [protocol.md — Message Chunks](protocol.md#message-chunks) for chunk layout.

## Graceful Shutdown

On `SIGTERM` or `SIGINT`:
1. Stop accepting new connections
2. Signal shutdown via `watch::Sender<bool>`
3. Clear all sessions (dropping mpsc senders closes outbound tasks)
4. Wait 2s grace period for in-flight messages
5. Exit

## Configuration

See `config/config.example.toml` for full reference. Key sections:
- `[server]` — host, port, ws_send_buffer_size
- `[database]` — PostgreSQL connection URL, max connections
- `[auth]` — JWT secret
- `[limits]` — message/frame/extra size limits
- `[rate_limits]` — per-IP, per-chat, per-user limits
- `[cluster]` — Redis for Pro tier
- `[webhooks]` — webhook configuration
- `[storage]` — S3 media storage
- `[license]` — license key (empty = Free)
