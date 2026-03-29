//! Server benchmarks: message throughput and connection latency.
//!
//! Requires `DATABASE_URL` env var pointing to a PostgreSQL instance.
//! Run: `DATABASE_URL=postgres://chat:chat@localhost/chat_db cargo bench -p chat_server`

use std::sync::Arc;
use std::time::Duration;

use bytes::Buf;
use chat_protocol::PROTOCOL_VERSION;
use chat_protocol::codec::{decode_frame, encode_frame};
use chat_protocol::types::{
    AckPayload, Frame, FramePayload, HelloPayload, MessageKind, SendMessagePayload, SubscribePayload, WelcomePayload,
};
use chat_server::config::{
    AuthSection, DatabaseSection, LimitsSection, RateLimitsSection, ServerConfig, ServerSection,
};
use chat_server::{app, db, state};
use criterion::{Criterion, criterion_group, criterion_main};
use futures_util::{SinkExt, StreamExt};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::Serialize;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use uuid::Uuid;

const BENCH_JWT_SECRET: &str = "bench-secret";

struct BenchServer {
    addr: std::net::SocketAddr,
    pool: PgPool,
    _handle: tokio::task::JoinHandle<()>,
}

async fn start_bench_server() -> BenchServer {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::create_pool(&database_url, 10).await.expect("pool");
    db::run_migrations(&pool).await.expect("migrations");

    // Truncate + seed.
    sqlx::query(
        "TRUNCATE idempotency_keys, reactions, read_receipts, messages, \
         chat_members, dm_index, chats, sessions, user_info, users CASCADE",
    )
    .execute(&pool)
    .await
    .expect("truncate");

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    sqlx::query("INSERT INTO users (id, external_id, created_at, updated_at) VALUES (1, 'sender', $1, $1)")
        .bind(now)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO user_info (user_id, updated_at) VALUES (1, $1)")
        .bind(now)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO users (id, external_id, created_at, updated_at) VALUES (2, 'receiver', $1, $1)")
        .bind(now)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO user_info (user_id, updated_at) VALUES (2, $1)")
        .bind(now)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("SELECT setval('users_id_seq', 2)")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO chats (id, kind, title, last_msg_id, created_at, updated_at) VALUES (1, 1, 'Bench', 0, $1, $1)",
    )
    .bind(now)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query("SELECT setval('chats_id_seq', 1)")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO chat_members (chat_id, user_id, role, joined_at, updated_at) VALUES (1, 1, 0, $1, $1)")
        .bind(now)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO chat_members (chat_id, user_id, role, joined_at, updated_at) VALUES (1, 2, 0, $1, $1)")
        .bind(now)
        .execute(&pool)
        .await
        .unwrap();

    let config = ServerConfig {
        server: ServerSection {
            host: "127.0.0.1".into(),
            port: 0,
            ws_send_buffer_size: 256,
        },
        database: DatabaseSection {
            url: database_url,
            max_connections: 10,
        },
        auth: AuthSection {
            jwt_secret: BENCH_JWT_SECRET.into(),
        },
        limits: LimitsSection::default(),
        rate_limits: RateLimitsSection::default(),
    };
    let state = Arc::new(state::AppState::new(pool.clone(), config));
    let router = app::build_router(state);
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let handle = tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });

    BenchServer {
        addr,
        pool,
        _handle: handle,
    }
}

fn make_jwt(sub: &str) -> String {
    #[derive(Serialize)]
    struct C {
        sub: String,
        exp: usize,
    }
    let c = C {
        sub: sub.into(),
        exp: (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 3600) as usize,
    };
    jsonwebtoken::encode(
        &Header::new(Algorithm::HS256),
        &c,
        &EncodingKey::from_secret(BENCH_JWT_SECRET.as_bytes()),
    )
    .unwrap()
}

type WsStream = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;

