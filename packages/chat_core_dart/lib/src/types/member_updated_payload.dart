// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

import 'package:chat_core/src/types/chat_role.dart';
import 'package:chat_core/src/types/permission.dart';

/// MemberUpdated event payload (server → client).
///
/// Pushed when a member's role or permissions change in a chat.
@immutable
class MemberUpdatedPayload {
  const MemberUpdatedPayload({
    required this.chatId,
    required this.userId,
    required this.role,
    this.permissions,
  });

  /// Target chat.
  final int chatId;

  /// User whose membership changed.
  final int userId;

  /// New role.
  final ChatRole role;

  /// New permission override. `None` = use role defaults.
  final Permission? permissions;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MemberUpdatedPayload &&
          chatId == other.chatId &&
          userId == other.userId &&
          role == other.role &&
          permissions == other.permissions;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, userId, role, permissions);
}
