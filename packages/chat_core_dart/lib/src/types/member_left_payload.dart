// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// MemberLeft event payload (server → client).
@immutable
class MemberLeftPayload {
  const MemberLeftPayload({required this.chatId, required this.userId});

  /// Target chat.
  final int chatId;

  /// User who left.
  final int userId;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MemberLeftPayload &&
          chatId == other.chatId &&
          userId == other.userId;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, userId);
}
