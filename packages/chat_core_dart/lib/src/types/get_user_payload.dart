// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// GetUser frame payload (client → server, RPC).
class GetUserPayload {
  const GetUserPayload({required this.userId});

  /// User ID to look up.
  final int userId;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is GetUserPayload && userId == other.userId;

  @override
  int get hashCode => userId.hashCode;

  @override
  String toString() => 'GetUserPayload(userId: $userId)';
}
