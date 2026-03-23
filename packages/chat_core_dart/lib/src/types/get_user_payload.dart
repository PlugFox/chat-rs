// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// GetUser frame payload (client → server, RPC).
@immutable
class GetUserPayload {
  const GetUserPayload({required this.userId});

  /// User ID to look up.
  final int userId;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is GetUserPayload && userId == other.userId;
  // coverage:ignore-end

  @override
  int get hashCode => userId.hashCode;
}
