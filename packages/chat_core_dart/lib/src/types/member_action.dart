// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

import 'package:chat_core/src/types/chat_role.dart';
import 'package:chat_core/src/types/permission.dart';

/// Action to perform on a chat member (used in `UpdateMember` frame).
///
/// Wire format: `action: u8` discriminant + action-specific payload.
/// Discriminant values: Kick=0, Ban=1, Mute=2, ChangeRole=3, UpdatePermissions=4, Unban=5.
@immutable
sealed class MemberAction {
  const MemberAction();
}

/// Remove member from chat. Wire: action=0, no payload.
class MemberActionKick extends MemberAction {
  const MemberActionKick();

  @override
  bool operator ==(Object other) =>
      identical(this, other) || other is MemberActionKick; // coverage:ignore-line

  @override
  int get hashCode => 0;
}

/// Ban member from chat. Wire: action=1, no payload.
class MemberActionBan extends MemberAction {
  const MemberActionBan();

  @override
  bool operator ==(Object other) =>
      identical(this, other) || other is MemberActionBan; // coverage:ignore-line

  @override
  int get hashCode => 0;
}

/// Mute member. Wire: action=2, payload: `duration_secs: u32` (0 = unmute).
class MemberActionMute extends MemberAction {
  const MemberActionMute({required this.durationSecs});

  final int durationSecs;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MemberActionMute && durationSecs == other.durationSecs;
  // coverage:ignore-end

  @override
  int get hashCode => durationSecs.hashCode;
}

/// Change member's role. Wire: action=3, payload: `role: u8`.
class MemberActionChangeRole extends MemberAction {
  const MemberActionChangeRole({required this.chatRole});

  final ChatRole chatRole;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MemberActionChangeRole && chatRole == other.chatRole;
  // coverage:ignore-end

  @override
  int get hashCode => chatRole.hashCode;
}

/// Set explicit permission override. Wire: action=4, payload: `permissions: u32`.
class MemberActionUpdatePermissions extends MemberAction {
  const MemberActionUpdatePermissions({required this.permission});

  final Permission permission;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MemberActionUpdatePermissions && permission == other.permission;
  // coverage:ignore-end

  @override
  int get hashCode => permission.hashCode;
}

/// Unban a previously banned member. Wire: action=5, no payload.
class MemberActionUnban extends MemberAction {
  const MemberActionUnban();

  @override
  bool operator ==(Object other) =>
      identical(this, other) || other is MemberActionUnban; // coverage:ignore-line

  @override
  int get hashCode => 0;
}
