// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// Typing frame payload (client → server, fire-and-forget).
class TypingPayload {
  const TypingPayload({required this.chatId, required this.expiresInMs});

  /// Target chat.
  final int chatId;

  /// How long this typing indicator is valid, in milliseconds.
  /// Server and other clients use this to auto-expire the indicator.
  final int expiresInMs;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TypingPayload &&
          chatId == other.chatId &&
          expiresInMs == other.expiresInMs;

  @override
  int get hashCode => Object.hash(chatId, expiresInMs);

  @override
  String toString() =>
      'TypingPayload(chatId: $chatId, expiresInMs: $expiresInMs)';
}
