/// TS codec benchmarks — mirrors crates/chat_protocol/benches/codec_bench.rs
/// and packages/chat_core_dart/benchmark/codec_benchmark.dart.
///
/// Run: npx vitest bench

import { bench, describe } from "vitest";
import {
  ProtocolWriter,
  ProtocolReader,
  FrameKind,
  MessageKind,
  RichStyle,
  encodeFrameHeader,
  decodeFrameHeader,
  encodeMessageBatch,
  decodeMessageBatch,
  encodeRichSpan,
  decodeRichSpan,
} from "../src/index.js";
import type {
  FrameHeader,
  Message,
  MessageBatch,
  RichSpan,
} from "../src/index.js";

// ---------------------------------------------------------------------------
// Fixtures — match Rust/Dart bench data exactly
// ---------------------------------------------------------------------------

function sampleMessage(id: number): Message {
  return {
    id,
    chatId: 1,
    senderId: 42,
    createdAt: 1711100000,
    updatedAt: 1711100000,
    kind: MessageKind.Text,
    flags: 0,
    replyToId: null,
    content: "Hello, this is a test message with some content.",
    richContent: null,
    extra: null,
  };
}

function sampleMessageWithRich(id: number): Message {
  return {
    id,
    chatId: 1,
    senderId: 42,
    createdAt: 1711100000,
    updatedAt: 1711100000,
    kind: MessageKind.Text,
    flags: 0x0001, // EDITED
    replyToId: null,
    content:
      "Hello, this is a rich message with bold and italic and links to places",
    richContent: [
      { start: 38, end: 42, style: RichStyle.BOLD, meta: null },
      { start: 47, end: 53, style: RichStyle.ITALIC, meta: null },
      {
        start: 58,
        end: 63,
        style: RichStyle.LINK,
        meta: '{"url":"https://example.com"}',
      },
    ],
    extra: '{"reply":{"chat_id":1,"msg_id":5}}',
  };
}

function richSpans50(): RichSpan[] {
  return Array.from({ length: 50 }, (_, i) => {
    const base = i * 20;
    if (i % 5 === 0) {
      return {
        start: base,
        end: base + 15,
        style: RichStyle.LINK,
        meta: `{"url":"https://example.com/${i}"}`,
      };
    } else if (i % 3 === 0) {
      return {
        start: base,
        end: base + 10,
        style: RichStyle.MENTION,
        meta: `{"user_id":${i}}`,
      };
    } else {
      return {
        start: base,
        end: base + 10,
        style: RichStyle.BOLD | RichStyle.ITALIC,
        meta: null,
      };
    }
  });
}

// ---------------------------------------------------------------------------
// Header benchmarks
// ---------------------------------------------------------------------------

describe("header", () => {
  const header: FrameHeader = {
    kind: FrameKind.SendMessage,
    seq: 42,
    eventSeq: 0,
  };

  bench("header_encode", () => {
    const w = new ProtocolWriter(16);
    encodeFrameHeader(w, header);
    w.toBytes();
  });

  const encodedHeader = (() => {
    const w = new ProtocolWriter(16);
    encodeFrameHeader(w, header);
    return w.toBytes();
  })();

  bench("header_decode", () => {
    const r = new ProtocolReader(encodedHeader);
    decodeFrameHeader(r);
  });
});

// ---------------------------------------------------------------------------
// Message batch 1000 (simple text)
// ---------------------------------------------------------------------------

describe("batch_1000", () => {
  const batch: MessageBatch = {
    messages: Array.from({ length: 1000 }, (_, i) => sampleMessage(i)),
    hasMore: false,
  };

  bench("batch_1000_encode", () => {
    const w = new ProtocolWriter(65536);
    encodeMessageBatch(w, batch);
  });

  const encodedBatch = (() => {
    const w = new ProtocolWriter(65536);
    encodeMessageBatch(w, batch);
    return w.toBytes();
  })();

  bench("batch_1000_decode", () => {
    const r = new ProtocolReader(encodedBatch);
    decodeMessageBatch(r);
  });
});

// ---------------------------------------------------------------------------
// Rich content — 50 spans
// ---------------------------------------------------------------------------

describe("rich_50spans", () => {
  const spans = richSpans50();

  bench("rich_50spans_encode", () => {
    const w = new ProtocolWriter(2048);
    w.writeU16(spans.length);
    for (const span of spans) encodeRichSpan(w, span);
  });

  const encodedSpans = (() => {
    const w = new ProtocolWriter(2048);
    w.writeU16(spans.length);
    for (const span of spans) encodeRichSpan(w, span);
    return w.toBytes();
  })();

  bench("rich_50spans_decode", () => {
    const r = new ProtocolReader(encodedSpans);
    r.readArray(r.readU16(), () => decodeRichSpan(r));
  });
});

// ---------------------------------------------------------------------------
// Message batch 100 (with rich content)
// ---------------------------------------------------------------------------

describe("batch_100_rich", () => {
  const batch: MessageBatch = {
    messages: Array.from({ length: 100 }, (_, i) =>
      sampleMessageWithRich(i),
    ),
    hasMore: false,
  };

  bench("batch_100_rich_encode", () => {
    const w = new ProtocolWriter(32768);
    encodeMessageBatch(w, batch);
  });

  const encodedBatch = (() => {
    const w = new ProtocolWriter(32768);
    encodeMessageBatch(w, batch);
    return w.toBytes();
  })();

  bench("batch_100_rich_decode", () => {
    const r = new ProtocolReader(encodedBatch);
    decodeMessageBatch(r);
  });
});
