// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import type { LoadDirection } from "./load-direction.js";

/**
 * LoadMessages frame payload (client → server).
 *
 * Three modes selected by discriminant:
 * - Mode 0: anchor-based pagination (history load)
 * - Mode 1: range update check (catch-up after reconnect)
 * - Mode 2: chunk load/update (chunk-based access)
 */
export type LoadMessagesPayload =
  | {
      readonly type: "paginate";
      readonly chatId: number;
      readonly direction: LoadDirection;
      readonly anchorId: number;
      readonly limit: number;
    }
  | {
      readonly type: "rangeCheck";
      readonly chatId: number;
      readonly fromId: number;
      readonly toId: number;
      readonly sinceTs: number;
    }
  | {
      readonly type: "chunk";
      readonly chatId: number;
      readonly chunkId: number;
      readonly sinceTs: number;
    };
