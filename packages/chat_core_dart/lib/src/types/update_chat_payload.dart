// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// UpdateChat frame payload (client → server).
///
/// **Clear semantics**: an empty string means "clear this field" (set to NULL on server).
/// `None` means "don't change". On the wire, `None` = `len 0` and empty string is not
/// distinguishable from `None`, so we use a `u8 flag` prefix:
/// `0` = don't change, `1` = set to following string (empty string = clear).
@immutable
class UpdateChatPayload {
  const UpdateChatPayload({required this.chatId, this.title, this.avatarUrl});

  /// Target chat.
  final int chatId;

  /// New title. `None` = don't change. `Some("")` = clear.
  final String? title;

  /// New avatar URL. `None` = don't change. `Some("")` = clear.
  final String? avatarUrl;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is UpdateChatPayload &&
          chatId == other.chatId &&
          title == other.title &&
          avatarUrl == other.avatarUrl;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, title, avatarUrl);
}
