//! WebSocket frame types and kinds.
//!
//! Every WS binary frame has a fixed 6-byte header:
//! ```text
//! ┌──────────┬──────────┬───────────┬──────────────────┐
//! │ ver: u8  │ kind: u8 │  seq: u32 │ payload: bytes   │
//! └──────────┴──────────┴───────────┴──────────────────┘
//! ```

/// All frame kinds in the chat protocol.
///
/// Ranges:
/// - `0x01..0x0F` — Handshake & keepalive
/// - `0x10..0x1F` — Commands & queries (client → server)
/// - `0x20..0x2F` — Events (server → client)
/// - `0x30..0x3F` — Responses
/// - `0x40..0x4F` — Chat management (client → server, RPC)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrameKind {
    // -- Handshake --
    Hello = 0x01,
    Welcome = 0x02,

    // -- Keepalive --
    Ping = 0x03,
    Pong = 0x04,

    // -- Commands (client → server) --
    SendMessage = 0x10,
    EditMessage = 0x11,
    DeleteMessage = 0x12,
    ReadReceipt = 0x13,
    Typing = 0x14,

    // -- Queries (client → server) --
    GetPresence = 0x15,
    LoadChats = 0x16,
    Search = 0x17,

    // -- Events (server → client) --
    MessageNew = 0x20,
    MessageEdited = 0x21,
    MessageDeleted = 0x22,
    ReceiptUpdate = 0x23,
    TypingUpdate = 0x24,
    MemberJoined = 0x25,
    MemberLeft = 0x26,
    SyncBatch = 0x27,
    SyncComplete = 0x28,
    PresenceResult = 0x29,
    ChatUpdated = 0x2A,
    ChatCreated = 0x2B,

    // -- Responses --
    Ack = 0x30,
    Error = 0x31,

    // -- Chat management (client → server, RPC) --
    CreateChat = 0x40,
    UpdateChat = 0x41,
    DeleteChat = 0x42,
    GetChatInfo = 0x43,
    GetChatMembers = 0x44,
    InviteMembers = 0x45,
    KickMember = 0x46,
    LeaveChat = 0x47,
    UpdateMemberRole = 0x48,
    MuteMember = 0x49,
    BanMember = 0x4A,
}

impl FrameKind {
    /// Try to parse a `FrameKind` from a raw `u8` value.
    pub fn from_u8(value: u8) -> Option<Self> {
        // SAFETY: We explicitly check all valid discriminants.
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
            0x20 => Some(Self::MessageNew),
            0x21 => Some(Self::MessageEdited),
            0x22 => Some(Self::MessageDeleted),
            0x23 => Some(Self::ReceiptUpdate),
            0x24 => Some(Self::TypingUpdate),
            0x25 => Some(Self::MemberJoined),
            0x26 => Some(Self::MemberLeft),
            0x27 => Some(Self::SyncBatch),
            0x28 => Some(Self::SyncComplete),
            0x29 => Some(Self::PresenceResult),
            0x2A => Some(Self::ChatUpdated),
            0x2B => Some(Self::ChatCreated),
            0x30 => Some(Self::Ack),
            0x31 => Some(Self::Error),
            0x40 => Some(Self::CreateChat),
            0x41 => Some(Self::UpdateChat),
            0x42 => Some(Self::DeleteChat),
            0x43 => Some(Self::GetChatInfo),
            0x44 => Some(Self::GetChatMembers),
            0x45 => Some(Self::InviteMembers),
            0x46 => Some(Self::KickMember),
            0x47 => Some(Self::LeaveChat),
            0x48 => Some(Self::UpdateMemberRole),
            0x49 => Some(Self::MuteMember),
            0x4A => Some(Self::BanMember),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_kind_roundtrip() {
        let kinds = [
            FrameKind::Hello,
            FrameKind::Welcome,
            FrameKind::SendMessage,
            FrameKind::Ack,
            FrameKind::Error,
            FrameKind::BanMember,
        ];
        for kind in kinds {
            let byte = kind as u8;
            assert_eq!(FrameKind::from_u8(byte), Some(kind));
        }
    }

    #[test]
    fn frame_kind_unknown_returns_none() {
        assert_eq!(FrameKind::from_u8(0x00), None);
        assert_eq!(FrameKind::from_u8(0xFF), None);
        assert_eq!(FrameKind::from_u8(0x50), None);
    }
}