async fn ws_connect(addr: &std::net::SocketAddr) -> WsStream {
    let (ws, _) = connect_async(format!("ws://{addr}/ws")).await.unwrap();
    ws
}

async fn ws_hello(ws: &mut WsStream, token: &str, device_id: Uuid) -> WelcomePayload {
    let frame = Frame {
        seq: 0,
        event_seq: 0,
        payload: FramePayload::Hello(HelloPayload {
            protocol_version: PROTOCOL_VERSION,
            sdk_version: "bench".into(),
            platform: "bench".into(),
            token: token.into(),
            device_id,
        }),
    };
    let mut buf = Vec::new();
    encode_frame(&mut buf, &frame).unwrap();
    ws.send(Message::Binary(buf.into())).await.unwrap();
    let msg = ws.next().await.unwrap().unwrap();
    let Message::Binary(data) = msg else {
        panic!("expected binary")
    };
    let resp = decode_frame(&mut &data[..]).unwrap();
    match resp.payload {
        FramePayload::Welcome(w) => w,
        other => panic!("expected Welcome, got {other:?}"),
    }
}

async fn ws_subscribe(ws: &mut WsStream, channels: Vec<String>) {
    let frame = Frame {
        seq: 1,
        event_seq: 0,
        payload: FramePayload::Subscribe(SubscribePayload { channels }),
    };
    let mut buf = Vec::new();
    encode_frame(&mut buf, &frame).unwrap();
    ws.send(Message::Binary(buf.into())).await.unwrap();
    // Read ack.
    let _ = ws.next().await.unwrap().unwrap();
}

fn bench_connection_latency(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let server = rt.block_on(start_bench_server());

    c.bench_function("connection_hello_welcome", |b| {
        b.to_async(&rt).iter(|| async {
            let mut ws = ws_connect(&server.addr).await;
            let _welcome = ws_hello(&mut ws, &make_jwt("sender"), Uuid::new_v4()).await;
            let _ = ws.close(None).await;
        });
    });
}

fn bench_message_throughput(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let server = rt.block_on(start_bench_server());

    c.bench_function("send_message_roundtrip", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                // Fresh connections per measurement to avoid stale state.
                let mut sender = ws_connect(&server.addr).await;
                ws_hello(&mut sender, &make_jwt("sender"), Uuid::new_v4()).await;
                ws_subscribe(&mut sender, vec!["chat#1".into()]).await;

                let mut receiver = ws_connect(&server.addr).await;
                ws_hello(&mut receiver, &make_jwt("receiver"), Uuid::new_v4()).await;
                ws_subscribe(&mut receiver, vec!["chat#1".into()]).await;

                let start = std::time::Instant::now();
                for i in 0..iters {
                    let seq = (i + 100) as u32;
                    let frame = Frame {
                        seq,
                        event_seq: 0,
                        payload: FramePayload::SendMessage(SendMessagePayload {
                            chat_id: 1,
                            kind: MessageKind::Text,
                            idempotency_key: Uuid::new_v4(),
                            reply_to_id: None,
                            content: "bench message".into(),
                            rich_content: None,
                            extra: None,
                            mentioned_user_ids: vec![],
                        }),
                    };
                    let mut buf = Vec::with_capacity(64);
                    encode_frame(&mut buf, &frame).unwrap();
                    sender.send(Message::Binary(buf.into())).await.unwrap();

                    // Read Ack.
                    let msg = sender.next().await.unwrap().unwrap();
                    let Message::Binary(_) = msg else {
                        panic!("expected binary ack")
                    };

                    // Read MessageNew on receiver.
                    let msg = timeout(Duration::from_secs(5), receiver.next())
                        .await
                        .unwrap()
                        .unwrap()
                        .unwrap();
                    let Message::Binary(_) = msg else {
                        panic!("expected binary event")
                    };
                }
                start.elapsed()
            })
        });
    });
}

criterion_group!(benches, bench_connection_latency, bench_message_throughput);
criterion_main!(benches);
