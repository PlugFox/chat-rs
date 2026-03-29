// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// Message property flags (u16 on wire, i16 in PostgreSQL).
extension type const MessageFlags(int value) implements int {
  /// Edited at least once; display "edited" label.
  static const MessageFlags edited = MessageFlags(0x0001);

  /// Soft-deleted tombstone; content is empty.
  static const MessageFlags deleted = MessageFlags(0x0002);

  /// Forwarded from another chat; origin in extra JSON.
  static const MessageFlags forwarded = MessageFlags(0x0004);

  /// Pinned in this chat.
  static const MessageFlags pinned = MessageFlags(0x0008);

  /// No push notification for this message.
  static const MessageFlags silent = MessageFlags(0x0010);

  /// System event message (member join/leave, etc.).
  static const MessageFlags system = MessageFlags(0x0020);

  /// Sent by a bot user (server-authoritative).
  static const MessageFlags bot = MessageFlags(0x0040);

  /// Reply to another message; origin in extra JSON.
  static const MessageFlags reply = MessageFlags(0x0080);

  static const List<MessageFlags> values = [
    edited,
    deleted,
    forwarded,
    pinned,
    silent,
    system,
    bot,
    reply,
  ];

  bool contains(MessageFlags flag) => (value & flag.value) != 0;
  MessageFlags add(MessageFlags flag) => MessageFlags(value | flag.value);
  MessageFlags remove(MessageFlags flag) => MessageFlags(value & ~flag.value);
  MessageFlags toggle(MessageFlags flag) => MessageFlags(value ^ flag.value);
  bool get isEmpty => value == 0;
  bool get isNotEmpty => value != 0;
  MessageFlags operator ^(MessageFlags other) =>
      MessageFlags(value ^ other.value);
}
