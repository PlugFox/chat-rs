// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// ReactionUpdate event payload (server → client).
@immutable
class ReactionUpdatePayload {
  const ReactionUpdatePayload({
    required this.chatId,
    required this.messageId,
    required this.userId,
    required this.packId,
    required this.emojiIndex,
    required this.added,
  });

  /// Chat containing the message.
  final int chatId;

  /// Message that was reacted to.
  final int messageId;

  /// User who added or removed the reaction.
  final int userId;

  /// Emoji pack ID.
  final int packId;

  /// Emoji index within the pack.
  final int emojiIndex;

  /// `true` = reaction added, `false` = reaction removed.
  final bool added;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ReactionUpdatePayload &&
          chatId == other.chatId &&
          messageId == other.messageId &&
          userId == other.userId &&
          packId == other.packId &&
          emojiIndex == other.emojiIndex &&
          added == other.added;
  // coverage:ignore-end

  @override
  int get hashCode =>
      Object.hash(chatId, messageId, userId, packId, emojiIndex, added);
}
