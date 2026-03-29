// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/**
 * Server-enforced limits sent in the Welcome payload.
 *
 * Clients use these for local enforcement (debouncing, UI limits).
 */
export interface ServerLimits {
  /** How often the client should send Ping (ms). */
  readonly pingIntervalMs: number;
  /** Pong timeout — disconnect if exceeded (ms). */
  readonly pingTimeoutMs: number;
  /** Max content size in bytes. */
  readonly maxMessageSize: number;
  /** Max extra JSON size in bytes. */
  readonly maxExtraSize: number;
  /** Max total WS frame size in bytes. */
  readonly maxFrameSize: number;
  /** Rate limit: messages per second per user per chat. */
  readonly messagesPerSec: number;
  /** Rate limit: concurrent connections per IP. */
  readonly connectionsPerIp: number;
}
