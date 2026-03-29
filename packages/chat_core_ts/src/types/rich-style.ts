// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/**
 * Rich text style flags (u16 on wire).
 *
 * Inline styles are freely combinable. Block-level styles (`CODE_BLOCK`,
 * `BLOCKQUOTE`) have special semantics — see docs/messages.md.
 */
export namespace RichStyle {
  /** Bold text. */
  export const BOLD = 0x0001;
  /** Italic text. */
  export const ITALIC = 0x0002;
  /** Underlined text. */
  export const UNDERLINE = 0x0004;
  /** Strikethrough text. */
  export const STRIKE = 0x0008;
  /** Spoiler text (hidden until tapped/clicked). */
  export const SPOILER = 0x0010;
  /** Inline monospace code. */
  export const CODE = 0x0020;
  /** Hyperlink. Meta: `{"url": "..."}`. */
  export const LINK = 0x0040;
  /** User mention. Meta: `{"user_id": u32}`. */
  export const MENTION = 0x0080;
  /** Colored text. Meta: `{"rgba": u32}`. */
  export const COLOR = 0x0100;
  /**
   * Fenced code block. Meta: `{"lang": "rust"}`.
   * When set, client ignores inline style bits on this span.
   */
  export const CODE_BLOCK = 0x0200;
  /** Block quote (`>` prefixed text). */
  export const BLOCKQUOTE = 0x0400;

  export function contains(flags: number, flag: number): boolean {
    return (flags & flag) !== 0;
  }
  export function add(flags: number, flag: number): number {
    return flags | flag;
  }
  export function remove(flags: number, flag: number): number {
    return flags & ~flag;
  }
  export function toggle(flags: number, flag: number): number {
    return flags ^ flag;
  }
}
export type RichStyle = number;
