// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** GetUsers frame payload (client → server, RPC). */
export interface GetUsersPayload {
  /** User IDs to look up (batch). */
  readonly userIds: readonly number[];
}
