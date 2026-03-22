# Chats

> See also: [protocol.md](protocol.md) for frame kinds, [codec.md](codec.md) for binary encoding, [database.md](database.md) for schema, [messages.md](messages.md) for message structure.

## Overview

A **chat** is the container for messages and members. There are three kinds:

- **Direct (DM)** — exactly two participants, no title, no hierarchy.
- **Group** — named conversation with multiple members; may have channels.
- **Channel** — read-mostly broadcast room nested inside a Group (`parent_id` required).

## Fields

| Field         | Wire type         | Rust type        | Description                                       |
| ------------- | ----------------- | ---------------- | ------------------------------------------------- |
| `id`          | `u32`             | `u32`            | Globally unique chat ID                           |
| `kind`        | `u8`              | `ChatKind`       | dm / group / channel                              |
| `parent_id`   | `u8 flag + u32`   | `Option<u32>`    | Parent group ID; present only for channels        |
| `title`       | `u32 len + UTF-8` | `Option<String>` | Display name; absent for DMs                      |
| `avatar_url`  | `u32 len + UTF-8` | `Option<String>` | Avatar URL; absent when `len = 0`                 |
| `created_at`  | `i64`             | `i64`            | Creation timestamp, Unix seconds (validated)      |
| `updated_at`  | `i64`             | `i64`            | Last modification timestamp, Unix seconds         |
| `last_message`| `u8 flag + fields`| `Option<LastMessagePreview>` | Last message preview; absent for empty chats |
| `unread_count`| `u32`             | `u32`            | Unread messages (server-computed)             |
| `member_count`| `u32`             | `u32`            | Total members in this chat                    |

## ChatKind

```rust
#[repr(u8)]
pub enum ChatKind {
    Direct  = 0,
    Group   = 1,
    Channel = 2,
}
```

`kind` values are stable — never renumbered.

## ChatRole

```rust
#[repr(u8)]
#[derive(PartialOrd, Ord)]
pub enum ChatRole {
    Member    = 0,
    Moderator = 1,
    Admin     = 2,
    Owner     = 3,
}
```

Roles are ordered by privilege: `Owner > Admin > Moderator > Member`. A user may not
assign a role equal to or higher than their own.

## Permissions

Per-member permission overrides are stored as `Permission` bitflags (`u32` on the wire,
`i32` in PostgreSQL — no unsigned types available).

`NULL` in the database means "use role defaults" — see `default_permissions()` in
`chat_protocol::types`.

```rust
bitflags! {
    pub struct Permission: u32 {
        // Messages
        const SEND_MESSAGES         = 1 << 0;
        const SEND_MEDIA            = 1 << 1;
        const SEND_LINKS            = 1 << 2;
        const PIN_MESSAGES          = 1 << 3;
        const EDIT_OWN_MESSAGES     = 1 << 4;
        const DELETE_OWN_MESSAGES   = 1 << 5;

        // Moderation
        const DELETE_OTHERS_MESSAGES = 1 << 10;
        const MUTE_MEMBERS          = 1 << 11;
        const BAN_MEMBERS           = 1 << 12;

        // Management
        const INVITE_MEMBERS        = 1 << 20;
        const KICK_MEMBERS          = 1 << 21;
        const MANAGE_CHAT_INFO      = 1 << 22;
        const MANAGE_ROLES          = 1 << 23;

        // Owner
        const TRANSFER_OWNERSHIP    = 1 << 30;
        const DELETE_CHAT           = 1 << 31;
    }
}
```

### Default permissions by role

```rust
fn default_permissions(role: ChatRole, chat_kind: ChatKind) -> Permission {
    match (role, chat_kind) {
        (ChatRole::Owner, _) => Permission::all(),
        (ChatRole::Admin, _) => Permission::SEND_MESSAGES
            | Permission::SEND_MEDIA | Permission::SEND_LINKS
            | Permission::PIN_MESSAGES | Permission::EDIT_OWN_MESSAGES
            | Permission::DELETE_OWN_MESSAGES | Permission::DELETE_OTHERS_MESSAGES
            | Permission::MUTE_MEMBERS | Permission::BAN_MEMBERS
            | Permission::INVITE_MEMBERS | Permission::KICK_MEMBERS
            | Permission::MANAGE_CHAT_INFO | Permission::MANAGE_ROLES,
        (ChatRole::Moderator, _) => Permission::SEND_MESSAGES
            | Permission::SEND_MEDIA | Permission::SEND_LINKS
            | Permission::PIN_MESSAGES | Permission::EDIT_OWN_MESSAGES
            | Permission::DELETE_OWN_MESSAGES | Permission::DELETE_OTHERS_MESSAGES
            | Permission::MUTE_MEMBERS,
        (ChatRole::Member, ChatKind::Channel) => Permission::empty(),
        (ChatRole::Member, _) => Permission::SEND_MESSAGES
            | Permission::SEND_MEDIA | Permission::SEND_LINKS
            | Permission::EDIT_OWN_MESSAGES | Permission::DELETE_OWN_MESSAGES,
    }
}
```

