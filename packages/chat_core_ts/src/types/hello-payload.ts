// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** Hello frame payload (client → server). */
export interface HelloPayload {
  /** Protocol version the client supports. */
  readonly protocolVersion: number;
  /** Client SDK version string (e.g. "1.0.0"). */
  readonly sdkVersion: string;
  /** Client platform string (e.g. "dart", "typescript", "rust"). */
  readonly platform: string;
  /** JWT authentication token. */
  readonly token: string;
  /** Unique device identifier (UUID v4, 16 bytes on wire). */
  readonly deviceId: string;
}
