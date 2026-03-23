// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:chat_core/src/types/chat_role.dart';
import 'package:chat_core/src/types/permission.dart';

/// A chat member entry as transmitted on the wire (GetChatMembers response).
class ChatMemberEntry {
  const ChatMemberEntry({
    required this.userId,
    required this.role,
    this.permissions,
  });

  /// Internal user ID.
  final int userId;

  /// Member's role.
  final ChatRole role;

  /// Permission override. `None` = use role defaults.
  final Permission? permissions;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ChatMemberEntry &&
          userId == other.userId &&
          role == other.role &&
          permissions == other.permissions;

  @override
  int get hashCode => Object.hash(userId, role, permissions);

  @override
  String toString() =>
      'ChatMemberEntry(userId: $userId, role: $role, permissions: $permissions)';
}
