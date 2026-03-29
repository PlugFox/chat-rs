// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import type { ChatKind } from "./chat-kind.js";

/** CreateChat frame payload (client → server). */
export interface CreateChatPayload {
  /** Chat type. */
  readonly kind: ChatKind;
  /** Parent group ID (required for channels). */
  readonly parentId: number | null;
  /** Chat title (absent for DMs). */
  readonly title: string | null;
  /** Chat avatar URL. */
  readonly avatarUrl: string | null;
  /** Initial member user IDs. */
  readonly memberIds: readonly number[];
}
