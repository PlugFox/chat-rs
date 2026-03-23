// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:chat_core/src/util/list_equals.dart';

/// GetPresence frame payload (client → server).
class GetPresencePayload {
  const GetPresencePayload({required this.userIds});

  /// User IDs to query.
  final List<int> userIds;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is GetPresencePayload && listEquals(userIds, other.userIds);

  @override
  int get hashCode => Object.hashAll(userIds);

  @override
  String toString() => 'GetPresencePayload(userIds: $userIds)';
}
