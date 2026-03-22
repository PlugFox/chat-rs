// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import type { ChatRole } from "./chat-role.js";
import type { Permission } from "./permission.js";

/**
 * MemberUpdated event payload (server → client).
 *
 * Pushed when a member's role or permissions change in a chat.
 */
export interface MemberUpdatedPayload {
  /** Target chat. */
  readonly chatId: number;
  /** User whose membership changed. */
  readonly userId: number;
  /** New role. */
  readonly role: ChatRole;
  /** New permission override. `None` = use role defaults. */
  readonly permissions: Permission | null;
}
