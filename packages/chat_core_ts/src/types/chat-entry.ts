// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import type { ChatKind } from "./chat-kind.js";
import type { LastMessagePreview } from "./last-message-preview.js";

/** A chat entry as transmitted on the wire (LoadChats, ChatCreated, ChatUpdated). */
export interface ChatEntry {
  /** Globally unique chat ID. */
  readonly id: number;
  /** Chat type. */
  readonly kind: ChatKind;
  /** Parent group ID (present only for channels). */
  readonly parentId: number | null;
  /** Creation timestamp, Unix seconds. */
  readonly createdAt: number;
  /** Last modification timestamp, Unix seconds. */
  readonly updatedAt: number;
  /** Display title. `None` for DMs. */
  readonly title: string | null;
  /** Avatar URL. `None` when absent. */
  readonly avatarUrl: string | null;
  /** Last message preview. `None` for empty chats. */
  readonly lastMessage: LastMessagePreview | null;
  /** Number of unread messages (server-computed: `last_msg_id - last_read_msg_id`). */
  readonly unreadCount: number;
  /** Total number of members in this chat. */
  readonly memberCount: number;
}
