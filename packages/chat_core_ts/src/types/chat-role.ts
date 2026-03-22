// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** Member role within a chat, ordered by privilege level. */
export const enum ChatRole {
  /** Regular member. */
  Member = 0,
  /** Can moderate (delete others' messages, mute). */
  Moderator = 1,
  /** Can manage (invite, kick, change settings, assign roles). */
  Admin = 2,
  /** Full control (transfer ownership, delete chat). */
  Owner = 3,
}

/** Convert wire value to ChatRole, or undefined if unknown. */
export function chatRoleFromValue(value: number): ChatRole | undefined {
  if (value >= 0 && value <= 3) return value as ChatRole;
  return undefined;
}
