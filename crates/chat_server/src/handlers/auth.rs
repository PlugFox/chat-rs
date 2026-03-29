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
pub(crate) struct Claims {
    pub sub: String,
    // exp is validated by jsonwebtoken automatically.
}

/// Verify a JWT token and extract claims.
///
/// Returns `Ok(Claims)` on success, or an appropriate `ErrorCode` on failure.
pub(crate) fn verify_jwt(token: &str, secret: &str) -> Result<Claims, (ErrorCode, String)> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&["sub", "exp"]);
    // 60s leeway: tokens expired <1min ago or issued <1min in the future are still valid.
    validation.leeway = 60;
    validation.validate_nbf = true;
    match jsonwebtoken::decode::<Claims>(token, &key, &validation) {
        Ok(data) => Ok(data.claims),
        Err(e) => {
            let code = if e.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                ErrorCode::TokenExpired
            } else {
                ErrorCode::Unauthorized
            };
            Err((code, format!("authentication failed: {e}")))
        }
    }
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
    let claims = match verify_jwt(&hello.token, &state.config.auth.jwt_secret) {
        Ok(c) => c,
        Err((code, msg)) => {
            warn!("JWT verification failed: {msg}");
            let err = encode_error_frame(0, code, &msg);
            let _ = outbound_tx.send(err).await;
            bail!("{msg}");
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

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{EncodingKey, Header};
    use serde::Serialize;

    const SECRET: &str = "test-secret";

    #[derive(Serialize)]
    struct TestClaims {
        sub: String,
        exp: usize,
    }

    fn make_token(sub: &str, exp: usize, secret: &str) -> String {
        let claims = TestClaims {
            sub: sub.to_owned(),
            exp,
        };
        jsonwebtoken::encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap()
    }

    fn future_exp() -> usize {
        (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 3600) as usize
    }

    #[test]
    fn valid_token() {
        let token = make_token("user_123", future_exp(), SECRET);
        let claims = verify_jwt(&token, SECRET).unwrap();
        assert_eq!(claims.sub, "user_123");
    }

    #[test]
    fn expired_token() {
        let token = make_token("user_123", 1_000_000, SECRET); // year ~2001
        let result = verify_jwt(&token, SECRET);
        let (code, _msg) = result.unwrap_err();
        assert_eq!(code, ErrorCode::TokenExpired);
    }

    #[test]
    fn invalid_signature() {
        let token = make_token("user_123", future_exp(), "wrong-secret");
        let result = verify_jwt(&token, SECRET);
        let (code, _msg) = result.unwrap_err();
        assert_eq!(code, ErrorCode::Unauthorized);
    }

    #[test]
    fn malformed_token() {
        let result = verify_jwt("not.a.jwt", SECRET);
        let (code, _msg) = result.unwrap_err();
        assert_eq!(code, ErrorCode::Unauthorized);
    }

    #[test]
    fn empty_token() {
        let result = verify_jwt("", SECRET);
        let (code, _msg) = result.unwrap_err();
        assert_eq!(code, ErrorCode::Unauthorized);
    }

    fn now_secs() -> usize {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
    }

    #[test]
    fn recently_expired_token_accepted_within_leeway() {
        // Expired 30s ago — within the 60s leeway.
        let token = make_token("user_123", now_secs() - 30, SECRET);
        let claims = verify_jwt(&token, SECRET).unwrap();
        assert_eq!(claims.sub, "user_123");
    }

    #[test]
    fn expired_beyond_leeway_rejected() {
        // Expired 120s ago — outside the 60s leeway.
        let token = make_token("user_123", now_secs() - 120, SECRET);
        let (code, _) = verify_jwt(&token, SECRET).unwrap_err();
        assert_eq!(code, ErrorCode::TokenExpired);
    }

    #[test]
    fn nbf_slightly_in_future_accepted() {
        // Token with nbf 30s in the future — within leeway.
        #[derive(Serialize)]
        struct NbfClaims {
            sub: String,
            exp: usize,
            nbf: usize,
        }
        let claims = NbfClaims {
            sub: "user_123".into(),
            exp: now_secs() + 3600,
            nbf: now_secs() + 30,
        };
        let token = jsonwebtoken::encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(SECRET.as_bytes()),
        )
        .unwrap();
        let result = verify_jwt(&token, SECRET);
        assert!(result.is_ok(), "nbf 30s in future should be within 60s leeway");
    }

    #[test]
    fn missing_sub_claim() {
        // Token with exp but no sub.
        #[derive(Serialize)]
        struct NoSub {
            exp: usize,
        }
        let claims = NoSub { exp: future_exp() };
        let token = jsonwebtoken::encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(SECRET.as_bytes()),
        )
        .unwrap();
        let result = verify_jwt(&token, SECRET);
        let (code, _msg) = result.unwrap_err();
        assert_eq!(code, ErrorCode::Unauthorized);
    }
}
