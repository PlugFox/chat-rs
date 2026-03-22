# WebSocket Protocol

> See also: [codec.md](codec.md) for binary encoding details, [error-codes.md](error-codes.md) for error handling.

## Frame Format

All WS binary frames share a 5-byte header:

```
┌──────────┬───────────┬──────────────────┐
│ kind: u8 │  seq: u32 │ payload: bytes   │
└──────────┴───────────┴──────────────────┘
```

- `kind` — frame type (`FrameKind` enum)
- `seq` — sequence number; see [codec.md](codec.md) for the request/response model

## Frame Kinds

### Handshake & Keepalive (0x01..0x04)

| Kind    | Value | Direction       | Purpose                            |
| ------- | ----- | --------------- | ---------------------------------- |
| Hello   | 0x01  | client → server | Protocol version, token, device_id |
| Welcome | 0x02  | server → client | session_id, server_time, limits    |
| Ping    | 0x03  | both            | Keepalive                          |
| Pong    | 0x04  | both            | Keepalive response                 |

### Commands (0x10..0x1A, client → server)

| Kind          | Value | Persist | Needs Ack |
| ------------- | ----- | ------- | --------- |
| SendMessage   | 0x10  | yes     | yes       |
| EditMessage   | 0x11  | yes     | yes       |
| DeleteMessage | 0x12  | yes     | yes       |
| ReadReceipt   | 0x13  | yes     | no        |
| Typing        | 0x14  | no      | no        |
| GetPresence   | 0x15  | —       | rpc       |
| LoadChats     | 0x16  | —       | rpc       |
| Search        | 0x17  | —       | rpc       |
| Subscribe     | 0x18  | —       | rpc       |
| Unsubscribe   | 0x19  | —       | no        |
| LoadMessages  | 0x1A  | —       | rpc       |

### Events (0x20..0x28, server → client)

| Kind           | Value | Purpose                                            |
| -------------- | ----- | -------------------------------------------------- |
| MessageNew     | 0x20  | New message delivered in real-time                 |
| MessageEdited  | 0x21  | Message content changed                            |
| MessageDeleted | 0x22  | Message marked deleted (content cleared, row kept) |
| ReceiptUpdate  | 0x23  | Read receipt update                                |
| TypingUpdate   | 0x24  | Typing indicator                                   |
| MemberJoined   | 0x25  | Member joined chat                                 |
| MemberLeft     | 0x26  | Member left chat                                   |
| PresenceResult | 0x27  | Response to GetPresence                            |
| ChatUpdated    | 0x28  | Chat metadata changed (title, avatar)              |
| ChatCreated    | 0x29  | New chat the user is a member of                   |

### Responses (0x30..0x31)

| Kind  | Value | Purpose              |
| ----- | ----- | -------------------- |
| Ack   | 0x30  | Command acknowledged |
| Error | 0x31  | Error response       |

### Chat Management (0x40..0x47, client → server, RPC)

| Kind           | Value | Purpose                                          |
| -------------- | ----- | ------------------------------------------------ |
| CreateChat     | 0x40  | Create a new chat                                |
| UpdateChat     | 0x41  | Update chat info (title, avatar)                 |
| DeleteChat     | 0x42  | Delete chat                                      |
| GetChatInfo    | 0x43  | Get chat details                                 |
| GetChatMembers | 0x44  | List chat members                                |
| InviteMembers  | 0x45  | Invite users                                     |
| UpdateMember   | 0x46  | Kick, ban, mute, change role/permissions         |
| LeaveChat      | 0x47  | Leave chat                                       |

`UpdateMember` replaces the previous separate `KickMember`, `BanMember`, `MuteMember`,
and `UpdateMemberRole` frames. The action is determined by an `action: u8` discriminant:

```
chat_id: u32 | user_id: u32 | action: u8 | action-specific payload
```

| Action | Value | Payload             | Description                    |
| ------ | ----- | ------------------- | ------------------------------ |
| Kick   | 0     | (none)              | Remove member from chat        |
| Ban    | 1     | (none)              | Ban member from chat           |
| Mute   | 2     | `duration_secs: u32`| Mute (0 = unmute)              |
| ChangeRole | 3 | `role: u8`          | Change member's role           |
| UpdatePermissions | 4 | `permissions: u32` | Set permission override    |

Response: `Ack` (empty).

## Handshake

**Hello** (client → server): `protocol_version`, `sdk_version`, `platform`, `token` (JWT), `device_id` (UUID, 16 bytes).

**Welcome** (server → client): `session_id: u32`, `server_time: i64` (clock sync, Unix seconds), `user_id: u32`, `ServerLimits`, `ServerCapabilities`.

### ServerLimits

Sent in Welcome. Client uses these values for local enforcement (debouncing, UI limits).

