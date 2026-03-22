// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import type { MessageFlags } from './message-flags.js';
import type { MessageKind } from './message-kind.js';
import type { RichSpan } from './rich-span.js';

/**
 * A decoded message (as transmitted in `MessageBatch`).
 *
 * TODO: Add `reactions` field (Vec of pack_id + emoji_index + count + user_reacted)
 * so that reactions are persisted and available when loading message history.
 * Currently reactions are only delivered as live `ReactionUpdate` events.
 */
export interface Message {
  /** Sequential per-chat ID (starts at 1). */
  readonly id: number;
  /** Chat this message belongs to. */
  readonly chatId: number;
  /** Internal user ID of the sender. */
  readonly senderId: number;
  /** Creation timestamp, Unix seconds. */
  readonly createdAt: number;
  /** Last modification timestamp, Unix seconds. */
  readonly updatedAt: number;
  /** Content type. */
  readonly kind: MessageKind;
  /** Bitfield of message properties. */
  readonly flags: MessageFlags;
  /**
   * Message this is replying to. `None` = not a reply.
   * When set, `MessageFlags::REPLY` is also set.
   */
  readonly replyToId: number | null;
  /** Plain text content; empty string for deleted tombstones. */
  readonly content: string;
  /** Rich content spans. `None` = no formatting. */
  readonly richContent: readonly RichSpan[] | null;
  /** Extra metadata JSON. `None` = no metadata. */
  readonly extra: string | null;
}
