// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'message_flags.dart';
import 'message_kind.dart';

/// Lightweight last-message preview included in `ChatEntry`.
///
/// Wire format: 21-byte fixed header + content preview string.
class LastMessagePreview {
  const LastMessagePreview({
    required this.id,
    required this.senderId,
    required this.createdAt,
    required this.kind,
    required this.flags,
    required this.contentPreview,
  });

  /// Message ID.
  final int id;

  /// Sender's internal user ID.
  final int senderId;

  /// Creation timestamp, Unix seconds.
  final int createdAt;

  /// Content type.
  final MessageKind kind;

  /// Message property flags.
  final MessageFlags flags;

  /// Truncated plain-text preview (server-side, up to 100 bytes UTF-8).
  final String contentPreview;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LastMessagePreview &&
          id == other.id &&
          senderId == other.senderId &&
          createdAt == other.createdAt &&
          kind == other.kind &&
          flags == other.flags &&
          contentPreview == other.contentPreview;

  @override
  int get hashCode =>
      Object.hash(id, senderId, createdAt, kind, flags, contentPreview);

  @override
  String toString() =>
      'LastMessagePreview(id: $id, senderId: $senderId, createdAt: $createdAt, kind: $kind, flags: $flags, contentPreview: $contentPreview)';
}
