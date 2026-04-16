//! Shared domain types used by both client and server.

use bitflags::bitflags;

/// Chat types.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChatKind {
    Direct = 0,
    Group = 1,
    Channel = 2,
}

/// Message kinds.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageKind {
    Text = 0,
    Image = 1,
    File = 2,
    System = 3,
}

/// Outbox state for messages sent by the local user.
///
/// Client-only — never transmitted over the wire.
/// Deletion state is tracked via [`MessageFlags::DELETED`].
/// Read/delivery state is tracked via the receipt system.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageStatus {
    Sending = 0,
    Delivered = 1,
    FailedPermanent = 2,
}

/// Chat member roles, ordered by privilege level.
#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChatRole {
    Member = 0,
    Moderator = 1,
    Admin = 2,
    Owner = 3,
}

bitflags! {
    /// User type and capability flags, transmitted as `u16` in the wire format.
    ///
    /// Stored as `SMALLINT` (i16) in PostgreSQL — no unsigned type available.
    /// Use `flags.bits() as i16` to write, `UserFlags::from_bits_truncate(raw as u16)` to read.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct UserFlags: u16 {
        /// System account (server-generated messages, join/leave notices).
        const SYSTEM  = 0x0001;
        /// Bot account; server sets MessageFlags::BOT on all messages from this user.
        const BOT     = 0x0002;
        /// Premium subscriber.
        const PREMIUM = 0x0004;
        // 0x0008–0x8000: reserved
    }
}

bitflags! {
    /// Message property flags, transmitted as `u16` in the wire format.
    ///
    /// Stored as `INTEGER` (i32) in PostgreSQL — no unsigned type available.
    /// Use `flags.bits() as i32` to write, `MessageFlags::from_bits_truncate(raw as u16)` to read.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MessageFlags: u16 {
        /// Edited at least once; clients show an "edited" label.
        const EDITED    = 0x0001;
        /// Soft-deleted: `content` and `rich` are cleared; row is never removed.
        const DELETED   = 0x0002;
        /// Forwarded from another chat; origin info in `extra.fwd`.
        const FORWARDED = 0x0004;
        /// Pinned in this chat.
        const PINNED    = 0x0008;
        /// No push notification for this message.
        const SILENT    = 0x0010;
        /// System event message (member join/leave, etc.); paired with `MessageKind::System`.
        const SYSTEM    = 0x0020;
        /// Sent by a bot user; set by the server from `users.is_bot`, never trusted from client.
        const BOT       = 0x0040;
        /// Reply to another message; origin info in `extra.reply`.
        const REPLY     = 0x0080;
        // 0x0100–0x8000: reserved
    }
}

bitflags! {
    /// Per-member permission flags (stored as `INTEGER` / `i32` in PostgreSQL).
    ///
    /// Use `permissions.bits() as i32` to write, `Permission::from_bits_truncate(raw as u32)` to read.
    /// Bits 0–31 only — fits in PostgreSQL `INTEGER` with correct 2's complement roundtrip.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

        // Administration
        const INVITE_MEMBERS        = 1 << 20;
        const KICK_MEMBERS          = 1 << 21;
        const MANAGE_CHAT_INFO      = 1 << 22;
        const MANAGE_ROLES          = 1 << 23;

        // Owner
        const TRANSFER_OWNERSHIP    = 1 << 30;
        const DELETE_CHAT           = 1 << 31;
    }
}

/// Default permissions for a role in a given chat kind.
pub fn default_permissions(role: ChatRole, chat_kind: ChatKind) -> Permission {
    match (role, chat_kind) {
        (ChatRole::Owner, _) => Permission::all(),

        (ChatRole::Admin, _) => {
            Permission::SEND_MESSAGES
                | Permission::SEND_MEDIA
                | Permission::SEND_LINKS
                | Permission::PIN_MESSAGES
                | Permission::EDIT_OWN_MESSAGES
                | Permission::DELETE_OWN_MESSAGES
                | Permission::DELETE_OTHERS_MESSAGES
                | Permission::MUTE_MEMBERS
                | Permission::BAN_MEMBERS
                | Permission::INVITE_MEMBERS
                | Permission::KICK_MEMBERS
                | Permission::MANAGE_CHAT_INFO
                | Permission::MANAGE_ROLES
        }

        (ChatRole::Moderator, _) => {
            Permission::SEND_MESSAGES
                | Permission::SEND_MEDIA
                | Permission::SEND_LINKS
                | Permission::PIN_MESSAGES
                | Permission::EDIT_OWN_MESSAGES
                | Permission::DELETE_OWN_MESSAGES
                | Permission::DELETE_OTHERS_MESSAGES
                | Permission::MUTE_MEMBERS
        }

        (ChatRole::Member, ChatKind::Channel) => Permission::empty(),

        (ChatRole::Member, _) => {
            Permission::SEND_MESSAGES
                | Permission::SEND_MEDIA
                | Permission::SEND_LINKS
                | Permission::EDIT_OWN_MESSAGES
                | Permission::DELETE_OWN_MESSAGES
        }
    }
}

