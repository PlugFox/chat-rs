// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// User type and capability flags (u16 on wire, i16 in PostgreSQL).
extension type const UserFlags(int value) implements int {
  /// System account (server-generated messages).
  static const UserFlags system = UserFlags(0x0001);
  /// Bot account; server sets `MessageFlags::BOT` on all messages.
  static const UserFlags bot = UserFlags(0x0002);
  /// Premium subscriber; clients may show a badge.
  static const UserFlags premium = UserFlags(0x0004);

  static const List<UserFlags> values = [system, bot, premium];

  bool contains(UserFlags flag) => (value & flag.value) != 0;
  UserFlags add(UserFlags flag) => UserFlags(value | flag.value);
  UserFlags remove(UserFlags flag) => UserFlags(value & ~flag.value);
  UserFlags toggle(UserFlags flag) => UserFlags(value ^ flag.value);
  bool get isEmpty => value == 0;
  bool get isNotEmpty => value != 0;
  UserFlags operator ^(UserFlags other) => UserFlags(value ^ other.value);
}
