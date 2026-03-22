# Binary Codec

All values are little-endian (native for ARM/x86).

## WS Frame Header

```
┌──────────┬───────────┬────────────────┬──────────────────┐
│ kind: u8 │  seq: u32 │ event_seq: u32 │ payload: bytes   │
└──────────┴───────────┴────────────────┴──────────────────┘
```

Total header: 9 bytes.

`event_seq` is a per-session monotonically increasing counter for server push events.
Client → server frames set `event_seq = 0`. See [protocol.md](protocol.md#event-ordering).

## Timestamp Validation

All `i64` timestamp fields carry Unix seconds. The codec **rejects** (returns `CodecError`)
any timestamp outside the valid range on both encode and decode:

```
MIN_TIMESTAMP = 0                    (1970-01-01 00:00:00 UTC)
MAX_TIMESTAMP = (1 << 41) - 1        (2_199_023_255_551 — year ~71,700)
```

Fast check: `value >> 41 != 0` → reject. Single shift + test — the cheapest possible
bounds check.

Why this range:
- Catches "milliseconds instead of seconds" bugs (2024 in ms ≈ 1.7 × 10¹², > 2⁴¹)
- Guaranteed safe in JavaScript `Number` (max safe = 2⁵³ − 1)
- Negative values rejected (no use case before 1970)
- On violation: return `CodecError::TimestampOutOfRange`, never clamp silently — silent
  clamp hides bugs

## Error Frame Payload

Payload of an `Error (0x31)` frame (header `seq` identifies which request failed):

```
┌──────────┬────────────┬──────────────────┬────────────────┬──────────────────────┐
│ code: u16│ slug_len:u8│ slug (UTF-8)     │ msg_len: u32   │ message (UTF-8)      │
└──────────┴────────────┴──────────────────┴────────────────┴──────────────────────┘
```

Followed by:
- `retry_after_ms: u32` (0 if not applicable — only set for `rate_limited`)
- `extra_len: u32` + extra JSON object (UTF-8) — server-provided diagnostic details; `extra_len = 0` means field is absent

## Request / Response Model

### seq

Each outgoing client frame carries a `seq: u32` in the header — a monotonically increasing counter maintained by the client.

- `seq = 0` — fire-and-forget: no response expected (used for `Ping`, `Typing`, `ReadReceipt`)
- `seq > 0` — the client expects either an `Ack` or an `Error` back with the same `seq`

The client keeps a pending-request map: `seq → pending`. When `Ack(seq)` or `Error(seq)` arrives, the matching entry is resolved and removed.

Server-to-client event frames (e.g. `MessageNew`, `SyncBatch`) always carry `seq = 0` — they are push notifications, not responses to a request.

### Ack (0x30)

Signals that the server successfully processed a command. The `seq` in the Ack header matches the `seq` of the original command.

Ack payload is command-specific and will be defined per command as they are added to the protocol.

### Error (0x31)

Signals that the server rejected or failed to process a command. The `seq` in the Error header matches the `seq` of the original command (or `0` for connection-level errors not tied to a specific request).

## Message Batch Format

Used as response to `LoadMessages (0x1A)` and as payload of `MessageNew (0x20)` events.

```
MessageBatch:
┌──────────────┬──────────────────────────────────────┐
│ count: u32   │ messages[count]                      │
└──────────────┴──────────────────────────────────────┘

Message (fixed header 35 bytes + variable):
┌─────────┬──────────┬───────────┬─────────┬──────────┬────────┬──────────┬─────────────┬──────────────────┐
│ id: u32 │ chat: u32│sender: u32│crtd_at:i64│upd_at:i64│kind: u8│flags: u16│ content_len │ content (UTF-8)  │
│  4 bytes│  4 bytes │  4 bytes  │  8 bytes│  8 bytes │  1 byte│  2 bytes │   u32 4bytes│  N bytes         │
└─────────┴──────────┴───────────┴─────────┴──────────┴────────┴──────────┴─────────────┴──────────────────┘
```

Followed by: `rich_len: u32` + rich blob, `extra_len: u32` + extra JSON bytes. If len = 0, no data and no allocation.

Timestamps (`created_at`, `updated_at`) are validated against the range defined above.

## Rich Content BLOB

```
┌───────────┬──────────────────────┐
│ count: u16│ spans[count]         │
└───────────┴──────────────────────┘

Span (10 bytes fixed + optional meta):
┌────────────┬──────────┬──────────┬──────────────────────────────┐
│ start: u32 │ end: u32 │ style:u16│ meta_len: u32 + JSON (UTF-8) │
└────────────┴──────────┴──────────┴──────────────────────────────┘
```

`start/end` are byte offsets into the plain text string.

`meta_len = 0` means no meta for this span (most spans: bold, italic, etc. — 14 bytes total).

When `meta_len > 0`, meta is a JSON object whose keys depend on the `style` bits set:

| Style bit    | Meta JSON key           | Example                         |
| ------------ | ----------------------- | ------------------------------- |
| `LINK`       | `"url": String`         | `{"url": "https://example.com"}`|
| `MENTION`    | `"user_id": u32`        | `{"user_id": 42}`               |
| `COLOR`      | `"rgba": u32`           | `{"rgba": 4278190335}`          |
| `CODE_BLOCK` | `"lang": String`        | `{"lang": "rust"}`              |

Multiple keys may be present in one meta object when multiple style bits with meta are
combined on a single span. Unknown keys must be tolerated (forward compatibility).

## String Encoding Convention

- `read_string()` / `write_string()` — length-prefixed UTF-8. Returns `""` when `len = 0`.
  Used for fields that are always present (e.g. `content` — empty string for deleted messages).
- `read_optional_string()` / `write_optional_string()` — same encoding, but returns `None`
  when `len = 0`. Used for fields that can be absent (e.g. `extra`, `title`, `avatar_url`).

Consequence: `Some("")` and `None` are indistinguishable on the wire — both encode as `len = 0`.
This is by design; empty optional strings have no semantic meaning.

## Type Mapping

| Rust                | Wire                 | Size          |
| ------------------- | -------------------- | ------------- |
| `i64`               | 8 bytes LE           | 8             | <!-- timestamps only (Unix seconds), validated: 0 ≤ v < 2⁴¹ --> |
| `u32`               | 4 bytes LE           | 4             | <!-- IDs, lengths, counts -->           |
| `i32`               | 4 bytes LE           | 4             | <!-- signed counters -->                |
| `u16`               | 2 bytes LE           | 2             |
| `u8`                | 1 byte               | 1             |
| `bool`              | 1 byte               | 1             |
| `String`            | u32 len + UTF-8      | 4 + N         |
| `Vec<u8>`           | u32 len + bytes      | 4 + N         |
| `Option<T>`         | u8 flag + T          | 1 + sizeof(T) |
| `Uuid`              | 16 bytes             | 16            |
| `serde_json::Value` | u32 len + JSON UTF-8 | 4 + N         |
