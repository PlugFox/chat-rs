// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** DeleteMessage frame payload (client → server). */
export interface DeleteMessagePayload {
  /** Target chat. */
  readonly chatId: number;
  /** Message to delete. */
  readonly messageId: number;
}
