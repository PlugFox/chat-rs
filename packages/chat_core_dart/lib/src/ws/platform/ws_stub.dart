// coverage:ignore-file

import 'dart:typed_data';

import 'package:chat_core/src/ws/ws.dart' show ChatWebSocket;
import 'package:meta/meta.dart';

@internal
Future<ChatWebSocket> $connectChatWebSocket({
  required String url,
  required void Function(Uint8List message) onMessage,
  required void Function(Object error, StackTrace stackTrace) onError,
  required void Function(int code, String reason) onClose,
  Iterable<String>? protocols,
  Duration? timeout,
}) => throw UnsupportedError('WebSocket is not supported on this platform');
