// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** User type and capability flags (u16 on wire, i16 in PostgreSQL). */
export namespace UserFlags {
  /** System account (server-generated messages). */
  export const SYSTEM = 0x0001;
  /** Bot account; server sets `MessageFlags::BOT` on all messages. */
  export const BOT = 0x0002;
  /** Premium subscriber; clients may show a badge. */
  export const PREMIUM = 0x0004;

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
export type UserFlags = number;
