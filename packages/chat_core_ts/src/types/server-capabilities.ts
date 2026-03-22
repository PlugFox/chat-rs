// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/**
 * Server-advertised feature capabilities (u32 on wire).
 *
 * Sent in Welcome. Client uses these to show/hide features.
 */
export namespace ServerCapabilities {
  /** File and image upload enabled. */
  export const MEDIA_UPLOAD = 0x01;
  /** Full-text message search enabled. */
  export const SEARCH = 0x02;
  /** Emoji reactions enabled. */
  export const REACTIONS = 0x04;
  /** Message threads/replies enabled. */
  export const THREADS = 0x08;
  /** Bot API enabled. */
  export const BOTS = 0x10;

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
export type ServerCapabilities = number;
