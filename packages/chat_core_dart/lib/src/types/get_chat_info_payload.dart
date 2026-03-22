// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// GetChatInfo frame payload (client → server).
class GetChatInfoPayload {
  const GetChatInfoPayload({required this.chatId});

  /// Target chat.
  final int chatId;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is GetChatInfoPayload && chatId == other.chatId;

  @override
  int get hashCode => chatId.hashCode;

  @override
  String toString() => 'GetChatInfoPayload(chatId: $chatId)';
}
