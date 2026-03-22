// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// UpdateProfile frame payload (client → server, RPC).
///
/// Uses updatable string semantics (u8 flag prefix):
/// `None` = don't change, `Some("")` = clear, `Some("value")` = set.
class UpdateProfilePayload {
  const UpdateProfilePayload({
    this.username,
    this.firstName,
    this.lastName,
    this.avatarUrl,
  });

  /// New username. `None` = don't change. `Some("")` = clear.
  final String? username;
  /// New first name.
  final String? firstName;
  /// New last name.
  final String? lastName;
  /// New avatar URL.
  final String? avatarUrl;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is UpdateProfilePayload &&
          username == other.username &&
          firstName == other.firstName &&
          lastName == other.lastName &&
          avatarUrl == other.avatarUrl;

  @override
  int get hashCode => Object.hash(
        username,
        firstName,
        lastName,
        avatarUrl,
      );

  @override
  String toString() => 'UpdateProfilePayload(username: $username, firstName: $firstName, lastName: $lastName, avatarUrl: $avatarUrl)';
}
