// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** GetChatMembers frame payload (client → server). */
export interface GetChatMembersPayload {
  /** Target chat. */
  readonly chatId: number;
  /** Pagination cursor (0 = first page). */
  readonly cursor: number;
  /** Max members to return. */
  readonly limit: number;
}
