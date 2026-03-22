// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// ReadReceipt frame payload (client → server, fire-and-forget).
class ReadReceiptPayload {
  const ReadReceiptPayload({required this.chatId, required this.messageId});

  /// Target chat.
  final int chatId;

  /// Highest read message ID.
  final int messageId;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ReadReceiptPayload &&
          chatId == other.chatId &&
          messageId == other.messageId;

  @override
  int get hashCode => Object.hash(chatId, messageId);

  @override
  String toString() =>
      'ReadReceiptPayload(chatId: $chatId, messageId: $messageId)';
}
