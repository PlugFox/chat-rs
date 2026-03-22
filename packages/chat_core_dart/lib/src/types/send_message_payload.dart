// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'dart:typed_data';

import '../_util.dart';
import 'message_kind.dart';

/// SendMessage frame payload (client → server).
class SendMessagePayload {
  const SendMessagePayload({
    required this.chatId,
    required this.kind,
    required this.idempotencyKey,
    this.replyToId,
    required this.content,
    this.richContent,
    this.extra,
    required this.mentionedUserIds,
  });

  /// Target chat.
  final int chatId;

  /// Content type. Defaults to `Text` if omitted by the client.
  final MessageKind kind;

  /// Client-generated UUID for deduplication. Persisted 24h server-side.
  final String idempotencyKey;

  /// Message this is replying to. `None` = not a reply.
  final int? replyToId;

  /// Plain-text message content.
  final String content;

  /// Rich content spans (encoded as blob). `None` = no formatting.
  final Uint8List? richContent;

  /// Extra metadata JSON. `None` = no metadata.
  final String? extra;

  /// User IDs explicitly mentioned in this message.
  ///
  /// Server uses this for push notification routing (avoids parsing rich content).
  /// When replying, the client should include the original message author's ID here.
  final List<int> mentionedUserIds;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SendMessagePayload &&
          chatId == other.chatId &&
          kind == other.kind &&
          idempotencyKey == other.idempotencyKey &&
          replyToId == other.replyToId &&
          content == other.content &&
          listEquals(richContent, other.richContent) &&
          extra == other.extra &&
          listEquals(mentionedUserIds, other.mentionedUserIds);

  @override
  int get hashCode => Object.hash(
    chatId,
    kind,
    idempotencyKey,
    replyToId,
    content,
    Object.hashAll(richContent ?? const []),
    extra,
    Object.hashAll(mentionedUserIds),
  );

  @override
  String toString() =>
      'SendMessagePayload(chatId: $chatId, kind: $kind, idempotencyKey: $idempotencyKey, replyToId: $replyToId, content: $content, richContent: $richContent, extra: $extra, mentionedUserIds: $mentionedUserIds)';
}
