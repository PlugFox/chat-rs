// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/**
 * ForwardMessage frame payload (client → server).
 *
 * Server copies the original message content to the target chat,
 * sets `MessageFlags::FORWARDED`, and populates `extra.fwd` JSON.
 */
export interface ForwardMessagePayload {
  /** Source chat containing the message to forward. */
  readonly fromChatId: number;
  /** Message to forward. */
  readonly messageId: number;
  /** Target chat to forward into. */
  readonly toChatId: number;
  /** Client-generated UUID for deduplication. */
  readonly idempotencyKey: string;
}
