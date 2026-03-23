// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// MuteChat frame payload (client → server, RPC).
@immutable
class MuteChatPayload {
  const MuteChatPayload({required this.chatId, required this.durationSecs});

  /// Target chat.
  final int chatId;

  /// Mute duration in seconds. `0` = mute forever.
  final int durationSecs;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MuteChatPayload &&
          chatId == other.chatId &&
          durationSecs == other.durationSecs;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(chatId, durationSecs);
}
