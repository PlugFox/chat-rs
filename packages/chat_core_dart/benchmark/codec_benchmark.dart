/// Dart codec benchmarks — mirrors crates/chat_protocol/benches/codec_bench.rs.
///
/// Run: dart run benchmark/codec_benchmark.dart
///
/// Baseline (2026-03-22, MacBook Pro M3 Pro):
///   Rust: criterion, release profile
///   Dart: benchmark_harness
///
/// After optimizations (writer reuse, ASCII fast path, zero-copy reads,
/// inline rich content encoding via reserve+patch):
///
/// | Benchmark              | Rust        | Dart JIT    | Dart AOT    | Ratio AOT |
/// |------------------------|-------------|-------------|-------------|-----------|
/// | header_encode          |    935 ps   |  186 ns     |  216 ns     |    230x   |
/// | header_decode          |   3.80 ns   |  198 ns     |  240 ns     |     63x   |
/// | batch_1000_encode      |   6.93 us   |    1.18 ms  |  965 us     |    139x   |
/// | batch_1000_decode      |  32.68 us   |  819 us     |  795 us     |     24x   |
/// | rich_50spans_encode    | 183.48 ns   |   14.2 us   |   14.5 us   |     79x   |
/// | rich_50spans_decode    | 863.81 ns   |   14.1 us   |   15.4 us   |     18x   |
/// | batch_100_rich_encode  |   2.27 us   |  308 us     |  269 us     |    119x   |
/// | batch_100_rich_decode  |  13.30 us   |  299 us     |  252 us     |     19x   |
///
/// Encode: 79-230x (remaining cost: utf8.encode alloc, object creation overhead).
/// Decode: 18-63x (remaining cost: object allocation, GC pressure).
/// Header benchmarks have highest ratio due to fixed per-call overhead vs Rust's
/// sub-nanosecond writes to a pre-allocated buffer.

import 'dart:typed_data';

import 'package:benchmark_harness/benchmark_harness.dart';
import 'package:chat_core/chat_core.dart';

// ---------------------------------------------------------------------------
// Fixtures — match Rust bench data exactly
// ---------------------------------------------------------------------------

Message _sampleMessage(int id) => Message(
  id: id,
  chatId: 1,
  senderId: 42,
  createdAt: 1711100000,
  updatedAt: 1711100000,
  kind: MessageKind.text,
  flags: MessageFlags(0),
  replyToId: null,
  content: 'Hello, this is a test message with some content.',
  richContent: null,
  extra: null,
);

Message _sampleMessageWithRich(int id) => Message(
  id: id,
  chatId: 1,
  senderId: 42,
  createdAt: 1711100000,
  updatedAt: 1711100000,
  kind: MessageKind.text,
  flags: MessageFlags.edited,
  replyToId: null,
  content:
      'Hello, this is a rich message with bold and italic and links to places',
  richContent: [
    RichSpan(start: 38, end: 42, style: RichStyle.bold, meta: null),
    RichSpan(start: 47, end: 53, style: RichStyle.italic, meta: null),
    RichSpan(
      start: 58,
      end: 63,
      style: RichStyle.link,
      meta: '{"url":"https://example.com"}',
    ),
  ],
  extra: '{"reply":{"chat_id":1,"msg_id":5}}',
);

List<RichSpan> _richSpans50() => List.generate(50, (i) {
  final base = i * 20;
  if (i % 5 == 0) {
    return RichSpan(
      start: base,
      end: base + 15,
      style: RichStyle.link,
      meta: '{"url":"https://example.com/$i"}',
    );
  } else if (i % 3 == 0) {
    return RichSpan(
      start: base,
      end: base + 10,
      style: RichStyle.mention,
      meta: '{"user_id":$i}',
    );
  } else {
    return RichSpan(
      start: base,
      end: base + 10,
      style: RichStyle.bold.add(RichStyle.italic),
      meta: null,
    );
  }
});

// ---------------------------------------------------------------------------
// Header benchmarks
// ---------------------------------------------------------------------------

class HeaderEncodeBenchmark extends BenchmarkBase {
  HeaderEncodeBenchmark() : super('header_encode');

  final _header = FrameHeader(
    kind: FrameKind.sendMessage,
    seq: 42,
    eventSeq: 0,
  );
  late ProtocolWriter _w;

  @override
  void setup() {
    _w = ProtocolWriter(16);
  }

  @override
  void run() {
    _w.reset();
    encodeFrameHeader(_w, _header);
    _w.toBytesView();
  }
}

class HeaderDecodeBenchmark extends BenchmarkBase {
  HeaderDecodeBenchmark() : super('header_decode');

  late Uint8List _encoded;

  @override
  void setup() {
    final w = ProtocolWriter(16);
    encodeFrameHeader(
      w,
      FrameHeader(kind: FrameKind.sendMessage, seq: 42, eventSeq: 0),
    );
    _encoded = w.toBytes();
  }

  @override
  void run() {
    final r = ProtocolReader(_encoded);
    decodeFrameHeader(r);
  }
}

// ---------------------------------------------------------------------------
// Message batch 1000 (simple text)
// ---------------------------------------------------------------------------

class Batch1000EncodeBenchmark extends BenchmarkBase {
  Batch1000EncodeBenchmark() : super('batch_1000_encode');

  late MessageBatch _batch;
  late ProtocolWriter _w;

  @override
  void setup() {
    _batch = MessageBatch(
      messages: List.generate(1000, _sampleMessage),
      hasMore: false,
    );
    // Warm up to pre-grow buffer
    _w = ProtocolWriter(65536);
    encodeMessageBatch(_w, _batch);
  }

