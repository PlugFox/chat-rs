// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'chat_role.dart';
import 'permission.dart';

/// MemberUpdated event payload (server → client).
///
/// Pushed when a member's role or permissions change in a chat.
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

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MemberUpdatedPayload &&
          chatId == other.chatId &&
          userId == other.userId &&
          role == other.role &&
          permissions == other.permissions;

  @override
  int get hashCode => Object.hash(chatId, userId, role, permissions);

  @override
  String toString() =>
      'MemberUpdatedPayload(chatId: $chatId, userId: $userId, role: $role, permissions: $permissions)';
}
