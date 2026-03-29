// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/**
 * ChatDeleted event payload (server → client).
 *
 * Pushed when a chat is deleted. Clients should remove it from the chat list.
 */
export interface ChatDeletedPayload {
  /** Deleted chat ID. */
  readonly chatId: number;
}