Notable defaults:
- **Channel / Member** — `Permission::empty()` (read-only by default).
- **Group / Member** — can send messages, media, links, and edit/delete their own messages.
- **Moderator** — all Member permissions + delete others' messages + mute.
- **Admin** — all Moderator permissions + ban + invite/kick + manage chat info and roles.
- **Owner** — all permissions including transfer ownership and delete chat.

## Wire Format

All values are little-endian. See [codec.md](codec.md) for type mapping.

### ChatEntry — variable length

Used in `LoadChats` responses and `ChatCreated` / `ChatUpdated` events.

```
┌────────┬────────┬──────────────────────────┬─────────────┬─────────────┬────────────────┐
│id: u32 │kind: u8│parent: u8 [+ parent: u32]│created_at   │updated_at   │title + avatar  │
│ 4 bytes│ 1 byte │ 1 byte   [+  4 bytes]    │  i64 8bytes │  i64 8bytes │ variable       │
└────────┴────────┴──────────────────────────┴─────────────┴─────────────┴────────────────┘
```

`parent` byte: `0` = no parent (DM or Group), `1` = parent follows as `u32`.

Followed by two length-prefixed strings (each absent when `len = 0`):

```
┌──────────────┬──────────────┬────────────────┬────────────────┐
│ title_len:u32│ title (UTF-8)│avatar_len: u32 │ avatar (UTF-8) │
└──────────────┴──────────────┴────────────────┴────────────────┘
```

Followed by last message preview (`u8 flag`: 0 = absent, 1 = preview follows):

```
┌───────────┬─────────────────────────────────────────────────────────────────┐
│ flag: u8  │ [id:u32 | sender_id:u32 | created_at:i64 | kind:u8 | flags:u16│
│           │  | content_preview: String]                                     │
└───────────┴─────────────────────────────────────────────────────────────────┘
```

`content_preview` is a server-truncated plain-text snippet (up to 100 bytes, UTF-8 safe).

Followed by: `unread_count: u32`, `member_count: u32`.

Minimum size: 39 bytes (DM, no parent, no strings, no last_message: 4+1+1+8+8+4+4+1+4+4).

