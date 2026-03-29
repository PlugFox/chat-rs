//! Message, MessageBatch, and RichContent encode/decode.

use bytes::{Buf, BufMut};

use crate::error::CodecError;
use crate::types::{Message, MessageBatch, MessageFlags, MessageKind, RichSpan, RichStyle};

use super::wire::*;

/// Encode a `MessageBatch` (has_more: u8, count: u32 + messages).
pub fn encode_message_batch(buf: &mut impl BufMut, batch: &MessageBatch) -> Result<(), CodecError> {
    write_u8(buf, batch.has_more as u8);
    write_u32(buf, batch.messages.len() as u32);
    for msg in &batch.messages {
        encode_message(buf, msg)?;
    }
    Ok(())
}

/// Decode a `MessageBatch` from the buffer.
pub fn decode_message_batch(buf: &mut impl Buf) -> Result<MessageBatch, CodecError> {
    let has_more = read_u8(buf)? != 0;
    let count = read_u32(buf)? as usize;
    let mut messages = Vec::with_capacity(count.min(1024));
    for _ in 0..count {
        messages.push(decode_message(buf)?);
    }
    Ok(MessageBatch { messages, has_more })
}

/// Encode a single `Message`.
///
/// Layout (fixed-size fields, then variable-size):
/// - id(4) + chat_id(4) + sender_id(4) + created_at(8) + updated_at(8) +
///   kind(1) + flags(2) = 31 bytes fixed
/// - reply_to: u8 flag [+ optional u32] (1 or 5 bytes)
/// - content: u32 length + UTF-8 bytes
/// - rich_content: u32 length + encoded spans (0 = absent)
/// - extra: optional string (u32 length + UTF-8 bytes)
pub fn encode_message(buf: &mut impl BufMut, msg: &Message) -> Result<(), CodecError> {
    write_u32(buf, msg.id);
    write_u32(buf, msg.chat_id);
    write_u32(buf, msg.sender_id);
    write_timestamp(buf, msg.created_at)?;
    write_timestamp(buf, msg.updated_at)?;
    write_u8(buf, msg.kind as u8);
    write_u16(buf, msg.flags.bits());
    write_option_u32(buf, msg.reply_to_id);

    // Content (length-prefixed string)
    write_string(buf, &msg.content);

    // Rich content blob — pre-compute size to avoid intermediate allocation
    match &msg.rich_content {
        Some(spans) => {
            let rich_size = rich_content_size(spans);
            write_u32(buf, rich_size as u32);
            encode_rich_content(buf, spans);
        }
        None => write_u32(buf, 0),
    }

    // Extra JSON
    write_optional_string(buf, msg.extra.as_deref());

    Ok(())
}

/// Decode a single `Message` from the buffer.
pub fn decode_message(buf: &mut impl Buf) -> Result<Message, CodecError> {
    // Fixed header
    let id = read_u32(buf)?;
    let chat_id = read_u32(buf)?;
    let sender_id = read_u32(buf)?;
    let created_at = read_timestamp(buf)?;
    let updated_at = read_timestamp(buf)?;

    let kind_byte = read_u8(buf)?;
    let kind = MessageKind::from_u8(kind_byte).ok_or(CodecError::UnknownDiscriminant {
        type_name: "MessageKind",
        value: kind_byte as u32,
    })?;

    // from_bits_truncate: intentionally drops unknown flag bits for forward compatibility.
    // A newer server may set flags the client doesn't know about yet.
    let flags = MessageFlags::from_bits_truncate(read_u16(buf)?);
    let reply_to_id = read_option_u32(buf)?;

    // Content
    let content = read_string(buf)?;

    // Rich content blob
    let rich_len = read_u32(buf)? as usize;
    let rich_content = if rich_len == 0 {
        None
    } else {
        ensure_remaining(buf, rich_len)?;
        let mut rich_buf = buf.copy_to_bytes(rich_len);
        let spans = decode_rich_content(&mut rich_buf)?;
        if rich_buf.has_remaining() {
            return Err(CodecError::InvalidData {
                reason: "trailing bytes in rich content",
            });
        }
        Some(spans)
    };

    // Extra JSON
    let extra = read_optional_string(buf)?;

    Ok(Message {
        id,
        chat_id,
        sender_id,
        created_at,
        updated_at,
        kind,
        flags,
        reply_to_id,
        content,
        rich_content,
        extra,
    })
}

/// Compute the wire size of rich content without encoding.
///
/// Each span: 10 bytes (start: u32, end: u32, style: u16) + 4 bytes (meta_len: u32) + meta bytes.
/// Plus 2 bytes for count: u16.
fn rich_content_size(spans: &[RichSpan]) -> usize {
    let mut size = 2; // count: u16
    for span in spans {
        size += 10; // start + end + style
        size += 4; // meta_len: u32
        if let Some(meta) = &span.meta {
            size += meta.len();
        }
    }
    size
}

/// Encode rich content spans (count: u16 + spans).
pub fn encode_rich_content(buf: &mut impl BufMut, spans: &[RichSpan]) {
    write_u16(buf, spans.len() as u16);
    for span in spans {
        encode_rich_span(buf, span);
    }
}

/// Decode rich content spans.
pub fn decode_rich_content(buf: &mut impl Buf) -> Result<Vec<RichSpan>, CodecError> {
    let count = read_u16(buf)? as usize;
    let mut spans = Vec::with_capacity(count.min(256));
    for _ in 0..count {
        spans.push(decode_rich_span(buf)?);
    }
    Ok(spans)
}

/// Encode a single rich span (10 bytes fixed + meta_len: u32 + optional JSON).
fn encode_rich_span(buf: &mut impl BufMut, span: &RichSpan) {
    write_u32(buf, span.start);
    write_u32(buf, span.end);
    write_u16(buf, span.style.bits());
    write_optional_string(buf, span.meta.as_deref());
}

/// Decode a single rich span.
fn decode_rich_span(buf: &mut impl Buf) -> Result<RichSpan, CodecError> {
    let start = read_u32(buf)?;
    let end = read_u32(buf)?;
    // from_bits_truncate: forward-compatible — unknown style bits are silently dropped.
    let style = RichStyle::from_bits_truncate(read_u16(buf)?);
    let meta = read_optional_string(buf)?;

    Ok(RichSpan {
        start,
        end,
        style,
        meta,
    })
}
