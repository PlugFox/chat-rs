// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// WebSocket disconnect / close code.
///
/// Determines whether the client should attempt reconnection.
enum DisconnectCode {
  /// Graceful server restart.
  serverShutdown(3000),
  /// Token expired mid-session.
  sessionExpired(3001),
  /// Same device_id connected from another location.
  duplicateSession(3002),
  /// Unrecoverable internal server error.
  serverError(3003),
  /// Client send buffer exceeded capacity.
  bufferOverflow(3004),
  /// Too many requests on this connection, backoff.
  rateLimited(3005),
  /// event_seq approaching u32 limit — reconnect to reset counter.
  eventSeqOverflow(3006),
  /// Token is malformed or has invalid signature.
  tokenInvalid(3500),
  /// User is banned.
  banned(3501),
  /// Protocol version not supported by server.
  unsupportedVersion(3502),
  /// Max connections per IP/user exceeded.
  connectionLimit(3503);

  const DisconnectCode(this.value);
  final int value;

  static DisconnectCode? fromValue(int value) => switch (value) {
    3000 => serverShutdown,
    3001 => sessionExpired,
    3002 => duplicateSession,
    3003 => serverError,
    3004 => bufferOverflow,
    3005 => rateLimited,
    3006 => eventSeqOverflow,
    3500 => tokenInvalid,
    3501 => banned,
    3502 => unsupportedVersion,
    3503 => connectionLimit,
    _ => null,
  };

  /// Whether the client should attempt reconnection.
  bool get shouldReconnect {
    return (value >= 0 && value < 1000) ||
        (value >= 3000 && value < 3500) ||
        (value >= 4000 && value < 4500);
  }
}
