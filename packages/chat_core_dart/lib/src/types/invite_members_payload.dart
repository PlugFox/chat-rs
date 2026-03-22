// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import '../_util.dart';

/// InviteMembers frame payload (client → server).
class InviteMembersPayload {
  const InviteMembersPayload({required this.chatId, required this.userIds});

  /// Target chat.
  final int chatId;

  /// User IDs to invite.
  final List<int> userIds;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is InviteMembersPayload &&
          chatId == other.chatId &&
          listEquals(userIds, other.userIds);

  @override
  int get hashCode => Object.hash(chatId, Object.hashAll(userIds));

  @override
  String toString() =>
      'InviteMembersPayload(chatId: $chatId, userIds: $userIds)';
}
