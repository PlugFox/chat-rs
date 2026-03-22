// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import type { MessageFlags } from './message-flags.js';
import type { MessageKind } from './message-kind.js';

/**
 * Lightweight last-message preview included in `ChatEntry`.
 *
 * Wire format: 21-byte fixed header + content preview string.
 */
export interface LastMessagePreview {
  /** Message ID. */
  readonly id: number;
  /** Sender's internal user ID. */
  readonly senderId: number;
  /** Creation timestamp, Unix seconds. */
  readonly createdAt: number;
  /** Content type. */
  readonly kind: MessageKind;
  /** Message property flags. */
  readonly flags: MessageFlags;
  /** Truncated plain-text preview (server-side, up to 100 bytes UTF-8). */
  readonly contentPreview: string;
}
