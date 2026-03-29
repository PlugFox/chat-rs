// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** AddReaction frame payload (client → server). */
export interface AddReactionPayload {
  /** Target chat. */
  readonly chatId: number;
  /** Target message. */
  readonly messageId: number;
  /** Emoji pack ID (0 = built-in Unicode set). */
  readonly packId: number;
  /** Emoji index within the pack (0–255). */
  readonly emojiIndex: number;
}
