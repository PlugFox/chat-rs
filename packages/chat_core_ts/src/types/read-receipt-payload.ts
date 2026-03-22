// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** ReadReceipt frame payload (client → server, fire-and-forget). */
export interface ReadReceiptPayload {
  /** Target chat. */
  readonly chatId: number;
  /** Highest read message ID. */
  readonly messageId: number;
}
