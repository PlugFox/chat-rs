//! Frame kinds and payload structures for the WebSocket protocol.

use bitflags::bitflags;
use uuid::Uuid;

/// Frame type identifier — first byte of every WS binary frame.
///
/// Values are stable and must never be renumbered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FrameKind {
    // Handshake & keepalive (0x01..0x04)
    /// Client → server: protocol version, token, device_id.
    Hello = 0x01,
    /// Server → client: session_id, server_time, limits.
    Welcome = 0x02,
    /// Keepalive ping (both directions).
    Ping = 0x03,
    /// Keepalive pong (both directions).
    Pong = 0x04,

    // Commands (0x10..0x1A, client → server)
    /// Send a new message (persist, needs Ack).
    SendMessage = 0x10,
    /// Edit an existing message (persist, needs Ack).
    EditMessage = 0x11,
    /// Soft-delete a message (persist, needs Ack).
    DeleteMessage = 0x12,
    /// Mark messages as read (persist, fire-and-forget).
    ReadReceipt = 0x13,
    /// Typing indicator (ephemeral, fire-and-forget).
    Typing = 0x14,
    /// Request online/offline status (RPC).
    GetPresence = 0x15,
    /// Load chat list (RPC).
    LoadChats = 0x16,
    /// Full-text message search (RPC).
    Search = 0x17,
    /// Subscribe to real-time events for a chat (RPC).
    Subscribe = 0x18,
    /// Unsubscribe from a chat (fire-and-forget).
    Unsubscribe = 0x19,
    /// Load message history (RPC).
    LoadMessages = 0x1A,

    // Events (0x20..0x29, server → client)
    /// New message delivered in real-time. Payload: single `Message`.
    MessageNew = 0x20,
    /// Message content changed. Payload: single `Message` with updated fields.
    MessageEdited = 0x21,
    /// Message marked deleted. Payload: `chat_id: u32, message_id: u32`.
    MessageDeleted = 0x22,
    /// Read receipt update.
    ReceiptUpdate = 0x23,
    /// Typing indicator broadcast.
    TypingUpdate = 0x24,
    /// Member joined chat.
    MemberJoined = 0x25,
    /// Member left chat.
    MemberLeft = 0x26,
    /// Response to GetPresence.
    PresenceResult = 0x27,
    /// Chat metadata changed (title, avatar). Payload: full `ChatEntry`.
    ChatUpdated = 0x28,
    /// New chat the user is a member of. Payload: full `ChatEntry`.
    ChatCreated = 0x29,

    // Responses (0x30..0x31)
    /// Command acknowledged.
    Ack = 0x30,
    /// Error response.
    Error = 0x31,

    // Chat management (0x40..0x47, client → server, RPC)
    /// Create a new chat.
    CreateChat = 0x40,
    /// Update chat info (title, avatar).
    UpdateChat = 0x41,
    /// Delete a chat.
    DeleteChat = 0x42,
    /// Get chat details.
    GetChatInfo = 0x43,
    /// List chat members.
    GetChatMembers = 0x44,
    /// Invite users to a chat.
    InviteMembers = 0x45,
    /// Kick, ban, mute, change role, or update permissions for a member.
    UpdateMember = 0x46,
    /// Leave a chat.
    LeaveChat = 0x47,
}

impl FrameKind {
    /// Convert from wire byte. Returns `None` for unknown values.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(Self::Hello),
            0x02 => Some(Self::Welcome),
            0x03 => Some(Self::Ping),
            0x04 => Some(Self::Pong),

            0x10 => Some(Self::SendMessage),
            0x11 => Some(Self::EditMessage),
            0x12 => Some(Self::DeleteMessage),
            0x13 => Some(Self::ReadReceipt),
            0x14 => Some(Self::Typing),
            0x15 => Some(Self::GetPresence),
            0x16 => Some(Self::LoadChats),
            0x17 => Some(Self::Search),
            0x18 => Some(Self::Subscribe),
            0x19 => Some(Self::Unsubscribe),
            0x1A => Some(Self::LoadMessages),

            0x20 => Some(Self::MessageNew),
            0x21 => Some(Self::MessageEdited),
            0x22 => Some(Self::MessageDeleted),
            0x23 => Some(Self::ReceiptUpdate),
            0x24 => Some(Self::TypingUpdate),
            0x25 => Some(Self::MemberJoined),
            0x26 => Some(Self::MemberLeft),
            0x27 => Some(Self::PresenceResult),
            0x28 => Some(Self::ChatUpdated),
            0x29 => Some(Self::ChatCreated),

            0x30 => Some(Self::Ack),
            0x31 => Some(Self::Error),

