// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// Hello frame payload (client → server).
class HelloPayload {
  const HelloPayload({
    required this.protocolVersion,
    required this.sdkVersion,
    required this.platform,
    required this.token,
    required this.deviceId,
  });

  /// Protocol version the client supports.
  final int protocolVersion;
  /// Client SDK version string (e.g. "1.0.0").
  final String sdkVersion;
  /// Client platform string (e.g. "dart", "typescript", "rust").
  final String platform;
  /// JWT authentication token.
  final String token;
  /// Unique device identifier (UUID v4, 16 bytes on wire).
  final String deviceId;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is HelloPayload &&
          protocolVersion == other.protocolVersion &&
          sdkVersion == other.sdkVersion &&
          platform == other.platform &&
          token == other.token &&
          deviceId == other.deviceId;

  @override
  int get hashCode => Object.hash(
        protocolVersion,
        sdkVersion,
        platform,
        token,
        deviceId,
      );

  @override
  String toString() => 'HelloPayload(protocolVersion: $protocolVersion, sdkVersion: $sdkVersion, platform: $platform, token: $token, deviceId: $deviceId)';
}
