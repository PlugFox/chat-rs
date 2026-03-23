// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

import 'package:chat_core/src/types/search_scope.dart';

/// Search frame payload (client → server).
@immutable
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

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SearchPayload &&
          scope == other.scope &&
          query == other.query &&
          cursor == other.cursor &&
          limit == other.limit;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(scope, query, cursor, limit);
}
