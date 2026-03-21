//! # chat_protocol
//!
//! Shared protocol types, codec, and error codes for the Chat SDK.
//! This crate is the single source of truth — used by both `chat_client` and `chat_server`.
//!
//! ## Modules
//!
//! - [`frames`] — Frame types and `FrameKind` enum for the WebSocket binary protocol
//! - [`codec`] — Serialization/deserialization of frames to/from bytes
//! - [`error`] — `ErrorCode` enum with slugs, transient/permanent classification
//! - [`types`] — Shared domain types (permissions, roles, message kinds, etc.)

pub mod codec;
pub mod error;
pub mod frames;
pub mod types;

/// Protocol version. Incremented on breaking wire-format changes.
pub const PROTOCOL_VERSION: u8 = 1;

/// Wire frame header size: ver(1) + kind(1) + seq(4) = 6 bytes.
pub const FRAME_HEADER_SIZE: usize = 6;
