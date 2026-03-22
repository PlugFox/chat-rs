// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import type { Message } from './message.js';

/** A batch of messages (used in SyncBatch events and LoadMessages responses). */
export interface MessageBatch {
  /** Messages in this batch. */
  readonly messages: readonly Message[];
  /** Whether more messages exist beyond this batch. */
  readonly hasMore: boolean;
}
