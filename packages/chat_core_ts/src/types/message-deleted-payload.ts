// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/**
 * MessageDeleted event payload (server → client).
 *
 * Content is already cleared server-side; this event tells the client
 * which message was deleted so it can update the UI.
 */
export interface MessageDeletedPayload {
  /** Chat containing the deleted message. */
  readonly chatId: number;
  /** Deleted message ID. */
  readonly messageId: number;
}
