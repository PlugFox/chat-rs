//! Message types — kinds, flags, rich content spans.

use bitflags::bitflags;

/// Message content type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MessageKind {
    /// Plain text message.
    Text = 0,
    /// Image message.
    Image = 1,
    /// File attachment.
    File = 2,
    /// System event (join/leave/etc). Always paired with `MessageFlags::SYSTEM`.
    System = 3,
}

impl MessageKind {
    /// Convert from wire byte. Returns `None` for unknown values.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Text),
            1 => Some(Self::Image),
            2 => Some(Self::File),
            3 => Some(Self::System),
            _ => None,
        }
    }
}

bitflags! {
    /// Message property flags (u16 on wire, i16 in PostgreSQL).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MessageFlags: u16 {
        /// Edited at least once; display "edited" label.
        const EDITED    = 0x0001;
        /// Soft-deleted tombstone; content is empty.
        const DELETED   = 0x0002;
        /// Forwarded from another chat; origin in extra JSON.
        const FORWARDED = 0x0004;
        /// Pinned in this chat.
        const PINNED    = 0x0008;
        /// No push notification for this message.
        const SILENT    = 0x0010;
        /// System event message (member join/leave, etc.).
        const SYSTEM    = 0x0020;
        /// Sent by a bot user (server-authoritative).
        const BOT       = 0x0040;
        /// Reply to another message; origin in extra JSON.
        const REPLY     = 0x0080;
        // 0x0100–0x8000: reserved
    }
}

bitflags! {
    /// Rich text style flags (u16 on wire).
    ///
    /// Inline styles are freely combinable. Block-level styles (`CODE_BLOCK`,
    /// `BLOCKQUOTE`) have special semantics — see docs/messages.md.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RichStyle: u16 {
        // Inline styles (combinable)
        /// Bold text.
        const BOLD       = 0x0001;
        /// Italic text.
        const ITALIC     = 0x0002;
        /// Underlined text.
        const UNDERLINE  = 0x0004;
        /// Strikethrough text.
        const STRIKE     = 0x0008;
        /// Spoiler text (hidden until tapped/clicked).
        const SPOILER    = 0x0010;
        /// Inline monospace code.
        const CODE       = 0x0020;

        // Styles with meta (combinable with inline)
        /// Hyperlink. Meta: `{"url": "..."}`.
        const LINK       = 0x0040;
        /// User mention. Meta: `{"user_id": u32}`.
        const MENTION    = 0x0080;
        /// Colored text. Meta: `{"rgba": u32}`.
        const COLOR      = 0x0100;

        // Block-level (exclusive — overrides inline styles on this span)
        /// Fenced code block. Meta: `{"lang": "rust"}`.
        /// When set, client ignores inline style bits on this span.
        const CODE_BLOCK = 0x0200;
        /// Block quote (`>` prefixed text).
        const BLOCKQUOTE = 0x0400;

        // 0x0800–0x8000: reserved
    }
}

impl RichStyle {
    /// Whether this style has associated meta JSON.
    pub fn has_meta(self) -> bool {
        self.intersects(Self::LINK | Self::MENTION | Self::COLOR | Self::CODE_BLOCK)
    }
}

/// A rich text span — a styled range within the plain-text content.
///
/// Wire format: 10 bytes fixed (start: u32, end: u32, style: u16)
/// + meta_len: u32 + optional JSON meta.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RichSpan {
    /// Start byte offset into the plain-text content (inclusive).
    pub start: u32,
    /// End byte offset into the plain-text content (exclusive).
    pub end: u32,
    /// Style flags for this span.
    pub style: RichStyle,
    /// Optional JSON metadata. `None` when no meta-bearing style bits are set.
    pub meta: Option<String>,
}

/// A decoded message (as transmitted in `MessageBatch`).
///
/// TODO: Add `reactions` field (Vec of pack_id + emoji_index + count + user_reacted)
/// so that reactions are persisted and available when loading message history.
/// Currently reactions are only delivered as live `ReactionUpdate` events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    /// Sequential per-chat ID (starts at 1).
    pub id: u32,
    /// Chat this message belongs to.
    pub chat_id: u32,
    /// Internal user ID of the sender.
    pub sender_id: u32,
    /// Creation timestamp, Unix seconds.
    pub created_at: i64,
    /// Last modification timestamp, Unix seconds.
    pub updated_at: i64,
    /// Content type.
    pub kind: MessageKind,
    /// Bitfield of message properties.
    pub flags: MessageFlags,
    /// Message this is replying to. `None` = not a reply.
    /// When set, `MessageFlags::REPLY` is also set.
    pub reply_to_id: Option<u32>,
    /// Plain text content; empty string for deleted tombstones.
    pub content: String,
    /// Rich content spans. `None` = no formatting.
    pub rich_content: Option<Vec<RichSpan>>,
    /// Extra metadata JSON. `None` = no metadata.
    pub extra: Option<String>,
}

/// A batch of messages (used in SyncBatch events and LoadMessages responses).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageBatch {
    /// Messages in this batch.
    pub messages: Vec<Message>,
    /// Whether more messages exist beyond this batch.
    pub has_more: bool,
}
