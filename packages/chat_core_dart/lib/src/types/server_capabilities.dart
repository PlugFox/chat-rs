// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// Server-advertised feature capabilities (u32 on wire).
///
/// Sent in Welcome. Client uses these to show/hide features.
extension type const ServerCapabilities(int value) implements int {
  /// File and image upload enabled.
  static const ServerCapabilities mediaUpload = ServerCapabilities(0x01);
  /// Full-text message search enabled.
  static const ServerCapabilities search = ServerCapabilities(0x02);
  /// Emoji reactions enabled.
  static const ServerCapabilities reactions = ServerCapabilities(0x04);
  /// Message threads/replies enabled.
  static const ServerCapabilities threads = ServerCapabilities(0x08);
  /// Bot API enabled.
  static const ServerCapabilities bots = ServerCapabilities(0x10);

  static const List<ServerCapabilities> values = [mediaUpload, search, reactions, threads, bots];

  bool contains(ServerCapabilities flag) => (value & flag.value) != 0;
  ServerCapabilities add(ServerCapabilities flag) => ServerCapabilities(value | flag.value);
  ServerCapabilities remove(ServerCapabilities flag) => ServerCapabilities(value & ~flag.value);
  ServerCapabilities toggle(ServerCapabilities flag) => ServerCapabilities(value ^ flag.value);
  bool get isEmpty => value == 0;
  bool get isNotEmpty => value != 0;
  ServerCapabilities operator ^(ServerCapabilities other) => ServerCapabilities(value ^ other.value);
}
