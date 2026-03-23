// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// RefreshToken frame payload (client → server).
///
/// Allows the client to refresh its JWT without disconnecting.
/// Server responds with Ack (empty) on success, or Error if the new token is invalid.
@immutable
class RefreshTokenPayload {
  const RefreshTokenPayload({required this.token});

  /// New JWT authentication token.
  final String token;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RefreshTokenPayload && token == other.token;
  // coverage:ignore-end

  @override
  int get hashCode => token.hashCode;
}
