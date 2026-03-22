// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// LeaveChat frame payload (client → server).
class LeaveChatPayload {
  const LeaveChatPayload({required this.chatId});

  /// Target chat.
  final int chatId;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LeaveChatPayload && chatId == other.chatId;

  @override
  int get hashCode => chatId.hashCode;

  @override
  String toString() => 'LeaveChatPayload(chatId: $chatId)';
}
