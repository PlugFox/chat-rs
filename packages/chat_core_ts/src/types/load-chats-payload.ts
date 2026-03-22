// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/**
 * LoadChats frame payload (client → server).
 *
 * Two modes selected by discriminant:
 * - Mode 0: first page (no cursor)
 * - Mode 1: subsequent page (cursor from previous response)
 */
export type LoadChatsPayload =
  | { readonly type: "firstPage"; readonly limit: number }
  | {
      readonly type: "after";
      readonly cursorTs: number;
      readonly limit: number;
    };
