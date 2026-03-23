import 'dart:typed_data';

import 'package:meta/meta.dart' show immutable;

import 'platform/ws_stub.dart'
    // ignore: uri_does_not_exist
    if (dart.library.js_interop) 'platform/ws_js.dart'
    // ignore: uri_does_not_exist
    if (dart.library.io) 'platform/ws_vm.dart';

/// {@template chat_websocket}
/// A WebSocket connection for handling real-time communication.
/// {@endtemplate}
@immutable
abstract class ChatWebSocket {
  /// Creates a new [ChatWebSocket] instance with the given parameters.
  ///
  /// The [url] is the WebSocket server URL to connect to.
  /// The [onOpen] callback is called when the connection is successfully opened.
  /// The [onMessage] callback is called when a message is received from the server.
  /// The [onError] callback is called when an error occurs during the connection.
  /// The [onClose] callback is called when the connection is closed.
  ///
  /// {@macro chat_websocket}
  factory ChatWebSocket({
    required String url,
    required void Function() onOpen,
    required void Function(Uint8List message) onMessage,
    required void Function(Object error, StackTrace stackTrace) onError,
    required void Function(int code, String reason) onClose,
  }) => $createChatWebSocket(
    url: url,
    onOpen: onOpen,
    onMessage: onMessage,
    onError: onError,
    onClose: onClose,
  );

  /// Sends a message through the WebSocket connection.
  void send(Uint8List message);

  /// Closes the WebSocket connection with an optional [code] and [reason].
  void close([int? code, String? reason]);
}
