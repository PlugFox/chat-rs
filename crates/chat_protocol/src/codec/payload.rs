//! Per-frame payload encode/decode.

use bytes::{Buf, BufMut};

use crate::error::CodecError;
use crate::types::*;

use super::wire::*;

// ---------------------------------------------------------------------------
// Hello
// ---------------------------------------------------------------------------

/// Encode a `HelloPayload`.
pub fn encode_hello(buf: &mut impl BufMut, p: &HelloPayload) -> Result<(), CodecError> {
    write_u8(buf, p.protocol_version);
    write_string(buf, &p.sdk_version);
    write_string(buf, &p.platform);
    write_string(buf, &p.token);
    write_uuid(buf, &p.device_id);
    Ok(())
}

/// Decode a `HelloPayload`.
pub fn decode_hello(buf: &mut impl Buf) -> Result<HelloPayload, CodecError> {
    let protocol_version = read_u8(buf)?;
    let sdk_version = read_string(buf)?;
    let platform = read_string(buf)?;
    let token = read_string(buf)?;
    let device_id = read_uuid(buf)?;

    Ok(HelloPayload {
        protocol_version,
        sdk_version,
        platform,
        token,
        device_id,
    })
}

// ---------------------------------------------------------------------------
// Welcome
// ---------------------------------------------------------------------------

/// Encode a `WelcomePayload`.
pub fn encode_welcome(buf: &mut impl BufMut, p: &WelcomePayload) -> Result<(), CodecError> {
    write_u32(buf, p.session_id);
    write_timestamp(buf, p.server_time)?;
    write_u32(buf, p.user_id);
    encode_server_limits(buf, &p.limits);
    write_u32(buf, p.capabilities.bits());
    Ok(())
}

/// Decode a `WelcomePayload`.
pub fn decode_welcome(buf: &mut impl Buf) -> Result<WelcomePayload, CodecError> {
    let session_id = read_u32(buf)?;
    let server_time = read_timestamp(buf)?;
    let user_id = read_u32(buf)?;
    let limits = decode_server_limits(buf)?;
    let capabilities = ServerCapabilities::from_bits_truncate(read_u32(buf)?);

    Ok(WelcomePayload {
        session_id,
        server_time,
        user_id,
        limits,
        capabilities,
    })
}

fn encode_server_limits(buf: &mut impl BufMut, l: &ServerLimits) {
    write_u32(buf, l.ping_interval_ms);
    write_u32(buf, l.ping_timeout_ms);
    write_u32(buf, l.max_message_size);
    write_u32(buf, l.max_extra_size);
    write_u32(buf, l.max_frame_size);
    write_u16(buf, l.messages_per_sec);
    write_u16(buf, l.connections_per_ip);
}

fn decode_server_limits(buf: &mut impl Buf) -> Result<ServerLimits, CodecError> {
    Ok(ServerLimits {
        ping_interval_ms: read_u32(buf)?,
        ping_timeout_ms: read_u32(buf)?,
        max_message_size: read_u32(buf)?,
        max_extra_size: read_u32(buf)?,
        max_frame_size: read_u32(buf)?,
        messages_per_sec: read_u16(buf)?,
        connections_per_ip: read_u16(buf)?,
    })
}

// ---------------------------------------------------------------------------
// SendMessage
// ---------------------------------------------------------------------------

/// Encode a `SendMessagePayload`.
pub fn encode_send_message(buf: &mut impl BufMut, p: &SendMessagePayload) {
    write_u32(buf, p.chat_id);
    write_u8(buf, p.kind as u8);
    write_uuid(buf, &p.idempotency_key);
    write_string(buf, &p.content);
    write_optional_bytes(buf, p.rich_content.as_deref());
    write_optional_string(buf, p.extra.as_deref());
}

