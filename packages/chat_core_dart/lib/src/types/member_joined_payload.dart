// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

import 'package:chat_core/src/types/chat_role.dart';

/// MemberJoined event payload (server → client).
@immutable
class MemberJoinedPayload {
  const MemberJoinedPayload({
    required this.chatId,
    required this.userId,
    required this.role,
    required this.invitedBy,
  });

  /// Target chat.
  final int chatId;

  /// User who joined.
  final int userId;

  /// Role assigned to the new member.
  final ChatRole role;

  /// User who invited them. `0` = self-join (e.g. via invite link).
  final int invitedBy;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MemberJoinedPayload &&
          chatId == other.chatId &&
          userId == other.userId &&
          role == other.role &&
          invitedBy == other.invitedBy;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, userId, role, invitedBy);
}
