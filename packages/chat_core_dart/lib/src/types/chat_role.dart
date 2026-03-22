// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// Member role within a chat, ordered by privilege level.
enum ChatRole {
  /// Regular member.
  member(0),
  /// Can moderate (delete others' messages, mute).
  moderator(1),
  /// Can manage (invite, kick, change settings, assign roles).
  admin(2),
  /// Full control (transfer ownership, delete chat).
  owner(3);

  const ChatRole(this.value);
  final int value;

  static ChatRole? fromValue(int value) => switch (value) {
    0 => member,
    1 => moderator,
    2 => admin,
    3 => owner,
    _ => null,
  };
}
