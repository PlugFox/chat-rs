// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** EditMessage frame payload (client → server). */
export interface EditMessagePayload {
  /** Target chat. */
  readonly chatId: number;
  /** Message to edit. */
  readonly messageId: number;
  /** New plain-text content. */
  readonly content: string;
  /** New rich content spans. `None` = remove formatting. */
  readonly richContent: Uint8Array | null;
  /** New extra metadata JSON. `None` = remove metadata. */
  readonly extra: string | null;
}
