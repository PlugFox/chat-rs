// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// Typing frame payload (client → server, fire-and-forget).
@immutable
class TypingPayload {
  const TypingPayload({required this.chatId, required this.expiresInMs});

  /// Target chat.
  final int chatId;

  /// How long this typing indicator is valid, in milliseconds.
  /// Server and other clients use this to auto-expire the indicator.
  final int expiresInMs;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TypingPayload &&
          chatId == other.chatId &&
          expiresInMs == other.expiresInMs;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, expiresInMs);
}
