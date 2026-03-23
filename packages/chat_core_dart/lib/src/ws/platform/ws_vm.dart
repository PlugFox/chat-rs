import 'dart:typed_data';

import 'package:chat_core/src/ws/ws.dart' show ChatWebSocket;

ChatWebSocket $createChatWebSocket({
  required String url,
  required void Function() onOpen,
  required void Function(Uint8List message) onMessage,
  required void Function(Object error, StackTrace stackTrace) onError,
  required void Function(int code, String reason) onClose,
}) => throw UnsupportedError('WebSocket is not supported on this platform');
