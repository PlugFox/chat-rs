// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** Online/offline status for a user. */
export const enum PresenceStatus {
  /** User is offline. */
  Offline = 0,
  /** User is online (has at least one active WS connection). */
  Online = 1,
}

/** Convert wire value to PresenceStatus, or undefined if unknown. */
export function presenceStatusFromValue(
  value: number,
): PresenceStatus | undefined {
  if (value >= 0 && value <= 1) return value as PresenceStatus;
  return undefined;
}
