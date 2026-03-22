// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** Typing frame payload (client → server, fire-and-forget). */
export interface TypingPayload {
  /** Target chat. */
  readonly chatId: number;
  /**
   * How long this typing indicator is valid, in milliseconds.
   * Server and other clients use this to auto-expire the indicator.
   */
  readonly expiresInMs: number;
}
