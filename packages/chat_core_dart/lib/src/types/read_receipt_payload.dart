// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// ReadReceipt frame payload (client → server, fire-and-forget).
@immutable
class ReadReceiptPayload {
  const ReadReceiptPayload({required this.chatId, required this.messageId});

  /// Target chat.
  final int chatId;

  /// Highest read message ID.
  final int messageId;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ReadReceiptPayload &&
          chatId == other.chatId &&
          messageId == other.messageId;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, messageId);
}
