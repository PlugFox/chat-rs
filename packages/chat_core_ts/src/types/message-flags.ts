// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** Message property flags (u16 on wire, i16 in PostgreSQL). */
export namespace MessageFlags {
  /** Edited at least once; display "edited" label. */
  export const EDITED = 0x0001;
  /** Soft-deleted tombstone; content is empty. */
  export const DELETED = 0x0002;
  /** Forwarded from another chat; origin in extra JSON. */
  export const FORWARDED = 0x0004;
  /** Pinned in this chat. */
  export const PINNED = 0x0008;
  /** No push notification for this message. */
  export const SILENT = 0x0010;
  /** System event message (member join/leave, etc.). */
  export const SYSTEM = 0x0020;
  /** Sent by a bot user (server-authoritative). */
  export const BOT = 0x0040;
  /** Reply to another message; origin in extra JSON. */
  export const REPLY = 0x0080;

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
export type MessageFlags = number;
