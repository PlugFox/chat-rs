// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import type { ChatRole } from "./chat-role.js";

/** MemberJoined event payload (server → client). */
export interface MemberJoinedPayload {
  /** Target chat. */
  readonly chatId: number;
  /** User who joined. */
  readonly userId: number;
  /** Role assigned to the new member. */
  readonly role: ChatRole;
  /** User who invited them. `0` = self-join (e.g. via invite link). */
  readonly invitedBy: number;
}
