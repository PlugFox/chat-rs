//! Frame encoding helpers and session utilities.
//!
//! The protocol codec's `encode_frame` is a no-op for `Ack` payloads (the server
//! must build the raw bytes itself). This module provides helpers for that.

use bytes::BufMut;
use chat_protocol::FRAME_HEADER_SIZE;
use chat_protocol::types::{ErrorCode, ErrorPayload, Frame, FramePayload};

/// Encode an Ack frame with no payload (e.g. Subscribe ack).
pub fn encode_ack_empty(seq: u32) -> Vec<u8> {
    let mut buf = Vec::with_capacity(FRAME_HEADER_SIZE);
    buf.put_u8(0x30); // FrameKind::Ack
    buf.put_u32_le(seq);
    buf.put_u32_le(0); // event_seq = 0 for responses
    buf
}

/// Encode an Ack frame carrying a single `u32` message ID.
pub fn encode_ack_message_id(seq: u32, message_id: u32) -> Vec<u8> {
    let mut buf = Vec::with_capacity(FRAME_HEADER_SIZE + 4);
    buf.put_u8(0x30); // FrameKind::Ack
    buf.put_u32_le(seq);
    buf.put_u32_le(0); // event_seq = 0 for responses
    buf.put_u32_le(message_id);
    buf
}

/// Encode an Error response frame.
pub fn encode_error_frame(seq: u32, code: ErrorCode, message: &str) -> Vec<u8> {
    let frame = Frame {
        seq,
        event_seq: 0,
        payload: FramePayload::Error(ErrorPayload {
            code,
            message: message.to_owned(),
            retry_after_ms: 0,
            extra: None,
        }),
    };
    let mut buf = Vec::with_capacity(64);
    // encode_frame always succeeds for Error payloads.
    chat_protocol::codec::encode_frame(&mut buf, &frame).expect("Error frame encoding must not fail");
    buf
}

/// Encode a server-push event frame with a per-recipient `event_seq`.
///
/// Takes an already-encoded payload (everything after the 9-byte header)
/// and prepends a header with the given `kind`, `seq=0`, and `event_seq`.
pub fn encode_event_frame(kind: u8, event_seq: u32, payload: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(FRAME_HEADER_SIZE + payload.len());
    buf.put_u8(kind);
    buf.put_u32_le(0); // seq = 0 for server pushes
    buf.put_u32_le(event_seq);
    buf.put_slice(payload);
    buf
}