/// Decode a `SendMessagePayload`.
pub fn decode_send_message(buf: &mut impl Buf) -> Result<SendMessagePayload, CodecError> {
    let chat_id = read_u32(buf)?;
    let kind_byte = read_u8(buf)?;
    let kind = MessageKind::from_u8(kind_byte).ok_or(CodecError::UnknownDiscriminant {
        type_name: "MessageKind",
        value: kind_byte as u32,
    })?;
    let idempotency_key = read_uuid(buf)?;
    let content = read_string(buf)?;
    let rich_content = read_optional_bytes(buf)?;
    let extra = read_optional_string(buf)?;

    Ok(SendMessagePayload {
        chat_id,
        kind,
        idempotency_key,
        content,
        rich_content,
        extra,
    })
}

// ---------------------------------------------------------------------------
// EditMessage
// ---------------------------------------------------------------------------

/// Encode an `EditMessagePayload`.
pub fn encode_edit_message(buf: &mut impl BufMut, p: &EditMessagePayload) {
    write_u32(buf, p.chat_id);
    write_u32(buf, p.message_id);
    write_uuid(buf, &p.idempotency_key);
    write_string(buf, &p.content);
    write_optional_bytes(buf, p.rich_content.as_deref());
    write_optional_string(buf, p.extra.as_deref());
}

/// Decode an `EditMessagePayload`.
pub fn decode_edit_message(buf: &mut impl Buf) -> Result<EditMessagePayload, CodecError> {
    let chat_id = read_u32(buf)?;
    let message_id = read_u32(buf)?;
    let idempotency_key = read_uuid(buf)?;
    let content = read_string(buf)?;
    let rich_content = read_optional_bytes(buf)?;
    let extra = read_optional_string(buf)?;

    Ok(EditMessagePayload {
        chat_id,
        message_id,
        idempotency_key,
        content,
        rich_content,
        extra,
    })
}

// ---------------------------------------------------------------------------
// DeleteMessage
// ---------------------------------------------------------------------------

/// Encode a `DeleteMessagePayload`.
pub fn encode_delete_message(buf: &mut impl BufMut, p: &DeleteMessagePayload) {
    write_u32(buf, p.chat_id);
    write_u32(buf, p.message_id);
    write_uuid(buf, &p.idempotency_key);
}

/// Decode a `DeleteMessagePayload`.
pub fn decode_delete_message(buf: &mut impl Buf) -> Result<DeleteMessagePayload, CodecError> {
    let chat_id = read_u32(buf)?;
    let message_id = read_u32(buf)?;
    let idempotency_key = read_uuid(buf)?;

    Ok(DeleteMessagePayload {
        chat_id,
        message_id,
        idempotency_key,
    })
}

// ---------------------------------------------------------------------------
// ReadReceipt
// ---------------------------------------------------------------------------

/// Encode a `ReadReceiptPayload`.
pub fn encode_read_receipt(buf: &mut impl BufMut, p: &ReadReceiptPayload) {
    write_u32(buf, p.chat_id);
    write_u32(buf, p.message_id);
}

/// Decode a `ReadReceiptPayload`.
pub fn decode_read_receipt(buf: &mut impl Buf) -> Result<ReadReceiptPayload, CodecError> {
    Ok(ReadReceiptPayload {
        chat_id: read_u32(buf)?,
        message_id: read_u32(buf)?,
    })
}

// ---------------------------------------------------------------------------
// Typing
// ---------------------------------------------------------------------------

/// Encode a `TypingPayload`.
pub fn encode_typing(buf: &mut impl BufMut, p: &TypingPayload) {
    write_u32(buf, p.chat_id);
}

/// Decode a `TypingPayload`.
pub fn decode_typing(buf: &mut impl Buf) -> Result<TypingPayload, CodecError> {
    Ok(TypingPayload {
        chat_id: read_u32(buf)?,
    })
}

// ---------------------------------------------------------------------------
// GetPresence
// ---------------------------------------------------------------------------

/// Encode a `GetPresencePayload`.
pub fn encode_get_presence(buf: &mut impl BufMut, p: &GetPresencePayload) {
    write_u16(buf, p.user_ids.len() as u16);
    for &id in &p.user_ids {
        write_u32(buf, id);
    }
}

/// Decode a `GetPresencePayload`.
pub fn decode_get_presence(buf: &mut impl Buf) -> Result<GetPresencePayload, CodecError> {
    let count = read_u16(buf)? as usize;
    let mut user_ids = Vec::with_capacity(count.min(256));
    for _ in 0..count {
        user_ids.push(read_u32(buf)?);
    }
    Ok(GetPresencePayload { user_ids })
}

