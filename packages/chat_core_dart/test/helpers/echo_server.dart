import 'dart:async';
import 'dart:io';

/// A WebSocket echo server for testing.
///
/// Supports two endpoints:
/// - `/echo` — echoes every binary message back to the sender.
/// - `/close?code=<int>&reason=<string>` — accepts the connection,
///   waits briefly, then closes with the given code and reason.
///
/// Also exposes a [hangPort] — a raw TCP socket that accepts connections
/// but never completes the HTTP upgrade, useful for testing connect timeouts.
class EchoServer {
  EchoServer._(this._httpServer, this._hangServer);

  final HttpServer _httpServer;
  final ServerSocket _hangServer;

  /// Port for WebSocket connections (echo / close endpoints).
  int get port => _httpServer.port;

  /// Port that accepts TCP but never responds — connections hang forever.
  int get hangPort => _hangServer.port;

  /// Starts the echo server on a random available port.
  static Future<EchoServer> start() async {
    final httpServer = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    final hangServer = await ServerSocket.bind(InternetAddress.loopbackIPv4, 0);

    // Accept TCP but never respond — causes WS connect to hang.
    hangServer.listen((_) {});

    httpServer.listen((request) async {
      if (!WebSocketTransformer.isUpgradeRequest(request)) {
        request.response
          ..statusCode = 400
          ..write('Not a WebSocket request');
        await request.response.close();
        return;
      }

      final socket = await WebSocketTransformer.upgrade(
        request,
        protocolSelector:
            (protocols) => protocols.isNotEmpty ? protocols.first : null,
      );
      final path = request.uri.path;
      final params = request.uri.queryParameters;

      if (path == '/close') {
        final code = int.parse(params['code'] ?? '1000');
        final reason = params['reason'] ?? 'server close';
        // Drain incoming messages to avoid backpressure.
        socket.listen((_) {}, onError: (_) {});
        // Wait briefly so the client is fully connected, then close.
        Timer(const Duration(milliseconds: 50), () {
          socket.close(code, reason);
        });
      } else {
        // Default: echo binary frames back.
        socket.listen((data) {
          try {
            socket.add(data);
          } on Object catch (_) {
            // Client may have disconnected.
          }
        }, onError: (_) {});
      }
    });

    return EchoServer._(httpServer, hangServer);
  }

  /// Stops the server and releases all resources.
  Future<void> stop() async {
    await _httpServer.close(force: true);
    await _hangServer.close();
  }
}
