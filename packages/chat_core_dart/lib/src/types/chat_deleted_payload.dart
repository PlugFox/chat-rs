// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// ChatDeleted event payload (server → client).
///
/// Pushed when a chat is deleted. Clients should remove it from the chat list.
@immutable
class ChatDeletedPayload {
  const ChatDeletedPayload({required this.chatId});

  /// Deleted chat ID.
  final int chatId;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ChatDeletedPayload && chatId == other.chatId;
  // coverage:ignore-end

  @override
  int get hashCode => chatId.hashCode;
}
