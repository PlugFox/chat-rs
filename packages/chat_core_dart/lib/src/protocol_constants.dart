// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// Protocol version. Incremented on breaking wire-format changes.
const int protocolVersion = 1;

/// Wire frame header size: kind(1) + seq(4) + event_seq(4) = 9 bytes.
const int frameHeaderSize = 9;

/// Minimum valid timestamp (1970-01-01 00:00:00 UTC).
const int minTimestamp = 0;

/// Maximum valid timestamp ((1 << 41) - 1 ≈ year 71,700).
/// Fast check: `value >> 41 != 0` → reject.
/// Catches milliseconds-instead-of-seconds bugs and is JS Number-safe.
const int maxTimestamp = (1 << 41) - 1;

/// Number of bits to shift a message ID right to get its chunk index.
///
/// `chunk_id = message_id >> CHUNK_SHIFT`
const int chunkShift = 6;

/// Number of messages per chunk (1 << CHUNK_SHIFT = 64).
///
/// Messages with IDs `[chunk_id * CHUNK_SIZE, (chunk_id + 1) * CHUNK_SIZE - 1]`
/// belong to the same chunk.
const int chunkSize = 1 << chunkShift;

/// Bitmask for detecting event_seq overflow.
///
/// When `event_seq & EVENT_SEQ_OVERFLOW_MASK != 0` (top 2 bits set),
/// the server should send `DisconnectCode::EventSeqOverflow` and close
/// so the client reconnects with a fresh counter.
const int eventSeqOverflowMask = 0xc0000000;
