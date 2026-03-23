// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:chat_core/src/util/list_equals.dart';
import 'package:chat_core/src/types/chat_kind.dart';

/// CreateChat frame payload (client → server).
class CreateChatPayload {
  const CreateChatPayload({
    required this.kind,
    this.parentId,
    this.title,
    this.avatarUrl,
    required this.memberIds,
  });

  /// Chat type.
  final ChatKind kind;

  /// Parent group ID (required for channels).
  final int? parentId;

  /// Chat title (absent for DMs).
  final String? title;

  /// Chat avatar URL.
  final String? avatarUrl;

  /// Initial member user IDs.
  final List<int> memberIds;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CreateChatPayload &&
          kind == other.kind &&
          parentId == other.parentId &&
          title == other.title &&
          avatarUrl == other.avatarUrl &&
          listEquals(memberIds, other.memberIds);

  @override
  int get hashCode =>
      Object.hash(kind, parentId, title, avatarUrl, Object.hashAll(memberIds));

  @override
  String toString() =>
      'CreateChatPayload(kind: $kind, parentId: $parentId, title: $title, avatarUrl: $avatarUrl, memberIds: $memberIds)';
}
