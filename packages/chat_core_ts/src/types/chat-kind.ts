// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** Chat type. */
export const enum ChatKind {
  /** Direct message (exactly two participants, no title). */
  Direct = 0,
  /** Group conversation with multiple members. */
  Group = 1,
  /** Read-mostly broadcast room nested inside a Group. */
  Channel = 2,
}

/** Convert wire value to ChatKind, or undefined if unknown. */
export function chatKindFromValue(value: number): ChatKind | undefined {
  if (value >= 0 && value <= 2) return value as ChatKind;
  return undefined;
}
