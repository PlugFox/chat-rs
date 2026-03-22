// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// RemoveReaction frame payload (client → server).
class RemoveReactionPayload {
  const RemoveReactionPayload({
    required this.chatId,
    required this.messageId,
    required this.packId,
    required this.emojiIndex,
  });

  /// Target chat.
  final int chatId;
  /// Target message.
  final int messageId;
  /// Emoji pack ID.
  final int packId;
  /// Emoji index within the pack.
  final int emojiIndex;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RemoveReactionPayload &&
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
  String toString() => 'RemoveReactionPayload(chatId: $chatId, messageId: $messageId, packId: $packId, emojiIndex: $emojiIndex)';
}
