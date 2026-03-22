// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** Message content type. */
export const enum MessageKind {
  /** Plain text message. */
  Text = 0,
  /** Image message. */
  Image = 1,
  /** File attachment. */
  File = 2,
  /** System event (join/leave/etc). Always paired with `MessageFlags::SYSTEM`. */
  System = 3,
}

/** Convert wire value to MessageKind, or undefined if unknown. */
export function messageKindFromValue(value: number): MessageKind | undefined {
  if (value >= 0 && value <= 3) return value as MessageKind;
  return undefined;
}
