// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// UnblockUser frame payload (client → server, RPC).
@immutable
class UnblockUserPayload {
  const UnblockUserPayload({required this.userId});

  /// User to unblock.
  final int userId;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is UnblockUserPayload && userId == other.userId;
  // coverage:ignore-end

  @override
  int get hashCode => userId.hashCode;
}
