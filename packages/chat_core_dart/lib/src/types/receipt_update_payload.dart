// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// ReceiptUpdate event payload (server → client).
class ReceiptUpdatePayload {
  const ReceiptUpdatePayload({
    required this.chatId,
    required this.userId,
    required this.messageId,
  });

  /// Chat where the receipt update occurred.
  final int chatId;

  /// User who read the messages.
  final int userId;

  /// Highest read message ID.
  final int messageId;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ReceiptUpdatePayload &&
          chatId == other.chatId &&
          userId == other.userId &&
          messageId == other.messageId;

  @override
  int get hashCode => Object.hash(chatId, userId, messageId);

  @override
  String toString() =>
      'ReceiptUpdatePayload(chatId: $chatId, userId: $userId, messageId: $messageId)';
}
