// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// GetChatMembers frame payload (client → server).
class GetChatMembersPayload {
  const GetChatMembersPayload({
    required this.chatId,
    required this.cursor,
    required this.limit,
  });

  /// Target chat.
  final int chatId;
  /// Pagination cursor (0 = first page).
  final int cursor;
  /// Max members to return.
  final int limit;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is GetChatMembersPayload &&
          chatId == other.chatId &&
          cursor == other.cursor &&
          limit == other.limit;

  @override
  int get hashCode => Object.hash(
        chatId,
        cursor,
        limit,
      );

  @override
  String toString() => 'GetChatMembersPayload(chatId: $chatId, cursor: $cursor, limit: $limit)';
}
