// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'dart:typed_data';

import '../_util.dart';

/// EditMessage frame payload (client → server).
class EditMessagePayload {
  const EditMessagePayload({
    required this.chatId,
    required this.messageId,
    required this.content,
    this.richContent,
    this.extra,
  });

  /// Target chat.
  final int chatId;
  /// Message to edit.
  final int messageId;
  /// New plain-text content.
  final String content;
  /// New rich content spans. `None` = remove formatting.
  final Uint8List? richContent;
  /// New extra metadata JSON. `None` = remove metadata.
  final String? extra;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is EditMessagePayload &&
          chatId == other.chatId &&
          messageId == other.messageId &&
          content == other.content &&
          listEquals(richContent, other.richContent) &&
          extra == other.extra;

  @override
  int get hashCode => Object.hash(
        chatId,
        messageId,
        content,
        Object.hashAll(richContent ?? const []),
        extra,
      );

  @override
  String toString() => 'EditMessagePayload(chatId: $chatId, messageId: $messageId, content: $content, richContent: $richContent, extra: $extra)';
}
