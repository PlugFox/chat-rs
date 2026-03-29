// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// GetChatMembers frame payload (client → server).
@immutable
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

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is GetChatMembersPayload &&
          chatId == other.chatId &&
          cursor == other.cursor &&
          limit == other.limit;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, cursor, limit);
}
