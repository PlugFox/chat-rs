// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// Message content type.
enum MessageKind {
  /// Plain text message.
  text(0),
  /// Image message.
  image(1),
  /// File attachment.
  file(2),
  /// System event (join/leave/etc). Always paired with `MessageFlags::SYSTEM`.
  system(3);

  const MessageKind(this.value);
  final int value;

  static MessageKind? fromValue(int value) => switch (value) {
    0 => text,
    1 => image,
    2 => file,
    3 => system,
    _ => null,
  };
}
