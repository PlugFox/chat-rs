// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// UnmuteChat frame payload (client → server, RPC).
@immutable
class UnmuteChatPayload {
  const UnmuteChatPayload({required this.chatId});

  /// Target chat.
  final int chatId;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is UnmuteChatPayload && chatId == other.chatId;
  // coverage:ignore-end

  @override
  int get hashCode => chatId.hashCode;
}
