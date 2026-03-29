// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// DeleteChat frame payload (client → server).
@immutable
class DeleteChatPayload {
  const DeleteChatPayload({required this.chatId});

  /// Target chat.
  final int chatId;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is DeleteChatPayload && chatId == other.chatId;
  // coverage:ignore-end

  @override
  int get hashCode => chatId.hashCode;
}
