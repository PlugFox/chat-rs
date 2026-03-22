// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** InviteMembers frame payload (client → server). */
export interface InviteMembersPayload {
  /** Target chat. */
  readonly chatId: number;
  /** User IDs to invite. */
  readonly userIds: readonly number[];
}
