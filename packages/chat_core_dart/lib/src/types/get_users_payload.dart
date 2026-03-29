// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

import 'package:chat_core/src/util/list_equals.dart';

/// GetUsers frame payload (client → server, RPC).
@immutable
class GetUsersPayload {
  const GetUsersPayload({required this.userIds});

  /// User IDs to look up (batch).
  final List<int> userIds;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is GetUsersPayload && listEquals(userIds, other.userIds);
  // coverage:ignore-end

  @override
  int get hashCode => Object.hashAll(userIds);
}
