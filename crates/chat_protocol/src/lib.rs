//! # chat_protocol
//!
//! Shared types, codec, and error codes — wire contract between clients and server.
//!
//! This crate defines the binary protocol used by all clients (Dart, TypeScript, Rust)
//! to communicate with the chat server over WebSocket. It has zero runtime dependencies
//! beyond `serde`, `bytes`, `thiserror`, `bitflags`, and `uuid`.
//!
//! ## Wire Format
//!
//! All values are little-endian. Every WS binary frame starts with a 5-byte header:
//!
//! ```text
//! ┌──────────┬───────────┬──────────────────┐
//! │ kind: u8 │  seq: u32 │ payload: bytes   │
//! └──────────┴───────────┴──────────────────┘
//! ```
//!
//! See `docs/codec.md` for the complete wire format specification.

pub mod codec;
pub mod error;
pub mod types;

/// Protocol version. Incremented on breaking wire-format changes.
pub const PROTOCOL_VERSION: u8 = 1;

/// Wire frame header size: kind(1) + seq(4) + event_seq(4) = 9 bytes.
pub const FRAME_HEADER_SIZE: usize = 9;

/// Minimum valid timestamp (1970-01-01 00:00:00 UTC).
pub const MIN_TIMESTAMP: i64 = 0;

/// Maximum valid timestamp ((1 << 41) - 1 ≈ year 71,700).
/// Fast check: `value >> 41 != 0` → reject.
/// Catches milliseconds-instead-of-seconds bugs and is JS Number-safe.
pub const MAX_TIMESTAMP: i64 = (1_i64 << 41) - 1;

/// Bitmask for detecting event_seq overflow.
///
/// When `event_seq & EVENT_SEQ_OVERFLOW_MASK != 0` (top 2 bits set),
/// the server should send `DisconnectCode::EventSeqOverflow` and close
/// so the client reconnects with a fresh counter.
pub const EVENT_SEQ_OVERFLOW_MASK: u32 = 0xC000_0000;
