//! Chat types — kinds, roles, permissions, and wire structures.

use bitflags::bitflags;

/// Chat type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ChatKind {
    /// Direct message (exactly two participants, no title).
    Direct = 0,
    /// Group conversation with multiple members.
    Group = 1,
    /// Read-mostly broadcast room nested inside a Group.
    Channel = 2,
}

impl ChatKind {
    /// Convert from wire byte. Returns `None` for unknown values.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Direct),
            1 => Some(Self::Group),
            2 => Some(Self::Channel),
            _ => None,
        }
    }
}

/// Member role within a chat, ordered by privilege level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ChatRole {
    /// Regular member.
    Member = 0,
    /// Can moderate (delete others' messages, mute).
    Moderator = 1,
    /// Can manage (invite, kick, change settings, assign roles).
    Admin = 2,
    /// Full control (transfer ownership, delete chat).
    Owner = 3,
}

impl ChatRole {
    /// Convert from wire byte. Returns `None` for unknown values.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Member),
            1 => Some(Self::Moderator),
            2 => Some(Self::Admin),
            3 => Some(Self::Owner),
            _ => None,
        }
    }
}

bitflags! {
    /// Per-member permission flags (u32 on wire, i32 in PostgreSQL).
    ///
    /// `NULL` / absent in the database means "use role defaults".
    /// See `default_permissions()` for the default set per role × chat kind.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Permission: u32 {
        // Messages (bits 0–5)
        /// Can send text messages.
        const SEND_MESSAGES         = 1 << 0;
        /// Can send media (images, files).
        const SEND_MEDIA            = 1 << 1;
        /// Can send link previews.
        const SEND_LINKS            = 1 << 2;
        /// Can pin messages.
        const PIN_MESSAGES          = 1 << 3;
        /// Can edit own messages.
        const EDIT_OWN_MESSAGES     = 1 << 4;
        /// Can delete own messages.
        const DELETE_OWN_MESSAGES   = 1 << 5;

        // Moderation (bits 10–12)
        /// Can delete other members' messages.
        const DELETE_OTHERS_MESSAGES = 1 << 10;
        /// Can mute members.
        const MUTE_MEMBERS          = 1 << 11;
        /// Can ban members.
        const BAN_MEMBERS           = 1 << 12;

        // Management (bits 20–23)
        /// Can invite new members.
        const INVITE_MEMBERS        = 1 << 20;
        /// Can kick members.
        const KICK_MEMBERS          = 1 << 21;
        /// Can change chat title, avatar.
        const MANAGE_CHAT_INFO      = 1 << 22;
        /// Can assign/change member roles.
        const MANAGE_ROLES          = 1 << 23;

        // Owner (bits 30–31)
        /// Can transfer ownership to another member.
        const TRANSFER_OWNERSHIP    = 1 << 30;
        /// Can delete the chat entirely.
        const DELETE_CHAT           = 1 << 31;
    }
}

/// Returns the default permission set for a given role and chat kind.
///
/// These defaults apply when no explicit permission override is stored
/// for a chat member (i.e. `permissions = NULL` in the database).
pub fn default_permissions(role: ChatRole, chat_kind: ChatKind) -> Permission {
    match (role, chat_kind) {
        (ChatRole::Owner, _) => Permission::all(),
        (ChatRole::Admin, _) => Permission::SEND_MESSAGES
            .union(Permission::SEND_MEDIA)
            .union(Permission::SEND_LINKS)
            .union(Permission::PIN_MESSAGES)
            .union(Permission::EDIT_OWN_MESSAGES)
            .union(Permission::DELETE_OWN_MESSAGES)
            .union(Permission::DELETE_OTHERS_MESSAGES)
            .union(Permission::MUTE_MEMBERS)
            .union(Permission::BAN_MEMBERS)
            .union(Permission::INVITE_MEMBERS)
            .union(Permission::KICK_MEMBERS)
            .union(Permission::MANAGE_CHAT_INFO)
            .union(Permission::MANAGE_ROLES),
        (ChatRole::Moderator, _) => Permission::SEND_MESSAGES
            .union(Permission::SEND_MEDIA)
            .union(Permission::SEND_LINKS)
            .union(Permission::PIN_MESSAGES)
            .union(Permission::EDIT_OWN_MESSAGES)
            .union(Permission::DELETE_OWN_MESSAGES)
            .union(Permission::DELETE_OTHERS_MESSAGES)
            .union(Permission::MUTE_MEMBERS),
        (ChatRole::Member, ChatKind::Channel) => Permission::empty(),
        (ChatRole::Member, _) => Permission::SEND_MESSAGES
            .union(Permission::SEND_MEDIA)
            .union(Permission::SEND_LINKS)
            .union(Permission::EDIT_OWN_MESSAGES)
            .union(Permission::DELETE_OWN_MESSAGES),
    }
}

/// A chat entry as transmitted on the wire (LoadChats, ChatCreated, ChatUpdated).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatEntry {
    /// Globally unique chat ID.
    pub id: u32,
    /// Chat type.
    pub kind: ChatKind,
    /// Parent group ID (present only for channels).
    pub parent_id: Option<u32>,
    /// Creation timestamp, Unix seconds.
    pub created_at: i64,
    /// Last modification timestamp, Unix seconds.
    pub updated_at: i64,
    /// Display title. `None` for DMs.
    pub title: Option<String>,
    /// Avatar URL. `None` when absent.
    pub avatar_url: Option<String>,
}

/// A chat member entry as transmitted on the wire (GetChatMembers response).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChatMemberEntry {
    /// Internal user ID.
    pub user_id: u32,
    /// Member's role.
    pub role: ChatRole,
    /// Permission override. `None` = use role defaults.
    pub permissions: Option<Permission>,
}