| Field                | Type  | Default | Description                                       |
| -------------------- | ----- | ------- | ------------------------------------------------- |
| `ping_interval_ms`   | `u32` | 30 000  | How often the client should send Ping             |
| `ping_timeout_ms`    | `u32` | 10 000  | Pong timeout — disconnect if exceeded             |
| `max_message_size`   | `u32` | 65 536  | Max content size in bytes (64 KB)                 |
| `max_extra_size`     | `u32` | 4 096   | Max extra JSON size in bytes (4 KB)               |
| `max_frame_size`     | `u32` | 131 072 | Max total WS frame size in bytes (128 KB)         |
| `messages_per_sec`   | `u16` | 10      | Rate limit: messages per second per user per chat |
| `connections_per_ip` | `u16` | 20      | Rate limit: concurrent connections per IP         |

### ServerCapabilities

Bitflags `u32` sent in Welcome. Client uses these to show/hide features.

| Flag           | Bit  | Feature                          |
| -------------- | ---- | -------------------------------- |
| `MEDIA_UPLOAD` | 0x01 | File and image upload enabled    |
| `SEARCH`       | 0x02 | Full-text message search enabled |
| `REACTIONS`    | 0x04 | Emoji reactions enabled          |
| `THREADS`      | 0x08 | Message threads/replies enabled  |
| `BOTS`         | 0x10 | Bot API enabled                  |

## Key Frame Payloads

### Event Payloads (server → client)

**MessageNew (0x20)**: payload is a single `Message` (same wire format as in `MessageBatch`).

**MessageEdited (0x21)**: payload is a single `Message` with updated content/rich/extra and `EDITED` flag set.

**MessageDeleted (0x22)**: `chat_id: u32`, `message_id: u32`. Content is already cleared server-side.

### Subscribe (0x18)

`chat_id: u32`

Registers the session to receive real-time events for a chat (`MessageNew`, `MessageEdited`, `MessageDeleted`, `ReceiptUpdate`, `TypingUpdate`, `MemberJoined`, `MemberLeft`).

Response: `Ack` (empty payload). No historical messages are pushed — client loads history explicitly via `LoadMessages`.

### Unsubscribe (0x19)

`chat_id: u32` — fire-and-forget, no Ack.

### LoadMessages (0x1A)

Two modes selected by `mode: u8`.

**Mode 0 — anchor-based pagination** (history load):

```
chat_id: u32 | mode: u8=0 | direction: u8 | anchor_id: u32 | limit: u16
```

`direction`: 0 = older, 1 = newer. `anchor_id = 0` means start from the newest message.

Response: `Ack` with `MessageBatch` of up to `limit` messages.

**Mode 1 — range update check** (catch-up after reconnect):

```
chat_id: u32 | mode: u8=1 | from_id: u32 | to_id: u32 | since_ts: i64
```

`since_ts` = `MAX(updated_at)` of messages `[from_id..to_id]` from the client's local
cache. Server-generated value — no clock skew risk.

Response: `Ack` with `MessageBatch` containing only messages where
`updated_at > since_ts` within `[from_id, to_id]`. Empty batch = nothing changed.

**Client-side chunk tracking**

To avoid redundant range checks within a session, clients maintain:

```
Map<chat_id: u32, Set<chunk: u32>>
```

where `chunk = message_id / 100` (bucket of 100 messages). A chunk is added to the set
after a successful Mode 1 response. On reconnect the set is cleared.

### LoadChats (0x16)

`cursor_ts: i64` (0 = first page), `limit: u16`

`cursor_ts = 0` is a sentinel meaning "no cursor / first page". It is technically a valid
timestamp (1970-01-01) but no real chat will have `updated_at = 0`, so there is no ambiguity.

Response: `Ack` with payload: `next_cursor_ts: i64`, then `count: u32` + chat entries.

### GetPresence (0x15)

`user_ids: [u32]` (u16 count prefix)

Response: `PresenceResult (0x27)` frame with the same seq.

### Search (0x17)

`chat_id: u32`, `query: String`, `cursor: u32` (0 = first page), `limit: u16`

Response: `Ack` with payload: `next_cursor: u32`, then `count: u32` + `(message_id: u32, snippet_len: u32, snippet: UTF-8)` entries.

### UpdateChat (0x41)

`chat_id: u32`, `title: String`, `avatar_url: String`

**Clear semantics**: an empty string (`len = 0`) means "clear this field" (set to NULL).
To leave a field unchanged, omit the update (send the current value). This avoids the
need for a separate "clear" flag.

## Versioning

Protocol version is negotiated once during the handshake via `protocol_version` in the **Hello** payload. If the server does not support the requested version, it responds with an `unsupported_version` error and closes the connection. There is no per-frame version field.

## Deduplication

Each persistent command (SendMessage, EditMessage, DeleteMessage) contains an `idempotency_key: UUID` (16 bytes) generated by the client. The server stores keys for 24 hours and returns the original result for duplicates without side effects.
