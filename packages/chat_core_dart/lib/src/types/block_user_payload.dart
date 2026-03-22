// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// BlockUser frame payload (client → server, RPC).
class BlockUserPayload {
  const BlockUserPayload({required this.userId});

  /// User to block.
  final int userId;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BlockUserPayload && userId == other.userId;

  @override
  int get hashCode => userId.hashCode;

  @override
  String toString() => 'BlockUserPayload(userId: $userId)';
}
