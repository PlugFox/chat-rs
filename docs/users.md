# Users

> See also: [protocol.md](protocol.md) for frame kinds, [codec.md](codec.md) for binary encoding, [database.md](database.md) for full schema.

## Overview

A **user** is the identity record for a human or bot account. All wire IDs are `u32`.
`external_id` maps an external auth identity to the internal `u32` — it is server-only
and never transmitted over the wire.

## Fields

| Field        | Wire type         | Rust type        | Description                                              |
| ------------ | ----------------- | ---------------- | -------------------------------------------------------- |
| `id`         | `u32`             | `u32`            | Internal sequential ID; server-assigned                  |
| `flags`      | `u16`             | `UserFlags`      | User type and capability flags                           |
| `username`   | `u32 len + UTF-8` | `Option<String>` | Lowercase latin slug (5–32 chars); absent when `len = 0` |
| `first_name` | `u32 len + UTF-8` | `Option<String>` | Display first name (1–64 chars); absent when `len = 0`   |
| `last_name`  | `u32 len + UTF-8` | `Option<String>` | Display last name (1–64 chars); absent when `len = 0`    |
| `avatar_url` | `u32 len + UTF-8` | `Option<String>` | Avatar URL; absent when `len = 0`                        |
| `created_at` | `i64`             | `i64`            | Account creation timestamp, Unix seconds                 |
| `updated_at` | `i64`             | `i64`            | Last profile modification timestamp, Unix seconds        |

`external_id` is **not transmitted** — it is a server-side mapping between the external
auth system and the internal `u32`. Clients always use `u32` IDs.

## UserFlags

```rust
bitflags! {
    /// User type and capability flags, transmitted as `u16` in the wire format.
    ///
    /// Stored as `SMALLINT` (i16) in PostgreSQL — no unsigned type available.
    /// Use `flags.bits() as i16` to write, `UserFlags::from_bits_truncate(raw as u16)` to read.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct UserFlags: u16 {
        /// System account (used for server-generated messages, e.g. join/leave notices).
        const SYSTEM  = 0x0001;
        /// Bot account; server sets MessageFlags::BOT on all messages from this user.
        const BOT     = 0x0002;
        /// Premium subscriber; clients may use this to show a badge.
        const PREMIUM = 0x0004;
        // 0x0008–0x8000: reserved for future extension
    }
}
```

`flags = 0` — regular user account (no special status).

`UserFlags::BOT` is the source of truth for bot status. The server sets
`MessageFlags::BOT` on every outgoing message from a bot user based on this flag;
the client never overrides it.

## Wire Format

All values are little-endian. See [codec.md](codec.md) for type mapping.

### UserEntry — 22-byte fixed header + variable

Used in `PresenceResult` responses and any future user-lookup RPCs.

```
 0    4    6       14      22
 ┌────┬────┬────────┬────────┐
 │ id │flgs│crtd_at │upd_at  │
 │ u32│ u16│  i64   │  i64   │
 └────┴────┴────────┴────────┘
```

Followed by four length-prefixed optional strings (each absent when `len = 0`):

```
┌──────────────────┬──────────────────┬──────────────────┬──────────────────┐
│ username_len: u32│ first_name_len:u32│ last_name_len: u32│ avatar_len: u32  │
│ + username UTF-8 │ + first_name UTF-8│ + last_name UTF-8 │ + avatar UTF-8   │
└──────────────────┴──────────────────┴──────────────────┴──────────────────┘
```

Minimum size: 38 bytes (fixed header + 4 × zero-length prefix, no string data).

### PresenceEntry — 13 bytes fixed

Used within `PresenceResult (0x27)`.

```
┌────────────┬──────────┬─────────────┐
│ user_id:u32│ status:u8│last_seen: i64│
│   4 bytes  │  1 byte  │   8 bytes   │
└────────────┴──────────┴─────────────┘
```

`status`:
- `0` — offline
- `1` — online

`last_seen`: Unix seconds of last observed activity; `0` when the user is currently online.

### GetPresence (0x15) Request

`count: u16`, `user_ids[count]: u32`

### PresenceResult (0x27) Response

Sent by the server in response to `GetPresence`; carries the same `seq` as the request.

```
┌──────────┬─────────────────────┐
│count: u32│ entries[count]      │
└──────────┴─────────────────────┘
```

Each entry is a `PresenceEntry` (13 bytes).

## Database

### PostgreSQL

#### users

Identity record — deliberately minimal. Profile data lives in `user_info`.

```sql
CREATE TABLE users (
    id          SERIAL PRIMARY KEY,            -- u32 in Rust; max ~2.1B
    external_id TEXT        NOT NULL UNIQUE,   -- opaque external auth identity
    flags       SMALLINT    NOT NULL DEFAULT 0, -- UserFlags (u16 stored as i16)
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

`flags` cast pattern (no unsigned types in PostgreSQL):

```rust
// write
let pg_flags = user.flags.bits() as i16;
// read
let flags = UserFlags::from_bits_truncate(row.flags as u16);
```

#### user_info

Mutable profile data stored separately to keep the hot `users` row narrow.

```sql
CREATE TABLE user_info (
    user_id     INTEGER     NOT NULL PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    username    VARCHAR(32),                   -- NULL if not set; unique when present
    first_name  VARCHAR(64),
    last_name   VARCHAR(64),
    avatar_url  TEXT,
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT username_format CHECK (username ~ '^[a-z0-9_]{5,32}$')
);

