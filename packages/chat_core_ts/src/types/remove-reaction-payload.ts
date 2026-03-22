// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** RemoveReaction frame payload (client → server). */
export interface RemoveReactionPayload {
  /** Target chat. */
  readonly chatId: number;
  /** Target message. */
  readonly messageId: number;
  /** Emoji pack ID. */
  readonly packId: number;
  /** Emoji index within the pack. */
  readonly emojiIndex: number;
}
