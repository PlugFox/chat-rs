// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// Search scope selector.
@immutable
sealed class SearchScope {
  const SearchScope();
}

/// Search within a specific chat.
class SearchScopeChat extends SearchScope {
  const SearchScopeChat({required this.chatId});

  final int chatId;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SearchScopeChat && chatId == other.chatId;
  // coverage:ignore-end

  @override
  int get hashCode => chatId.hashCode;
}

/// Search across all chats the user is a member of.
class SearchScopeGlobal extends SearchScope {
  const SearchScopeGlobal();

  @override
  bool operator ==(Object other) =>
      identical(this, other) || other is SearchScopeGlobal; // coverage:ignore-line

  @override
  int get hashCode => 0;
}

/// Search messages from a specific user across all chats.
class SearchScopeUser extends SearchScope {
  const SearchScopeUser({required this.userId});

  final int userId;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SearchScopeUser && userId == other.userId;
  // coverage:ignore-end

  @override
  int get hashCode => userId.hashCode;
}
