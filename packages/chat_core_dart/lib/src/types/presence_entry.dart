// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'presence_status.dart';

/// A presence entry as transmitted in `PresenceResult` (13 bytes fixed).
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

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is PresenceEntry &&
          userId == other.userId &&
          status == other.status &&
          lastSeen == other.lastSeen;

  @override
  int get hashCode => Object.hash(userId, status, lastSeen);

  @override
  String toString() =>
      'PresenceEntry(userId: $userId, status: $status, lastSeen: $lastSeen)';
}
