// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// RefreshToken frame payload (client → server).
///
/// Allows the client to refresh its JWT without disconnecting.
/// Server responds with Ack (empty) on success, or Error if the new token is invalid.
class RefreshTokenPayload {
  const RefreshTokenPayload({required this.token});

  /// New JWT authentication token.
  final String token;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RefreshTokenPayload && token == other.token;

  @override
  int get hashCode => token.hashCode;

  @override
  String toString() => 'RefreshTokenPayload(token: $token)';
}
