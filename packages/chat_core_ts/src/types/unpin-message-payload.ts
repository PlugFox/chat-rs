// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** UnpinMessage frame payload (client → server). */
export interface UnpinMessagePayload {
  /** Target chat. */
  readonly chatId: number;
  /** Message to unpin. */
  readonly messageId: number;
}
