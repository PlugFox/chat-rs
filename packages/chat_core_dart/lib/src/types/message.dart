// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

import 'package:chat_core/src/util/list_equals.dart';
import 'package:chat_core/src/types/message_flags.dart';
import 'package:chat_core/src/types/message_kind.dart';
import 'package:chat_core/src/types/rich_span.dart';

/// A decoded message (as transmitted in `MessageBatch`).
///
/// TODO: Add `reactions` field (Vec of pack_id + emoji_index + count + user_reacted)
/// so that reactions are persisted and available when loading message history.
/// Currently reactions are only delivered as live `ReactionUpdate` events.
@immutable
class Message {
  const Message({
    required this.id,
    required this.chatId,
    required this.senderId,
    required this.createdAt,
    required this.updatedAt,
    required this.kind,
    required this.flags,
    this.replyToId,
    required this.content,
    this.richContent,
    this.extra,
  });

  /// Sequential per-chat ID (starts at 1).
  final int id;

  /// Chat this message belongs to.
  final int chatId;

  /// Internal user ID of the sender.
  final int senderId;

  /// Creation timestamp, Unix seconds.
  final int createdAt;

  /// Last modification timestamp, Unix seconds.
  final int updatedAt;

  /// Content type.
  final MessageKind kind;

  /// Bitfield of message properties.
  final MessageFlags flags;

  /// Message this is replying to. `None` = not a reply.
  /// When set, `MessageFlags::REPLY` is also set.
  final int? replyToId;

  /// Plain text content; empty string for deleted tombstones.
  final String content;

  /// Rich content spans. `None` = no formatting.
  final List<RichSpan>? richContent;

  /// Extra metadata JSON. `None` = no metadata.
  final String? extra;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Message &&
          id == other.id &&
          chatId == other.chatId &&
          senderId == other.senderId &&
          createdAt == other.createdAt &&
          updatedAt == other.updatedAt &&
          kind == other.kind &&
          flags == other.flags &&
          replyToId == other.replyToId &&
          content == other.content &&
          listEquals(richContent, other.richContent) &&
          extra == other.extra;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(
    id,
    chatId,
    senderId,
    createdAt,
    updatedAt,
    kind,
    flags,
    replyToId,
    content,
    Object.hashAll(richContent ?? const []),
    extra,
  );
}
