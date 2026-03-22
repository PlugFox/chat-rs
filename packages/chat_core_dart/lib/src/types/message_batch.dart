// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import '../_util.dart';
import 'message.dart';

/// A batch of messages (used in SyncBatch events and LoadMessages responses).
class MessageBatch {
  const MessageBatch({
    required this.messages,
    required this.hasMore,
  });

  /// Messages in this batch.
  final List<Message> messages;
  /// Whether more messages exist beyond this batch.
  final bool hasMore;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MessageBatch &&
          listEquals(messages, other.messages) &&
          hasMore == other.hasMore;

  @override
  int get hashCode => Object.hash(
        Object.hashAll(messages),
        hasMore,
      );

  @override
  String toString() => 'MessageBatch(messages: $messages, hasMore: $hasMore)';
}
