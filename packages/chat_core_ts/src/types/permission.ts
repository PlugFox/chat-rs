// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/**
 * Per-member permission flags (u32 on wire, i32 in PostgreSQL).
 *
 * `NULL` / absent in the database means "use role defaults".
 * See `default_permissions()` for the default set per role × chat kind.
 */
export namespace Permission {
  /** Can send text messages. */
  export const SEND_MESSAGES = 1 << 0;
  /** Can send media (images, files). */
  export const SEND_MEDIA = 1 << 1;
  /** Can send link previews. */
  export const SEND_LINKS = 1 << 2;
  /** Can pin messages. */
  export const PIN_MESSAGES = 1 << 3;
  /** Can edit own messages. */
  export const EDIT_OWN_MESSAGES = 1 << 4;
  /** Can delete own messages. */
  export const DELETE_OWN_MESSAGES = 1 << 5;
  /** Can delete other members' messages. */
  export const DELETE_OTHERS_MESSAGES = 1 << 10;
  /** Can mute members. */
  export const MUTE_MEMBERS = 1 << 11;
  /** Can ban members. */
  export const BAN_MEMBERS = 1 << 12;
  /** Can invite new members. */
  export const INVITE_MEMBERS = 1 << 20;
  /** Can kick members. */
  export const KICK_MEMBERS = 1 << 21;
  /** Can change chat title, avatar. */
  export const MANAGE_CHAT_INFO = 1 << 22;
  /** Can assign/change member roles. */
  export const MANAGE_ROLES = 1 << 23;
  /** Can transfer ownership to another member. */
  export const TRANSFER_OWNERSHIP = 1 << 30;
  /** Can delete the chat entirely. */
  export const DELETE_CHAT = 1 << 31;

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
export type Permission = number;
