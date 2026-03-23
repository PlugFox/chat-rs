// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// MessageDeleted event payload (server → client).
///
/// Content is already cleared server-side; this event tells the client
/// which message was deleted so it can update the UI.
@immutable
class MessageDeletedPayload {
  const MessageDeletedPayload({required this.chatId, required this.messageId});

  /// Chat containing the deleted message.
  final int chatId;

  /// Deleted message ID.
  final int messageId;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MessageDeletedPayload &&
          chatId == other.chatId &&
          messageId == other.messageId;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, messageId);
}
