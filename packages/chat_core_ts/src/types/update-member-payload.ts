// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import type { MemberAction } from "./member-action.js";

/**
 * UpdateMember frame payload (client → server).
 *
 * Unified frame for kick, ban, mute, role change, and permission override.
 * Replaces the previous separate `KickMember`, `BanMember`, `MuteMember`,
 * and `UpdateMemberRole` frames.
 */
export interface UpdateMemberPayload {
  /** Target chat. */
  readonly chatId: number;
  /** Target user. */
  readonly userId: number;
  /** Action to perform. */
  readonly action: MemberAction;
}
