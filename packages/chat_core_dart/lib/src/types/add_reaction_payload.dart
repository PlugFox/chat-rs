// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// AddReaction frame payload (client → server).
class AddReactionPayload {
  const AddReactionPayload({
    required this.chatId,
    required this.messageId,
    required this.packId,
    required this.emojiIndex,
  });

  /// Target chat.
  final int chatId;
  /// Target message.
  final int messageId;
  /// Emoji pack ID (0 = built-in Unicode set).
  final int packId;
  /// Emoji index within the pack (0–255).
  final int emojiIndex;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is AddReactionPayload &&
          chatId == other.chatId &&
          messageId == other.messageId &&
          packId == other.packId &&
          emojiIndex == other.emojiIndex;

  @override
  int get hashCode => Object.hash(
        chatId,
        messageId,
        packId,
        emojiIndex,
      );

  @override
  String toString() => 'AddReactionPayload(chatId: $chatId, messageId: $messageId, packId: $packId, emojiIndex: $emojiIndex)';
}
