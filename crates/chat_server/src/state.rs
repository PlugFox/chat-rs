//! Shared application state and session management.

use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use dashmap::DashMap;
use parking_lot::Mutex;
use sqlx::PgPool;
use tokio::sync::{mpsc, watch};
use uuid::Uuid;

use crate::config::ServerConfig;

/// Per-connection session state, shared via `Arc`.
pub struct SessionHandle {
    pub user_id: u32,
    pub device_id: Uuid,
    pub session_id: u32,
    /// Channels this session is subscribed to (e.g. `"chat#1"`, `"general"`).
    pub subscriptions: Mutex<HashSet<String>>,
    /// Outbound frame sender — bounded mpsc channel.
    pub sender: mpsc::Sender<Vec<u8>>,
    /// Monotonically increasing counter for server-push events.
    pub event_seq: AtomicU32,
}

impl SessionHandle {
    /// Allocate the next `event_seq` value for a server-push frame.
    pub fn next_event_seq(&self) -> u32 {
        self.event_seq.fetch_add(1, Ordering::Relaxed) + 1
    }
}

/// Application-wide shared state, passed to handlers via `Arc<AppState>`.
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<ServerConfig>,
    /// Active WS sessions keyed by `(user_id, device_id)`.
    pub sessions: DashMap<(u32, Uuid), Arc<SessionHandle>>,
    /// Shutdown signal sender — set to `true` to initiate graceful shutdown.
    pub shutdown_tx: watch::Sender<bool>,
    /// Shutdown signal receiver — cloned per connection.
    pub shutdown_rx: watch::Receiver<bool>,
    /// Counter for assigning transient session IDs.
    next_session_id: AtomicU32,
}

impl AppState {
    pub fn new(db: PgPool, config: ServerConfig) -> Self {
        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        Self {
            db,
            config: Arc::new(config),
            sessions: DashMap::new(),
            shutdown_tx,
            shutdown_rx,
            next_session_id: AtomicU32::new(1),
        }
    }

    /// Assign a new transient session ID.
    pub fn next_session_id(&self) -> u32 {
        self.next_session_id.fetch_add(1, Ordering::Relaxed)
    }
}
