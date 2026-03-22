// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// UnblockUser frame payload (client → server, RPC).
class UnblockUserPayload {
  const UnblockUserPayload({
    required this.userId,
  });

  /// User to unblock.
  final int userId;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is UnblockUserPayload &&
          userId == other.userId;

  @override
  int get hashCode => userId.hashCode;

  @override
  String toString() => 'UnblockUserPayload(userId: $userId)';
}
