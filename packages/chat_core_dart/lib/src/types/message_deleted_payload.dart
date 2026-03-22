// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// MessageDeleted event payload (server → client).
///
/// Content is already cleared server-side; this event tells the client
/// which message was deleted so it can update the UI.
class MessageDeletedPayload {
  const MessageDeletedPayload({required this.chatId, required this.messageId});

  /// Chat containing the deleted message.
  final int chatId;

  /// Deleted message ID.
  final int messageId;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MessageDeletedPayload &&
          chatId == other.chatId &&
          messageId == other.messageId;

  @override
  int get hashCode => Object.hash(chatId, messageId);

  @override
  String toString() =>
      'MessageDeletedPayload(chatId: $chatId, messageId: $messageId)';
}
