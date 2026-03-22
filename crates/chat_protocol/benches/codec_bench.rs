//! Criterion benchmarks for chat_protocol codec.
//!
//! ## Baseline (2026-03-22, MacBook Pro M3 Pro, Rust 1.85, release profile)
//!
//! | Benchmark              | Time        |
//! |------------------------|-------------|
//! | header_encode          |    935 ps   |
//! | header_decode          |   3.80 ns   |
//! | batch_1000_encode      |   6.93 µs   |
//! | batch_1000_decode      |  32.68 µs   |
//! | rich_50spans_encode    | 183.48 ns   |
//! | rich_50spans_decode    | 863.81 ns   |
//! | batch_100_rich_encode  |   2.27 µs   |
//! | batch_100_rich_decode  |  13.30 µs   |

use bytes::BytesMut;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

use chat_protocol::codec::*;
use chat_protocol::types::*;

fn sample_message(id: u32) -> Message {
    Message {
        id,
        chat_id: 1,
        sender_id: 42,
        created_at: 1_711_100_000,
        updated_at: 1_711_100_000,
        kind: MessageKind::Text,
        flags: MessageFlags::empty(),
        content: "Hello, this is a test message with some content.".into(),
        rich_content: None,
        extra: None,
    }
}

fn sample_message_with_rich(id: u32) -> Message {
    Message {
        id,
        chat_id: 1,
        sender_id: 42,
        created_at: 1_711_100_000,
        updated_at: 1_711_100_000,
        kind: MessageKind::Text,
        flags: MessageFlags::EDITED,
        content: "Hello, this is a rich message with bold and italic and links to places".into(),
        rich_content: Some(vec![
            RichSpan {
                start: 38,
                end: 42,
                style: RichStyle::BOLD,
                meta: None,
            },
            RichSpan {
                start: 47,
                end: 53,
                style: RichStyle::ITALIC,
                meta: None,
            },
            RichSpan {
                start: 58,
                end: 63,
                style: RichStyle::LINK,
                meta: Some(r#"{"url":"https://example.com"}"#.into()),
            },
        ]),
        extra: Some(r#"{"reply":{"chat_id":1,"msg_id":5}}"#.into()),
    }
}

fn bench_frame_header(c: &mut Criterion) {
    let header = FrameHeader {
        kind: FrameKind::SendMessage,
        seq: 42,
        event_seq: 0,
    };

    c.bench_function("header_encode", |b| {
        let mut buf = BytesMut::with_capacity(9);
        b.iter(|| {
            buf.clear();
            encode_header(&mut buf, black_box(&header));
        });
    });

    c.bench_function("header_decode", |b| {
        let mut encode_buf = BytesMut::with_capacity(9);
        encode_header(&mut encode_buf, &header);
        let data = encode_buf.freeze();
        b.iter(|| {
            let mut cursor = data.clone();
            black_box(decode_header(&mut cursor).unwrap());
        });
    });
}

fn bench_message_batch_1000(c: &mut Criterion) {
    let batch = MessageBatch {
        messages: (0..1000).map(sample_message).collect(),
    };

    // Pre-encode for decode benchmark
    let mut pre_encoded = BytesMut::new();
    encode_message_batch(&mut pre_encoded, &batch).unwrap();
    let encoded_data = pre_encoded.freeze();

    c.bench_function("batch_1000_encode", |b| {
        let mut buf = BytesMut::with_capacity(encoded_data.len());
        b.iter(|| {
            buf.clear();
            encode_message_batch(&mut buf, black_box(&batch)).unwrap();
        });
    });

    c.bench_function("batch_1000_decode", |b| {
        b.iter(|| {
            let mut cursor = encoded_data.clone();
            black_box(decode_message_batch(&mut cursor).unwrap());
        });
    });
}

fn bench_rich_content_50_spans(c: &mut Criterion) {
    let spans: Vec<RichSpan> = (0..50)
        .map(|i| {
            let base = i * 20;
            if i % 5 == 0 {
                RichSpan {
                    start: base,
                    end: base + 15,
                    style: RichStyle::LINK,
                    meta: Some(format!(r#"{{"url":"https://example.com/{i}"}}"#)),
                }
            } else if i % 3 == 0 {
                RichSpan {
                    start: base,
                    end: base + 10,
                    style: RichStyle::MENTION,
                    meta: Some(format!(r#"{{"user_id":{i}}}"#)),
                }
            } else {
                RichSpan {
                    start: base,
                    end: base + 10,
                    style: RichStyle::BOLD | RichStyle::ITALIC,
                    meta: None,
                }
            }
        })
        .collect();

    let mut pre_encoded = BytesMut::new();
    encode_rich_content(&mut pre_encoded, &spans);
    let encoded_data = pre_encoded.freeze();

    c.bench_function("rich_50spans_encode", |b| {
        let mut buf = BytesMut::with_capacity(encoded_data.len());
        b.iter(|| {
            buf.clear();
            encode_rich_content(&mut buf, black_box(&spans));
        });
    });

    c.bench_function("rich_50spans_decode", |b| {
        b.iter(|| {
            let mut cursor = encoded_data.clone();
            black_box(decode_rich_content(&mut cursor).unwrap());
        });
    });
}

fn bench_message_with_rich(c: &mut Criterion) {
    let batch = MessageBatch {
        messages: (0..100).map(sample_message_with_rich).collect(),
    };

    let mut pre_encoded = BytesMut::new();
    encode_message_batch(&mut pre_encoded, &batch).unwrap();
    let encoded_data = pre_encoded.freeze();

    c.bench_function("batch_100_rich_encode", |b| {
        let mut buf = BytesMut::with_capacity(encoded_data.len());
        b.iter(|| {
            buf.clear();
            encode_message_batch(&mut buf, black_box(&batch)).unwrap();
        });
    });

    c.bench_function("batch_100_rich_decode", |b| {
        b.iter(|| {
            let mut cursor = encoded_data.clone();
            black_box(decode_message_batch(&mut cursor).unwrap());
        });
    });
}

criterion_group!(
    benches,
    bench_frame_header,
    bench_message_batch_1000,
    bench_rich_content_50_spans,
    bench_message_with_rich,
);
criterion_main!(benches);
