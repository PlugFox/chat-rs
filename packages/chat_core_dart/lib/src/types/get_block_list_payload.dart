// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// GetBlockList frame payload (client → server, RPC).
class GetBlockListPayload {
  const GetBlockListPayload({required this.cursor, required this.limit});

  /// Pagination cursor (0 = first page).
  final int cursor;

  /// Max entries to return.
  final int limit;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is GetBlockListPayload &&
          cursor == other.cursor &&
          limit == other.limit;

  @override
  int get hashCode => Object.hash(cursor, limit);

  @override
  String toString() => 'GetBlockListPayload(cursor: $cursor, limit: $limit)';
}