CREATE UNIQUE INDEX idx_user_info_username ON user_info(username) WHERE username IS NOT NULL;
```

#### sessions

Persistent device sessions. Distinct from the transient **WS session** (`session_id: u32`
in the Welcome frame), which is an in-memory per-connection counter and is not stored here.

```sql
CREATE TABLE sessions (
    id             UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id        INTEGER     NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Device / client info (from Hello frame)
    device_id      VARCHAR(255),              -- client-generated UUID
    device_name    VARCHAR(255),              -- e.g. "iPhone 15 Pro"
    device_type    VARCHAR(30),               -- 'mobile', 'tablet', 'desktop', 'unknown'
    user_agent     TEXT,
    client_version VARCHAR(50),

    -- Network info
    created_ip     INET,                      -- IP at session creation
    ip_address     INET,                      -- last seen IP
    ip_country     VARCHAR(2),                -- ISO 3166-1 alpha-2

    -- Lifecycle
    revoked_at     TIMESTAMPTZ,
    revoked_reason VARCHAR(100),              -- 'logout', 'security', 'expired', 'admin'

    -- Activity
    last_seen      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    activity_count INTEGER     NOT NULL DEFAULT 0,

    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sessions_user_active ON sessions(user_id) WHERE revoked_at IS NULL;
CREATE INDEX idx_sessions_revoked     ON sessions(revoked_at) WHERE revoked_at IS NOT NULL;
CREATE INDEX idx_sessions_last_seen   ON sessions(last_seen) WHERE revoked_at IS NULL;
```

`session_id` in the Welcome frame is a transient u32 assigned per WS connection
(e.g. an atomic counter on the server). It is not stored in this table and has no
relation to the UUID primary key.

### Presence Architecture

Online/offline state and `last_seen` are tracked via two separate mechanisms:

#### Online status — in-memory RoaringBitmap

The server keeps a single `RoaringBitmap` (via the [`roaring`](https://crates.io/crates/roaring) crate) of currently-online user IDs in memory:

```rust
// Conceptually:
struct PresenceTracker {
    online: RwLock<RoaringBitmap>,
}
```

- **Connect**: `online.insert(user_id)`
- **Disconnect**: `online.remove(user_id)` (only when the last session for that user closes)
- **Query**: `online.contains(user_id)` — O(1), no DB round-trip

This is ephemeral: a server restart clears the bitmap. Users reconnect automatically and presence is restored on the next WS handshake.

**Multi-server (future):** swap the in-memory bitmap for a Redis set (`SADD`/`SREM`/`SISMEMBER`). The `PresenceTracker` is a trait object so the implementation can be swapped without changing call sites.

#### last_seen — sessions table

`last_seen` is sourced from the existing `sessions` table:

```sql
SELECT MAX(last_seen) FROM sessions
WHERE user_id = $1 AND revoked_at IS NULL;
```

`sessions.last_seen` is updated on **disconnect** (not on every heartbeat), which avoids write amplification. `users.updated_at` is reserved exclusively for profile changes (name, avatar, username) so clients can use it for cache invalidation without false positives.

## Constraints

| Field        | Rule                                                                                   |
| ------------ | -------------------------------------------------------------------------------------- |
| `username`   | 5–32 chars; lowercase ASCII letters, digits, underscores only; must be globally unique |
| `first_name` | 1–64 chars; any Unicode except control characters                                      |
| `last_name`  | 1–64 chars; any Unicode except control characters                                      |
| `avatar_url` | No server-enforced length limit; clients should cap display at a reasonable length     |

## Notes

**`external_id` is never on the wire** — it exists solely to map an external auth token
(OAuth sub, LDAP uid, etc.) to the internal `u32`. Once mapped, all protocol traffic uses
the internal ID exclusively.

**Bot flag is server-authoritative** — `UserFlags::BOT` may only be set by the server
(via an admin API or provisioning script). The server ignores any client attempt to set
`MessageFlags::BOT` directly on `SendMessage`.

**`username` uniqueness is partial-index enforced** — the `WHERE username IS NOT NULL`
predicate means `NULL` (no username) is not subject to the unique constraint, allowing
many users to have no username simultaneously.

**WS session ID vs. device session** — `session_id: u32` in the Welcome frame is a
lightweight per-connection identifier (used only for logging and diagnostics). The
durable device identity is the UUID in the `sessions` table, keyed on `device_id`.

**Presence is eventually consistent** — `status` in `PresenceResult` reflects the
in-memory bitmap which is authoritative for the current server process. `last_seen`
is the timestamp of the last disconnect, sourced from `sessions.last_seen`; it is not
updated on heartbeats. Clients should treat both values as approximate.

**Deleted users** — when a user is deleted, `ON DELETE CASCADE` removes their
`user_info` and `sessions` rows. Messages are not deleted (they are orphaned with the
original `sender_id` intact); clients should fall back to a placeholder display name
when no `UserEntry` can be resolved.
