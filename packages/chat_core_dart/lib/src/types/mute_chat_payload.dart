// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// MuteChat frame payload (client → server, RPC).
class MuteChatPayload {
  const MuteChatPayload({
    required this.chatId,
    required this.durationSecs,
  });

  /// Target chat.
  final int chatId;
  /// Mute duration in seconds. `0` = mute forever.
  final int durationSecs;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MuteChatPayload &&
          chatId == other.chatId &&
          durationSecs == other.durationSecs;

  @override
  int get hashCode => Object.hash(
        chatId,
        durationSecs,
      );

  @override
  String toString() => 'MuteChatPayload(chatId: $chatId, durationSecs: $durationSecs)';
}
