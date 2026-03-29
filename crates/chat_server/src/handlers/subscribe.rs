//! Subscribe/Unsubscribe handlers.

use std::sync::Arc;

use anyhow::Context;
use chat_protocol::types::ErrorCode;
use tokio::sync::mpsc;
use tracing::debug;

use crate::db::queries;
use crate::state::{AppState, SessionHandle};
use crate::ws::session::{encode_ack_empty, encode_error_frame};

/// Process a Subscribe frame: verify membership, add to subscriptions, ack.
pub async fn handle_subscribe(
    payload: chat_protocol::types::SubscribePayload,
    seq: u32,
    session: &Arc<SessionHandle>,
    outbound_tx: &mpsc::Sender<Vec<u8>>,
    state: &Arc<AppState>,
) -> anyhow::Result<()> {
    for channel in &payload.channels {
        // For "chat#<id>" channels, verify membership.
        if let Some(chat_id_str) = channel.strip_prefix("chat#") {
            let chat_id: i32 = chat_id_str
                .parse()
                .with_context(|| format!("invalid channel: {channel}"))?;
            let is_member = queries::check_chat_membership(&state.db, chat_id, session.user_id as i32).await?;
            if !is_member {
                let err = encode_error_frame(seq, ErrorCode::NotChatMember, "not a member of this chat");
                let _ = outbound_tx.send(err).await;
                return Ok(());
            }
        }

        session.subscriptions.lock().insert(channel.clone());
        debug!(user_id = session.user_id, channel, "subscribed");
    }

    let ack = encode_ack_empty(seq);
    outbound_tx.send(ack).await.context("send subscribe ack")?;
    Ok(())
}

/// Process an Unsubscribe frame: remove channels from subscriptions.
pub fn handle_unsubscribe(payload: chat_protocol::types::UnsubscribePayload, session: &Arc<SessionHandle>) {
    let mut subs = session.subscriptions.lock();
    for channel in &payload.channels {
        subs.remove(channel);
        debug!(user_id = session.user_id, channel, "unsubscribed");
    }
}
