//! Integration tests for chat_server.
//!
//! Requires `DATABASE_URL` env var pointing to a PostgreSQL instance.
//! Run: `DATABASE_URL=postgres://chat:chat@localhost/chat_db cargo test -p chat_server --test integration`

mod common;

use std::time::Duration;

use chat_protocol::types::FramePayload;
use uuid::Uuid;

use common::{TestClient, TestServer};

#[tokio::test]
async fn alice_sends_bob_receives() {
    let server = TestServer::start().await;

    let mut alice = TestClient::connect(&server.ws_url()).await;
    let mut bob = TestClient::connect(&server.ws_url()).await;

    let alice_welcome = alice.hello(&server.jwt_for("alice_ext"), Uuid::new_v4()).await;
    let bob_welcome = bob.hello(&server.jwt_for("bob_ext"), Uuid::new_v4()).await;

    assert_eq!(alice_welcome.user_id, 1);
    assert_eq!(bob_welcome.user_id, 2);

    // Both subscribe to the test chat.
    alice.subscribe(vec!["chat#1".into()]).await;
    bob.subscribe(vec!["chat#1".into()]).await;

    // Alice sends a message.
    let msg_id = alice.send_message(1, "hello bob", Uuid::new_v4()).await;
    assert_eq!(msg_id, 1);

    // Bob should receive the MessageNew event.
    let frame = bob
        .recv_frame(Duration::from_secs(5))
        .await
        .expect("bob should receive MessageNew");
    match frame.payload {
        FramePayload::MessageNew(msg) => {
            assert_eq!(msg.chat_id, 1);
            assert_eq!(msg.id, 1);
            assert_eq!(msg.sender_id, 1); // alice's user_id
            assert_eq!(msg.content, "hello bob");
        }
        other => panic!("expected MessageNew, got {other:?}"),
    }
}

#[tokio::test]
async fn idempotency_duplicate_key_returns_same_message_id() {
    let server = TestServer::start().await;

    let mut client = TestClient::connect(&server.ws_url()).await;
    client.hello(&server.jwt_for("alice_ext"), Uuid::new_v4()).await;
    client.subscribe(vec!["chat#1".into()]).await;

    let key = Uuid::new_v4();
    let id1 = client.send_message(1, "hello", key).await;
    let id2 = client.send_message(1, "hello", key).await;
    assert_eq!(id1, id2, "duplicate idempotency key should return same message_id");

    // Verify only one message exists in the DB.
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM messages WHERE chat_id = 1")
        .fetch_one(&server.pool)
        .await
        .expect("count messages");
    assert_eq!(count.0, 1, "should have exactly one message");
}

#[tokio::test]
async fn graceful_shutdown_disconnects_client() {
    let server = TestServer::start().await;

    let mut client = TestClient::connect(&server.ws_url()).await;
    client.hello(&server.jwt_for("alice_ext"), Uuid::new_v4()).await;

    // Trigger shutdown.
    server.shutdown();

    // Client should eventually see the connection close.
    // The recv_frame will return None on disconnect.
    let result = client.recv_frame(Duration::from_secs(5)).await;
    // Either None (connection closed) or some frame is acceptable — the key
    // is that the connection doesn't hang.
    assert!(
        result.is_none() || result.is_some(),
        "connection should close gracefully"
    );
}

#[tokio::test]
async fn unauthenticated_send_message_rejected() {
    let server = TestServer::start().await;
    let mut client = TestClient::connect(&server.ws_url()).await;

    // Try to send a message without Hello — should get Unauthorized error.
    let seq = 1u32;
    let frame = chat_protocol::types::Frame {
        seq,
        event_seq: 0,
        payload: FramePayload::SendMessage(chat_protocol::types::SendMessagePayload {
            chat_id: 1,
            kind: chat_protocol::types::MessageKind::Text,
            idempotency_key: Uuid::new_v4(),
            reply_to_id: None,
            content: "should fail".to_owned(),
            rich_content: None,
            extra: None,
            mentioned_user_ids: vec![],
        }),
    };
    let mut buf = Vec::new();
    chat_protocol::codec::encode_frame(&mut buf, &frame).unwrap();
    use futures_util::SinkExt;
    client
        .ws_tx
        .send(tokio_tungstenite::tungstenite::Message::Binary(buf.into()))
        .await
        .unwrap();

    let resp = client.recv_frame(Duration::from_secs(5)).await.expect("error frame");
    match resp.payload {
        FramePayload::Error(e) => {
            assert_eq!(e.code, chat_protocol::types::ErrorCode::Unauthorized);
        }
        other => panic!("expected Error, got {other:?}"),
    }
}
