import 'dart:async';
import 'dart:typed_data';

import 'package:chat_core/src/ws/ws.dart';
import 'package:test/test.dart';

/// Registers all ChatWebSocket tests.
///
/// [echoPort] — port of the echo server (endpoints: /echo, /close).
/// [hangPort] — port of a TCP server that never completes WS handshake.
void wsTests({
  required int Function() echoPort,
  required int Function() hangPort,
}) {
  String echoUrl() => 'ws://127.0.0.1:${echoPort()}/echo';

  String closeUrl(int code, String reason) =>
      'ws://127.0.0.1:${echoPort()}/close'
      '?code=$code&reason=${Uri.encodeComponent(reason)}';

  String hangUrl() => 'ws://127.0.0.1:${hangPort()}/';

  /// Fill a buffer with a repeating byte pattern.
  Uint8List fillBytes(int size, [int seed = 0]) {
    final data = Uint8List(size);
    for (var i = 0; i < size; i++) {
      data[i] = (i + seed) & 0xFF;
    }
    return data;
  }

  /// Connect to echo, run [body], then close cleanly.
  Future<void> withEcho(
    Future<void> Function(ChatWebSocket ws, Future<Uint8List> Function() next)
        body,
  ) async {
    final incoming = StreamController<Uint8List>();
    final closed = Completer<void>();

    final ws = await ChatWebSocket.connect(
      url: echoUrl(),
      onMessage: incoming.add,
      onError: (e, st) {},
      onClose: (code, reason) {
        if (!closed.isCompleted) closed.complete();
        incoming.close();
      },
    );

    Future<Uint8List> next() =>
        incoming.stream.first.timeout(const Duration(seconds: 10));

    try {
      await body(ws, next);
    } finally {
      ws.close();
      await closed.future.timeout(const Duration(seconds: 5));
    }
  }

  // ---------------------------------------------------------------------------
  // Lifecycle
  // ---------------------------------------------------------------------------

  group('lifecycle', () {
    test('connect and close with default code', () async {
      final closed = Completer<(int, String)>();
      final ws = await ChatWebSocket.connect(
        url: echoUrl(),
        onMessage: (_) {},
        onError: (e, st) {},
        onClose: (code, reason) => closed.complete((code, reason)),
      );
      ws.close();
      final (code, reason) =
          await closed.future.timeout(const Duration(seconds: 5));
      expect(code, 1000);
      expect(reason, 'closed');
    });

    test('close with custom code and reason', () async {
      final closed = Completer<(int, String)>();
      final ws = await ChatWebSocket.connect(
        url: echoUrl(),
        onMessage: (_) {},
        onError: (e, st) {},
        onClose: (code, reason) => closed.complete((code, reason)),
      );
      ws.close(4001, 'custom reason');
      final (code, reason) =
          await closed.future.timeout(const Duration(seconds: 5));
      expect(code, 4001);
      expect(reason, 'custom reason');
    });

    test('double close is silently ignored', () async {
      var closeCount = 0;
      final closed = Completer<void>();
      final ws = await ChatWebSocket.connect(
        url: echoUrl(),
        onMessage: (_) {},
        onError: (e, st) {},
        onClose: (code, reason) {
          closeCount++;
          if (!closed.isCompleted) closed.complete();
        },
      );
      ws.close();
      ws.close(); // must not throw or double-fire
      await closed.future.timeout(const Duration(seconds: 5));
      // Allow time for any hypothetical delayed second callback.
      await Future<void>.delayed(const Duration(milliseconds: 200));
      expect(closeCount, 1);
    });

    test('send after close throws StateError', () async {
      final closed = Completer<void>();
      final ws = await ChatWebSocket.connect(
        url: echoUrl(),
        onMessage: (_) {},
        onError: (e, st) {},
        onClose: (code, reason) {
          if (!closed.isCompleted) closed.complete();
        },
      );
      ws.close();
      expect(() => ws.send(Uint8List(1)), throwsStateError);
      await closed.future.timeout(const Duration(seconds: 5));
    });

    test('protocols parameter accepted', () async {
      final closed = Completer<void>();
      final ws = await ChatWebSocket.connect(
        url: echoUrl(),
        onMessage: (_) {},
        onError: (e, st) {},
        onClose: (code, reason) {
          if (!closed.isCompleted) closed.complete();
        },
        protocols: ['chat-v1'],
      );
      ws.close();
      await closed.future.timeout(const Duration(seconds: 5));
    });
  });

  // ---------------------------------------------------------------------------
  // Echo — data integrity
  // ---------------------------------------------------------------------------

  group('echo', () {
    test('1 byte', () async {
      await withEcho((ws, next) async {
        final data = Uint8List.fromList([42]);
        ws.send(data);
        expect(await next(), equals(data));
      });
    });

    test('256 bytes — all byte values', () async {
      await withEcho((ws, next) async {
        final data = fillBytes(256);
        ws.send(data);
        expect(await next(), equals(data));
      });
    });

    test('1 KB', () async {
      await withEcho((ws, next) async {
        final data = fillBytes(1024, 7);
        ws.send(data);
        expect(await next(), equals(data));
      });
    });

    test('64 KB', () async {
      await withEcho((ws, next) async {
        final data = fillBytes(65536, 13);
        ws.send(data);
        expect(await next(), equals(data));
      });
    });

    test('1 MB', () async {
      await withEcho((ws, next) async {
        final data = fillBytes(1024 * 1024, 37);
        ws.send(data);
        expect(await next(), equals(data));
      });
    });

    test('all zeros', () async {
      await withEcho((ws, next) async {
        final data = Uint8List(512);
        ws.send(data);
        expect(await next(), equals(data));
      });
    });

    test('all 0xFF', () async {
      await withEcho((ws, next) async {
        final data = Uint8List(512)..fillRange(0, 512, 0xFF);
        ws.send(data);
        expect(await next(), equals(data));
      });
    });
  });

  // ---------------------------------------------------------------------------
  // Multiple messages
  // ---------------------------------------------------------------------------

  group('multiple messages', () {
    test('100 messages preserve order', () async {
      const count = 100;
      final received = <Uint8List>[];
      final allReceived = Completer<void>();
      final closed = Completer<void>();

      final ws = await ChatWebSocket.connect(
        url: echoUrl(),
        onMessage: (msg) {
          received.add(Uint8List.fromList(msg));
          if (received.length == count && !allReceived.isCompleted) {
            allReceived.complete();
          }
        },
        onError: (e, st) {},
        onClose: (code, reason) {
          if (!closed.isCompleted) closed.complete();
        },
      );

      for (var i = 0; i < count; i++) {
        final data = Uint8List(4);
        ByteData.sublistView(data).setUint32(0, i, Endian.big);
        ws.send(data);
      }

      await allReceived.future.timeout(const Duration(seconds: 15));

      for (var i = 0; i < count; i++) {
        expect(
          ByteData.sublistView(received[i]).getUint32(0, Endian.big),
          i,
          reason: 'message $i out of order',
        );
      }

      ws.close();
      await closed.future.timeout(const Duration(seconds: 5));
    });

    test('varying sizes interleaved', () async {
      final sizes = [1, 10, 100, 1000, 10000, 100, 10, 1];
      final sent = <Uint8List>[];
      final received = <Uint8List>[];
      final allReceived = Completer<void>();
      final closed = Completer<void>();

      final ws = await ChatWebSocket.connect(
        url: echoUrl(),
        onMessage: (msg) {
          received.add(Uint8List.fromList(msg));
          if (received.length == sizes.length && !allReceived.isCompleted) {
            allReceived.complete();
          }
        },
        onError: (e, st) {},
        onClose: (code, reason) {
          if (!closed.isCompleted) closed.complete();
        },
      );

      for (var i = 0; i < sizes.length; i++) {
        final data = fillBytes(sizes[i], i * 31);
        sent.add(data);
        ws.send(data);
      }

      await allReceived.future.timeout(const Duration(seconds: 15));

      for (var i = 0; i < sizes.length; i++) {
        expect(received[i], equals(sent[i]), reason: 'message $i mismatch');
      }

      ws.close();
      await closed.future.timeout(const Duration(seconds: 5));
    });
  });

  // ---------------------------------------------------------------------------
  // Server-initiated close
  // ---------------------------------------------------------------------------

  group('server-initiated close', () {
    test('receives close code and reason', () async {
      final closed = Completer<(int, String)>();
      final ws = await ChatWebSocket.connect(
        url: closeUrl(4000, 'goodbye'),
        onMessage: (_) {},
        onError: (e, st) {},
        onClose: (code, reason) {
          if (!closed.isCompleted) closed.complete((code, reason));
        },
      );

      final (code, reason) =
          await closed.future.timeout(const Duration(seconds: 5));
      expect(code, 4000);
      expect(reason, 'goodbye');

      // After server close, send must throw.
      expect(() => ws.send(Uint8List(1)), throwsStateError);
    });
  });

  // ---------------------------------------------------------------------------
  // Error handling
  // ---------------------------------------------------------------------------

  group('error handling', () {
    test('connection to unreachable host fails', () async {
      await expectLater(
        ChatWebSocket.connect(
          url: 'ws://127.0.0.1:1/nonexistent',
          onMessage: (_) {},
          onError: (e, st) {},
          onClose: (code, reason) {},
        ),
        throwsA(anything),
      );
    });

    test('connection timeout', () async {
      await expectLater(
        ChatWebSocket.connect(
          url: hangUrl(),
          onMessage: (_) {},
          onError: (e, st) {},
          onClose: (code, reason) {},
          timeout: const Duration(milliseconds: 500),
        ),
        throwsA(isA<TimeoutException>()),
      );
    }, timeout: const Timeout(Duration(seconds: 10)));
  });
}
