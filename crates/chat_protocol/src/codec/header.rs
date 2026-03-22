//! Frame header encode/decode (9 bytes: kind + seq + event_seq).

use bytes::{Buf, BufMut};

use crate::FRAME_HEADER_SIZE;
use crate::error::CodecError;
use crate::types::FrameHeader;
use crate::types::FrameKind;

/// Encode a frame header (9 bytes: kind: u8 + seq: u32 LE + event_seq: u32 LE).
pub fn encode_header(buf: &mut impl BufMut, header: &FrameHeader) {
    buf.put_u8(header.kind as u8);
    buf.put_u32_le(header.seq);
    buf.put_u32_le(header.event_seq);
}

/// Decode a frame header from the buffer.
///
/// Returns `CodecError::Truncated` if fewer than 9 bytes remain.
/// Returns `CodecError::UnknownFrameKind` if the kind byte is unrecognized.
pub fn decode_header(buf: &mut impl Buf) -> Result<FrameHeader, CodecError> {
    let available = buf.remaining();
    if available < FRAME_HEADER_SIZE {
        return Err(CodecError::Truncated {
            needed: FRAME_HEADER_SIZE,
            available,
        });
    }

    let kind_byte = buf.get_u8();
    let kind = FrameKind::from_u8(kind_byte).ok_or(CodecError::UnknownFrameKind(kind_byte))?;
    let seq = buf.get_u32_le();
    let event_seq = buf.get_u32_le();

    Ok(FrameHeader { kind, seq, event_seq })
}
