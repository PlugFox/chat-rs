//! WebSocket upgrade handler and connection management.

pub mod dispatch;
pub mod session;

use std::sync::Arc;

use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use tokio::sync::mpsc;
use tracing::debug;

use crate::state::AppState;

/// Axum handler for WebSocket upgrade at `GET /ws`.
pub async fn upgrade_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Process a single WebSocket connection.
async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (ws_tx, ws_rx) = socket.split();
    let buffer_size = state.config.server.ws_send_buffer_size as usize;
    let (outbound_tx, mut outbound_rx) = mpsc::channel::<Vec<u8>>(buffer_size);

    // Outbound task: reads from the mpsc channel and forwards to the WS sink.
    let mut ws_tx = ws_tx;
    let outbound_handle = tokio::spawn(async move {
        while let Some(data) = outbound_rx.recv().await {
            if ws_tx.send(Message::Binary(data.into())).await.is_err() {
                break;
            }
        }
        // Try to close the WS cleanly.
        let _ = ws_tx.close().await;
    });

    // Inbound frame loop (blocking on this task).
    dispatch::frame_loop(ws_rx, outbound_tx, state).await;

    // Wait for outbound task to finish.
    debug!("waiting for outbound task to finish");
    outbound_handle.abort();
}
