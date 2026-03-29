// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

import 'package:chat_core/src/types/load_direction.dart';

/// LoadMessages frame payload (client → server).
///
/// Three modes selected by discriminant:
/// - Mode 0: anchor-based pagination (history load)
/// - Mode 1: range update check (catch-up after reconnect)
/// - Mode 2: chunk load/update (chunk-based access)
@immutable
sealed class LoadMessagesPayload {
  const LoadMessagesPayload();
}

/// Anchor-based pagination (mode 0).
class LoadMessagesPaginate extends LoadMessagesPayload {
  const LoadMessagesPaginate({
    required this.chatId,
    required this.direction,
    required this.anchorId,
    required this.limit,
  });

  /// Target chat.
  final int chatId;

  /// Scroll direction.
  final LoadDirection direction;

  /// Anchor message ID (0 = start from newest).
  final int anchorId;

  /// Max messages to return.
  final int limit;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LoadMessagesPaginate &&
          chatId == other.chatId &&
          direction == other.direction &&
          anchorId == other.anchorId &&
          limit == other.limit;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, direction, anchorId, limit);
}

/// Range update check (mode 1).
class LoadMessagesRangeCheck extends LoadMessagesPayload {
  const LoadMessagesRangeCheck({
    required this.chatId,
    required this.fromId,
    required this.toId,
    required this.sinceTs,
  });

  /// Target chat.
  final int chatId;

  /// Start of the range (inclusive).
  final int fromId;

  /// End of the range (inclusive).
  final int toId;

  /// `MAX(updated_at)` from client's local cache for this range.
  final int sinceTs;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LoadMessagesRangeCheck &&
          chatId == other.chatId &&
          fromId == other.fromId &&
          toId == other.toId &&
          sinceTs == other.sinceTs;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, fromId, toId, sinceTs);
}

/// Chunk load/update (mode 2).
///
/// Request all messages in a chunk, or only those updated after `since_ts`.
/// `chunk_id = message_id >> CHUNK_SHIFT`. See [`CHUNK_SHIFT`](crate::CHUNK_SHIFT).
class LoadMessagesChunk extends LoadMessagesPayload {
  const LoadMessagesChunk({
    required this.chatId,
    required this.chunkId,
    required this.sinceTs,
  });

  /// Target chat.
  final int chatId;

  /// Chunk index (`message_id >> CHUNK_SHIFT`).
  final int chunkId;

  /// Return only messages with `updated_at > since_ts`.
  /// `0` = return all messages in the chunk.
  final int sinceTs;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LoadMessagesChunk &&
          chatId == other.chatId &&
          chunkId == other.chunkId &&
          sinceTs == other.sinceTs;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, chunkId, sinceTs);
}