// ---------------------------------------------------------------------------
// PresenceResult
// ---------------------------------------------------------------------------

/// Encode a list of `PresenceEntry` values.
pub fn encode_presence_result(buf: &mut impl BufMut, entries: &[PresenceEntry]) -> Result<(), CodecError> {
    write_u16(buf, entries.len() as u16);
    for e in entries {
        write_u32(buf, e.user_id);
        write_u8(buf, e.status as u8);
        write_timestamp(buf, e.last_seen)?;
    }
    Ok(())
}

/// Decode a list of `PresenceEntry` values.
pub fn decode_presence_result(buf: &mut impl Buf) -> Result<Vec<PresenceEntry>, CodecError> {
    let count = read_u16(buf)? as usize;
    let mut entries = Vec::with_capacity(count.min(256));
    for _ in 0..count {
        let user_id = read_u32(buf)?;
        let status_byte = read_u8(buf)?;
        let status = PresenceStatus::from_u8(status_byte).ok_or(CodecError::UnknownDiscriminant {
            type_name: "PresenceStatus",
            value: status_byte as u32,
        })?;
        let last_seen = read_timestamp(buf)?;
        entries.push(PresenceEntry {
            user_id,
            status,
            last_seen,
        });
    }
    Ok(entries)
}

// ---------------------------------------------------------------------------
// LoadChats
// ---------------------------------------------------------------------------

/// Encode a `LoadChatsPayload`.
pub fn encode_load_chats(buf: &mut impl BufMut, p: &LoadChatsPayload) -> Result<(), CodecError> {
    match p {
        LoadChatsPayload::FirstPage { limit } => {
            write_u8(buf, 0); // mode
            write_u16(buf, *limit);
        }
        LoadChatsPayload::After { cursor_ts, limit } => {
            write_u8(buf, 1); // mode
            write_timestamp(buf, *cursor_ts)?;
            write_u16(buf, *limit);
        }
    }
    Ok(())
}

/// Decode a `LoadChatsPayload`.
pub fn decode_load_chats(buf: &mut impl Buf) -> Result<LoadChatsPayload, CodecError> {
    let mode = read_u8(buf)?;
    match mode {
        0 => {
            let limit = read_u16(buf)?;
            Ok(LoadChatsPayload::FirstPage { limit })
        }
        1 => {
            let cursor_ts = read_timestamp(buf)?;
            let limit = read_u16(buf)?;
            Ok(LoadChatsPayload::After { cursor_ts, limit })
        }
        _ => Err(CodecError::UnknownDiscriminant {
            type_name: "LoadChats mode",
            value: mode as u32,
        }),
    }
}

// ---------------------------------------------------------------------------
// Search
// ---------------------------------------------------------------------------

/// Encode a `SearchPayload`.
pub fn encode_search(buf: &mut impl BufMut, p: &SearchPayload) {
    write_u32(buf, p.chat_id);
    write_string(buf, &p.query);
    write_u32(buf, p.cursor);
    write_u16(buf, p.limit);
}

/// Decode a `SearchPayload`.
pub fn decode_search(buf: &mut impl Buf) -> Result<SearchPayload, CodecError> {
    let chat_id = read_u32(buf)?;
    let query = read_string(buf)?;
    let cursor = read_u32(buf)?;
    let limit = read_u16(buf)?;
    Ok(SearchPayload {
        chat_id,
        query,
        cursor,
        limit,
    })
}

// ---------------------------------------------------------------------------
// Subscribe / Unsubscribe
// ---------------------------------------------------------------------------

/// Encode a `SubscribePayload`.
pub fn encode_subscribe(buf: &mut impl BufMut, p: &SubscribePayload) {
    write_u16(buf, p.chat_ids.len() as u16);
    for &id in &p.chat_ids {
        write_u32(buf, id);
    }
}