Timestamps are validated against codec range (see [codec.md](codec.md#timestamp-validation)).

### ChatMemberEntry — 6–10 bytes

Used in `GetChatMembers` response.

```
┌────────────┬─────────┬──────────────────────────────────┐
│ user_id:u32│ role: u8│ perm: u8 [+ permissions: u32]    │
│   4 bytes  │  1 byte │ 1 byte   [+      4 bytes]        │
└────────────┴─────────┴──────────────────────────────────┘
```

`perm` byte: `0` = use role defaults, `1` = override follows as `u32`.

### LoadChats (0x16) Request

```
┌─────────────────┬────────────┐
│ cursor_ts: i64  │ limit: u16 │
└─────────────────┴────────────┘
```

`cursor_ts = 0` — first page. Server returns chats ordered by `updated_at DESC`.

### LoadChats Response (Ack payload)

```
┌──────────────────────┬──────────────┬─────────────────────────┐
│ next_cursor_ts: i64  │ count: u32   │ entries[count]           │
└──────────────────────┴──────────────┴─────────────────────────┘
```

`next_cursor_ts = 0` means no more pages.

### CreateChat (0x40) Request

```
┌────────┬──────────────────────────┬────────────────┬───────────────────────────────────┐
│kind: u8│ parent: u8 [+ id: u32]  │ title + avatar │ member_count: u16 │ user_ids[]: u32│
└────────┴──────────────────────────┴────────────────┴───────────────────────────────────┘
```

Response: `Ack` with `chat_id: u32`.

### GetChatInfo (0x43) Request / Response

Request: `chat_id: u32`

Response: `Ack` with a single `ChatEntry`.

### GetChatMembers (0x44) Request / Response

Request: `chat_id: u32`, `cursor: u32` (0 = first page), `limit: u16`

Response: `Ack` with `next_cursor: u32`, `count: u32`, `entries[count]: ChatMemberEntry`.

### InviteMembers (0x45)

`chat_id: u32`, `count: u16`, `user_ids[count]: u32`

Response: `Ack` (empty).

### UpdateMember (0x46)

`chat_id: u32`, `user_id: u32`, `action: u8` + action-specific payload.

See [protocol.md](protocol.md#updatemember-0x46) for action discriminant values
(Kick=0, Ban=1, Mute=2, ChangeRole=3, UpdatePermissions=4, Unban=5).

Response: `Ack` (empty).

### ChatUpdated (0x28) Event

Server pushes this when chat metadata changes (title, avatar).
Payload is a full `ChatEntry` — clients replace their cached copy.

### ChatCreated (0x29) Event

Server pushes this when the user is added to a new chat (invited, or DM initiated by
someone else). Payload is a full `ChatEntry`.

## Subscription Model

Receiving real-time events for a chat requires an explicit `Subscribe (0x18)` call
with `chat_id: u32`. The server then pushes:
`MessageNew`, `MessageEdited`, `MessageDeleted`, `ReceiptUpdate`, `TypingUpdate`,
`MemberJoined`, `MemberLeft`.

`Subscribe` does **not** push historical messages — the client loads history separately
via `LoadMessages`. This keeps the subscription lightweight and avoids redundant data
on reconnect.

`Unsubscribe (0x19)` is fire-and-forget (no Ack). The client should call it when
navigating away to stop receiving events for chats not in view.

## Sync Model

The client tracks its own sync cursor locally — the highest message ID it has seen
per chat. After reconnect, it requests new messages via `LoadMessages` for the chats
currently in view:

```
LoadMessages(chat_id, mode=0, direction=newer, anchor_id=<last_known_id>, limit=100)
```

This is cheaper than the server pushing `last_msg_id` in every `ChatEntry`, because
the client only syncs chats it actually opens.

## DM Semantics

- `title` is `NULL` / absent — the client derives the display name from the other
  participant's profile.
- `parent_id` is always absent (`CHECK dm_no_parent`).
- At most one DM may exist between any pair of users (`dm_index` unique constraint).
  When creating a DM the server first checks `dm_index` and returns the existing
  `chat_id` if a conversation already exists, rather than creating a duplicate.
- `dm_index` normalizes the pair: always `user_a = min(id_a, id_b)`,
  `user_b = max(id_a, id_b)`. Violating this constraint causes the INSERT to fail.

## Channel Hierarchy

A channel must have `parent_id` pointing to a Group (`CHECK channel_requires_parent`).
Channels are one level deep — a channel cannot be a parent of another channel (enforced
in application logic; the DB constraint only checks that `kind = 2` has a parent).

Membership in a channel is independent of group membership. A user can be a member of
a channel without being a member of the parent group (e.g. public read-only channels).
The application layer enforces any desired inheritance rules.

Subscribing to a group does **not** auto-subscribe to its channels.

## Pitfalls

**`kind` is not `type`** — `type` is a reserved word in PostgreSQL. All schema and
query code must use `kind`.

**DM title is client-derived** — never send or cache a title for a DM; it must be
built from the other user's display name at render time to stay current.

**`parent_id` is not a folder** — it is a hard constraint: channels belong to exactly
one group and cannot be moved. There is no concept of a chat changing its `parent_id`
after creation.

**Permission override vs. role default** — `permissions = NULL` means "derive from
role". Do not store `default_permissions(role, kind).bits()` as an explicit override;
keep it `NULL` so future default changes propagate automatically.

**`dm_index` ordering** — always enforce `user_a < user_b` before querying or inserting.
Relying on insertion order will silently create duplicate DMs.

**Channel members have no default permissions** — `default_permissions(Member, Channel)`
returns `Permission::empty()`. New channel members cannot send anything until explicitly
granted permissions or promoted.

**`ChatUpdated` replaces, not patches** — the event carries a full `ChatEntry`.
Clients must replace their entire cached entry, not merge individual fields.
