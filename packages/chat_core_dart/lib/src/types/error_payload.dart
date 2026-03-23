// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:meta/meta.dart';

import 'package:chat_core/src/types/error_code.dart';

/// Error frame payload (server → client).
@immutable
class ErrorPayload {
  const ErrorPayload({
    required this.code,
    required this.message,
    required this.retryAfterMs,
    this.extra,
  });

  /// Numeric error code.
  final ErrorCode code;

  /// Developer-facing error description (not for end users).
  final String message;

  /// Retry delay in milliseconds (only set for `rate_limited`, 0 otherwise).
  final int retryAfterMs;

  /// Server-provided diagnostic JSON details. `None` = absent.
  final String? extra;

  // coverage:ignore-start
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ErrorPayload &&
          code == other.code &&
          message == other.message &&
          retryAfterMs == other.retryAfterMs &&
          extra == other.extra;
  // coverage:ignore-end

  @override
  int get hashCode => Object.hash(code, message, retryAfterMs, extra);
}
