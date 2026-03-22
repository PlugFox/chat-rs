// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** Protocol version. Incremented on breaking wire-format changes. */
export const protocolVersion = 1;

/** Wire frame header size: kind(1) + seq(4) + event_seq(4) = 9 bytes. */
export const frameHeaderSize = 9;

/** Minimum valid timestamp (1970-01-01 00:00:00 UTC). */
export const minTimestamp = 0;

/**
 * Maximum valid timestamp ((1 << 41) - 1 ≈ year 71,700).
 * Fast check: `value >> 41 != 0` → reject.
 * Catches milliseconds-instead-of-seconds bugs and is JS Number-safe.
 */
export const maxTimestamp = (1 << 41) - 1;

/**
 * Bitmask for detecting event_seq overflow.
 *
 * When `event_seq & EVENT_SEQ_OVERFLOW_MASK != 0` (top 2 bits set),
 * the server should send `DisconnectCode::EventSeqOverflow` and close
 * so the client reconnects with a fresh counter.
 */
export const eventSeqOverflowMask = 0xC000_0000;
