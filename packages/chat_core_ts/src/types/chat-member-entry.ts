// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import type { ChatRole } from "./chat-role.js";
import type { Permission } from "./permission.js";

/** A chat member entry as transmitted on the wire (GetChatMembers response). */
export interface ChatMemberEntry {
  /** Internal user ID. */
  readonly userId: number;
  /** Member's role. */
  readonly role: ChatRole;
  /** Permission override. `None` = use role defaults. */
  readonly permissions: Permission | null;
}