            0x40 => Some(Self::CreateChat),
            0x41 => Some(Self::UpdateChat),
            0x42 => Some(Self::DeleteChat),
            0x43 => Some(Self::GetChatInfo),
            0x44 => Some(Self::GetChatMembers),
            0x45 => Some(Self::InviteMembers),
            0x46 => Some(Self::UpdateMember),
            0x47 => Some(Self::LeaveChat),

            _ => None,
        }
    }

    /// Returns all valid frame kinds (for testing/iteration).
    pub fn all() -> &'static [FrameKind] {
        &[
            Self::Hello,
            Self::Welcome,
            Self::Ping,
            Self::Pong,
            Self::SendMessage,
            Self::EditMessage,
            Self::DeleteMessage,
            Self::ReadReceipt,
            Self::Typing,
            Self::GetPresence,
            Self::LoadChats,
            Self::Search,
            Self::Subscribe,
            Self::Unsubscribe,
            Self::LoadMessages,
            Self::MessageNew,
            Self::MessageEdited,
            Self::MessageDeleted,
            Self::ReceiptUpdate,
            Self::TypingUpdate,
            Self::MemberJoined,
            Self::MemberLeft,
            Self::PresenceResult,
            Self::ChatUpdated,
            Self::ChatCreated,
            Self::Ack,
            Self::Error,
            Self::CreateChat,
            Self::UpdateChat,
            Self::DeleteChat,
            Self::GetChatInfo,
            Self::GetChatMembers,
            Self::InviteMembers,
            Self::UpdateMember,
            Self::LeaveChat,
        ]
    }
}

/// Decoded frame header (5 bytes on the wire).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameHeader {
    /// Frame type.
    pub kind: FrameKind,
    /// Sequence number. `0` = fire-and-forget / server push.
    pub seq: u32,
}

// ---------------------------------------------------------------------------
// Payload structs
// ---------------------------------------------------------------------------

/// Hello frame payload (client → server).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelloPayload {
    /// Protocol version the client supports.
    pub protocol_version: u8,
    /// Client SDK version string (e.g. "1.0.0").
    pub sdk_version: String,
    /// Client platform string (e.g. "dart", "typescript", "rust").
    pub platform: String,
    /// JWT authentication token.
    pub token: String,
    /// Unique device identifier (UUID v4, 16 bytes on wire).
    pub device_id: Uuid,
}

/// Welcome frame payload (server → client).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WelcomePayload {
    /// Transient session identifier for this connection.
    pub session_id: u32,
    /// Server clock, Unix seconds. Client uses for clock-sync.
    pub server_time: i64,
    /// Authenticated user's internal ID.
    pub user_id: u32,
    /// Server-enforced limits.
    pub limits: ServerLimits,
    /// Server-advertised feature capabilities.
    pub capabilities: ServerCapabilities,
}

/// Server-enforced limits sent in the Welcome payload.
///
/// Clients use these for local enforcement (debouncing, UI limits).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerLimits {
    /// How often the client should send Ping (ms).
    pub ping_interval_ms: u32,
    /// Pong timeout — disconnect if exceeded (ms).
    pub ping_timeout_ms: u32,
    /// Max content size in bytes.
    pub max_message_size: u32,
    /// Max extra JSON size in bytes.
    pub max_extra_size: u32,
    /// Max total WS frame size in bytes.
    pub max_frame_size: u32,
    /// Rate limit: messages per second per user per chat.
    pub messages_per_sec: u16,
    /// Rate limit: concurrent connections per IP.
    pub connections_per_ip: u16,
}

impl Default for ServerLimits {
    fn default() -> Self {
        Self {
            ping_interval_ms: 30_000,
            ping_timeout_ms: 10_000,
            max_message_size: 65_536,
            max_extra_size: 4_096,
            max_frame_size: 131_072,
            messages_per_sec: 10,
            connections_per_ip: 20,
        }
    }
}

bitflags! {
    /// Server-advertised feature capabilities (u32 on wire).
    ///
    /// Sent in Welcome. Client uses these to show/hide features.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ServerCapabilities: u32 {
        /// File and image upload enabled.
        const MEDIA_UPLOAD = 0x01;
        /// Full-text message search enabled.
        const SEARCH       = 0x02;
        /// Emoji reactions enabled.
        const REACTIONS    = 0x04;
        /// Message threads/replies enabled.
        const THREADS      = 0x08;
        /// Bot API enabled.
        const BOTS         = 0x10;
    }
}

