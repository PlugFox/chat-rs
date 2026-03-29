// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

import 'package:chat_core/src/util/list_equals.dart';

/// GetPresence frame payload (client → server).
@immutable
class GetPresencePayload {
  const GetPresencePayload({required this.userIds});

  /// User IDs to query.
  final List<int> userIds;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is GetPresencePayload && listEquals(userIds, other.userIds);
  // coverage:ignore-end

  @override
  int get hashCode => Object.hashAll(userIds);
}
