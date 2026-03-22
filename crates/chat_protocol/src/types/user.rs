//! User types — flags, entries, presence.

use bitflags::bitflags;

bitflags! {
    /// User type and capability flags (u16 on wire, i16 in PostgreSQL).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct UserFlags: u16 {
        /// System account (server-generated messages).
        const SYSTEM  = 0x0001;
        /// Bot account; server sets `MessageFlags::BOT` on all messages.
        const BOT     = 0x0002;
        /// Premium subscriber; clients may show a badge.
        const PREMIUM = 0x0004;
        // 0x0008–0x8000: reserved
    }
}

/// A user entry as transmitted on the wire (PresenceResult, user lookups).
///
/// Wire format: 22-byte fixed header + 4 length-prefixed optional strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserEntry {
    /// Internal sequential user ID.
    pub id: u32,
    /// User type and capability flags.
    pub flags: UserFlags,
    /// Account creation timestamp, Unix seconds.
    pub created_at: i64,
    /// Last profile modification timestamp, Unix seconds.
    pub updated_at: i64,
    /// Lowercase latin slug (5–32 chars). `None` when not set.
    pub username: Option<String>,
    /// Display first name (1–64 chars). `None` when not set.
    pub first_name: Option<String>,
    /// Display last name (1–64 chars). `None` when not set.
    pub last_name: Option<String>,
    /// Avatar URL. `None` when not set.
    pub avatar_url: Option<String>,
}

/// Online/offline status for a user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PresenceStatus {
    /// User is offline.
    Offline = 0,
    /// User is online (has at least one active WS connection).
    Online = 1,
}

impl PresenceStatus {
    /// Convert from wire byte. Returns `None` for unknown values.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Offline),
            1 => Some(Self::Online),
            _ => None,
        }
    }
}

/// A presence entry as transmitted in `PresenceResult` (13 bytes fixed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PresenceEntry {
    /// User ID.
    pub user_id: u32,
    /// Current online/offline status.
    pub status: PresenceStatus,
    /// Last seen timestamp, Unix seconds. `0` when user is currently online.
    pub last_seen: i64,
}
