//! SendMessage handler.

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Context;
use chat_protocol::codec::encode_frame;
use chat_protocol::types::{ErrorCode, Frame, FramePayload, Message, MessageFlags};
use tokio::sync::mpsc;
use tracing::{debug, warn};

use crate::db::queries;
use crate::state::{AppState, SessionHandle};
use crate::ws::session::{encode_ack_message_id, encode_error_frame, encode_event_frame};

/// Process a SendMessage frame: validate, insert, ack, fan-out.
pub async fn handle_send_message(
    payload: chat_protocol::types::SendMessagePayload,
    seq: u32,
    session: &Arc<SessionHandle>,
    outbound_tx: &mpsc::Sender<Vec<u8>>,
    state: &Arc<AppState>,
) -> anyhow::Result<()> {
    let chat_id = payload.chat_id as i32;
    let user_id = session.user_id as i32;

    // 1. Check chat membership.
    let is_member = queries::check_chat_membership(&state.db, chat_id, user_id).await?;
    if !is_member {
        let err = encode_error_frame(seq, ErrorCode::NotChatMember, "not a member of this chat");
        let _ = outbound_tx.send(err).await;
        return Ok(());
    }

    // 2. Validate content length.
    if payload.content.len() > state.config.limits.max_message_content_length as usize {
        let err = encode_error_frame(seq, ErrorCode::MessageTooLarge, "message content too large");
        let _ = outbound_tx.send(err).await;
        return Ok(());
    }

    // 3. Check idempotency key.
    if let Some((_existing_chat_id, existing_msg_id)) =
        queries::check_idempotency_key(&state.db, payload.idempotency_key).await?
    {
        debug!(
            key = %payload.idempotency_key,
            msg_id = existing_msg_id,
            "idempotency key hit"
        );
        let ack = encode_ack_message_id(seq, existing_msg_id as u32);
        outbound_tx.send(ack).await.context("send idempotency ack")?;
        return Ok(());
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system time")?
        .as_secs() as i64;

    // 4. Compute flags.
    let mut flags = MessageFlags::empty();
    if payload.reply_to_id.is_some() {
        flags |= MessageFlags::REPLY;
    }

    // 5. Atomic insert.
    let (msg_id, created_at) = queries::insert_message_atomic(
        &state.db,
        chat_id,
        user_id,
        payload.kind as i16,
        flags.bits() as i16,
        payload.reply_to_id.map(|id| id as i32),
        &payload.content,
        payload.rich_content.as_deref(),
        payload.extra.as_deref(),
        now,
    )
    .await?;

    // 6. Insert idempotency key.
    if let Err(e) = queries::insert_idempotency_key(&state.db, payload.idempotency_key, chat_id, msg_id, now).await {
        warn!("failed to insert idempotency key: {e:#}");
        // Non-fatal — the message was already inserted.
    }

    // 7. Send Ack to sender.
    let ack = encode_ack_message_id(seq, msg_id as u32);
    outbound_tx.send(ack).await.context("send message ack")?;

    // 8. Build MessageNew event for fan-out.
    let message = Message {
        id: msg_id as u32,
        chat_id: payload.chat_id,
        sender_id: session.user_id,
        created_at,
        updated_at: created_at,
        kind: payload.kind,
        flags,
        reply_to_id: payload.reply_to_id,
        content: payload.content,
        rich_content: decode_rich_content_or_none(&payload.rich_content),
        extra: payload.extra,
    };

    // Encode the MessageNew payload (without header — header is per-recipient).
    let event_payload = {
        let frame = Frame {
            seq: 0,
            event_seq: 0, // placeholder — replaced per-recipient
            payload: FramePayload::MessageNew(message),
        };
        let mut buf = Vec::with_capacity(128);
        encode_frame(&mut buf, &frame).context("encode MessageNew")?;
        // Strip the 9-byte header — we'll re-add it per-recipient with correct event_seq.
        buf[chat_protocol::FRAME_HEADER_SIZE..].to_vec()
    };

    // 9. Fan-out to all sessions subscribed to this chat.
    let channel = format!("chat#{}", payload.chat_id);
    broadcast_to_channel(state, &channel, 0x20, &event_payload, Some(session));

    debug!(chat_id = payload.chat_id, msg_id, "message delivered");
    Ok(())
}

/// Broadcast an event frame to all sessions subscribed to a channel.
fn broadcast_to_channel(
    state: &AppState,
    channel: &str,
    kind: u8,
    payload: &[u8],
    exclude: Option<&Arc<SessionHandle>>,
) {
    for entry in state.sessions.iter() {
        let handle = entry.value();

        // Skip the sender's session.
        if let Some(excl) = exclude {
            if handle.session_id == excl.session_id {
                continue;
            }
        }

        // Check if subscribed to this channel.
        if !handle.subscriptions.lock().contains(channel) {
            continue;
        }

        // Build frame with per-recipient event_seq.
        let event_seq = handle.next_event_seq();
        let frame_bytes = encode_event_frame(kind, event_seq, payload);

        if handle.sender.try_send(frame_bytes).is_err() {
            warn!(user_id = handle.user_id, "send buffer full, dropping event");
        }
    }
}

/// Decode rich_content bytes into Vec<RichSpan>, or None if absent/empty.
fn decode_rich_content_or_none(raw: &Option<Vec<u8>>) -> Option<Vec<chat_protocol::types::RichSpan>> {
    let bytes = raw.as_ref()?;
    if bytes.is_empty() {
        return None;
    }
    // Rich content is stored as raw bytes; decoding is done by the client.
    // For the server, we pass it through as-is via the codec.
    let mut buf = &bytes[..];
    chat_protocol::codec::decode_rich_content(&mut buf).ok()
}
