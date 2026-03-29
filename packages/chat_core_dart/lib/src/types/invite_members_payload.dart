// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

import 'package:chat_core/src/util/list_equals.dart';

/// InviteMembers frame payload (client → server).
@immutable
class InviteMembersPayload {
  const InviteMembersPayload({required this.chatId, required this.userIds});

  /// Target chat.
  final int chatId;

  /// User IDs to invite.
  final List<int> userIds;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is InviteMembersPayload &&
          chatId == other.chatId &&
          listEquals(userIds, other.userIds);
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, Object.hashAll(userIds));
}
