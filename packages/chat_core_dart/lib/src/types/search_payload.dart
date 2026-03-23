// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:chat_core/src/types/search_scope.dart';

/// Search frame payload (client → server).
class SearchPayload {
  const SearchPayload({
    required this.scope,
    required this.query,
    required this.cursor,
    required this.limit,
  });

  /// Search scope.
  final SearchScope scope;

  /// Search query string.
  final String query;

  /// Pagination cursor (0 = first page).
  final int cursor;

  /// Max results to return.
  final int limit;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SearchPayload &&
          scope == other.scope &&
          query == other.query &&
          cursor == other.cursor &&
          limit == other.limit;

  @override
  int get hashCode => Object.hash(scope, query, cursor, limit);

  @override
  String toString() =>
      'SearchPayload(scope: $scope, query: $query, cursor: $cursor, limit: $limit)';
}
