// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// MemberLeft event payload (server → client).
class MemberLeftPayload {
  const MemberLeftPayload({required this.chatId, required this.userId});

  /// Target chat.
  final int chatId;

  /// User who left.
  final int userId;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MemberLeftPayload &&
          chatId == other.chatId &&
          userId == other.userId;

  @override
  int get hashCode => Object.hash(chatId, userId);

  @override
  String toString() => 'MemberLeftPayload(chatId: $chatId, userId: $userId)';
}
