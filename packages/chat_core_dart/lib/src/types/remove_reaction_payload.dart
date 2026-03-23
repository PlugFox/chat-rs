// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// RemoveReaction frame payload (client → server).
@immutable
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

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RemoveReactionPayload &&
          chatId == other.chatId &&
          messageId == other.messageId &&
          packId == other.packId &&
          emojiIndex == other.emojiIndex;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, messageId, packId, emojiIndex);
}
