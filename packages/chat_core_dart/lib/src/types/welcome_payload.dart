// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'server_capabilities.dart';
import 'server_limits.dart';

/// Welcome frame payload (server → client).
class WelcomePayload {
  const WelcomePayload({
    required this.sessionId,
    required this.serverTime,
    required this.userId,
    required this.limits,
    required this.capabilities,
  });

  /// Transient session identifier for this connection.
  final int sessionId;

  /// Server clock, Unix seconds. Client uses for clock-sync.
  final int serverTime;

  /// Authenticated user's internal ID.
  final int userId;

  /// Server-enforced limits.
  final ServerLimits limits;

  /// Server-advertised feature capabilities.
  final ServerCapabilities capabilities;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is WelcomePayload &&
          sessionId == other.sessionId &&
          serverTime == other.serverTime &&
          userId == other.userId &&
          limits == other.limits &&
          capabilities == other.capabilities;

  @override
  int get hashCode =>
      Object.hash(sessionId, serverTime, userId, limits, capabilities);

  @override
  String toString() =>
      'WelcomePayload(sessionId: $sessionId, serverTime: $serverTime, userId: $userId, limits: $limits, capabilities: $capabilities)';
}
