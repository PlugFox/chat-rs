// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/**
 * Application-level error code sent in Error frames.
 *
 * Slugs are stable identifiers that never change between protocol versions.
 * Client code should match on slugs, not numeric codes.
 */
export const enum ErrorCode {
  /** Invalid token. */
  Unauthorized = 1000,
  /** Token expired. */
  TokenExpired = 1001,
  /** No permission. */
  Forbidden = 1002,
  /** Session revoked. */
  SessionRevoked = 1003,
  /** Protocol version not supported. */
  UnsupportedVersion = 1004,
  /** Chat doesn't exist. */
  ChatNotFound = 2000,
  /** Direct chat already exists between these users. */
  ChatAlreadyExists = 2001,
  /** User is not a member of this chat. */
  NotChatMember = 2002,
  /** Member limit reached for this chat. */
  ChatFull = 2003,
  /** Message doesn't exist. */
  MessageNotFound = 3000,
  /** Content exceeds max_message_size limit. */
  MessageTooLarge = 3001,
  /** Extra JSON exceeds max_extra_size limit. */
  ExtraTooLarge = 3002,
  /** Too many messages — retry after `retry_after_ms`. */
  RateLimited = 3003,
  /** Content interceptor/filter rejected the message. */
  ContentFiltered = 3004,
  /** File exceeds upload size limit. */
  FileTooLarge = 4000,
  /** File type not allowed. */
  UnsupportedMediaType = 4001,
  /** Upload processing error. */
  UploadFailed = 4002,
  /** Server internal error. */
  InternalError = 5000,
  /** Service temporarily unavailable. */
  ServiceUnavailable = 5001,
  /** Database error. */
  DatabaseError = 5002,
  /** Bad frame format / cannot decode. */
  MalformedFrame = 9000,
  /** Unknown frame kind byte. */
  UnknownCommand = 9001,
  /** Frame exceeds max_frame_size. */
  FrameTooLarge = 9002,
}

/** Convert wire value to ErrorCode, or undefined if unknown. */
export function errorCodeFromValue(value: number): ErrorCode | undefined {
  switch (value) {
    case 1000:
      return ErrorCode.Unauthorized;
    case 1001:
      return ErrorCode.TokenExpired;
    case 1002:
      return ErrorCode.Forbidden;
    case 1003:
      return ErrorCode.SessionRevoked;
    case 1004:
      return ErrorCode.UnsupportedVersion;
    case 2000:
      return ErrorCode.ChatNotFound;
    case 2001:
      return ErrorCode.ChatAlreadyExists;
    case 2002:
      return ErrorCode.NotChatMember;
    case 2003:
      return ErrorCode.ChatFull;
    case 3000:
      return ErrorCode.MessageNotFound;
    case 3001:
      return ErrorCode.MessageTooLarge;
    case 3002:
      return ErrorCode.ExtraTooLarge;
    case 3003:
      return ErrorCode.RateLimited;
    case 3004:
      return ErrorCode.ContentFiltered;
    case 4000:
      return ErrorCode.FileTooLarge;
    case 4001:
      return ErrorCode.UnsupportedMediaType;
    case 4002:
      return ErrorCode.UploadFailed;
    case 5000:
      return ErrorCode.InternalError;
    case 5001:
      return ErrorCode.ServiceUnavailable;
    case 5002:
      return ErrorCode.DatabaseError;
    case 9000:
      return ErrorCode.MalformedFrame;
    case 9001:
      return ErrorCode.UnknownCommand;
    case 9002:
      return ErrorCode.FrameTooLarge;
    default:
      return undefined;
  }
}

/** Stable snake_case identifier for client matching. */
export function errorCodeSlug(code: ErrorCode): string {
  switch (code) {
    case ErrorCode.Unauthorized:
      return "unauthorized";
    case ErrorCode.TokenExpired:
      return "token_expired";
    case ErrorCode.Forbidden:
      return "forbidden";
    case ErrorCode.SessionRevoked:
      return "session_revoked";
    case ErrorCode.UnsupportedVersion:
      return "unsupported_version";
    case ErrorCode.ChatNotFound:
      return "chat_not_found";
    case ErrorCode.ChatAlreadyExists:
      return "chat_already_exists";
    case ErrorCode.NotChatMember:
      return "not_chat_member";
    case ErrorCode.ChatFull:
      return "chat_full";
    case ErrorCode.MessageNotFound:
      return "message_not_found";
    case ErrorCode.MessageTooLarge:
      return "message_too_large";
    case ErrorCode.ExtraTooLarge:
      return "extra_too_large";
    case ErrorCode.RateLimited:
      return "rate_limited";
    case ErrorCode.ContentFiltered:
      return "content_filtered";
    case ErrorCode.FileTooLarge:
      return "file_too_large";
    case ErrorCode.UnsupportedMediaType:
      return "unsupported_media_type";
    case ErrorCode.UploadFailed:
      return "upload_failed";
    case ErrorCode.InternalError:
      return "internal_error";
    case ErrorCode.ServiceUnavailable:
      return "service_unavailable";
    case ErrorCode.DatabaseError:
      return "database_error";
    case ErrorCode.MalformedFrame:
      return "malformed_frame";
    case ErrorCode.UnknownCommand:
      return "unknown_command";
    case ErrorCode.FrameTooLarge:
      return "frame_too_large";
  }
}

/** Whether this error is permanent (do not retry). */
export function isErrorPermanent(code: ErrorCode): boolean {
  switch (code) {
    case ErrorCode.Forbidden:
    case ErrorCode.ChatNotFound:
    case ErrorCode.NotChatMember:
    case ErrorCode.MessageTooLarge:
    case ErrorCode.ExtraTooLarge:
    case ErrorCode.ContentFiltered:
    case ErrorCode.UnsupportedMediaType:
      return true;
    default:
      return false;
  }
}

/** Whether this error is transient (retry with backoff). */
export function isErrorTransient(code: ErrorCode): boolean {
  switch (code) {
    case ErrorCode.InternalError:
    case ErrorCode.ServiceUnavailable:
    case ErrorCode.DatabaseError:
    case ErrorCode.RateLimited:
      return true;
    default:
      return false;
  }
}
