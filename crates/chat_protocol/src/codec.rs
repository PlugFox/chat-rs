//! Binary codec for encoding/decoding protocol frames.
//!
//! All values are little-endian (native for ARM/x86).
//!
//! Frame header (6 bytes):
//! ```text
//! ┌──────────┬──────────┬───────────┐
//! │ ver: u8  │ kind: u8 │  seq: u32 │
//! └──────────┴──────────┴───────────┘
//! ```

use bytes::{Buf, BufMut};

use crate::PROTOCOL_VERSION;
use crate::error::ProtocolError;
use crate::frames::FrameKind;

/// Parsed frame header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameHeader {
    pub version: u8,
    pub kind: FrameKind,
    pub seq: u32,
}

/// Encode a frame header into a buffer.
pub fn encode_header(buf: &mut impl BufMut, kind: FrameKind, seq: u32) {
    buf.put_u8(PROTOCOL_VERSION);
    buf.put_u8(kind as u8);
    buf.put_u32_le(seq);
}

/// Decode a frame header from a buffer.
pub fn decode_header(buf: &mut impl Buf) -> Result<FrameHeader, ProtocolError> {
    if buf.remaining() < 6 {
        return Err(ProtocolError::UnexpectedEof {
            needed: 6,
            available: buf.remaining(),
        });
    }

    let version = buf.get_u8();
    let kind_byte = buf.get_u8();
    let seq = buf.get_u32_le();

    let kind = FrameKind::from_u8(kind_byte).ok_or(ProtocolError::UnknownFrameKind(kind_byte))?;

    Ok(FrameHeader { version, kind, seq })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_roundtrip() {
        let mut buf = Vec::new();
        encode_header(&mut buf, FrameKind::SendMessage, 42);

        let header = decode_header(&mut buf.as_slice()).unwrap();
        assert_eq!(header.version, PROTOCOL_VERSION);
        assert_eq!(header.kind, FrameKind::SendMessage);
        assert_eq!(header.seq, 42);
    }

    #[test]
    fn decode_too_short() {
        let buf = [0u8; 3];
        let err = decode_header(&mut buf.as_slice()).unwrap_err();
        assert!(matches!(err, ProtocolError::UnexpectedEof { .. }));
    }

    #[test]
    fn decode_unknown_kind() {
        let mut buf = Vec::new();
        buf.put_u8(PROTOCOL_VERSION);
        buf.put_u8(0xFF);
        buf.put_u32_le(1);

        let err = decode_header(&mut buf.as_slice()).unwrap_err();
        assert!(matches!(err, ProtocolError::UnknownFrameKind(0xFF)));
    }
}
