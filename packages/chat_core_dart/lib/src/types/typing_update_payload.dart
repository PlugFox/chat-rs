// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// TypingUpdate event payload (server → client).
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

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TypingUpdatePayload &&
          chatId == other.chatId &&
          userId == other.userId &&
          expiresInMs == other.expiresInMs;

  @override
  int get hashCode => Object.hash(chatId, userId, expiresInMs);

  @override
  String toString() =>
      'TypingUpdatePayload(chatId: $chatId, userId: $userId, expiresInMs: $expiresInMs)';
}
