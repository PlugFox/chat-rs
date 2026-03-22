// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'load_direction.dart';

/// LoadMessages frame payload (client → server).
///
/// Two modes selected by discriminant:
/// - Mode 0: anchor-based pagination (history load)
/// - Mode 1: range update check (catch-up after reconnect)
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

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LoadMessagesPaginate &&
          chatId == other.chatId &&
          direction == other.direction &&
          anchorId == other.anchorId &&
          limit == other.limit;

  @override
  int get hashCode => Object.hash(chatId, direction, anchorId, limit);

  @override
  String toString() =>
      'LoadMessagesPaginate(chatId: $chatId, direction: $direction, anchorId: $anchorId, limit: $limit)';
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

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LoadMessagesRangeCheck &&
          chatId == other.chatId &&
          fromId == other.fromId &&
          toId == other.toId &&
          sinceTs == other.sinceTs;

  @override
  int get hashCode => Object.hash(chatId, fromId, toId, sinceTs);

  @override
  String toString() =>
      'LoadMessagesRangeCheck(chatId: $chatId, fromId: $fromId, toId: $toId, sinceTs: $sinceTs)';
}
