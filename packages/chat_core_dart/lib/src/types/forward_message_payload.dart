// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// ForwardMessage frame payload (client → server).
///
/// Server copies the original message content to the target chat,
/// sets `MessageFlags::FORWARDED`, and populates `extra.fwd` JSON.
class ForwardMessagePayload {
  const ForwardMessagePayload({
    required this.fromChatId,
    required this.messageId,
    required this.toChatId,
    required this.idempotencyKey,
  });

  /// Source chat containing the message to forward.
  final int fromChatId;

  /// Message to forward.
  final int messageId;

  /// Target chat to forward into.
  final int toChatId;

  /// Client-generated UUID for deduplication.
  final String idempotencyKey;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ForwardMessagePayload &&
          fromChatId == other.fromChatId &&
          messageId == other.messageId &&
          toChatId == other.toChatId &&
          idempotencyKey == other.idempotencyKey;

  @override
  int get hashCode =>
      Object.hash(fromChatId, messageId, toChatId, idempotencyKey);

  @override
  String toString() =>
      'ForwardMessagePayload(fromChatId: $fromChatId, messageId: $messageId, toChatId: $toChatId, idempotencyKey: $idempotencyKey)';
}