bitflags! {
    /// Rich text span style flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RichStyle: u16 {
        const BOLD    = 0b0000_0001;
        const ITALIC  = 0b0000_0010;
        const CODE    = 0b0000_0100;
        const STRIKE  = 0b0000_1000;
        const SPOILER = 0b0001_0000;
        const LINK    = 0b0010_0000;
        const MENTION = 0b0100_0000;
        const COLOR   = 0b1000_0000;
    }
}

bitflags! {
    /// Server capability flags, sent in Welcome frame.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ServerCapabilities: u32 {
        const MEDIA_UPLOAD = 1 << 0;
        const THUMBNAILS   = 1 << 1;
        const RICH_TEXT    = 1 << 2;
        const REACTIONS    = 1 << 3;
        const THREADS      = 1 << 4;
        const WEBHOOKS     = 1 << 5;
        const BOT_API      = 1 << 6;
    }
}

/// WebSocket disconnect codes.
///
/// Ranges:
/// - `0..999`     — Internal/transport (reconnect: yes)
/// - `3000..3499` — Server non-terminal (reconnect: yes)
/// - `3500..3999` — Server terminal (reconnect: no)
/// - `4000..4499` — Custom non-terminal (reconnect: yes)
/// - `4500..4999` — Custom terminal (reconnect: no)
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisconnectCode {
    // Internal (reconnect: yes)
    Normal = 0,
    NoPing = 1,

    // Server non-terminal (reconnect: yes)
    ServerShutdown = 3000,
    ServerRestart = 3001,
    InsufficientState = 3002,
    ServerError = 3003,
    BufferOverflow = 3004,

    // Server terminal (reconnect: no)
    InvalidToken = 3500,
    TokenExpired = 3501,
    Forbidden = 3502,
    BadRequest = 3503,
    ConnectionLimit = 3504,
    SessionRevoked = 3505,
    TooManyRequests = 3506,
    MessageSizeLimit = 3507,
}

impl DisconnectCode {
    /// Whether the client should attempt to reconnect after this disconnect.
    pub fn should_reconnect(self) -> bool {
        let code = self as u16;
        match code {
            0..1000 => true,
            3000..3500 => true,
            3500..4000 => false,
            4000..4500 => true,
            4500..5000 => false,
            _ => true, // unknown → try reconnect
        }
    }
}

/// Connection state events sent to the UI layer.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionState {
    Connecting = 0x01,
    Connected = 0x02,
    Reconnecting = 0x03,
    Disconnected = 0x04,
    AuthError = 0x05,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_member_has_no_permissions() {
        let perms = default_permissions(ChatRole::Member, ChatKind::Channel);
        assert!(perms.is_empty());
    }

    #[test]
    fn owner_has_all_permissions() {
        let perms = default_permissions(ChatRole::Owner, ChatKind::Group);
        assert_eq!(perms, Permission::all());
    }

    #[test]
    fn role_ordering() {
        assert!(ChatRole::Owner > ChatRole::Admin);
        assert!(ChatRole::Admin > ChatRole::Moderator);
        assert!(ChatRole::Moderator > ChatRole::Member);
    }

    #[test]
    fn disconnect_reconnect_policy() {
        assert!(DisconnectCode::Normal.should_reconnect());
        assert!(DisconnectCode::ServerShutdown.should_reconnect());
        assert!(DisconnectCode::BufferOverflow.should_reconnect());
        assert!(!DisconnectCode::InvalidToken.should_reconnect());
        assert!(!DisconnectCode::SessionRevoked.should_reconnect());
    }
}