  @override
  void run() {
    _w.reset();
    encodeMessageBatch(_w, _batch);
  }
}

class Batch1000DecodeBenchmark extends BenchmarkBase {
  Batch1000DecodeBenchmark() : super('batch_1000_decode');

  late Uint8List _encoded;

  @override
  void setup() {
    final batch = MessageBatch(
      messages: List.generate(1000, _sampleMessage),
      hasMore: false,
    );
    final w = ProtocolWriter(65536);
    encodeMessageBatch(w, batch);
    _encoded = w.toBytes();
  }

  @override
  void run() {
    final r = ProtocolReader(_encoded);
    decodeMessageBatch(r);
  }
}

// ---------------------------------------------------------------------------
// Rich content — 50 spans
// ---------------------------------------------------------------------------

class Rich50SpansEncodeBenchmark extends BenchmarkBase {
  Rich50SpansEncodeBenchmark() : super('rich_50spans_encode');

  late List<RichSpan> _spans;
  late ProtocolWriter _w;

  @override
  void setup() {
    _spans = _richSpans50();
    _w = ProtocolWriter(2048);
  }

  @override
  void run() {
    _w.reset();
    _w.writeU16(_spans.length);
    for (final span in _spans) {
      encodeRichSpan(_w, span);
    }
  }
}

class Rich50SpansDecodeBenchmark extends BenchmarkBase {
  Rich50SpansDecodeBenchmark() : super('rich_50spans_decode');

  late Uint8List _encoded;

  @override
  void setup() {
    final spans = _richSpans50();
    final w = ProtocolWriter(2048);
    w.writeU16(spans.length);
    for (final span in spans) {
      encodeRichSpan(w, span);
    }
    _encoded = w.toBytes();
  }

  @override
  void run() {
    final r = ProtocolReader(_encoded);
    r.readArray(r.readU16(), () => decodeRichSpan(r));
  }
}

// ---------------------------------------------------------------------------
// Message batch 100 (with rich content)
// ---------------------------------------------------------------------------

class Batch100RichEncodeBenchmark extends BenchmarkBase {
  Batch100RichEncodeBenchmark() : super('batch_100_rich_encode');

  late MessageBatch _batch;
  late ProtocolWriter _w;

  @override
  void setup() {
    _batch = MessageBatch(
      messages: List.generate(100, _sampleMessageWithRich),
      hasMore: false,
    );
    _w = ProtocolWriter(32768);
    encodeMessageBatch(_w, _batch);
  }

  @override
  void run() {
    _w.reset();
    encodeMessageBatch(_w, _batch);
  }
}

class Batch100RichDecodeBenchmark extends BenchmarkBase {
  Batch100RichDecodeBenchmark() : super('batch_100_rich_decode');

  late Uint8List _encoded;

  @override
  void setup() {
    final batch = MessageBatch(
      messages: List.generate(100, _sampleMessageWithRich),
      hasMore: false,
    );
    final w = ProtocolWriter(32768);
    encodeMessageBatch(w, batch);
    _encoded = w.toBytes();
  }

  @override
  void run() {
    final r = ProtocolReader(_encoded);
    decodeMessageBatch(r);
  }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

void main() {
  // Rust baselines (MacBook Pro M3 Pro, 2026-03-22)
  const rustBaselines = {
    'header_encode': 0.000935, // 935 ps in us
    'header_decode': 0.00380, // 3.80 ns in us
    'batch_1000_encode': 6.93,
    'batch_1000_decode': 32.68,
    'rich_50spans_encode': 0.18348,
    'rich_50spans_decode': 0.86381,
    'batch_100_rich_encode': 2.27,
    'batch_100_rich_decode': 13.30,
  };

  final benchmarks = <BenchmarkBase>[
    HeaderEncodeBenchmark(),
    HeaderDecodeBenchmark(),
    Batch1000EncodeBenchmark(),
    Batch1000DecodeBenchmark(),
    Rich50SpansEncodeBenchmark(),
    Rich50SpansDecodeBenchmark(),
    Batch100RichEncodeBenchmark(),
    Batch100RichDecodeBenchmark(),
  ];

  print('');
  print('Chat Protocol Codec Benchmarks (Dart)');
  print('=' * 70);
  print('');

  final results = <String, double>{};

  for (final bench in benchmarks) {
    final us = bench.measure();
    results[bench.name] = us;
  }

  // Print comparison table
  print('');
  print(
    '${'Benchmark'.padRight(24)} '
    '${'Rust'.padLeft(12)} '
    '${'Dart'.padLeft(12)} '
    '${'Ratio'.padLeft(8)}',
  );
  print('-' * 58);

  for (final name in rustBaselines.keys) {
    final rustUs = rustBaselines[name]!;
    final dartUs = results[name];
    if (dartUs == null) continue;

    final ratio = dartUs / rustUs;
    print(
      '${name.padRight(24)} '
      '${_formatUs(rustUs).padLeft(12)} '
      '${_formatUs(dartUs).padLeft(12)} '
      '${ratio.toStringAsFixed(1).padLeft(7)}x',
    );
  }

  print('');
}

String _formatUs(double us) {
  if (us < 0.001) {
    return '${(us * 1000).toStringAsFixed(2)} ns';
  } else if (us < 1.0) {
    return '${(us * 1000).toStringAsFixed(2)} ns';
  } else if (us < 1000.0) {
    return '${us.toStringAsFixed(2)} us';
  } else {
    return '${(us / 1000).toStringAsFixed(2)} ms';
  }
}
