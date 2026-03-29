// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// LoadChats frame payload (client → server).
///
/// Two modes selected by discriminant:
/// - Mode 0: first page (no cursor)
/// - Mode 1: subsequent page (cursor from previous response)
@immutable
sealed class LoadChatsPayload {
  const LoadChatsPayload();
}

/// First page — no cursor needed.
class LoadChatsFirstPage extends LoadChatsPayload {
  const LoadChatsFirstPage({required this.limit});

  /// Max entries to return.
  final int limit;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LoadChatsFirstPage && limit == other.limit;
  // coverage:ignore-end

  @override
  int get hashCode => limit.hashCode;
}

/// Subsequent page — uses `next_cursor_ts` from previous response.
class LoadChatsAfter extends LoadChatsPayload {
  const LoadChatsAfter({required this.cursorTs, required this.limit});

  /// Cursor timestamp from previous response's `next_cursor_ts`.
  final int cursorTs;

  /// Max entries to return.
  final int limit;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LoadChatsAfter &&
          cursorTs == other.cursorTs &&
          limit == other.limit;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(cursorTs, limit);
}
