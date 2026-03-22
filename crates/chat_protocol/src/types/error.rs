//! Error codes and disconnect codes.

/// Application-level error code sent in Error frames.
///
/// Slugs are stable identifiers that never change between protocol versions.
/// Client code should match on slugs, not numeric codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ErrorCode {
    // Authentication & authorization (1000–1999)
    /// Invalid token.
    Unauthorized = 1000,
    /// Token expired.
    TokenExpired = 1001,
    /// No permission.
    Forbidden = 1002,
    /// Session revoked.
    SessionRevoked = 1003,
    /// Protocol version not supported.
    UnsupportedVersion = 1004,

    // Chats (2000–2999)
    /// Chat doesn't exist.
    ChatNotFound = 2000,
    /// Direct chat already exists between these users.
    ChatAlreadyExists = 2001,
    /// User is not a member of this chat.
    NotChatMember = 2002,
    /// Member limit reached for this chat.
    ChatFull = 2003,

    // Messages (3000–3999)
    /// Message doesn't exist.
    MessageNotFound = 3000,
    /// Content exceeds max_message_size limit.
    MessageTooLarge = 3001,
    /// Extra JSON exceeds max_extra_size limit.
    ExtraTooLarge = 3002,
    /// Too many messages — retry after `retry_after_ms`.
    RateLimited = 3003,
    /// Content interceptor/filter rejected the message.
    ContentFiltered = 3004,

    // Media (4000–4999)
    /// File exceeds upload size limit.
    FileTooLarge = 4000,
    /// File type not allowed.
    UnsupportedMediaType = 4001,
    /// Upload processing error.
    UploadFailed = 4002,

    // Server internal (5000–5999)
    /// Server internal error.
    InternalError = 5000,
    /// Service temporarily unavailable.
    ServiceUnavailable = 5001,
    /// Database error.
    DatabaseError = 5002,

    // Protocol (9000–9999)
    /// Bad frame format / cannot decode.
    MalformedFrame = 9000,
    /// Unknown frame kind byte.
    UnknownCommand = 9001,
    /// Frame exceeds max_frame_size.
    FrameTooLarge = 9002,
}

impl ErrorCode {
    /// Stable snake_case identifier. Client code should match on this, not the numeric code.
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

    /// Convert from wire u16. Returns `None` for unknown codes.
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            1000 => Some(Self::Unauthorized),
            1001 => Some(Self::TokenExpired),
            1002 => Some(Self::Forbidden),
            1003 => Some(Self::SessionRevoked),
            1004 => Some(Self::UnsupportedVersion),
            2000 => Some(Self::ChatNotFound),
            2001 => Some(Self::ChatAlreadyExists),
            2002 => Some(Self::NotChatMember),
            2003 => Some(Self::ChatFull),
            3000 => Some(Self::MessageNotFound),
            3001 => Some(Self::MessageTooLarge),
            3002 => Some(Self::ExtraTooLarge),
            3003 => Some(Self::RateLimited),
            3004 => Some(Self::ContentFiltered),
            4000 => Some(Self::FileTooLarge),
            4001 => Some(Self::UnsupportedMediaType),
            4002 => Some(Self::UploadFailed),
            5000 => Some(Self::InternalError),
            5001 => Some(Self::ServiceUnavailable),
            5002 => Some(Self::DatabaseError),
            9000 => Some(Self::MalformedFrame),
            9001 => Some(Self::UnknownCommand),
            9002 => Some(Self::FrameTooLarge),
            _ => None,
        }
    }

    /// Whether this error is permanent (do not retry).
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

    /// Whether this error is transient (retry with exponential backoff).
    pub fn is_transient(self) -> bool {
        matches!(
            self,
            Self::InternalError | Self::ServiceUnavailable | Self::DatabaseError | Self::RateLimited
        )
    }

    /// Returns all valid error codes (for testing/iteration).
    pub fn all() -> &'static [ErrorCode] {
        &[
            Self::Unauthorized,
            Self::TokenExpired,
            Self::Forbidden,
            Self::SessionRevoked,
            Self::UnsupportedVersion,
            Self::ChatNotFound,
            Self::ChatAlreadyExists,
            Self::NotChatMember,
            Self::ChatFull,
            Self::MessageNotFound,
            Self::MessageTooLarge,
            Self::ExtraTooLarge,
            Self::RateLimited,
            Self::ContentFiltered,
            Self::FileTooLarge,
            Self::UnsupportedMediaType,
            Self::UploadFailed,
            Self::InternalError,
            Self::ServiceUnavailable,
            Self::DatabaseError,
            Self::MalformedFrame,
            Self::UnknownCommand,
            Self::FrameTooLarge,
        ]
    }
}

/// Error frame payload (server → client).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorPayload {
    /// Numeric error code.
    pub code: ErrorCode,
    /// Developer-facing error description (not for end users).
    pub message: String,
    /// Retry delay in milliseconds (only set for `rate_limited`, 0 otherwise).
    pub retry_after_ms: u32,
    /// Server-provided diagnostic JSON details. `None` = absent.
    pub extra: Option<String>,
}

/// WebSocket disconnect / close code.
///
/// Determines whether the client should attempt reconnection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum DisconnectCode {
    // Server non-terminal (3000–3499) — reconnect: yes
    /// Graceful server restart.
    ServerShutdown = 3000,
    /// Token expired mid-session.
    SessionExpired = 3001,
    /// Same device_id connected from another location.
    DuplicateSession = 3002,
    /// Unrecoverable internal server error.
    ServerError = 3003,
    /// Client send buffer exceeded capacity.
    BufferOverflow = 3004,
    /// Too many requests on this connection, backoff.
    RateLimited = 3005,

    // Server terminal (3500–3999) — reconnect: no
    /// Token is malformed or has invalid signature.
    TokenInvalid = 3500,
    /// User is banned.
    Banned = 3501,
    /// Protocol version not supported by server.
    UnsupportedVersion = 3502,
    /// Max connections per IP/user exceeded.
    ConnectionLimit = 3503,
}

impl DisconnectCode {
    /// Whether the client should attempt reconnection.
    ///
    /// Range-based logic:
    /// - `0..1000` — internal/transport → reconnect
    /// - `3000..3500` — server non-terminal → reconnect
    /// - `3500..4000` — server terminal → no reconnect
    /// - `4000..4500` — custom non-terminal → reconnect
    /// - `4500..5000` — custom terminal → no reconnect
    pub fn should_reconnect(self) -> bool {
        let code = self as u16;
        matches!(code, 0..1000 | 3000..3500 | 4000..4500)
    }

    /// Convert from wire u16. Returns `None` for unknown codes.
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            3000 => Some(Self::ServerShutdown),
            3001 => Some(Self::SessionExpired),
            3002 => Some(Self::DuplicateSession),
            3003 => Some(Self::ServerError),
            3004 => Some(Self::BufferOverflow),
            3005 => Some(Self::RateLimited),
            3500 => Some(Self::TokenInvalid),
            3501 => Some(Self::Banned),
            3502 => Some(Self::UnsupportedVersion),
            3503 => Some(Self::ConnectionLimit),
            _ => None,
        }
    }

    /// Returns all valid disconnect codes (for testing/iteration).
    pub fn all() -> &'static [DisconnectCode] {
        &[
            Self::ServerShutdown,
            Self::SessionExpired,
            Self::DuplicateSession,
            Self::ServerError,
            Self::BufferOverflow,
            Self::RateLimited,
            Self::TokenInvalid,
            Self::Banned,
            Self::UnsupportedVersion,
            Self::ConnectionLimit,
        ]
    }
}
