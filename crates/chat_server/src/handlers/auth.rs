//! Hello/Welcome authentication handler.

use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, bail};
use chat_protocol::PROTOCOL_VERSION;
use chat_protocol::codec::encode_frame;
use chat_protocol::types::{ErrorCode, FramePayload, ServerCapabilities, WelcomePayload};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use parking_lot::Mutex;
use serde::Deserialize;
use tokio::sync::mpsc;
use tracing::{info, warn};

use crate::db::queries;
use crate::state::{AppState, SessionHandle};
use crate::ws::session::encode_error_frame;

/// JWT claims: `sub` = external_id (string), `exp` = expiration timestamp.
#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    // exp is validated by jsonwebtoken automatically.
}

/// Process a Hello frame: verify JWT, find/create user, send Welcome.
pub async fn handle_hello(
    hello: chat_protocol::types::HelloPayload,
    outbound_tx: &mpsc::Sender<Vec<u8>>,
    state: &Arc<AppState>,
) -> anyhow::Result<Arc<SessionHandle>> {
    // 1. Check protocol version.
    if hello.protocol_version != PROTOCOL_VERSION {
        let err = encode_error_frame(0, ErrorCode::UnsupportedVersion, "unsupported protocol version");
        let _ = outbound_tx.send(err).await;
        bail!("unsupported protocol version: {}", hello.protocol_version);
    }

    // 2. Verify JWT.
    let key = DecodingKey::from_secret(state.config.auth.jwt_secret.as_bytes());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&["sub", "exp"]);
    let token_data = jsonwebtoken::decode::<Claims>(&hello.token, &key, &validation);
    let claims = match token_data {
        Ok(data) => data.claims,
        Err(e) => {
            warn!("JWT verification failed: {e}");
            let code = if e.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                ErrorCode::TokenExpired
            } else {
                ErrorCode::Unauthorized
            };
            let err = encode_error_frame(0, code, &format!("authentication failed: {e}"));
            let _ = outbound_tx.send(err).await;
            bail!("JWT verification failed: {e}");
        }
    };

    let external_id = &claims.sub;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system time before epoch")?
        .as_secs() as i64;

    // 3. Find or create user.
    let (user_id_i32, _flags) = queries::find_or_create_user(&state.db, external_id, now).await?;
    let user_id = user_id_i32 as u32;

    // 4. Upsert device session.
    let _db_session_id = queries::upsert_device_session(&state.db, user_id_i32, hello.device_id, now).await?;

    // 5. Handle duplicate session (same user_id + device_id).
    if let Some((_, old_handle)) = state.sessions.remove(&(user_id, hello.device_id)) {
        // Send DuplicateSession disconnect to old session by dropping its sender.
        // The outbound task will close the WS when the channel is dropped.
        info!(user_id, device_id = %hello.device_id, "replaced duplicate session");
        drop(old_handle);
    }

    // 6. Create SessionHandle.
    let session_id = state.next_session_id();
    let handle = Arc::new(SessionHandle {
        user_id,
        device_id: hello.device_id,
        session_id,
        subscriptions: Mutex::new(HashSet::new()),
        sender: outbound_tx.clone(),
        event_seq: AtomicU32::new(0),
    });
    state.sessions.insert((user_id, hello.device_id), handle.clone());

    // 7. Build and send Welcome frame.
    let welcome = WelcomePayload {
        session_id,
        server_time: now,
        user_id,
        limits: state.config.limits.to_server_limits(&state.config.rate_limits),
        capabilities: ServerCapabilities::empty(),
    };
    let frame = chat_protocol::types::Frame {
        seq: 0,
        event_seq: 0,
        payload: FramePayload::Welcome(welcome),
    };
    let mut buf = Vec::with_capacity(64);
    encode_frame(&mut buf, &frame).context("encode Welcome")?;
    outbound_tx.send(buf).await.context("send Welcome")?;

    info!(user_id, session_id, external_id, "authenticated");
    Ok(handle)
}
