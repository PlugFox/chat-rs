// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// GetBlockList frame payload (client → server, RPC).
@immutable
class GetBlockListPayload {
  const GetBlockListPayload({required this.cursor, required this.limit});

  /// Pagination cursor (0 = first page).
  final int cursor;

  /// Max entries to return.
  final int limit;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is GetBlockListPayload &&
          cursor == other.cursor &&
          limit == other.limit;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(cursor, limit);
}
