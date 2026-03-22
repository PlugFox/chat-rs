// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/**
 * WebSocket disconnect / close code.
 *
 * Determines whether the client should attempt reconnection.
 */
export const enum DisconnectCode {
  /** Graceful server restart. */
  ServerShutdown = 3000,
  /** Token expired mid-session. */
  SessionExpired = 3001,
  /** Same device_id connected from another location. */
  DuplicateSession = 3002,
  /** Unrecoverable internal server error. */
  ServerError = 3003,
  /** Client send buffer exceeded capacity. */
  BufferOverflow = 3004,
  /** Too many requests on this connection, backoff. */
  RateLimited = 3005,
  /** event_seq approaching u32 limit — reconnect to reset counter. */
  EventSeqOverflow = 3006,
  /** Token is malformed or has invalid signature. */
  TokenInvalid = 3500,
  /** User is banned. */
  Banned = 3501,
  /** Protocol version not supported by server. */
  UnsupportedVersion = 3502,
  /** Max connections per IP/user exceeded. */
  ConnectionLimit = 3503,
}

/** Convert wire value to DisconnectCode, or undefined if unknown. */
export function disconnectCodeFromValue(
  value: number,
): DisconnectCode | undefined {
  switch (value) {
    case 3000:
      return DisconnectCode.ServerShutdown;
    case 3001:
      return DisconnectCode.SessionExpired;
    case 3002:
      return DisconnectCode.DuplicateSession;
    case 3003:
      return DisconnectCode.ServerError;
    case 3004:
      return DisconnectCode.BufferOverflow;
    case 3005:
      return DisconnectCode.RateLimited;
    case 3006:
      return DisconnectCode.EventSeqOverflow;
    case 3500:
      return DisconnectCode.TokenInvalid;
    case 3501:
      return DisconnectCode.Banned;
    case 3502:
      return DisconnectCode.UnsupportedVersion;
    case 3503:
      return DisconnectCode.ConnectionLimit;
    default:
      return undefined;
  }
}

/** Whether the client should attempt reconnection. */
export function shouldReconnect(code: DisconnectCode): boolean {
  const v = code as number;
  return (
    (v >= 0 && v < 1000) || (v >= 3000 && v < 3500) || (v >= 4000 && v < 4500)
  );
}
