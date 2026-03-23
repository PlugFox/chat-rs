// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

import 'package:chat_core/src/types/presence_status.dart';

/// A presence entry as transmitted in `PresenceResult` (13 bytes fixed).
@immutable
class PresenceEntry {
  const PresenceEntry({
    required this.userId,
    required this.status,
    required this.lastSeen,
  });

  /// User ID.
  final int userId;

  /// Current online/offline status.
  final PresenceStatus status;

  /// Last seen timestamp, Unix seconds. `0` when user is currently online.
  final int lastSeen;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is PresenceEntry &&
          userId == other.userId &&
          status == other.status &&
          lastSeen == other.lastSeen;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(userId, status, lastSeen);
}
