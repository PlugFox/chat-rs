// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// UnpinMessage frame payload (client → server).
class UnpinMessagePayload {
  const UnpinMessagePayload({required this.chatId, required this.messageId});

  /// Target chat.
  final int chatId;

  /// Message to unpin.
  final int messageId;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is UnpinMessagePayload &&
          chatId == other.chatId &&
          messageId == other.messageId;

  @override
  int get hashCode => Object.hash(chatId, messageId);

  @override
  String toString() =>
      'UnpinMessagePayload(chatId: $chatId, messageId: $messageId)';
}
