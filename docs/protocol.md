# WebSocket Protocol

> See also: [codec.md](codec.md) for binary encoding details, [error-codes.md](error-codes.md) for error handling.

## Frame Format

All WS binary frames share a 9-byte header:

```
┌──────────┬───────────┬────────────────┬──────────────────┐
│ kind: u8 │  seq: u32 │ event_seq: u32 │ payload: bytes   │
└──────────┴───────────┴────────────────┴──────────────────┘
```

- `kind` — frame type (`FrameKind` enum)
- `seq` — sequence number; see [codec.md](codec.md) for the request/response model
- `event_seq` — server push event counter (see [Event Ordering](#event-ordering))

## Frame Kinds

### Handshake & Keepalive (0x01..0x05)

| Kind         | Value | Direction       | Purpose                              |
| ------------ | ----- | --------------- | ------------------------------------ |
| Hello        | 0x01  | client → server | Protocol version, token, device_id   |
| Welcome      | 0x02  | server → client | session_id, server_time, limits      |
| Ping         | 0x03  | both            | Keepalive                            |
| Pong         | 0x04  | both            | Keepalive response                   |
| RefreshToken | 0x05  | client → server | Refresh JWT without disconnecting    |

### Commands (0x10..0x1F, client → server)

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
| ForwardMessage | 0x1F  | yes     | yes       |

### Events (0x20..0x2B, server → client)

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
| UserUpdated    | 0x2B  | User profile changed                               |
| ChatDeleted    | 0x2C  | Chat was deleted                                   |
| MemberUpdated  | 0x2D  | Member role or permissions changed                 |

### Responses (0x30..0x31)

| Kind  | Value | Purpose              |
| ----- | ----- | -------------------- |
| Ack   | 0x30  | Command acknowledged |
| Error | 0x31  | Error response       |

### Chat Management (0x40..0x49, client → server, RPC)

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
| MuteChat       | 0x48  | Mute chat notifications                          |
| UnmuteChat     | 0x49  | Unmute chat notifications                        |

### User Management (0x50..0x55, client → server, RPC)

| Kind         | Value | Purpose                        |
| ------------ | ----- | ------------------------------ |
| GetUser      | 0x50  | Get a single user's profile    |
| GetUsers     | 0x51  | Get multiple users' profiles   |
| UpdateProfile| 0x52  | Update own profile             |
| BlockUser    | 0x53  | Block a user                   |
| UnblockUser  | 0x54  | Unblock a user                 |
| GetBlockList | 0x55  | Get blocked users (paginated)  |

## Event Ordering

Server-push events (seq=0) carry a monotonically increasing `event_seq: u32` per session.
Client → server frames and server responses (seq>0) always set `event_seq = 0`.

When `event_seq & 0xC000_0000 != 0` (top 2 bits set, ~3 billion events), the server
sends `DisconnectCode::EventSeqOverflow` (3006) and closes the connection.
The client should reconnect — the counter resets on new sessions.

## Handshake

**Hello** (client → server): `protocol_version`, `sdk_version`, `platform`, `token` (JWT), `device_id` (UUID, 16 bytes).

**Welcome** (server → client): `session_id: u32`, `server_time: i64` (clock sync, Unix seconds), `user_id: u32`, `ServerLimits`, `ServerCapabilities`.

**RefreshToken** (client → server): `token: String` (new JWT).
Response: `Ack` (empty) on success. The session continues without interruption.

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
chat_id: u32 | kind: u8 | idempotency_key: UUID | reply_to_id: Option<u32> | content: String | rich_content: optional bytes | extra: optional String | mention_count: u16 | mentioned_user_ids: [u32]
```

`kind` is `MessageKind` (Text=0, Image=1, File=2, System=3).

`reply_to_id` — the message ID being replied to. `None` = not a reply. When set,
the server sets `MessageFlags::REPLY` on the created message and populates `extra.reply`
JSON with the original message's metadata (quote, sender_id, etc.).

`mentioned_user_ids` — user IDs explicitly mentioned in this message. Server uses this
for push notification routing without parsing rich content. When replying, the client
should include the original message author's ID here.

### ForwardMessage (0x1F)

```
from_chat_id: u32 | message_id: u32 | to_chat_id: u32 | idempotency_key: UUID
```

Server copies the original message content to `to_chat_id`, sets `MessageFlags::FORWARDED`,
and populates `extra.fwd` JSON with the original chat_id, msg_id, and sender_id.

Response: `Ack` with `MessageId(u32)` (the new message ID in the target chat).

### Typing (0x14)

```
chat_id: u32 | expires_in_ms: u16
```

`expires_in_ms` — how long this typing indicator is valid. Server forwards this value
to other clients in `TypingUpdate`. Clients auto-expire the indicator after this duration.

**Stop signal**: `expires_in_ms = 0` means "typing stopped immediately" — the user
cleared the input field or navigated away. Clients should remove the indicator on receipt.

### Event Payloads (server → client)

**MessageNew (0x20)**: payload is a single `Message` (same wire format as in `MessageBatch`).

**MessageEdited (0x21)**: payload is a single `Message` with updated content/rich/extra and `EDITED` flag set.

**MessageDeleted (0x22)**: `chat_id: u32`, `message_id: u32`. Content is already cleared server-side.

**TypingUpdate (0x24)**: `chat_id: u32`, `user_id: u32`, `expires_in_ms: u16`.

**MemberJoined (0x25)**: `chat_id: u32`, `user_id: u32`, `role: u8`, `invited_by: u32`.
`invited_by = 0` means self-join (e.g. via invite link).

**MemberLeft (0x26)**: `chat_id: u32`, `user_id: u32`.

**ReactionUpdate (0x2A)**: `chat_id: u32`, `message_id: u32`, `user_id: u32`, `pack_id: u32`, `emoji_index: u8`, `added: u8` (1 = added, 0 = removed).

**UserUpdated (0x2B)**: payload is a full `UserEntry`. Pushed when a user the client
is subscribed to changes their profile.

**ChatDeleted (0x2C)**: `chat_id: u32`. Pushed when a chat is deleted. Clients should
remove it from the chat list and close any open views.

**MemberUpdated (0x2D)**: `chat_id: u32`, `user_id: u32`, `role: u8`, `perm_flag: u8` [+ `permissions: u32`].
Pushed when a member's role or permissions change. `perm_flag = 0` means use role defaults,
`perm_flag = 1` means explicit override follows. Same wire format as `ChatMemberEntry`.

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

### Subscription Model (0x18..0x19)

Subscribe/Unsubscribe use a **channel-based model**. Instead of subscribing to specific
chat IDs, clients subscribe to named channels:

```
count: u16 | channels: [String]
```

**Channel naming conventions:**

| Channel pattern | Purpose                                              |
| --------------- | ---------------------------------------------------- |
| `general`       | Account-level events (chat list updates, etc.)       |
| `push`          | Push notification events                             |
| `chat#<id>`     | Real-time events for a specific chat                 |
| `user#<id>`     | Presence and profile events for a specific user      |

This decouples subscription from specific chat IDs, allowing flexible event routing
and future extensibility. Response: `Ack` (empty).

`Unsubscribe` is fire-and-forget (no Ack).

### LoadMessages (0x1A)

Three modes selected by `mode: u8`.

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

Response: `Ack` with `MessageBatch` containing only messages where
`updated_at > since_ts` within `[from_id, to_id]`. Empty batch = nothing changed.

**Mode 2 — chunk load/update** (chunk-based access):

```
chat_id: u32 | mode: u8=2 | chunk_id: u32 | since_ts: i64
```

Requests messages belonging to a chunk. `chunk_id = message_id >> CHUNK_SHIFT` (see
[Message Chunks](#message-chunks)). A chunk covers IDs `[chunk_id × 64, chunk_id × 64 + 63]`.

- `since_ts = 0` — return all messages in the chunk (full load).
- `since_ts > 0` — return only messages with `updated_at > since_ts` (delta update).

Response: `Ack` with `MessageBatch` (up to 64 messages, `has_more = false`).

### Message Chunks

Messages are grouped into fixed-size **chunks** of 64 messages based on their per-chat ID:

```
CHUNK_SHIFT = 6
CHUNK_SIZE  = 1 << CHUNK_SHIFT  = 64

chunk_id    = message_id >> CHUNK_SHIFT
first_id    = chunk_id << CHUNK_SHIFT       (= chunk_id × 64)
last_id     = first_id + CHUNK_SIZE - 1     (= chunk_id × 64 + 63)
```

Because message IDs are sequential per-chat and messages are never physically deleted,
chunk boundaries are stable and predictable. This enables:

- **Server-side caching** — completed chunks (64 messages) are immutable and can be
  cached in Redis by `(chat_id, chunk_id)` with no invalidation needed. The last
  (incomplete) chunk is invalidated on new messages or edits.
- **Client-side caching** — clients store and manage messages in chunk granularity.
  To check for updates, the client sends a chunk request with `since_ts` set to the
  maximum `updated_at` it has locally for that chunk.
- **Frontend layout** — UI can render and virtualize messages in chunk-sized blocks,
  managing loading/unloading state per chunk.

### LoadChats (0x16)

Two modes selected by `mode: u8`.

**Mode 0 — first page**: `mode: u8=0 | limit: u16`

**Mode 1 — subsequent page**: `mode: u8=1 | cursor_ts: i64 | limit: u16`

Response: `Ack` with payload: `next_cursor_ts: i64`, then `count: u32` + chat entries.

### Search (0x17)

```
scope: u8 | scope_payload | query: String | cursor: u32 | limit: u16
```

| Scope  | Value | Payload        | Description                              |
| ------ | ----- | -------------- | ---------------------------------------- |
| Chat   | 0     | `chat_id: u32` | Search within a specific chat            |
| Global | 1     | (none)         | Search across all chats user is member of|
| User   | 2     | `user_id: u32` | Search messages from a specific user     |

Response: `Ack` with payload: `next_cursor: u32`, then `count: u32` + search result entries.

### UpdateChat (0x41)

```
chat_id: u32 | title_flag: u8 [+ title: String] | avatar_flag: u8 [+ avatar_url: String]
```

Each field uses a `u8 flag` prefix: `0` = don't change, `1` = set to following string
(empty string = clear the field, i.e. set to NULL on server).

### UpdateMember (0x46)

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

### MuteChat / UnmuteChat (0x48..0x49)

**MuteChat (0x48)**: `chat_id: u32`, `duration_secs: u32` (0 = mute forever).
**UnmuteChat (0x49)**: `chat_id: u32`.
Both respond with `Ack` (empty).

### User Management (0x50..0x55)

**GetUser (0x50)**: `user_id: u32`. Response: `Ack` with `UserInfo` (single `UserEntry`).

**GetUsers (0x51)**: `count: u16`, `user_ids: [u32]`. Response: `Ack` with `UserList` (user entries).

**UpdateProfile (0x52)**: Uses updatable string semantics (u8 flag prefix per field):
`username`, `first_name`, `last_name`, `avatar_url`. Response: `Ack` (empty).
Server broadcasts `UserUpdated` to subscribed clients.

**BlockUser (0x53)**: `user_id: u32`. Response: `Ack` (empty).
Blocked user cannot send DMs to the blocker; their messages are hidden in shared groups (client-side).

**UnblockUser (0x54)**: `user_id: u32`. Response: `Ack` (empty).

**GetBlockList (0x55)**: `cursor: u32`, `limit: u16`.
Response: `Ack` with `BlockList` (paginated list of blocked user IDs).

### GetPresence (0x15)

`count: u16`, `user_ids: [u32]`

Response: `PresenceResult (0x27)` frame with the same seq.

## Versioning

Protocol version is negotiated once during the handshake via `protocol_version` in the **Hello** payload. If the server does not support the requested version, it responds with an `unsupported_version` error and closes the connection. There is no per-frame version field.

## Deduplication

Each create command (SendMessage, ForwardMessage) contains an `idempotency_key: UUID` (16 bytes) generated by the client. The server stores keys for 24 hours and returns the original result for duplicates without side effects.

EditMessage and DeleteMessage do not carry idempotency keys — these operations are inherently idempotent (editing to the same content or deleting an already-deleted message are no-ops).