/// Decode a `SubscribePayload`.
pub fn decode_subscribe(buf: &mut impl Buf) -> Result<SubscribePayload, CodecError> {
    let count = read_u16(buf)? as usize;
    let mut chat_ids = Vec::with_capacity(count.min(256));
    for _ in 0..count {
        chat_ids.push(read_u32(buf)?);
    }
    Ok(SubscribePayload { chat_ids })
}

/// Encode an `UnsubscribePayload`.
pub fn encode_unsubscribe(buf: &mut impl BufMut, p: &UnsubscribePayload) {
    write_u16(buf, p.chat_ids.len() as u16);
    for &id in &p.chat_ids {
        write_u32(buf, id);
    }
}

/// Decode an `UnsubscribePayload`.
pub fn decode_unsubscribe(buf: &mut impl Buf) -> Result<UnsubscribePayload, CodecError> {
    let count = read_u16(buf)? as usize;
    let mut chat_ids = Vec::with_capacity(count.min(256));
    for _ in 0..count {
        chat_ids.push(read_u32(buf)?);
    }
    Ok(UnsubscribePayload { chat_ids })
}

// ---------------------------------------------------------------------------
// LoadMessages
// ---------------------------------------------------------------------------

/// Encode a `LoadMessagesPayload`.
pub fn encode_load_messages(buf: &mut impl BufMut, p: &LoadMessagesPayload) -> Result<(), CodecError> {
    match p {
        LoadMessagesPayload::Paginate {
            chat_id,
            direction,
            anchor_id,
            limit,
        } => {
            write_u32(buf, *chat_id);
            write_u8(buf, 0); // mode
            write_u8(buf, *direction as u8);
            write_u32(buf, *anchor_id);
            write_u16(buf, *limit);
        }
        LoadMessagesPayload::RangeCheck {
            chat_id,
            from_id,
            to_id,
            since_ts,
        } => {
            write_u32(buf, *chat_id);
            write_u8(buf, 1); // mode
            write_u32(buf, *from_id);
            write_u32(buf, *to_id);
            write_timestamp(buf, *since_ts)?;
        }
    }
    Ok(())
}

/// Decode a `LoadMessagesPayload`.
pub fn decode_load_messages(buf: &mut impl Buf) -> Result<LoadMessagesPayload, CodecError> {
    let chat_id = read_u32(buf)?;
    let mode = read_u8(buf)?;

    match mode {
        0 => {
            let dir_byte = read_u8(buf)?;
            let direction = LoadDirection::from_u8(dir_byte).ok_or(CodecError::UnknownDiscriminant {
                type_name: "LoadDirection",
                value: dir_byte as u32,
            })?;
            let anchor_id = read_u32(buf)?;
            let limit = read_u16(buf)?;
            Ok(LoadMessagesPayload::Paginate {
                chat_id,
                direction,
                anchor_id,
                limit,
            })
        }
        1 => {
            let from_id = read_u32(buf)?;
            let to_id = read_u32(buf)?;
            let since_ts = read_timestamp(buf)?;
            Ok(LoadMessagesPayload::RangeCheck {
                chat_id,
                from_id,
                to_id,
                since_ts,
            })
        }
        _ => Err(CodecError::UnknownDiscriminant {
            type_name: "LoadMessages mode",
            value: mode as u32,
        }),
    }
}

// ---------------------------------------------------------------------------
// Error frame payload
// ---------------------------------------------------------------------------

/// Encode an `ErrorPayload`.
pub fn encode_error(buf: &mut impl BufMut, p: &ErrorPayload) {
    write_u16(buf, p.code as u16);
    let slug = p.code.slug();
    write_u8(buf, slug.len() as u8);
    buf.put_slice(slug.as_bytes());
    write_string(buf, &p.message);
    write_u32(buf, p.retry_after_ms);
    write_optional_string(buf, p.extra.as_deref());
}

