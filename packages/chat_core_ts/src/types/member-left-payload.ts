// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** MemberLeft event payload (server → client). */
export interface MemberLeftPayload {
  /** Target chat. */
  readonly chatId: number;
  /** User who left. */
  readonly userId: number;
}
