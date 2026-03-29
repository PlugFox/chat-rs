// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

import 'package:chat_core/src/types/member_action.dart';

/// UpdateMember frame payload (client → server).
///
/// Unified frame for kick, ban, mute, role change, and permission override.
/// Replaces the previous separate `KickMember`, `BanMember`, `MuteMember`,
/// and `UpdateMemberRole` frames.
@immutable
class UpdateMemberPayload {
  const UpdateMemberPayload({
    required this.chatId,
    required this.userId,
    required this.action,
  });

  /// Target chat.
  final int chatId;

  /// Target user.
  final int userId;

  /// Action to perform.
  final MemberAction action;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is UpdateMemberPayload &&
          chatId == other.chatId &&
          userId == other.userId &&
          action == other.action;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, userId, action);
}
