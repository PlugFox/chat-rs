// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// Search scope selector.
sealed class SearchScope {
  const SearchScope();
}

/// Search within a specific chat.
class SearchScopeChat extends SearchScope {
  const SearchScopeChat({required this.chatId});

  final int chatId;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SearchScopeChat && chatId == other.chatId;

  @override
  int get hashCode => chatId.hashCode;

  @override
  String toString() => 'SearchScopeChat(chatId: $chatId)';
}

/// Search across all chats the user is a member of.
class SearchScopeGlobal extends SearchScope {
  const SearchScopeGlobal();

  @override
  bool operator ==(Object other) =>
      identical(this, other) || other is SearchScopeGlobal;

  @override
  int get hashCode => 0;

  @override
  String toString() => 'SearchScopeGlobal()';
}

/// Search messages from a specific user across all chats.
class SearchScopeUser extends SearchScope {
  const SearchScopeUser({required this.userId});

  final int userId;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SearchScopeUser && userId == other.userId;

  @override
  int get hashCode => userId.hashCode;

  @override
  String toString() => 'SearchScopeUser(userId: $userId)';
}
