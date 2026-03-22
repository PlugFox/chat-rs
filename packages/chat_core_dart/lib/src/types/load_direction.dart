// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// LoadMessages mode selector.
enum LoadDirection {
  /// Load older messages (before anchor).
  older(0),
  /// Load newer messages (after anchor).
  newer(1);

  const LoadDirection(this.value);
  final int value;

  static LoadDirection? fromValue(int value) => switch (value) {
    0 => older,
    1 => newer,
    _ => null,
  };
}
