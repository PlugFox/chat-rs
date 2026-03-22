// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import type { PresenceStatus } from './presence-status.js';

/** A presence entry as transmitted in `PresenceResult` (13 bytes fixed). */
export interface PresenceEntry {
  /** User ID. */
  readonly userId: number;
  /** Current online/offline status. */
  readonly status: PresenceStatus;
  /** Last seen timestamp, Unix seconds. `0` when user is currently online. */
  readonly lastSeen: number;
}
