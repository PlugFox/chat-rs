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

### Commands (0x10..0x1E, client → server)

| Kind           | Value | Persist | Needs Ack |
| -------------- | ----- | ------- | --------- |
| SendMessage    | 0x10  | yes     | yes       |
| EditMessage    | 0x11  | yes     | yes       |
| DeleteMessage  | 0x12  | yes     | yes       |
| ReadReceipt    | 0x13  | yes     | no        |
| Typing         | 0x14  | no      | no        |
| GetPresence    | 0x15  | —       | rpc       |
| LoadChats      | 0x16  | —       | rpc       |
| Search         | 0x17  | —       | rpc       |
| Subscribe      | 0x18  | —       | rpc       |
| Unsubscribe    | 0x19  | —       | no        |
| LoadMessages   | 0x1A  | —       | rpc       |
| AddReaction    | 0x1B  | yes     | yes       |
| RemoveReaction | 0x1C  | yes     | yes       |
| PinMessage     | 0x1D  | yes     | yes       |
| UnpinMessage   | 0x1E  | yes     | yes       |

### Events (0x20..0x2A, server → client)

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
| ReactionUpdate | 0x2A  | Reaction added or removed on a message             |

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
| GetChatMembers | 0x44  | List chat members (paginated)                    |
| InviteMembers  | 0x45  | Invite users                                     |
| UpdateMember   | 0x46  | Kick, ban, unban, mute, change role/permissions  |
| LeaveChat      | 0x47  | Leave chat                                       |

### UpdateMember (0x46)

Unified frame for member management. The action is determined by an `action: u8` discriminant:

```
chat_id: u32 | user_id: u32 | action: u8 | action-specific payload
```

| Action            | Value | Payload              | Description                    |
| ----------------- | ----- | -------------------- | ------------------------------ |
| Kick              | 0     | (none)               | Remove member from chat        |
| Ban               | 1     | (none)               | Ban member from chat           |
| Mute              | 2     | `duration_secs: u32` | Mute (0 = unmute)              |
| ChangeRole        | 3     | `role: u8`           | Change member's role           |
| UpdatePermissions | 4     | `permissions: u32`   | Set permission override        |
| Unban             | 5     | (none)               | Unban a previously banned user |

Response: `Ack` (empty).

### GetChatMembers (0x44)

```
chat_id: u32 | cursor: u32 | limit: u16
```

`cursor = 0` means first page. Server returns members ordered by `user_id`.

Response: `Ack` with payload: `next_cursor: u32`, then `count: u16` + `ChatMemberEntry` list.

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

### SendMessage (0x10)

```
chat_id: u32 | kind: u8 | idempotency_key: UUID | content_len: u32 | content (UTF-8) | rich_len: u32 | rich blob | extra_len: u32 | extra JSON
```

`kind` is `MessageKind` (Text=0, Image=1, File=2, System=3). Defaults to `Text` if the
client omits it (server-side default).

### Typing (0x14)

```
chat_id: u32 | expires_in_ms: u16
```

`expires_in_ms` — how long this typing indicator is valid. Server forwards this value
to other clients in `TypingUpdate`. Clients auto-expire the indicator after this duration.

### Event Payloads (server → client)

**MessageNew (0x20)**: payload is a single `Message` (same wire format as in `MessageBatch`).

**MessageEdited (0x21)**: payload is a single `Message` with updated content/rich/extra and `EDITED` flag set.

**MessageDeleted (0x22)**: `chat_id: u32`, `message_id: u32`. Content is already cleared server-side.

**TypingUpdate (0x24)**: `chat_id: u32`, `user_id: u32`, `expires_in_ms: u16`.

**MemberJoined (0x25)**: `chat_id: u32`, `user_id: u32`, `role: u8`, `invited_by: u32`.
`invited_by = 0` means self-join (e.g. via invite link).

**MemberLeft (0x26)**: `chat_id: u32`, `user_id: u32`.

**ReactionUpdate (0x2A)**: `chat_id: u32`, `message_id: u32`, `user_id: u32`, `pack_id: u32`, `emoji_index: u8`, `added: u8` (1 = added, 0 = removed).

### Reactions (0x1B..0x1C)

**AddReaction (0x1B)**: `chat_id: u32`, `message_id: u32`, `pack_id: u32`, `emoji_index: u8`.
Response: `Ack` (empty).

**RemoveReaction (0x1C)**: same wire format as AddReaction.
Response: `Ack` (empty).

Reactions use a pack-based emoji system. `pack_id` identifies the emoji pack (0 = built-in
Unicode set). Each pack contains up to 256 emoji (`emoji_index: u8`). The server broadcasts
`ReactionUpdate` events to subscribed clients.

### Pin/Unpin (0x1D..0x1E)

**PinMessage (0x1D)**: `chat_id: u32`, `message_id: u32`.
Response: `Ack` (empty). Server sets `MessageFlags::PINNED` and broadcasts `MessageEdited`.

**UnpinMessage (0x1E)**: same wire format.
Response: `Ack` (empty). Server clears `MessageFlags::PINNED` and broadcasts `MessageEdited`.

### Subscribe (0x18)

`count: u16`, `chat_ids: [u32]`

Batch-subscribes the session to receive real-time events for one or more chats (`MessageNew`, `MessageEdited`, `MessageDeleted`, `ReceiptUpdate`, `TypingUpdate`, `MemberJoined`, `MemberLeft`, `ReactionUpdate`).

Response: `Ack` (empty payload). No historical messages are pushed — client loads history explicitly via `LoadMessages`.

### Unsubscribe (0x19)

`count: u16`, `chat_ids: [u32]` — fire-and-forget, no Ack.

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

Two modes selected by `mode: u8`.

**Mode 0 — first page** (no cursor):

```
mode: u8=0 | limit: u16
```

**Mode 1 — subsequent page** (cursor from previous response):

```
mode: u8=1 | cursor_ts: i64 | limit: u16
```

Response: `Ack` with payload: `next_cursor_ts: i64`, then `count: u32` + chat entries.

### GetPresence (0x15)

`count: u16`, `user_ids: [u32]`

Response: `PresenceResult (0x27)` frame with the same seq. Payload: `count: u16` + `PresenceEntry` list.

### Search (0x17)

```
scope: u8 | scope_payload | query: String | cursor: u32 | limit: u16
```

Search scope is selected by `scope: u8` discriminant:

| Scope  | Value | Payload        | Description                              |
| ------ | ----- | -------------- | ---------------------------------------- |
| Chat   | 0     | `chat_id: u32` | Search within a specific chat            |
| Global | 1     | (none)         | Search across all chats user is member of|
| User   | 2     | `user_id: u32` | Search messages from a specific user     |

`cursor = 0` means first page.

Response: `Ack` with payload: `next_cursor: u32`, then `count: u32` + `(message_id: u32, snippet_len: u32, snippet: UTF-8)` entries.

### UpdateChat (0x41)

```
chat_id: u32 | title_flag: u8 [+ title: String] | avatar_flag: u8 [+ avatar_url: String]
```

Each field uses a `u8 flag` prefix: `0` = don't change, `1` = set to following string
(empty string = clear the field, i.e. set to NULL on server).

## Versioning

Protocol version is negotiated once during the handshake via `protocol_version` in the **Hello** payload. If the server does not support the requested version, it responds with an `unsupported_version` error and closes the connection. There is no per-frame version field.

## Deduplication

Each persistent command (SendMessage, EditMessage, DeleteMessage) contains an `idempotency_key: UUID` (16 bytes) generated by the client. The server stores keys for 24 hours and returns the original result for duplicates without side effects.
