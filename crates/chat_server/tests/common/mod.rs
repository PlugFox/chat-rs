//! Test infrastructure: TestServer and TestClient.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use bytes::Buf;
use chat_protocol::PROTOCOL_VERSION;
use chat_protocol::codec::{decode_frame, encode_frame};
use chat_protocol::types::{
    AckPayload, Frame, FramePayload, HelloPayload, SendMessagePayload, SubscribePayload, WelcomePayload,
};
use chat_server::config::{
    AuthSection, DatabaseSection, LimitsSection, RateLimitsSection, ServerConfig, ServerSection,
};
use chat_server::{app, db, state};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::Serialize;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use uuid::Uuid;

// Serialize tests so they don't race on the shared database.
static DB_LOCK: std::sync::OnceLock<Arc<Mutex<()>>> = std::sync::OnceLock::new();

fn db_lock() -> Arc<Mutex<()>> {
    DB_LOCK.get_or_init(|| Arc::new(Mutex::new(()))).clone()
}

static TRACING_INIT: std::sync::Once = std::sync::Once::new();

fn init_tracing() {
    TRACING_INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("chat_server=debug")),
            )
            .with_test_writer()
            .try_init()
            .ok();
    });
}

// ---------------------------------------------------------------------------
// TestServer
// ---------------------------------------------------------------------------

pub struct TestServer {
    pub addr: SocketAddr,
    pub pool: PgPool,
    pub state: Arc<state::AppState>,
    jwt_secret: String,
    _server_handle: tokio::task::JoinHandle<()>,
    _db_guard: tokio::sync::OwnedMutexGuard<()>,
}

const TEST_JWT_SECRET: &str = "test-secret-for-integration-tests";

impl TestServer {
    /// Start a test server on a random port.
    ///
    /// Acquires a global lock so tests run serially against the shared DB.
    /// Requires `DATABASE_URL` env var pointing to a PostgreSQL instance.
    pub async fn start() -> Self {
        init_tracing();

        // Acquire DB lock — prevents parallel test races.
        let db_guard = db_lock().lock_owned().await;

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for integration tests");

        // Create pool and run migrations.
        let pool = db::create_pool(&database_url, 5).await.expect("test pool");
        db::run_migrations(&pool).await.expect("migrations");

        // Clean up test data from previous runs.
        Self::truncate_tables(&pool).await;

        let config = ServerConfig {
            server: ServerSection {
                host: "127.0.0.1".to_owned(),
                port: 0,
                ws_send_buffer_size: 64,
            },
            database: DatabaseSection {
                url: database_url,
                max_connections: 5,
            },
            auth: AuthSection {
                jwt_secret: TEST_JWT_SECRET.to_owned(),
            },
            limits: LimitsSection::default(),
            rate_limits: RateLimitsSection::default(),
        };

        let state = Arc::new(state::AppState::new(pool.clone(), config));
        let router = app::build_router(state.clone());

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("local_addr");

        let server_handle = tokio::spawn(async move {
            axum::serve(listener, router).await.expect("server");
        });

        let server = Self {
            addr,
            pool: pool.clone(),
            state: state.clone(),
            jwt_secret: TEST_JWT_SECRET.to_owned(),
            _server_handle: server_handle,
            _db_guard: db_guard,
        };

        // Seed test data.
        server.seed_data().await;

        server
    }

    async fn truncate_tables(pool: &PgPool) {
        sqlx::query(
            "TRUNCATE idempotency_keys, reactions, read_receipts, messages, \
             chat_members, dm_index, chats, sessions, user_info, users CASCADE",
        )
        .execute(pool)
        .await
        .expect("truncate");
    }

    /// Seed test data: two users and a shared chat.
    async fn seed_data(&self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // Users
        sqlx::query("INSERT INTO users (id, external_id, created_at, updated_at) VALUES (1, 'alice_ext', $1, $1)")
            .bind(now)
            .execute(&self.pool)
            .await
            .expect("seed alice");
        sqlx::query("INSERT INTO user_info (user_id, updated_at) VALUES (1, $1)")
            .bind(now)
            .execute(&self.pool)
            .await
            .expect("seed alice info");

        sqlx::query("INSERT INTO users (id, external_id, created_at, updated_at) VALUES (2, 'bob_ext', $1, $1)")
            .bind(now)
            .execute(&self.pool)
            .await
            .expect("seed bob");
        sqlx::query("INSERT INTO user_info (user_id, updated_at) VALUES (2, $1)")
            .bind(now)
            .execute(&self.pool)
            .await
            .expect("seed bob info");

        // Reset sequence so next auto-id starts after our seeded IDs.
        sqlx::query("SELECT setval('users_id_seq', 2)")
            .execute(&self.pool)
            .await
            .expect("reset users_id_seq");

        // Chat (group, id=1)
        sqlx::query(
            "INSERT INTO chats (id, kind, title, last_msg_id, created_at, updated_at) \
             VALUES (1, 1, 'Test Chat', 0, $1, $1)",
        )
        .bind(now)
        .execute(&self.pool)
        .await
        .expect("seed chat");

        sqlx::query("SELECT setval('chats_id_seq', 1)")
            .execute(&self.pool)
            .await
            .expect("reset chats_id_seq");

        // Members
        sqlx::query(
            "INSERT INTO chat_members (chat_id, user_id, role, joined_at, updated_at) VALUES (1, 1, 0, $1, $1)",
        )
        .bind(now)
        .execute(&self.pool)
        .await
        .expect("seed alice member");

        sqlx::query(
            "INSERT INTO chat_members (chat_id, user_id, role, joined_at, updated_at) VALUES (1, 2, 0, $1, $1)",
        )
        .bind(now)
        .execute(&self.pool)
        .await
        .expect("seed bob member");
    }

