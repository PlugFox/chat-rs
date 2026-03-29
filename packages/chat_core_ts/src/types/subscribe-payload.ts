// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/**
 * Subscribe frame payload (client → server).
 *
 * Subscribes to one or more named channels. Channel names follow conventions:
 * - `"general"` — account-level events (chat list updates, user profile changes)
 * - `"push"` — push notification events
 * - `"chat#123"` — real-time events for chat 123 (messages, typing, receipts)
 * - `"user#42"` — presence events for user 42
 *
 * This channel-based model decouples subscription from specific chat IDs,
 * allowing flexible event routing and future extensibility.
 */
export interface SubscribePayload {
  /** Channel names to subscribe to. */
  readonly channels: readonly string[];
}
