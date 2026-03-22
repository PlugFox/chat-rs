// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import type { ServerCapabilities } from "./server-capabilities.js";
import type { ServerLimits } from "./server-limits.js";

/** Welcome frame payload (server → client). */
export interface WelcomePayload {
  /** Transient session identifier for this connection. */
  readonly sessionId: number;
  /** Server clock, Unix seconds. Client uses for clock-sync. */
  readonly serverTime: number;
  /** Authenticated user's internal ID. */
  readonly userId: number;
  /** Server-enforced limits. */
  readonly limits: ServerLimits;
  /** Server-advertised feature capabilities. */
  readonly capabilities: ServerCapabilities;
}
