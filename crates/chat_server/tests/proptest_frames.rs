//! Property-based tests: random frame sequences must not panic the server.
//!
//! Requires `DATABASE_URL` env var pointing to a PostgreSQL instance.

#[allow(dead_code)]
mod common;

use std::time::Duration;

use common::{TestClient, TestServer};
use futures_util::SinkExt;
use proptest::prelude::*;
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

/// Send a sequence of random byte blobs to the server over WS.
/// The server must not panic — it should either ignore or respond with Error frames.
fn run_random_bytes_test(payloads: Vec<Vec<u8>>) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let server = TestServer::start().await;
        let mut client = TestClient::connect(&server.ws_url()).await;

        // Authenticate first so we exercise the dispatch path.
        client.hello(&server.jwt_for("alice_ext"), Uuid::new_v4()).await;

        for payload in &payloads {
            // Ignore send errors — connection may close.
            let _ = client.ws_tx.send(Message::Binary(payload.clone().into())).await;
        }

        // Small delay to let the server process.
        tokio::time::sleep(Duration::from_millis(50)).await;

        // The test passes if we reach this point without the server panicking.
        // Try to read any pending response (may be error frames or nothing).
        while let Some(_frame) = client.recv_frame(Duration::from_millis(100)).await {
            // Drain responses.
        }
    });
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    #[test]
    fn random_bytes_dont_crash_server(
        payloads in prop::collection::vec(
            prop::collection::vec(any::<u8>(), 0..256),
            1..10
        )
    ) {
        run_random_bytes_test(payloads);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    #[test]
    fn random_valid_header_with_garbage_payload(
        kind in any::<u8>(),
        seq in any::<u32>(),
        event_seq in any::<u32>(),
        payload in prop::collection::vec(any::<u8>(), 0..128),
    ) {
        // Build a frame with valid 9-byte header but random payload.
        let mut frame_bytes = Vec::with_capacity(9 + payload.len());
        use bytes::BufMut;
        frame_bytes.put_u8(kind);
        frame_bytes.put_u32_le(seq);
        frame_bytes.put_u32_le(event_seq);
        frame_bytes.extend_from_slice(&payload);

        run_random_bytes_test(vec![frame_bytes]);
    }
}
