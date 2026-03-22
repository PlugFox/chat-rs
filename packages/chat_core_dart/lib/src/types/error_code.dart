// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// Application-level error code sent in Error frames.
///
/// Slugs are stable identifiers that never change between protocol versions.
/// Client code should match on slugs, not numeric codes.
enum ErrorCode {
  /// Invalid token.
  unauthorized(1000),
  /// Token expired.
  tokenExpired(1001),
  /// No permission.
  forbidden(1002),
  /// Session revoked.
  sessionRevoked(1003),
  /// Protocol version not supported.
  unsupportedVersion(1004),
  /// Chat doesn't exist.
  chatNotFound(2000),
  /// Direct chat already exists between these users.
  chatAlreadyExists(2001),
  /// User is not a member of this chat.
  notChatMember(2002),
  /// Member limit reached for this chat.
  chatFull(2003),
  /// Message doesn't exist.
  messageNotFound(3000),
  /// Content exceeds max_message_size limit.
  messageTooLarge(3001),
  /// Extra JSON exceeds max_extra_size limit.
  extraTooLarge(3002),
  /// Too many messages — retry after `retry_after_ms`.
  rateLimited(3003),
  /// Content interceptor/filter rejected the message.
  contentFiltered(3004),
  /// File exceeds upload size limit.
  fileTooLarge(4000),
  /// File type not allowed.
  unsupportedMediaType(4001),
  /// Upload processing error.
  uploadFailed(4002),
  /// Server internal error.
  internalError(5000),
  /// Service temporarily unavailable.
  serviceUnavailable(5001),
  /// Database error.
  databaseError(5002),
  /// Bad frame format / cannot decode.
  malformedFrame(9000),
  /// Unknown frame kind byte.
  unknownCommand(9001),
  /// Frame exceeds max_frame_size.
  frameTooLarge(9002);

  const ErrorCode(this.value);
  final int value;

  static ErrorCode? fromValue(int value) => switch (value) {
    1000 => unauthorized,
    1001 => tokenExpired,
    1002 => forbidden,
    1003 => sessionRevoked,
    1004 => unsupportedVersion,
    2000 => chatNotFound,
    2001 => chatAlreadyExists,
    2002 => notChatMember,
    2003 => chatFull,
    3000 => messageNotFound,
    3001 => messageTooLarge,
    3002 => extraTooLarge,
    3003 => rateLimited,
    3004 => contentFiltered,
    4000 => fileTooLarge,
    4001 => unsupportedMediaType,
    4002 => uploadFailed,
    5000 => internalError,
    5001 => serviceUnavailable,
    5002 => databaseError,
    9000 => malformedFrame,
    9001 => unknownCommand,
    9002 => frameTooLarge,
    _ => null,
  };

  /// Stable snake_case identifier for client matching.
  String get slug => switch (this) {
    unauthorized => 'unauthorized',
    tokenExpired => 'token_expired',
    forbidden => 'forbidden',
    sessionRevoked => 'session_revoked',
    unsupportedVersion => 'unsupported_version',
    chatNotFound => 'chat_not_found',
    chatAlreadyExists => 'chat_already_exists',
    notChatMember => 'not_chat_member',
    chatFull => 'chat_full',
    messageNotFound => 'message_not_found',
    messageTooLarge => 'message_too_large',
    extraTooLarge => 'extra_too_large',
    rateLimited => 'rate_limited',
    contentFiltered => 'content_filtered',
    fileTooLarge => 'file_too_large',
    unsupportedMediaType => 'unsupported_media_type',
    uploadFailed => 'upload_failed',
    internalError => 'internal_error',
    serviceUnavailable => 'service_unavailable',
    databaseError => 'database_error',
    malformedFrame => 'malformed_frame',
    unknownCommand => 'unknown_command',
    frameTooLarge => 'frame_too_large',
  };

  /// Whether this error is permanent (do not retry).
  bool get isPermanent => switch (this) {
    forbidden || chatNotFound || notChatMember || messageTooLarge || extraTooLarge || contentFiltered || unsupportedMediaType => true,
    _ => false,
  };

  /// Whether this error is transient (retry with backoff).
  bool get isTransient => switch (this) {
    internalError || serviceUnavailable || databaseError || rateLimited => true,
    _ => false,
  };
}
