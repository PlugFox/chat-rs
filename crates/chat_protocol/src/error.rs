//! Codec and protocol error types.

use thiserror::Error;

/// Errors that can occur during binary encoding/decoding.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CodecError {
    /// Not enough bytes in the input buffer.
    #[error("truncated input: need {needed} bytes, have {available}")]
    Truncated {
        /// Bytes required to complete the decode.
        needed: usize,
        /// Bytes actually available.
        available: usize,
    },

    /// Unknown frame kind byte.
    #[error("unknown frame kind: 0x{0:02x}")]
    UnknownFrameKind(u8),

    /// Unknown enum discriminant.
    #[error("unknown {type_name} discriminant: {value}")]
    UnknownDiscriminant {
        /// Name of the enum type.
        type_name: &'static str,
        /// The invalid discriminant value.
        value: u32,
    },

    /// Timestamp outside valid range (0 ≤ v < 2⁴¹).
    #[error("timestamp out of range: {0}")]
    TimestampOutOfRange(i64),

    /// Frame exceeds maximum allowed size.
    #[error("frame too large: {size} bytes (max {max})")]
    FrameTooLarge {
        /// Actual frame size.
        size: usize,
        /// Maximum allowed size.
        max: usize,
    },

    /// String is not valid UTF-8.
    #[error("invalid UTF-8 at offset {offset}")]
    InvalidUtf8 {
        /// Byte offset where the invalid sequence starts.
        offset: usize,
    },

    /// Structurally invalid data (e.g. trailing bytes after a sub-message).
    #[error("invalid data: {reason}")]
    InvalidData {
        /// Human-readable description of the problem.
        reason: &'static str,
    },

    /// String length exceeds a protocol-defined limit.
    #[error("{field} too long: {len} bytes (max {max})")]
    StringTooLong {
        /// Name of the field.
        field: &'static str,
        /// Actual length in bytes.
        len: usize,
        /// Maximum allowed length.
        max: usize,
    },
}
