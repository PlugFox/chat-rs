// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// Server-enforced limits sent in the Welcome payload.
///
/// Clients use these for local enforcement (debouncing, UI limits).
class ServerLimits {
  const ServerLimits({
    required this.pingIntervalMs,
    required this.pingTimeoutMs,
    required this.maxMessageSize,
    required this.maxExtraSize,
    required this.maxFrameSize,
    required this.messagesPerSec,
    required this.connectionsPerIp,
  });

  /// How often the client should send Ping (ms).
  final int pingIntervalMs;

  /// Pong timeout — disconnect if exceeded (ms).
  final int pingTimeoutMs;

  /// Max content size in bytes.
  final int maxMessageSize;

  /// Max extra JSON size in bytes.
  final int maxExtraSize;

  /// Max total WS frame size in bytes.
  final int maxFrameSize;

  /// Rate limit: messages per second per user per chat.
  final int messagesPerSec;

  /// Rate limit: concurrent connections per IP.
  final int connectionsPerIp;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ServerLimits &&
          pingIntervalMs == other.pingIntervalMs &&
          pingTimeoutMs == other.pingTimeoutMs &&
          maxMessageSize == other.maxMessageSize &&
          maxExtraSize == other.maxExtraSize &&
          maxFrameSize == other.maxFrameSize &&
          messagesPerSec == other.messagesPerSec &&
          connectionsPerIp == other.connectionsPerIp;

  @override
  int get hashCode => Object.hash(
    pingIntervalMs,
    pingTimeoutMs,
    maxMessageSize,
    maxExtraSize,
    maxFrameSize,
    messagesPerSec,
    connectionsPerIp,
  );

  @override
  String toString() =>
      'ServerLimits(pingIntervalMs: $pingIntervalMs, pingTimeoutMs: $pingTimeoutMs, maxMessageSize: $maxMessageSize, maxExtraSize: $maxExtraSize, maxFrameSize: $maxFrameSize, messagesPerSec: $messagesPerSec, connectionsPerIp: $connectionsPerIp)';
}