/// Decode an `ErrorPayload`.
pub fn decode_error(buf: &mut impl Buf) -> Result<ErrorPayload, CodecError> {
    let code_raw = read_u16(buf)?;
    let code = ErrorCode::from_u16(code_raw).ok_or(CodecError::UnknownDiscriminant {
        type_name: "ErrorCode",
        value: code_raw as u32,
    })?;

    // Read slug (u8 len + UTF-8) — we validate but don't store it (derived from code)
    let slug_len = read_u8(buf)? as usize;
    ensure_remaining(buf, slug_len)?;
    buf.advance(slug_len);

    let message = read_string(buf)?;
    let retry_after_ms = read_u32(buf)?;
    let extra = read_optional_string(buf)?;

    Ok(ErrorPayload {
        code,
        message,
        retry_after_ms,
        extra,
    })
}

// ---------------------------------------------------------------------------
// ChatEntry
// ---------------------------------------------------------------------------

/// Encode a `ChatEntry`.
pub fn encode_chat_entry(buf: &mut impl BufMut, e: &ChatEntry) -> Result<(), CodecError> {
    write_u32(buf, e.id);
    write_u8(buf, e.kind as u8);
    write_option_u32(buf, e.parent_id);
    write_timestamp(buf, e.created_at)?;
    write_timestamp(buf, e.updated_at)?;
    write_optional_string(buf, e.title.as_deref());
    write_optional_string(buf, e.avatar_url.as_deref());
    Ok(())
}

/// Decode a `ChatEntry`.
pub fn decode_chat_entry(buf: &mut impl Buf) -> Result<ChatEntry, CodecError> {
    let id = read_u32(buf)?;
    let kind_byte = read_u8(buf)?;
    let kind = ChatKind::from_u8(kind_byte).ok_or(CodecError::UnknownDiscriminant {
        type_name: "ChatKind",
        value: kind_byte as u32,
    })?;
    let parent_id = read_option_u32(buf)?;
    let created_at = read_timestamp(buf)?;
    let updated_at = read_timestamp(buf)?;
    let title = read_optional_string(buf)?;
    let avatar_url = read_optional_string(buf)?;

    Ok(ChatEntry {
        id,
        kind,
        parent_id,
        created_at,
        updated_at,
        title,
        avatar_url,
    })
}

// ---------------------------------------------------------------------------
// ChatMemberEntry
// ---------------------------------------------------------------------------

/// Encode a `ChatMemberEntry`.
pub fn encode_chat_member_entry(buf: &mut impl BufMut, e: &ChatMemberEntry) {
    write_u32(buf, e.user_id);
    write_u8(buf, e.role as u8);
    match e.permissions {
        Some(p) => {
            write_u8(buf, 1);
            write_u32(buf, p.bits());
        }
        None => write_u8(buf, 0),
    }
}

/// Decode a `ChatMemberEntry`.
pub fn decode_chat_member_entry(buf: &mut impl Buf) -> Result<ChatMemberEntry, CodecError> {
    let user_id = read_u32(buf)?;
    let role_byte = read_u8(buf)?;
    let role = ChatRole::from_u8(role_byte).ok_or(CodecError::UnknownDiscriminant {
        type_name: "ChatRole",
        value: role_byte as u32,
    })?;
    let perm_flag = read_u8(buf)?;
    let permissions = match perm_flag {
        0 => None,
        1 => Some(Permission::from_bits_truncate(read_u32(buf)?)),
        _ => {
            return Err(CodecError::UnknownDiscriminant {
                type_name: "Permission flag",
                value: perm_flag as u32,
            });
        }
    };

    Ok(ChatMemberEntry {
        user_id,
        role,
        permissions,
    })
}

// ---------------------------------------------------------------------------
// UserEntry
// ---------------------------------------------------------------------------

/// Encode a `UserEntry`.
pub fn encode_user_entry(buf: &mut impl BufMut, e: &UserEntry) -> Result<(), CodecError> {
    write_u32(buf, e.id);
    write_u16(buf, e.flags.bits());
    write_timestamp(buf, e.created_at)?;
    write_timestamp(buf, e.updated_at)?;
    write_optional_string(buf, e.username.as_deref());
    write_optional_string(buf, e.first_name.as_deref());
    write_optional_string(buf, e.last_name.as_deref());
    write_optional_string(buf, e.avatar_url.as_deref());
    Ok(())
}

