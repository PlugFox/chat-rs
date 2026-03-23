// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

import 'package:chat_core/src/util/list_equals.dart';
import 'package:chat_core/src/types/message.dart';

/// A batch of messages (used in SyncBatch events and LoadMessages responses).
@immutable
class MessageBatch {
  const MessageBatch({required this.messages, required this.hasMore});

  /// Messages in this batch.
  final List<Message> messages;

  /// Whether more messages exist beyond this batch.
  final bool hasMore;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MessageBatch &&
          listEquals(messages, other.messages) &&
          hasMore == other.hasMore;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(Object.hashAll(messages), hasMore);
}
