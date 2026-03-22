// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** TypingUpdate event payload (server → client). */
export interface TypingUpdatePayload {
  /** Chat where typing is happening. */
  readonly chatId: number;
  /** User who is typing. */
  readonly userId: number;
  /** How long this typing indicator is valid, in milliseconds. */
  readonly expiresInMs: number;
}
