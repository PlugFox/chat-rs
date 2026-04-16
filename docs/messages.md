# Messages

> See also: [protocol.md](protocol.md) for frame kinds, [codec.md](codec.md) for binary encoding conventions, [database.md](database.md) for full schema.

## Overview

`Message` is the core data unit. Messages travel in batches (`MessageBatch`) — as payloads of `SyncBatch` events (real-time push) and `LoadMessages` responses (history load).

Messages are also addressed in fixed-size **chunks** of 64 (`CHUNK_SIZE`). See [protocol.md — Message Chunks](protocol.md#message-chunks) for the chunking scheme and how `LoadMessages` mode 2 uses it.

## Fields

| Field         | Wire type         | Rust type               | Description                                               |
| ------------- | ----------------- | ----------------------- | --------------------------------------------------------- |
| `id`          | `u32`             | `u32`                   | Sequential per-chat ID; scoped to `chat_id`, starts at 1  |
| `chat_id`     | `u32`             | `u32`                   | Chat this message belongs to                              |
| `sender_id`   | `u32`             | `u32`                   | Internal user ID of the sender                            |
| `created_at`  | `i64`             | `i64`                   | Creation timestamp, Unix seconds (validated: 0 ≤ v < 2⁴¹) |
| `updated_at`  | `i64`             | `i64`                   | Last modification timestamp, Unix seconds (validated)     |
| `kind`        | `u8`              | `MessageKind`           | Content type                                              |
| `flags`       | `u16`             | `MessageFlags`          | Bitfield of message properties                            |
| `reply_to_id` | `u8 flag + u32`   | `Option<u32>`           | Message being replied to; absent = not a reply            |
| `content`     | `u32 len + UTF-8` | `String`                | Plain text; empty string for deleted tombstones           |
| `rich`        | `u32 len + blob`  | `Option<Vec<RichSpan>>` | Formatted text spans; absent when `len = 0`               |
| `extra`       | `u32 len + JSON`  | `Option<String>`        | Optional metadata JSON; absent when `len = 0`             |

## MessageKind

```rust
#[repr(u8)]
pub enum MessageKind {
    Text   = 0,
    Image  = 1,
    File   = 2,
    System = 3,
}
```

`System` messages carry a human-readable event string in `content` (e.g. `"Alice joined the group"`) and always have `MessageFlags::SYSTEM` set.

## MessageFlags

```rust
bitflags! {
    pub struct MessageFlags: u16 {
        const EDITED    = 0x0001; // edited at least once; display "edited" label
        const DELETED   = 0x0002; // soft-deleted tombstone; content is empty
        const FORWARDED = 0x0004; // forwarded from another chat; origin in extra JSON
        const PINNED    = 0x0008; // pinned in this chat
        const SILENT    = 0x0010; // no push notification for this message
        const SYSTEM    = 0x0020; // system event message (member join/leave, etc.)
        const BOT       = 0x0040; // sent by a bot user
        const REPLY     = 0x0080; // reply to another message; origin in extra JSON
        // 0x0100–0x8000: reserved
    }
}
```

### Flag semantics

**DELETED** — marks the message as deleted. The server issues an `UPDATE` that clears
`content` to an empty string and `rich` to absent, then sets this flag. The row is
never physically removed. Clients display "This message was deleted." Reply chains and
ID sequences remain intact.

**FORWARDED** — origin metadata lives in the `extra` JSON field:
```json
{ "fwd": { "chat_id": 123, "msg_id": 456, "sender_id": 789 } }
```

**BOT** — set by the server based on `users.is_bot`. The client never sends this flag;
the server ignores it if present in a `SendMessage` command.

**REPLY** — this message is a reply to another message. The `reply_to_id` field
contains the ID of the original message. Quoted origin metadata lives in the `extra`
JSON field (only parse when this flag is set):
```json
{ "reply": { "chat_id": 123, "msg_id": 456, "sender_id": 789, "quote": "first 100 chars…" } }
```
`quote` is a plain-text snippet of the original message content (up to 100 bytes, UTF-8
truncated at a character boundary). It is stored denormalized so clients can render the
reply preview without fetching the original message. When the original message is
deleted, `quote` is cleared to an empty string on the server.

**SYSTEM** — always paired with `MessageKind::System`. Use `kind` to gate rendering
logic; use the flag for fast filtering without inspecting `kind`.

## Wire Format

All values are little-endian.

### MessageBatch

```
┌───────────────┬─────────────┬──────────────────────────────────────┐
│ has_more: u8  │ count: u32  │ messages[count]                       │
└───────────────┴─────────────┴──────────────────────────────────────┘
```

`has_more`: 1 = more messages exist beyond this batch, 0 = last page.

### Message — 36-byte min fixed header + variable

```
 0    4    8   12      20      28  29     31  32                   36
 ┌────┬────┬───┬───────┬───────┬──┬──────┬────────────────────────┬───────────┬──────────────────┐
 │ id │chat│snd│crtd_at│upd_at │ki│flags │reply_to: u8 [+ u32]   │content_len│ content (UTF-8)  │
 │ u32│ u32│u32│  i64  │  i64  │u8│ u16  │ 1 byte  [+ 4 bytes]   │    u32    │    N bytes       │
 └────┴────┴───┴───────┴───────┴──┴──────┴────────────────────────┴───────────┴──────────────────┘
```

`reply_to` byte: `0` = not a reply (1 byte), `1` = reply_to_id follows as `u32` (5 bytes).

Followed by variable-length tail (each section absent when its `len = 0`):

```
┌──────────────┬─────────────────┬──────────────┬──────────────────┐
│ rich_len: u32│ rich blob       │extra_len: u32│ extra JSON UTF-8 │
│   4 bytes    │  M bytes        │   4 bytes    │  P bytes         │
└──────────────┴─────────────────┴──────────────┴──────────────────┘
```

`len = 0` means the field is absent — no bytes follow and no allocation is made.

Timestamps are validated against codec range (see [codec.md](codec.md#timestamp-validation)).

### Rich Content Blob

```
┌───────────┬───────────────────┐
│ count: u16│ spans[count]      │
└───────────┴───────────────────┘

Span — 10 bytes fixed + optional meta:
┌────────────┬──────────┬───────────┬──────────────────────────────┐
│ start: u32 │ end: u32 │ style: u16│ meta_len: u32 + JSON (UTF-8) │
└────────────┴──────────┴───────────┴──────────────────────────────┘
```

`start`/`end` are byte offsets into the plain-text `content` string.

`meta_len = 0` — no meta for this span (14 bytes total: 10 fixed + 4 for meta_len).

When `meta_len > 0`, meta is a JSON object. Keys depend on the `style` bits set:

| Style bit    | Meta JSON key    | Example                          |
| ------------ | ---------------- | -------------------------------- |
| `LINK`       | `"url": String`  | `{"url": "https://example.com"}` |
| `MENTION`    | `"user_id": u32` | `{"user_id": 42}`                |
| `COLOR`      | `"rgba": u32`    | `{"rgba": 4278190335}`           |
| `CODE_BLOCK` | `"lang": String` | `{"lang": "rust"}`               |

Multiple keys may be present when multiple style bits with meta are combined.
Unknown keys must be tolerated (forward compatibility).

### RichStyle flags

```rust
bitflags! {
    pub struct RichStyle: u16 {
        // Inline styles (combinable)
        const BOLD       = 0x0001;
        const ITALIC     = 0x0002;
        const UNDERLINE  = 0x0004;
        const STRIKE     = 0x0008;
        const SPOILER    = 0x0010;
        const CODE       = 0x0020; // inline monospace `code`

        // Styles with meta (combinable with inline)
        const LINK       = 0x0040; // meta: {"url": "..."}
        const MENTION    = 0x0080; // meta: {"user_id": u32}
        const COLOR      = 0x0100; // meta: {"rgba": u32}

        // Block-level (exclusive — overrides inline styles on this span)
        const CODE_BLOCK = 0x0200; // meta: {"lang": "rust"}, see below
        const BLOCKQUOTE = 0x0400; // > quoted text

        // 0x0800–0x8000: reserved
    }
}
```

Multiple style bits may be combined on a single span (e.g. bold + italic + link).

**Block-level semantics:** When `CODE_BLOCK` is set, the client ignores inline style
bits (BOLD, ITALIC, etc.) on this span — code blocks render as-is. `BLOCKQUOTE` may
contain nested inline-styled spans at different offsets within the quoted range.

## Extra JSON

The `extra` field carries optional structured metadata. Clients must tolerate unknown
keys (forward compatibility). Known conventions:

| Key     | Present when    | Schema                                                                         |
| ------- | --------------- | ------------------------------------------------------------------------------ |
| `fwd`   | `FORWARDED` set | `{ "chat_id": u32, "msg_id": u32, "sender_id": u32 }`                          |
| `reply` | `REPLY` set     | `{ "chat_id": u32, "msg_id": u32, "sender_id": u32, "quote": "<= 100 bytes" }` |

## Insertion (server-side)

Message IDs are generated atomically via a CTE that increments the chat's counter and
inserts the message in a single statement. Concurrent inserts into the same chat are
serialized by the row-level lock on `chats`; different chats are fully independent.

```rust
async fn insert_message(db: &PgPool, chat_id: u32, content: &str) -> Result<u32> {
    let row = sqlx::query!(
        r#"
        WITH next AS (
            UPDATE chats
            SET last_msg_id = last_msg_id + 1
            WHERE id = $1
            RETURNING last_msg_id
        )
        INSERT INTO messages (chat_id, id, content)
        SELECT $1, last_msg_id, $2
        FROM next
        RETURNING id
        "#,
        chat_id,
        content,
    )
    .fetch_one(db)
    .await?;

    Ok(row.id)
}
```

If `chat_id` does not exist the UPDATE returns no rows, the INSERT is skipped, and
`fetch_one` returns an error — no ID is wasted.

## Database

### PostgreSQL (server)

`messages` rows are **never physically deleted**. `DELETE` is not issued against this
table. Deletion is an `UPDATE` that clears `content`/`rich` and sets `flags |= DELETED`.

Key columns in the `messages` table:

```sql
id         INTEGER      NOT NULL,            -- per-chat sequential, assigned by CTE; u32 in Rust
chat_id    INTEGER      NOT NULL,
sender_id  INTEGER      NOT NULL,
created_at BIGINT       NOT NULL,            -- Unix seconds; i64 required
kind       SMALLINT     NOT NULL,
flags      SMALLINT     NOT NULL DEFAULT 0,  -- MessageFlags (u16 stored as i16, no unsigned in PG)
content    TEXT         NOT NULL,
rich       BYTEA,
extra      TEXT,                             -- JSON, NULL when absent
updated_at TIMESTAMPTZ  NOT NULL,
PRIMARY KEY (chat_id, id)
```

### SQLite (client — `chat_client_rs` repo)

The client never generates message IDs — all IDs arrive from the server.

```sql
id           INTEGER NOT NULL,
chat_id      INTEGER NOT NULL,
sender_id    INTEGER NOT NULL,
created_at   INTEGER NOT NULL,
kind         INTEGER NOT NULL,
flags        INTEGER NOT NULL DEFAULT 0,
content      TEXT    NOT NULL,
rich_content BLOB,
extra        TEXT,
updated_at   INTEGER NOT NULL,
PRIMARY KEY (chat_id, id)
```

### Flags and permissions storage

PostgreSQL has no unsigned integer types. Cast pattern:

```rust
// MessageFlags: u16 ↔ SMALLINT (i16)
let pg_flags = msg.flags.bits() as i16;
let flags = MessageFlags::from_bits_truncate(row.flags as u16);

// Permission: u32 ↔ INTEGER (i32)
let pg_perms = perms.bits() as i32;
let perms = Permission::from_bits_truncate(row.permissions as u32);
```

## MessageStatus (client-only)

`MessageStatus` tracks the **outbox state** of a message the local user is currently
sending. It is never included in the wire format.

```rust
#[repr(u8)]
pub enum MessageStatus {
    Sending          = 0, // queued in outbox, not yet acked by server
    Delivered        = 1, // server returned Ack
    FailedPermanent  = 2, // permanently rejected (no retry)
}
```

Deletion state is tracked via `MessageFlags::DELETED`.
Read/delivery state is tracked via the receipt system (`ReadReceipt` / `ReceiptUpdate`).
