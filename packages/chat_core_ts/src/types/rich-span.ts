// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import type { RichStyle } from './rich-style.js';

/**
 * A rich text span — a styled range within the plain-text content.
 *
 * Wire format: 10 bytes fixed (start: u32, end: u32, style: u16)
 * + meta_len: u32 + optional JSON meta.
 */
export interface RichSpan {
  /** Start byte offset into the plain-text content (inclusive). */
  readonly start: number;
  /** End byte offset into the plain-text content (exclusive). */
  readonly end: number;
  /** Style flags for this span. */
  readonly style: RichStyle;
  /** Optional JSON metadata. `None` when no meta-bearing style bits are set. */
  readonly meta: string | null;
}
