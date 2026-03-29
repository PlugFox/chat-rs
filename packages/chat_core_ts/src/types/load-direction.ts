// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** LoadMessages mode selector. */
export const enum LoadDirection {
  /** Load older messages (before anchor). */
  Older = 0,
  /** Load newer messages (after anchor). */
  Newer = 1,
}

/** Convert wire value to LoadDirection, or undefined if unknown. */
export function loadDirectionFromValue(
  value: number,
): LoadDirection | undefined {
  if (value >= 0 && value <= 1) return value as LoadDirection;
  return undefined;
}
