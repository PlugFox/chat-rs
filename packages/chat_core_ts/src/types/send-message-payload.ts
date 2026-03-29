// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import type { MessageKind } from "./message-kind.js";

/** SendMessage frame payload (client → server). */
export interface SendMessagePayload {
  /** Target chat. */
  readonly chatId: number;
  /** Content type. Defaults to `Text` if omitted by the client. */
  readonly kind: MessageKind;
  /** Client-generated UUID for deduplication. Persisted 24h server-side. */
  readonly idempotencyKey: string;
  /** Message this is replying to. `None` = not a reply. */
  readonly replyToId: number | null;
  /** Plain-text message content. */
  readonly content: string;
  /** Rich content spans (encoded as blob). `None` = no formatting. */
  readonly richContent: Uint8Array | null;
  /** Extra metadata JSON. `None` = no metadata. */
  readonly extra: string | null;
  /**
   * User IDs explicitly mentioned in this message.
   *
   * Server uses this for push notification routing (avoids parsing rich content).
   * When replying, the client should include the original message author's ID here.
   */
  readonly mentionedUserIds: readonly number[];
}
