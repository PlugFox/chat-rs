// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// UnpinMessage frame payload (client → server).
@immutable
class UnpinMessagePayload {
  const UnpinMessagePayload({required this.chatId, required this.messageId});

  /// Target chat.
  final int chatId;

  /// Message to unpin.
  final int messageId;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is UnpinMessagePayload &&
          chatId == other.chatId &&
          messageId == other.messageId;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, messageId);
}
