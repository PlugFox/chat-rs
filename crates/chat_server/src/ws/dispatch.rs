//! WebSocket frame processing loop.

use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use chat_protocol::codec::decode_frame;
use chat_protocol::types::{ErrorCode, FramePayload};
use futures_util::StreamExt;
use futures_util::stream::SplitStream;
use tokio::sync::{mpsc, watch};
use tracing::{debug, warn};

use crate::state::{AppState, SessionHandle};
use crate::ws::session::encode_error_frame;

/// Run the inbound frame processing loop for a single WebSocket connection.
///
/// Returns when the connection closes or a shutdown signal is received.
pub async fn frame_loop(mut ws_rx: SplitStream<WebSocket>, outbound_tx: mpsc::Sender<Vec<u8>>, state: Arc<AppState>) {
    let mut shutdown_rx: watch::Receiver<bool> = state.shutdown_rx.clone();
    let mut session: Option<Arc<SessionHandle>> = None;

    loop {
        tokio::select! {
            biased;

            _ = shutdown_rx.changed() => {
                if *shutdown_rx.borrow() {
                    debug!("shutdown signal received, closing connection");
                    // Graceful disconnect is handled by the caller (main shutdown loop).
                    break;
                }
            }

            msg = ws_rx.next() => {
                match msg {
                    Some(Ok(Message::Binary(data))) => {
                        let mut buf = &data[..];
                        match decode_frame(&mut buf) {
                            Ok(frame) => {
                                dispatch_frame(
                                    frame.seq,
                                    frame.payload,
                                    &mut session,
                                    &outbound_tx,
                                    &state,
                                ).await;
                            }
                            Err(e) => {
                                warn!("malformed frame: {e}");
                                let err = encode_error_frame(0, ErrorCode::MalformedFrame, "malformed frame");
                                let _ = outbound_tx.send(err).await;
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => {
                        debug!("client disconnected");
                        break;
                    }
                    // Ignore text, ping, pong at WS level (protocol uses binary frames).
                    _ => {}
                }
            }
        }
    }

    // Cleanup: remove session from registry.
    if let Some(handle) = session.take() {
        state.sessions.remove(&(handle.user_id, handle.device_id));
        debug!(user_id = handle.user_id, "session removed");
    }
}

/// Dispatch a decoded frame to the appropriate handler.
async fn dispatch_frame(
    seq: u32,
    payload: FramePayload,
    session: &mut Option<Arc<SessionHandle>>,
    outbound_tx: &mpsc::Sender<Vec<u8>>,
    state: &Arc<AppState>,
) {
    match payload {
        FramePayload::Hello(hello) => {
            match crate::handlers::auth::handle_hello(hello, outbound_tx, state).await {
                Ok(handle) => {
                    *session = Some(handle);
                }
                Err(e) => {
                    warn!("hello failed: {e:#}");
                    // Error frame already sent by handle_hello.
                }
            }
        }

        FramePayload::Ping => {
            // Respond with Pong carrying the same seq.
            let pong = {
                let mut buf = Vec::with_capacity(chat_protocol::FRAME_HEADER_SIZE);
                use bytes::BufMut;
                buf.put_u8(0x04); // FrameKind::Pong
                buf.put_u32_le(seq);
                buf.put_u32_le(0);
                buf
            };
            let _ = outbound_tx.send(pong).await;
        }

        // All other frames require authentication.
        _ => {
            let Some(handle) = session.as_ref() else {
                let err = encode_error_frame(seq, ErrorCode::Unauthorized, "not authenticated");
                let _ = outbound_tx.send(err).await;
                return;
            };

            match payload {
                FramePayload::SendMessage(p) => {
                    if let Err(e) =
                        crate::handlers::message::handle_send_message(p, seq, handle, outbound_tx, state).await
                    {
                        warn!("send_message failed: {e:#}");
                        let err = encode_error_frame(seq, ErrorCode::InternalError, &format!("{e:#}"));
                        let _ = outbound_tx.send(err).await;
                    }
                }
                FramePayload::Subscribe(p) => {
                    if let Err(e) =
                        crate::handlers::subscribe::handle_subscribe(p, seq, handle, outbound_tx, state).await
                    {
                        warn!("subscribe failed: {e:#}");
                        let err = encode_error_frame(seq, ErrorCode::InternalError, &format!("{e:#}"));
                        let _ = outbound_tx.send(err).await;
                    }
                }
                FramePayload::Unsubscribe(p) => {
                    crate::handlers::subscribe::handle_unsubscribe(p, handle);
                }
                _ => {
                    let err = encode_error_frame(seq, ErrorCode::UnknownCommand, "not implemented");
                    let _ = outbound_tx.send(err).await;
                }
            }
        }
    }
}