/// SendMessage frame payload (client → server).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendMessagePayload {
    /// Target chat.
    pub chat_id: u32,
    /// Client-generated UUID for deduplication. Persisted 24h server-side.
    pub idempotency_key: Uuid,
    /// Plain-text message content.
    pub content: String,
    /// Rich content spans (encoded as blob). `None` = no formatting.
    pub rich_content: Option<Vec<u8>>,
    /// Extra metadata JSON. `None` = no metadata.
    pub extra: Option<String>,
}

/// EditMessage frame payload (client → server).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditMessagePayload {
    /// Target chat.
    pub chat_id: u32,
    /// Message to edit.
    pub message_id: u32,
    /// Client-generated UUID for deduplication.
    pub idempotency_key: Uuid,
    /// New plain-text content.
    pub content: String,
    /// New rich content spans. `None` = remove formatting.
    pub rich_content: Option<Vec<u8>>,
    /// New extra metadata JSON. `None` = remove metadata.
    pub extra: Option<String>,
}

/// DeleteMessage frame payload (client → server).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteMessagePayload {
    /// Target chat.
    pub chat_id: u32,
    /// Message to delete.
    pub message_id: u32,
    /// Client-generated UUID for deduplication.
    pub idempotency_key: Uuid,
}

/// ReadReceipt frame payload (client → server, fire-and-forget).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadReceiptPayload {
    /// Target chat.
    pub chat_id: u32,
    /// Highest read message ID.
    pub message_id: u32,
}

/// Typing frame payload (client → server, fire-and-forget).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypingPayload {
    /// Target chat.
    pub chat_id: u32,
}

/// GetPresence frame payload (client → server).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPresencePayload {
    /// User IDs to query.
    pub user_ids: Vec<u32>,
}

/// LoadChats frame payload (client → server).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadChatsPayload {
    /// Pagination cursor. `0` = first page (sentinel — no real chat has `updated_at = 0`).
    /// Subsequent pages use the `next_cursor_ts` from the previous response.
    pub cursor_ts: i64,
    /// Max entries to return.
    pub limit: u16,
}

/// Search frame payload (client → server).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchPayload {
    /// Chat to search in.
    pub chat_id: u32,
    /// Search query string.
    pub query: String,
    /// Pagination cursor (0 = first page).
    pub cursor: u32,
    /// Max results to return.
    pub limit: u16,
}

/// Subscribe frame payload (client → server).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubscribePayload {
    /// Chat to subscribe to.
    pub chat_id: u32,
}

/// Unsubscribe frame payload (client → server, fire-and-forget).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnsubscribePayload {
    /// Chat to unsubscribe from.
    pub chat_id: u32,
}

/// LoadMessages mode selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LoadDirection {
    /// Load older messages (before anchor).
    Older = 0,
    /// Load newer messages (after anchor).
    Newer = 1,
}

impl LoadDirection {
    /// Convert from wire byte. Returns `None` for unknown values.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Older),
            1 => Some(Self::Newer),
            _ => None,
        }
    }
}

/// LoadMessages frame payload (client → server).
///
/// Two modes selected by discriminant:
/// - Mode 0: anchor-based pagination (history load)
/// - Mode 1: range update check (catch-up after reconnect)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadMessagesPayload {
    /// Anchor-based pagination (mode 0).
    Paginate {
        /// Target chat.
        chat_id: u32,
        /// Scroll direction.
        direction: LoadDirection,
        /// Anchor message ID (0 = start from newest).
        anchor_id: u32,
        /// Max messages to return.
        limit: u16,
    },
    /// Range update check (mode 1).
    RangeCheck {
        /// Target chat.
        chat_id: u32,
        /// Start of the range (inclusive).
        from_id: u32,
        /// End of the range (inclusive).
        to_id: u32,
        /// `MAX(updated_at)` from client's local cache for this range.
        since_ts: i64,
    },
}

// --- Chat management payloads ---

/// CreateChat frame payload (client → server).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateChatPayload {
    /// Chat type.
    pub kind: super::ChatKind,
    /// Parent group ID (required for channels).
    pub parent_id: Option<u32>,
    /// Chat title (absent for DMs).
    pub title: Option<String>,
    /// Chat avatar URL.
    pub avatar_url: Option<String>,
    /// Initial member user IDs.
    pub member_ids: Vec<u32>,
}

/// UpdateChat frame payload (client → server).
///
/// **Clear semantics**: an empty string means "clear this field" (set to NULL on server).
/// `None` means "don't change". On the wire, `None` = `len 0` and empty string is not
/// distinguishable from `None`, so we use a `u8 flag` prefix:
/// `0` = don't change, `1` = set to following string (empty string = clear).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateChatPayload {
    /// Target chat.
    pub chat_id: u32,
    /// New title. `None` = don't change. `Some("")` = clear.
    pub title: Option<String>,
    /// New avatar URL. `None` = don't change. `Some("")` = clear.
    pub avatar_url: Option<String>,
}