    /// Generate a JWT for the given external_id.
    pub fn jwt_for(&self, external_id: &str) -> String {
        #[derive(Serialize)]
        struct Claims {
            sub: String,
            exp: usize,
        }
        let claims = Claims {
            sub: external_id.to_owned(),
            exp: (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 3600) as usize,
        };
        jsonwebtoken::encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .expect("encode JWT")
    }

    pub fn ws_url(&self) -> String {
        format!("ws://{}/ws", self.addr)
    }

    /// Trigger shutdown.
    pub fn shutdown(&self) {
        let _ = self.state.shutdown_tx.send(true);
    }
}

// ---------------------------------------------------------------------------
// TestClient
// ---------------------------------------------------------------------------

type WsStream = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;

pub struct TestClient {
    pub ws_tx: SplitSink<WsStream, Message>,
    ws_rx: SplitStream<WsStream>,
    seq: u32,
}

impl TestClient {
    /// Connect to the test server's WebSocket endpoint.
    pub async fn connect(url: &str) -> Self {
        let (ws, _) = connect_async(url).await.expect("WS connect");
        let (ws_tx, ws_rx) = ws.split();
        Self { ws_tx, ws_rx, seq: 0 }
    }

    fn next_seq(&mut self) -> u32 {
        self.seq += 1;
        self.seq
    }

    /// Send a raw frame.
    async fn send_frame(&mut self, frame: &Frame) {
        let mut buf = Vec::with_capacity(128);
        encode_frame(&mut buf, frame).expect("encode frame");
        self.ws_tx.send(Message::Binary(buf.into())).await.expect("WS send");
    }

    /// Receive and decode the next frame, with timeout.
    pub async fn recv_frame(&mut self, dur: Duration) -> Option<Frame> {
        let msg = timeout(dur, self.ws_rx.next()).await.ok()??;
        match msg {
            Ok(Message::Binary(data)) => {
                let mut buf = &data[..];
                Some(decode_frame(&mut buf).expect("decode frame"))
            }
            Ok(Message::Close(_)) => None,
            // Connection reset without close handshake — treat as disconnection.
            Err(_) => None,
            _ => None,
        }
    }

    /// Perform Hello handshake, return WelcomePayload.
    pub async fn hello(&mut self, token: &str, device_id: Uuid) -> WelcomePayload {
        let frame = Frame {
            seq: 0,
            event_seq: 0,
            payload: FramePayload::Hello(HelloPayload {
                protocol_version: PROTOCOL_VERSION,
                sdk_version: "test".to_owned(),
                platform: "test".to_owned(),
                token: token.to_owned(),
                device_id,
            }),
        };
        self.send_frame(&frame).await;

        let resp = self.recv_frame(Duration::from_secs(5)).await.expect("Welcome frame");
        match resp.payload {
            FramePayload::Welcome(w) => w,
            FramePayload::Error(e) => panic!("Hello failed: {:?} — {}", e.code, e.message),
            other => panic!("expected Welcome, got {other:?}"),
        }
    }

    /// Subscribe to channels.
    pub async fn subscribe(&mut self, channels: Vec<String>) {
        let seq = self.next_seq();
        let frame = Frame {
            seq,
            event_seq: 0,
            payload: FramePayload::Subscribe(SubscribePayload { channels }),
        };
        self.send_frame(&frame).await;

        let resp = self.recv_frame(Duration::from_secs(5)).await.expect("Subscribe ack");
        match resp.payload {
            FramePayload::Ack(_) => {}
            FramePayload::Error(e) => panic!("Subscribe failed: {:?} — {}", e.code, e.message),
            other => panic!("expected Ack, got {other:?}"),
        }
    }

    /// Send a message and return the server-assigned message ID.
    pub async fn send_message(&mut self, chat_id: u32, content: &str, key: Uuid) -> u32 {
        let seq = self.next_seq();
        let frame = Frame {
            seq,
            event_seq: 0,
            payload: FramePayload::SendMessage(SendMessagePayload {
                chat_id,
                kind: chat_protocol::types::MessageKind::Text,
                idempotency_key: key,
                reply_to_id: None,
                content: content.to_owned(),
                rich_content: None,
                extra: None,
                mentioned_user_ids: vec![],
            }),
        };
        self.send_frame(&frame).await;

        let resp = self.recv_frame(Duration::from_secs(5)).await.expect("SendMessage ack");
        match resp.payload {
            FramePayload::Ack(ack) => {
                // Ack payload is raw bytes — extract the u32 message ID.
                match ack {
                    AckPayload::Empty => panic!("expected MessageId ack, got Empty"),
                    AckPayload::MessageId(id) => id,
                    _ => {
                        // Raw bytes variant — decode u32 from first 4 bytes.
                        let raw = match ack {
                            AckPayload::MessageBatch(b) => b,
                            _ => panic!("unexpected ack variant"),
                        };
                        if raw.len() >= 4 {
                            let mut buf = &raw[..];
                            buf.get_u32_le()
                        } else {
                            panic!("ack payload too short: {} bytes", raw.len());
                        }
                    }
                }
            }
            FramePayload::Error(e) => panic!("SendMessage failed: {:?} — {}", e.code, e.message),
            other => panic!("expected Ack, got {other:?}"),
        }
    }
}
