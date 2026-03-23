// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// LeaveChat frame payload (client → server).
@immutable
class LeaveChatPayload {
  const LeaveChatPayload({required this.chatId});

  /// Target chat.
  final int chatId;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LeaveChatPayload && chatId == other.chatId;
  // coverage:ignore-end

  @override
  int get hashCode => chatId.hashCode;
}
