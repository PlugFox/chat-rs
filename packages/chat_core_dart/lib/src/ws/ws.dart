import 'dart:typed_data';

import 'platform/ws_stub.dart'
    // ignore: uri_does_not_exist
    if (dart.library.js_interop) 'platform/ws_js.dart'
    // ignore: uri_does_not_exist
    if (dart.library.io) 'platform/ws_vm.dart';

export '_disposable.dart';

/// {@template chat_websocket}
/// A WebSocket transport for binary real-time communication.
///
/// Obtain a connected instance via [ChatWebSocket.connect].
/// The returned [Future] completes when the connection is established;
/// if the handshake fails, the future completes with an error.
///
/// After [close] is called (or the remote end closes the connection),
/// the [onClose] callback fires exactly once.
/// Calling [send] after close throws [StateError].
/// Calling [close] more than once is silently ignored.
/// {@endtemplate}
abstract class ChatWebSocket {
  /// Connects to the WebSocket server at [url].
  ///
  /// Returns a [Future] that completes with a connected [ChatWebSocket].
  /// If the connection cannot be established, the future completes with
  /// an error — no callbacks will be invoked in that case.
  ///
  /// [onMessage] — called when a binary frame arrives.
  /// [onError] — called on transport errors *after* the connection is open.
  /// [onClose] — called exactly once when the connection ends, whether
  ///   initiated locally via [close] or by the remote peer.
  ///
  /// {@macro chat_websocket}
  static Future<ChatWebSocket> connect({
    required String url,
    required void Function(Uint8List message) onMessage,
    required void Function(Object error, StackTrace stackTrace) onError,
    required void Function(int code, String reason) onClose,
  }) =>
      $connectChatWebSocket(
        url: url,
        onMessage: onMessage,
        onError: onError,
        onClose: onClose,
      );

  /// Sends a binary message through the WebSocket connection.
  ///
  /// Throws [StateError] if the connection is already closed.
  void send(Uint8List message);

  /// Closes the WebSocket connection with an optional [code] and [reason].
  ///
  /// Defaults to code `1000` and reason `"closed"`.
  /// Calling this more than once is silently ignored.
  void close([int? code, String? reason]);
}
