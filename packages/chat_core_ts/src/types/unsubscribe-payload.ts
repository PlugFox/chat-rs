// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/**
 * Unsubscribe frame payload (client → server, fire-and-forget).
 *
 * Unsubscribes from one or more named channels.
 */
export interface UnsubscribePayload {
  /** Channel names to unsubscribe from. */
  readonly channels: readonly string[];
}