/// Decode a `UserEntry`.
pub fn decode_user_entry(buf: &mut impl Buf) -> Result<UserEntry, CodecError> {
    let id = read_u32(buf)?;
    let flags = UserFlags::from_bits_truncate(read_u16(buf)?);
    let created_at = read_timestamp(buf)?;
    let updated_at = read_timestamp(buf)?;
    let username = read_optional_string(buf)?;
    let first_name = read_optional_string(buf)?;
    let last_name = read_optional_string(buf)?;
    let avatar_url = read_optional_string(buf)?;

    Ok(UserEntry {
        id,
        flags,
        created_at,
        updated_at,
        username,
        first_name,
        last_name,
        avatar_url,
    })
}

// ---------------------------------------------------------------------------
// Chat management payloads
// ---------------------------------------------------------------------------

/// Encode a `CreateChatPayload`.
pub fn encode_create_chat(buf: &mut impl BufMut, p: &CreateChatPayload) {
    write_u8(buf, p.kind as u8);
    write_option_u32(buf, p.parent_id);
    write_optional_string(buf, p.title.as_deref());
    write_optional_string(buf, p.avatar_url.as_deref());
    write_u16(buf, p.member_ids.len() as u16);
    for &id in &p.member_ids {
        write_u32(buf, id);
    }
}

/// Decode a `CreateChatPayload`.
pub fn decode_create_chat(buf: &mut impl Buf) -> Result<CreateChatPayload, CodecError> {
    let kind_byte = read_u8(buf)?;
    let kind = ChatKind::from_u8(kind_byte).ok_or(CodecError::UnknownDiscriminant {
        type_name: "ChatKind",
        value: kind_byte as u32,
    })?;
    let parent_id = read_option_u32(buf)?;
    let title = read_optional_string(buf)?;
    let avatar_url = read_optional_string(buf)?;
    let member_count = read_u16(buf)? as usize;
    let mut member_ids = Vec::with_capacity(member_count.min(256));
    for _ in 0..member_count {
        member_ids.push(read_u32(buf)?);
    }

    Ok(CreateChatPayload {
        kind,
        parent_id,
        title,
        avatar_url,
        member_ids,
    })
}

/// Encode event payloads (ReceiptUpdate, TypingUpdate, MemberJoined, MemberLeft).
pub fn encode_receipt_update(buf: &mut impl BufMut, p: &ReceiptUpdatePayload) {
    write_u32(buf, p.chat_id);
    write_u32(buf, p.user_id);
    write_u32(buf, p.message_id);
}

/// Decode a `ReceiptUpdatePayload`.
pub fn decode_receipt_update(buf: &mut impl Buf) -> Result<ReceiptUpdatePayload, CodecError> {
    Ok(ReceiptUpdatePayload {
        chat_id: read_u32(buf)?,
        user_id: read_u32(buf)?,
        message_id: read_u32(buf)?,
    })
}

/// Encode a `TypingUpdatePayload`.
pub fn encode_typing_update(buf: &mut impl BufMut, p: &TypingUpdatePayload) {
    write_u32(buf, p.chat_id);
    write_u32(buf, p.user_id);
}

/// Decode a `TypingUpdatePayload`.
pub fn decode_typing_update(buf: &mut impl Buf) -> Result<TypingUpdatePayload, CodecError> {
    Ok(TypingUpdatePayload {
        chat_id: read_u32(buf)?,
        user_id: read_u32(buf)?,
    })
}

/// Encode a `MemberJoinedPayload`.
pub fn encode_member_joined(buf: &mut impl BufMut, p: &MemberJoinedPayload) {
    write_u32(buf, p.chat_id);
    write_u32(buf, p.user_id);
}

/// Decode a `MemberJoinedPayload`.
pub fn decode_member_joined(buf: &mut impl Buf) -> Result<MemberJoinedPayload, CodecError> {
    Ok(MemberJoinedPayload {
        chat_id: read_u32(buf)?,
        user_id: read_u32(buf)?,
    })
}

/// Encode a `MemberLeftPayload`.
pub fn encode_member_left(buf: &mut impl BufMut, p: &MemberLeftPayload) {
    write_u32(buf, p.chat_id);
    write_u32(buf, p.user_id);
}

