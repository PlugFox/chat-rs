// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// DeleteMessage frame payload (client → server).
@immutable
class DeleteMessagePayload {
  const DeleteMessagePayload({required this.chatId, required this.messageId});

  /// Target chat.
  final int chatId;

  /// Message to delete.
  final int messageId;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is DeleteMessagePayload &&
          chatId == other.chatId &&
          messageId == other.messageId;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, messageId);
}
