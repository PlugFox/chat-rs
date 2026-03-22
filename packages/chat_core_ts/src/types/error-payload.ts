// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import type { ErrorCode } from './error-code.js';

/** Error frame payload (server → client). */
export interface ErrorPayload {
  /** Numeric error code. */
  readonly code: ErrorCode;
  /** Developer-facing error description (not for end users). */
  readonly message: string;
  /** Retry delay in milliseconds (only set for `rate_limited`, 0 otherwise). */
  readonly retryAfterMs: number;
  /** Server-provided diagnostic JSON details. `None` = absent. */
  readonly extra: string | null;
}
