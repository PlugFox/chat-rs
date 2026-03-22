// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// PinMessage frame payload (client → server).
class PinMessagePayload {
  const PinMessagePayload({required this.chatId, required this.messageId});

  /// Target chat.
  final int chatId;

  /// Message to pin.
  final int messageId;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is PinMessagePayload &&
          chatId == other.chatId &&
          messageId == other.messageId;

  @override
  int get hashCode => Object.hash(chatId, messageId);

  @override
  String toString() =>
      'PinMessagePayload(chatId: $chatId, messageId: $messageId)';
}
