// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// Online/offline status for a user.
enum PresenceStatus {
  /// User is offline.
  offline(0),

  /// User is online (has at least one active WS connection).
  online(1);

  const PresenceStatus(this.value);
  final int value;

  static PresenceStatus? fromValue(int value) => switch (value) {
    0 => offline,
    1 => online,
    _ => null,
  };
}