/// DeleteChat frame payload (client → server).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteChatPayload {
    /// Target chat.
    pub chat_id: u32,
}

/// GetChatInfo frame payload (client → server).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetChatInfoPayload {
    /// Target chat.
    pub chat_id: u32,
}

/// GetChatMembers frame payload (client → server).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetChatMembersPayload {
    /// Target chat.
    pub chat_id: u32,
    /// Pagination cursor (0 = first page).
    pub cursor: u32,
    /// Max members to return.
    pub limit: u16,
}

/// InviteMembers frame payload (client → server).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InviteMembersPayload {
    /// Target chat.
    pub chat_id: u32,
    /// User IDs to invite.
    pub user_ids: Vec<u32>,
}

/// LeaveChat frame payload (client → server).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeaveChatPayload {
    /// Target chat.
    pub chat_id: u32,
}

/// Action to perform on a chat member (used in `UpdateMember` frame).
///
/// Wire format: `action: u8` discriminant + action-specific payload.
/// Discriminant values: Kick=0, Ban=1, Mute=2, ChangeRole=3, UpdatePermissions=4.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberAction {
    /// Remove member from chat. Wire: action=0, no payload.
    Kick,
    /// Ban member from chat. Wire: action=1, no payload.
    Ban,
    /// Mute member. Wire: action=2, payload: `duration_secs: u32` (0 = unmute).
    Mute { duration_secs: u32 },
    /// Change member's role. Wire: action=3, payload: `role: u8`.
    ChangeRole(super::ChatRole),
    /// Set explicit permission override. Wire: action=4, payload: `permissions: u32`.
    UpdatePermissions(super::Permission),
}

/// UpdateMember frame payload (client → server).
///
/// Unified frame for kick, ban, mute, role change, and permission override.
/// Replaces the previous separate `KickMember`, `BanMember`, `MuteMember`,
/// and `UpdateMemberRole` frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateMemberPayload {
    /// Target chat.
    pub chat_id: u32,
    /// Target user.
    pub user_id: u32,
    /// Action to perform.
    pub action: MemberAction,
}

/// MessageDeleted event payload (server → client).
///
/// Content is already cleared server-side; this event tells the client
/// which message was deleted so it can update the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageDeletedPayload {
    /// Chat containing the deleted message.
    pub chat_id: u32,
    /// Deleted message ID.
    pub message_id: u32,
}

// --- Event payloads (server → client) ---

/// ReceiptUpdate event payload (server → client).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReceiptUpdatePayload {
    /// Chat where the receipt update occurred.
    pub chat_id: u32,
    /// User who read the messages.
    pub user_id: u32,
    /// Highest read message ID.
    pub message_id: u32,
}

/// TypingUpdate event payload (server → client).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypingUpdatePayload {
    /// Chat where typing is happening.
    pub chat_id: u32,
    /// User who is typing.
    pub user_id: u32,
}

/// MemberJoined event payload (server → client).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemberJoinedPayload {
    /// Target chat.
    pub chat_id: u32,
    /// User who joined.
    pub user_id: u32,
}

/// MemberLeft event payload (server → client).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemberLeftPayload {
    /// Target chat.
    pub chat_id: u32,
    /// User who left.
    pub user_id: u32,
}

/// Ack payload — command-specific response data.
///
/// The variant is determined by the `FrameKind` of the original request.
/// Some variants carry raw bytes that must be decoded with the appropriate
/// codec function (e.g. `decode_message_batch` for `MessageBatch`).
/// This is intentional: the codec layer does not track which request
/// generated the Ack, so the caller provides the context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AckPayload {
    /// Empty ack (Subscribe, UpdateMember, Leave, etc.).
    Empty,
    /// SendMessage ack: server-assigned message ID.
    MessageId(u32),
    /// CreateChat ack: server-assigned chat ID.
    ChatId(u32),
    /// LoadMessages: message batch (raw bytes, decode with `decode_message_batch`).
    MessageBatch(Vec<u8>),
    /// LoadChats: next cursor + chat entries (raw bytes).
    ChatList(Vec<u8>),
    /// GetChatInfo: single chat entry (raw bytes).
    ChatInfo(Vec<u8>),
    /// GetChatMembers: member list (raw bytes).
    MemberList(Vec<u8>),
    /// Search results (raw bytes).
    SearchResults(Vec<u8>),
}
