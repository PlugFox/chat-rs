// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** PinMessage frame payload (client → server). */
export interface PinMessagePayload {
  /** Target chat. */
  readonly chatId: number;
  /** Message to pin. */
  readonly messageId: number;
}
