// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import '../_util.dart';

/// GetUsers frame payload (client → server, RPC).
class GetUsersPayload {
  const GetUsersPayload({
    required this.userIds,
  });

  /// User IDs to look up (batch).
  final List<int> userIds;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is GetUsersPayload &&
          listEquals(userIds, other.userIds);

  @override
  int get hashCode => Object.hashAll(userIds);

  @override
  String toString() => 'GetUsersPayload(userIds: $userIds)';
}
