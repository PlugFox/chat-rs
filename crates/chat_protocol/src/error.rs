//! Error codes for the chat protocol.
//!
//! Each error has:
//! - `code: u16` — numeric code with category ranges
//! - `slug` — stable snake_case identifier for client-side logic
//! - `message` — developer-facing description (not for end users)

use thiserror::Error;

/// Protocol-level error codes.
///
/// Ranges:
/// - `1xxx` — Authentication & authorization
/// - `2xxx` — Chats
/// - `3xxx` — Messages
/// - `4xxx` — Media
/// - `5xxx` — Server internal
/// - `9xxx` — Protocol
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    // 1xxx — Authentication & authorization
    Unauthorized = 1000,
    TokenExpired = 1001,
    Forbidden = 1002,
    SessionRevoked = 1003,
    UnsupportedVersion = 1004,

    // 2xxx — Chats
    ChatNotFound = 2000,
    ChatAlreadyExists = 2001,
    NotChatMember = 2002,
    ChatFull = 2003,

    // 3xxx — Messages
    MessageNotFound = 3000,
    MessageTooLarge = 3001,
    ExtraTooLarge = 3002,
    RateLimited = 3003,
    ContentFiltered = 3004,

    // 4xxx — Media
    FileTooLarge = 4000,
    UnsupportedMediaType = 4001,
    UploadFailed = 4002,

    // 5xxx — Server
    InternalError = 5000,
    ServiceUnavailable = 5001,
    DatabaseError = 5002,

    // 9xxx — Protocol
    MalformedFrame = 9000,
    UnknownCommand = 9001,
    FrameTooLarge = 9002,
}

impl ErrorCode {
    /// Stable slug for client-side logic. Guaranteed to not change between versions.
    pub fn slug(self) -> &'static str {
        match self {
            Self::Unauthorized => "unauthorized",
            Self::TokenExpired => "token_expired",
            Self::Forbidden => "forbidden",
            Self::SessionRevoked => "session_revoked",
            Self::UnsupportedVersion => "unsupported_version",
            Self::ChatNotFound => "chat_not_found",
            Self::ChatAlreadyExists => "chat_already_exists",
            Self::NotChatMember => "not_chat_member",
            Self::ChatFull => "chat_full",
            Self::MessageNotFound => "message_not_found",
            Self::MessageTooLarge => "message_too_large",
            Self::ExtraTooLarge => "extra_too_large",
            Self::RateLimited => "rate_limited",
            Self::ContentFiltered => "content_filtered",
            Self::FileTooLarge => "file_too_large",
            Self::UnsupportedMediaType => "unsupported_media_type",
            Self::UploadFailed => "upload_failed",
            Self::InternalError => "internal_error",
            Self::ServiceUnavailable => "service_unavailable",
            Self::DatabaseError => "database_error",
            Self::MalformedFrame => "malformed_frame",
            Self::UnknownCommand => "unknown_command",
            Self::FrameTooLarge => "frame_too_large",
        }
    }

    /// Permanent errors — do not retry in outbox.
    pub fn is_permanent(self) -> bool {
        matches!(
            self,
            Self::Forbidden
                | Self::ChatNotFound
                | Self::NotChatMember
                | Self::MessageTooLarge
                | Self::ExtraTooLarge
                | Self::ContentFiltered
                | Self::UnsupportedMediaType
        )
    }

    /// Transient errors — retry with backoff.
    pub fn is_transient(self) -> bool {
        matches!(
            self,
            Self::InternalError | Self::ServiceUnavailable | Self::DatabaseError | Self::RateLimited
        )
    }
}

/// Typed protocol error for cross-crate use.
#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("malformed frame: {0}")]
    MalformedFrame(String),

    #[error("frame too large: {size} bytes (max {max})")]
    FrameTooLarge { size: usize, max: usize },

    #[error("unknown frame kind: 0x{0:02X}")]
    UnknownFrameKind(u8),

    #[error("codec error: {0}")]
    Codec(String),

    #[error("unexpected end of input: need {needed} bytes, have {available}")]
    UnexpectedEof { needed: usize, available: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permanent_errors_are_not_transient() {
        let permanent = [
            ErrorCode::Forbidden,
            ErrorCode::ChatNotFound,
            ErrorCode::MessageTooLarge,
        ];
        for code in permanent {
            assert!(code.is_permanent(), "{code:?} should be permanent");
            assert!(!code.is_transient(), "{code:?} should not be transient");
        }
    }

    #[test]
    fn transient_errors_are_not_permanent() {
        let transient = [
            ErrorCode::InternalError,
            ErrorCode::ServiceUnavailable,
            ErrorCode::RateLimited,
        ];
        for code in transient {
            assert!(code.is_transient(), "{code:?} should be transient");
            assert!(!code.is_permanent(), "{code:?} should not be permanent");
        }
    }

    #[test]
    fn slug_is_snake_case() {
        let codes = [ErrorCode::Unauthorized, ErrorCode::ChatNotFound, ErrorCode::RateLimited];
        for code in codes {
            let slug = code.slug();
            assert!(
                slug.chars().all(|c| c.is_ascii_lowercase() || c == '_'),
                "slug {slug:?} is not snake_case"
            );
        }
    }
}
