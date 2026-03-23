// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// BlockUser frame payload (client → server, RPC).
@immutable
class BlockUserPayload {
  const BlockUserPayload({required this.userId});

  /// User to block.
  final int userId;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BlockUserPayload && userId == other.userId;
  // coverage:ignore-end

  @override
  int get hashCode => userId.hashCode;
}
