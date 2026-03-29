// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// ReceiptUpdate event payload (server → client).
@immutable
class ReceiptUpdatePayload {
  const ReceiptUpdatePayload({
    required this.chatId,
    required this.userId,
    required this.messageId,
  });

  /// Chat where the receipt update occurred.
  final int chatId;

  /// User who read the messages.
  final int userId;

  /// Highest read message ID.
  final int messageId;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ReceiptUpdatePayload &&
          chatId == other.chatId &&
          userId == other.userId &&
          messageId == other.messageId;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, userId, messageId);
}
