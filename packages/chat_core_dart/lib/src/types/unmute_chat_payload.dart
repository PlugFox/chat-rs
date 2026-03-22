// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// UnmuteChat frame payload (client → server, RPC).
class UnmuteChatPayload {
  const UnmuteChatPayload({required this.chatId});

  /// Target chat.
  final int chatId;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is UnmuteChatPayload && chatId == other.chatId;

  @override
  int get hashCode => chatId.hashCode;

  @override
  String toString() => 'UnmuteChatPayload(chatId: $chatId)';
}
