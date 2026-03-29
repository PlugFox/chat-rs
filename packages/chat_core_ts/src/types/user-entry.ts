// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import type { UserFlags } from "./user-flags.js";

/**
 * A user entry as transmitted on the wire (PresenceResult, user lookups).
 *
 * Wire format: 22-byte fixed header + 4 length-prefixed optional strings.
 */
export interface UserEntry {
  /** Internal sequential user ID. */
  readonly id: number;
  /** User type and capability flags. */
  readonly flags: UserFlags;
  /** Account creation timestamp, Unix seconds. */
  readonly createdAt: number;
  /** Last profile modification timestamp, Unix seconds. */
  readonly updatedAt: number;
  /** Lowercase latin slug (5–32 chars). `None` when not set. */
  readonly username: string | null;
  /** Display first name (1–64 chars). `None` when not set. */
  readonly firstName: string | null;
  /** Display last name (1–64 chars). `None` when not set. */
  readonly lastName: string | null;
  /** Avatar URL. `None` when not set. */
  readonly avatarUrl: string | null;
}
