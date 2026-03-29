// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

/// Hello frame payload (client → server).
@immutable
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

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is HelloPayload &&
          protocolVersion == other.protocolVersion &&
          sdkVersion == other.sdkVersion &&
          platform == other.platform &&
          token == other.token &&
          deviceId == other.deviceId;
  // coverage:ignore-end

  @override
  int get hashCode =>
      Object.hash(protocolVersion, sdkVersion, platform, token, deviceId);
}
