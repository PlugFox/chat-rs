// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// PinMessage frame payload (client → server).
@immutable
class PinMessagePayload {
  const PinMessagePayload({required this.chatId, required this.messageId});

  /// Target chat.
  final int chatId;

  /// Message to pin.
  final int messageId;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is PinMessagePayload &&
          chatId == other.chatId &&
          messageId == other.messageId;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, messageId);
}
