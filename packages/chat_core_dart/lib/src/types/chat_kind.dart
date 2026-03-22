// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// Chat type.
enum ChatKind {
  /// Direct message (exactly two participants, no title).
  direct(0),
  /// Group conversation with multiple members.
  group(1),
  /// Read-mostly broadcast room nested inside a Group.
  channel(2);

  const ChatKind(this.value);
  final int value;

  static ChatKind? fromValue(int value) => switch (value) {
    0 => direct,
    1 => group,
    2 => channel,
    _ => null,
  };
}
