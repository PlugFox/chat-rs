// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// DeleteChat frame payload (client → server).
class DeleteChatPayload {
  const DeleteChatPayload({required this.chatId});

  /// Target chat.
  final int chatId;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is DeleteChatPayload && chatId == other.chatId;

  @override
  int get hashCode => chatId.hashCode;

  @override
  String toString() => 'DeleteChatPayload(chatId: $chatId)';
}
