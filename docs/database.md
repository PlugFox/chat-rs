# Database Design

> **Note:** The client-side SQLite schema lives in the separate `chat_client_rs` repository. The schema documented here is a reference for understanding the data model. The authoritative source is `chat_client_rs/migrations/`.

## Client-Side SQLite

### Architecture: WAL + Read/Write Separation

```sql
PRAGMA journal_mode = WAL;
```

- **Write connection**: single, via `tokio-rusqlite` (dedicated thread). All INSERT/UPDATE/DELETE.
- **Read pool**: `r2d2` + `r2d2_sqlite`, 2–4 connections. LoadWindow, Search, LoadChunk — don't block write or each other.

```rust
struct Database {
    writer: tokio_rusqlite::Connection,
    reader: r2d2::Pool<SqliteConnectionManager>,
}
```

### Schema

#### Messages

Rows are **never physically deleted**. Deletion is an UPDATE that clears `content` /
`rich_content` and sets `flags |= 0x0002` (DELETED). This preserves reply chains and
keeps IDs contiguous within each chat.

```sql
CREATE TABLE messages (
    id                 INTEGER NOT NULL,  -- per-chat sequential, assigned by server
    chat_id            INTEGER NOT NULL,
    sender_id          INTEGER NOT NULL,
    created_at         INTEGER NOT NULL,
    kind               INTEGER NOT NULL,
    flags              INTEGER NOT NULL DEFAULT 0,
    content            TEXT    NOT NULL,
    rich_content       BLOB,
    extra              TEXT,
    reply_to_id        INTEGER,           -- planned (threads milestone)
    reply_to_sender_id INTEGER,           -- denormalized preview (M5)
    reply_to_content   TEXT,              -- first N chars of parent (M5)
    updated_at         INTEGER NOT NULL,
    PRIMARY KEY (chat_id, id)
);

CREATE INDEX idx_messages_updated ON messages(chat_id, updated_at);
```

#### Outbox (offline send queue)

```sql
CREATE TABLE outbox (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    idempotency_key BLOB NOT NULL,     -- 16 bytes UUID
    chat_id         INTEGER NOT NULL,
    kind            INTEGER NOT NULL,
    payload         BLOB NOT NULL,
    created_at      INTEGER NOT NULL,
    status          INTEGER NOT NULL DEFAULT 0,  -- 0=pending, 1=sending, 2=failed_permanent
    attempts        INTEGER NOT NULL DEFAULT 0,
    last_attempt    INTEGER,
    error           TEXT
);
```

#### Read Receipts

```sql
CREATE TABLE read_receipts (
    chat_id    INTEGER NOT NULL,
    user_id    INTEGER NOT NULL,
    message_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    PRIMARY KEY (chat_id, user_id)
);
```

#### Reactions

```sql
CREATE TABLE reactions (
    chat_id    INTEGER NOT NULL,
    message_id INTEGER NOT NULL,
    user_id    INTEGER NOT NULL,
    emoji      TEXT    NOT NULL,
    created_at INTEGER NOT NULL,
    PRIMARY KEY (chat_id, message_id, user_id, emoji)
);
```

#### Chat Sync State

```sql
CREATE TABLE chat_sync (
    chat_id        INTEGER PRIMARY KEY,
    last_update_ts INTEGER NOT NULL DEFAULT 0
);
```

#### Chat List Cache

`last_message_id` doubles as the sync cursor — used in `WHERE id > last_message_id`
to fetch unseen messages after reconnect.

```sql
CREATE TABLE chats (
    id                     INTEGER PRIMARY KEY,
    kind                   INTEGER NOT NULL,
    title                  TEXT,
    avatar_url             TEXT,
    last_message_id        INTEGER,           -- highest known msg id; sync cursor
    last_message_content   TEXT,
    last_message_sender_id INTEGER,
    last_message_at        INTEGER,
    unread_count           INTEGER NOT NULL DEFAULT 0,
    updated_at             INTEGER NOT NULL
);
```

### Migrations

Via `rusqlite_migration`, version tracked in `PRAGMA user_version`.

```rust
const MIGRATIONS: &[Migration] = &[
    Migration::new(1, include_str!("migrations/001_initial.sql")),
    // ...
];
```

### No Client-Side FTS

Search is server-only via PostgreSQL `tsvector/tsquery`. Client FTS5 was removed — increases DB size, client doesn't have full history.

