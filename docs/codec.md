# Binary Codec

All values are little-endian (native for ARM/x86).

## WS Frame Header

```
┌──────────┬───────────┬──────────────────┐
│ kind: u8 │  seq: u32 │ payload: bytes   │
└──────────┴───────────┴──────────────────┘
```

Total header: 5 bytes.

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

Used in `SyncBatch (0x27)` and as response to `LoadMessages (0x1A)` and `Subscribe (0x18)`.

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

## Rich Content BLOB

```
┌───────────┬──────────────────────┐
│ count: u16│ spans[count]         │
└───────────┴──────────────────────┘

Span (10 bytes fixed + optional meta):
┌────────────┬──────────┬──────────┬──────────────────────┐
│ start: u32 │ end: u32 │ style:u16│ meta (if present)    │
└────────────┴──────────┴──────────┴──────────────────────┘
```

`start/end` are byte offsets into the plain text string.

`meta` is present when `style` has `LINK`, `MENTION`, or `CODE_BLOCK` bits set:
- `LINK`: `url_len: u32` + UTF-8 URL
- `MENTION`: `user_id: u32`
- `CODE_BLOCK`: `lang_len: u8` + UTF-8 language tag (e.g. `rust`)

## Server-Side Batching

The server accumulates up to 20 events or 16 ms (whichever comes first) into a single `SyncBatch` frame. This reduces per-frame overhead during burst delivery. Clients must be prepared to receive multiple messages in a single WS frame.

## Type Mapping

| Rust                | Wire                 | Size          |
| ------------------- | -------------------- | ------------- |
| `i64`               | 8 bytes LE           | 8             | <!-- timestamps only (Unix seconds) --> |
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