/// Decode a `MemberLeftPayload`.
pub fn decode_member_left(buf: &mut impl Buf) -> Result<MemberLeftPayload, CodecError> {
    Ok(MemberLeftPayload {
        chat_id: read_u32(buf)?,
        user_id: read_u32(buf)?,
    })
}

// ---------------------------------------------------------------------------
// UpdateChat
// ---------------------------------------------------------------------------

/// Encode an `UpdateChatPayload`.
///
/// Uses `u8 flag` prefix per field: `0` = don't change, `1` = set (empty string = clear).
pub fn encode_update_chat(buf: &mut impl BufMut, p: &UpdateChatPayload) {
    write_u32(buf, p.chat_id);
    encode_updatable_string(buf, p.title.as_deref());
    encode_updatable_string(buf, p.avatar_url.as_deref());
}

/// Decode an `UpdateChatPayload`.
pub fn decode_update_chat(buf: &mut impl Buf) -> Result<UpdateChatPayload, CodecError> {
    let chat_id = read_u32(buf)?;
    let title = decode_updatable_string(buf)?;
    let avatar_url = decode_updatable_string(buf)?;
    Ok(UpdateChatPayload {
        chat_id,
        title,
        avatar_url,
    })
}

/// Encode an updatable string field: `0` = don't change, `1` + string = set/clear.
fn encode_updatable_string(buf: &mut impl BufMut, value: Option<&str>) {
    match value {
        None => write_u8(buf, 0),
        Some(s) => {
            write_u8(buf, 1);
            write_string(buf, s);
        }
    }
}

/// Decode an updatable string field.
fn decode_updatable_string(buf: &mut impl Buf) -> Result<Option<String>, CodecError> {
    let flag = read_u8(buf)?;
    match flag {
        0 => Ok(None),
        1 => {
            let s = read_string(buf)?;
            Ok(Some(s))
        }
        _ => Err(CodecError::UnknownDiscriminant {
            type_name: "updatable string flag",
            value: flag as u32,
        }),
    }
}

// ---------------------------------------------------------------------------
// DeleteChat / GetChatInfo / LeaveChat (single chat_id payloads)
// ---------------------------------------------------------------------------

/// Encode a `DeleteChatPayload`.
pub fn encode_delete_chat(buf: &mut impl BufMut, p: &DeleteChatPayload) {
    write_u32(buf, p.chat_id);
}

/// Decode a `DeleteChatPayload`.
pub fn decode_delete_chat(buf: &mut impl Buf) -> Result<DeleteChatPayload, CodecError> {
    Ok(DeleteChatPayload {
        chat_id: read_u32(buf)?,
    })
}

/// Encode a `GetChatInfoPayload`.
pub fn encode_get_chat_info(buf: &mut impl BufMut, p: &GetChatInfoPayload) {
    write_u32(buf, p.chat_id);
}

/// Decode a `GetChatInfoPayload`.
pub fn decode_get_chat_info(buf: &mut impl Buf) -> Result<GetChatInfoPayload, CodecError> {
    Ok(GetChatInfoPayload {
        chat_id: read_u32(buf)?,
    })
}

/// Encode a `LeaveChatPayload`.
pub fn encode_leave_chat(buf: &mut impl BufMut, p: &LeaveChatPayload) {
    write_u32(buf, p.chat_id);
}

/// Decode a `LeaveChatPayload`.
pub fn decode_leave_chat(buf: &mut impl Buf) -> Result<LeaveChatPayload, CodecError> {
    Ok(LeaveChatPayload {
        chat_id: read_u32(buf)?,
    })
}

// ---------------------------------------------------------------------------
// GetChatMembers
// ---------------------------------------------------------------------------

/// Encode a `GetChatMembersPayload`.
pub fn encode_get_chat_members(buf: &mut impl BufMut, p: &GetChatMembersPayload) {
    write_u32(buf, p.chat_id);
    write_u32(buf, p.cursor);
    write_u16(buf, p.limit);
}

/// Decode a `GetChatMembersPayload`.
pub fn decode_get_chat_members(buf: &mut impl Buf) -> Result<GetChatMembersPayload, CodecError> {
    Ok(GetChatMembersPayload {
        chat_id: read_u32(buf)?,
        cursor: read_u32(buf)?,
        limit: read_u16(buf)?,
    })
}