## Server-Side PostgreSQL

### User ID Strategy

Internal `u32` (stored as PostgreSQL `SERIAL` / `INTEGER`) for wire compactness + external `TEXT` for developer integration:

```sql
CREATE TABLE users (
    id          SERIAL PRIMARY KEY,            -- u32 in Rust; max ~2.1B (sufficient for target scale)
    external_id TEXT        NOT NULL UNIQUE,   -- opaque external auth identity
    flags       SMALLINT    NOT NULL DEFAULT 0, -- UserFlags (u16 stored as i16)
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

`is_bot` is no longer a separate column — use `flags & 0x0002` (`UserFlags::BOT`).

See [users.md](users.md) for the full schema including `user_info` and `sessions` tables.

### Full Schema

See `crates/chat_server/migrations/` for the authoritative schema. Key tables:
- `users` — minimal user record with external_id mapping and UserFlags
- `user_info` — mutable profile data (username, first/last name, avatar)
- `sessions` — persistent device sessions; `last_seen` updated on disconnect (source of truth for "last seen"); see [users.md](users.md)
- `chats` — chat rooms (dm, group, channel); carries `last_msg_id` counter; channels have `parent_id`
- `chat_members` — membership with roles and permission overrides
- `dm_index` — unique-pair lookup for direct messages
- `messages` — append-only log; `PRIMARY KEY (chat_id, id)`; rows never deleted
- `read_receipts` — per-user read position per chat
- `reactions` — emoji reactions; keyed on `(chat_id, message_id, user_id, emoji)`
- `idempotency_keys` — deduplication (TTL 24h)

### Chats

`kind` values match `ChatKind` in `chat_protocol`:
- `0` — Direct (DM between two users)
- `1` — Group
- `2` — Channel (must have `parent_id` pointing to a Group)

```sql
CREATE TABLE chats (
    id          SERIAL PRIMARY KEY,            -- u32 in Rust
    parent_id   INTEGER REFERENCES chats(id),
    kind        SMALLINT    NOT NULL,
    title       TEXT,                          -- NULL for dm
    avatar_url  TEXT,
    last_msg_id INTEGER     NOT NULL DEFAULT 0,  -- u32 in Rust; per-chat message counter
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT channel_requires_parent CHECK (kind != 2 OR parent_id IS NOT NULL),
    CONSTRAINT dm_no_parent            CHECK (kind != 0 OR parent_id IS NULL)
);

CREATE TABLE chat_members (
    chat_id     INTEGER     NOT NULL REFERENCES chats(id),
    user_id     INTEGER     NOT NULL,
    role        SMALLINT    NOT NULL DEFAULT 0,  -- ChatRole: 0=Member,1=Moderator,2=Admin,3=Owner
    permissions INTEGER,                         -- NULL = use role defaults; Permission (u32 as i32)
    joined_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (chat_id, user_id)
);

-- Guarantees at most one DM between any pair of users.
-- App always inserts with min(user_a, user_b) as user_a.
CREATE TABLE dm_index (
    chat_id INTEGER NOT NULL PRIMARY KEY REFERENCES chats(id),
    user_a  INTEGER NOT NULL,
    user_b  INTEGER NOT NULL,
    UNIQUE  (user_a, user_b),
    CHECK   (user_a < user_b)
);
```

### Message Immutability

`DELETE` is never issued against the `messages` table. All "deletion" is an `UPDATE`:

```sql
UPDATE messages
SET content = '', rich = NULL, flags = flags | 2  -- 2 = DELETED flag
WHERE chat_id = $1 AND id = $2;
```

### Per-Chat Sequential IDs

The `chats` table carries a `last_msg_id INTEGER NOT NULL DEFAULT 0` counter.
Each insert atomically increments it and uses the result as the new message ID:

```sql
WITH next AS (
    UPDATE chats SET last_msg_id = last_msg_id + 1
    WHERE id = $chat_id
    RETURNING last_msg_id
)
INSERT INTO messages (chat_id, id, ...)
SELECT $chat_id, last_msg_id, ...
FROM next
RETURNING id;
```

Concurrent inserts into the same chat are serialized by the row lock on `chats`;
inserts into different chats are fully independent.

### Compile-Time Checked Queries

Via `sqlx::query!` macro. SQL errors are caught at compile time.

```bash
sqlx migrate run --database-url postgres://...
```
