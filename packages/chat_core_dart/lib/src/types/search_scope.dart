// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// Search scope selector.
sealed class SearchScope {
  const SearchScope();
}

/// Search within a specific chat.
class SearchScopeChat extends SearchScope {
  const SearchScopeChat({
    required this.chatId,
  });

  final int chatId;
}

/// Search across all chats the user is a member of.
class SearchScopeGlobal extends SearchScope {
  const SearchScopeGlobal();
}

/// Search messages from a specific user across all chats.
class SearchScopeUser extends SearchScope {
  const SearchScopeUser({
    required this.userId,
  });

  final int userId;
}
