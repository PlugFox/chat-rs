// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// DeleteMessage frame payload (client → server).
class DeleteMessagePayload {
  const DeleteMessagePayload({
    required this.chatId,
    required this.messageId,
  });

  /// Target chat.
  final int chatId;
  /// Message to delete.
  final int messageId;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is DeleteMessagePayload &&
          chatId == other.chatId &&
          messageId == other.messageId;

  @override
  int get hashCode => Object.hash(
        chatId,
        messageId,
      );

  @override
  String toString() => 'DeleteMessagePayload(chatId: $chatId, messageId: $messageId)';
}
