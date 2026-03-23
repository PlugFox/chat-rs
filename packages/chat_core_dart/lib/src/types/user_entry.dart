// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:chat_core/src/types/user_flags.dart';

/// A user entry as transmitted on the wire (PresenceResult, user lookups).
///
/// Wire format: 22-byte fixed header + 4 length-prefixed optional strings.
class UserEntry {
  const UserEntry({
    required this.id,
    required this.flags,
    required this.createdAt,
    required this.updatedAt,
    this.username,
    this.firstName,
    this.lastName,
    this.avatarUrl,
  });

  /// Internal sequential user ID.
  final int id;

  /// User type and capability flags.
  final UserFlags flags;

  /// Account creation timestamp, Unix seconds.
  final int createdAt;

  /// Last profile modification timestamp, Unix seconds.
  final int updatedAt;

  /// Lowercase latin slug (5–32 chars). `None` when not set.
  final String? username;

  /// Display first name (1–64 chars). `None` when not set.
  final String? firstName;

  /// Display last name (1–64 chars). `None` when not set.
  final String? lastName;

  /// Avatar URL. `None` when not set.
  final String? avatarUrl;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is UserEntry &&
          id == other.id &&
          flags == other.flags &&
          createdAt == other.createdAt &&
          updatedAt == other.updatedAt &&
          username == other.username &&
          firstName == other.firstName &&
          lastName == other.lastName &&
          avatarUrl == other.avatarUrl;

  @override
  int get hashCode => Object.hash(
    id,
    flags,
    createdAt,
    updatedAt,
    username,
    firstName,
    lastName,
    avatarUrl,
  );

  @override
  String toString() =>
      'UserEntry(id: $id, flags: $flags, createdAt: $createdAt, updatedAt: $updatedAt, username: $username, firstName: $firstName, lastName: $lastName, avatarUrl: $avatarUrl)';
}
