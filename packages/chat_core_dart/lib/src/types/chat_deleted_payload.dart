// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// ChatDeleted event payload (server → client).
///
/// Pushed when a chat is deleted. Clients should remove it from the chat list.
class ChatDeletedPayload {
  const ChatDeletedPayload({required this.chatId});

  /// Deleted chat ID.
  final int chatId;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ChatDeletedPayload && chatId == other.chatId;

  @override
  int get hashCode => chatId.hashCode;

  @override
  String toString() => 'ChatDeletedPayload(chatId: $chatId)';
}
