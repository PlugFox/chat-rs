// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'chat_kind.dart';
import 'last_message_preview.dart';

/// A chat entry as transmitted on the wire (LoadChats, ChatCreated, ChatUpdated).
class ChatEntry {
  const ChatEntry({
    required this.id,
    required this.kind,
    this.parentId,
    required this.createdAt,
    required this.updatedAt,
    this.title,
    this.avatarUrl,
    this.lastMessage,
    required this.unreadCount,
    required this.memberCount,
  });

  /// Globally unique chat ID.
  final int id;
  /// Chat type.
  final ChatKind kind;
  /// Parent group ID (present only for channels).
  final int? parentId;
  /// Creation timestamp, Unix seconds.
  final int createdAt;
  /// Last modification timestamp, Unix seconds.
  final int updatedAt;
  /// Display title. `None` for DMs.
  final String? title;
  /// Avatar URL. `None` when absent.
  final String? avatarUrl;
  /// Last message preview. `None` for empty chats.
  final LastMessagePreview? lastMessage;
  /// Number of unread messages (server-computed: `last_msg_id - last_read_msg_id`).
  final int unreadCount;
  /// Total number of members in this chat.
  final int memberCount;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ChatEntry &&
          id == other.id &&
          kind == other.kind &&
          parentId == other.parentId &&
          createdAt == other.createdAt &&
          updatedAt == other.updatedAt &&
          title == other.title &&
          avatarUrl == other.avatarUrl &&
          lastMessage == other.lastMessage &&
          unreadCount == other.unreadCount &&
          memberCount == other.memberCount;

  @override
  int get hashCode => Object.hash(
        id,
        kind,
        parentId,
        createdAt,
        updatedAt,
        title,
        avatarUrl,
        lastMessage,
        unreadCount,
        memberCount,
      );

  @override
  String toString() => 'ChatEntry(id: $id, kind: $kind, parentId: $parentId, createdAt: $createdAt, updatedAt: $updatedAt, title: $title, avatarUrl: $avatarUrl, lastMessage: $lastMessage, unreadCount: $unreadCount, memberCount: $memberCount)';
}
