// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** GetBlockList frame payload (client → server, RPC). */
export interface GetBlockListPayload {
  /** Pagination cursor (0 = first page). */
  readonly cursor: number;
  /** Max entries to return. */
  readonly limit: number;
}
