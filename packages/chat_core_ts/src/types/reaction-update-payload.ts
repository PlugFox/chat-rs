// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** ReactionUpdate event payload (server → client). */
export interface ReactionUpdatePayload {
  /** Chat containing the message. */
  readonly chatId: number;
  /** Message that was reacted to. */
  readonly messageId: number;
  /** User who added or removed the reaction. */
  readonly userId: number;
  /** Emoji pack ID. */
  readonly packId: number;
  /** Emoji index within the pack. */
  readonly emojiIndex: number;
  /** `true` = reaction added, `false` = reaction removed. */
  readonly added: boolean;
}
