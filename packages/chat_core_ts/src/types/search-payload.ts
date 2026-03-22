// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import type { SearchScope } from "./search-scope.js";

/** Search frame payload (client → server). */
export interface SearchPayload {
  /** Search scope. */
  readonly scope: SearchScope;
  /** Search query string. */
  readonly query: string;
  /** Pagination cursor (0 = first page). */
  readonly cursor: number;
  /** Max results to return. */
  readonly limit: number;
}
