// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

import 'package:chat_core/src/util/list_equals.dart';

/// Subscribe frame payload (client → server).
///
/// Subscribes to one or more named channels. Channel names follow conventions:
/// - `"general"` — account-level events (chat list updates, user profile changes)
/// - `"push"` — push notification events
/// - `"chat#123"` — real-time events for chat 123 (messages, typing, receipts)
/// - `"user#42"` — presence events for user 42
///
/// This channel-based model decouples subscription from specific chat IDs,
/// allowing flexible event routing and future extensibility.
@immutable
class SubscribePayload {
  const SubscribePayload({required this.channels});

  /// Channel names to subscribe to.
  final List<String> channels;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SubscribePayload && listEquals(channels, other.channels);
  // coverage:ignore-end

  @override
  int get hashCode => Object.hashAll(channels);
}
