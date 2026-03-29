// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** ReceiptUpdate event payload (server → client). */
export interface ReceiptUpdatePayload {
  /** Chat where the receipt update occurred. */
  readonly chatId: number;
  /** User who read the messages. */
  readonly userId: number;
  /** Highest read message ID. */
  readonly messageId: number;
}
