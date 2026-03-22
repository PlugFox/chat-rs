// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import type { ChatRole } from "./chat-role.js";
import type { Permission } from "./permission.js";

/**
 * Action to perform on a chat member (used in `UpdateMember` frame).
 *
 * Wire format: `action: u8` discriminant + action-specific payload.
 * Discriminant values: Kick=0, Ban=1, Mute=2, ChangeRole=3, UpdatePermissions=4, Unban=5.
 */
export type MemberAction =
  | { readonly type: "kick" }
  | { readonly type: "ban" }
  | { readonly type: "mute"; readonly durationSecs: number }
  | { readonly type: "changeRole"; readonly chatRole: ChatRole }
  | { readonly type: "updatePermissions"; readonly permission: Permission }
  | { readonly type: "unban" };
