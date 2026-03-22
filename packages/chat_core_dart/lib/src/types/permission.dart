// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// Per-member permission flags (u32 on wire, i32 in PostgreSQL).
///
/// `NULL` / absent in the database means "use role defaults".
/// See `default_permissions()` for the default set per role × chat kind.
extension type const Permission(int value) implements int {
  /// Can send text messages.
  static const Permission sendMessages = Permission(1 << 0);
  /// Can send media (images, files).
  static const Permission sendMedia = Permission(1 << 1);
  /// Can send link previews.
  static const Permission sendLinks = Permission(1 << 2);
  /// Can pin messages.
  static const Permission pinMessages = Permission(1 << 3);
  /// Can edit own messages.
  static const Permission editOwnMessages = Permission(1 << 4);
  /// Can delete own messages.
  static const Permission deleteOwnMessages = Permission(1 << 5);
  /// Can delete other members' messages.
  static const Permission deleteOthersMessages = Permission(1 << 10);
  /// Can mute members.
  static const Permission muteMembers = Permission(1 << 11);
  /// Can ban members.
  static const Permission banMembers = Permission(1 << 12);
  /// Can invite new members.
  static const Permission inviteMembers = Permission(1 << 20);
  /// Can kick members.
  static const Permission kickMembers = Permission(1 << 21);
  /// Can change chat title, avatar.
  static const Permission manageChatInfo = Permission(1 << 22);
  /// Can assign/change member roles.
  static const Permission manageRoles = Permission(1 << 23);
  /// Can transfer ownership to another member.
  static const Permission transferOwnership = Permission(1 << 30);
  /// Can delete the chat entirely.
  static const Permission deleteChat = Permission(1 << 31);

  static const List<Permission> values = [sendMessages, sendMedia, sendLinks, pinMessages, editOwnMessages, deleteOwnMessages, deleteOthersMessages, muteMembers, banMembers, inviteMembers, kickMembers, manageChatInfo, manageRoles, transferOwnership, deleteChat];

  bool contains(Permission flag) => (value & flag.value) != 0;
  Permission add(Permission flag) => Permission(value | flag.value);
  Permission remove(Permission flag) => Permission(value & ~flag.value);
  Permission toggle(Permission flag) => Permission(value ^ flag.value);
  bool get isEmpty => value == 0;
  bool get isNotEmpty => value != 0;
  Permission operator ^(Permission other) => Permission(value ^ other.value);
}
