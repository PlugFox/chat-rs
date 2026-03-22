// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import '../_util.dart';

/// Unsubscribe frame payload (client → server, fire-and-forget).
///
/// Unsubscribes from one or more named channels.
class UnsubscribePayload {
  const UnsubscribePayload({required this.channels});

  /// Channel names to unsubscribe from.
  final List<String> channels;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is UnsubscribePayload && listEquals(channels, other.channels);

  @override
  int get hashCode => Object.hashAll(channels);

  @override
  String toString() => 'UnsubscribePayload(channels: $channels)';
}
