// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/**
 * RefreshToken frame payload (client → server).
 *
 * Allows the client to refresh its JWT without disconnecting.
 * Server responds with Ack (empty) on success, or Error if the new token is invalid.
 */
export interface RefreshTokenPayload {
  /** New JWT authentication token. */
  readonly token: string;
}