// ---------------------------------------------------------------------------
// InviteMembers
// ---------------------------------------------------------------------------

/// Encode an `InviteMembersPayload`.
pub fn encode_invite_members(buf: &mut impl BufMut, p: &InviteMembersPayload) {
    write_u32(buf, p.chat_id);
    write_u16(buf, p.user_ids.len() as u16);
    for &id in &p.user_ids {
        write_u32(buf, id);
    }
}

/// Decode an `InviteMembersPayload`.
pub fn decode_invite_members(buf: &mut impl Buf) -> Result<InviteMembersPayload, CodecError> {
    let chat_id = read_u32(buf)?;
    let count = read_u16(buf)? as usize;
    let mut user_ids = Vec::with_capacity(count.min(256));
    for _ in 0..count {
        user_ids.push(read_u32(buf)?);
    }
    Ok(InviteMembersPayload { chat_id, user_ids })
}

// ---------------------------------------------------------------------------
// UpdateMember
// ---------------------------------------------------------------------------

/// Encode an `UpdateMemberPayload`.
pub fn encode_update_member(buf: &mut impl BufMut, p: &UpdateMemberPayload) {
    write_u32(buf, p.chat_id);
    write_u32(buf, p.user_id);
    encode_member_action(buf, &p.action);
}

/// Decode an `UpdateMemberPayload`.
pub fn decode_update_member(buf: &mut impl Buf) -> Result<UpdateMemberPayload, CodecError> {
    let chat_id = read_u32(buf)?;
    let user_id = read_u32(buf)?;
    let action = decode_member_action(buf)?;
    Ok(UpdateMemberPayload {
        chat_id,
        user_id,
        action,
    })
}

fn encode_member_action(buf: &mut impl BufMut, action: &MemberAction) {
    match action {
        MemberAction::Kick => write_u8(buf, 0),
        MemberAction::Ban => write_u8(buf, 1),
        MemberAction::Mute { duration_secs } => {
            write_u8(buf, 2);
            write_u32(buf, *duration_secs);
        }
        MemberAction::ChangeRole(role) => {
            write_u8(buf, 3);
            write_u8(buf, *role as u8);
        }
        MemberAction::UpdatePermissions(perms) => {
            write_u8(buf, 4);
            write_u32(buf, perms.bits());
        }
    }
}

fn decode_member_action(buf: &mut impl Buf) -> Result<MemberAction, CodecError> {
    let action_byte = read_u8(buf)?;
    match action_byte {
        0 => Ok(MemberAction::Kick),
        1 => Ok(MemberAction::Ban),
        2 => {
            let duration_secs = read_u32(buf)?;
            Ok(MemberAction::Mute { duration_secs })
        }
        3 => {
            let role_byte = read_u8(buf)?;
            let role = ChatRole::from_u8(role_byte).ok_or(CodecError::UnknownDiscriminant {
                type_name: "ChatRole",
                value: role_byte as u32,
            })?;
            Ok(MemberAction::ChangeRole(role))
        }
        4 => {
            let perms = Permission::from_bits_truncate(read_u32(buf)?);
            Ok(MemberAction::UpdatePermissions(perms))
        }
        _ => Err(CodecError::UnknownDiscriminant {
            type_name: "MemberAction",
            value: action_byte as u32,
        }),
    }
}

// ---------------------------------------------------------------------------
// MessageDeleted event
// ---------------------------------------------------------------------------

/// Encode a `MessageDeletedPayload`.
pub fn encode_message_deleted(buf: &mut impl BufMut, p: &MessageDeletedPayload) {
    write_u32(buf, p.chat_id);
    write_u32(buf, p.message_id);
}

/// Decode a `MessageDeletedPayload`.
pub fn decode_message_deleted(buf: &mut impl Buf) -> Result<MessageDeletedPayload, CodecError> {
    Ok(MessageDeletedPayload {
        chat_id: read_u32(buf)?,
        message_id: read_u32(buf)?,
    })
}
