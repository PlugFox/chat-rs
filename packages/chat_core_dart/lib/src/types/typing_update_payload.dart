// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// TypingUpdate event payload (server → client).
@immutable
class TypingUpdatePayload {
  const TypingUpdatePayload({
    required this.chatId,
    required this.userId,
    required this.expiresInMs,
  });

  /// Chat where typing is happening.
  final int chatId;

  /// User who is typing.
  final int userId;

  /// How long this typing indicator is valid, in milliseconds.
  final int expiresInMs;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TypingUpdatePayload &&
          chatId == other.chatId &&
          userId == other.userId &&
          expiresInMs == other.expiresInMs;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, userId, expiresInMs);
}
