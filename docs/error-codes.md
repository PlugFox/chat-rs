# Error Codes

## Structure

Each error has:
- `code: u16` — numeric code with category ranges
- `slug: &str` — stable snake_case identifier (never changes between versions)
- `message: String` — developer description (not for end users)

## Code Ranges

| Range     | Category                       |
| --------- | ------------------------------ |
| 1000–1999 | Authentication & authorization |
| 2000–2999 | Chats                          |
| 3000–3999 | Messages                       |
| 4000–4999 | Media                          |
| 5000–5999 | Server internal                |
| 9000–9999 | Protocol                       |

## Codes

| Code | Slug                     | Permanent?     | Description                                      |
| ---- | ------------------------ | -------------- | ------------------------------------------------ |
| 1000 | `unauthorized`           | —              | Invalid token                                    |
| 1001 | `token_expired`          | —              | Token expired                                    |
| 1002 | `forbidden`              | yes            | No permission                                    |
| 1003 | `session_revoked`        | —              | Session revoked                                  |
| 1004 | `unsupported_version`    | —              | Protocol version not supported                   |
| 2000 | `chat_not_found`         | yes            | Chat doesn't exist                               |
| 2001 | `chat_already_exists`    | —              | Direct chat already exists                       |
| 2002 | `not_chat_member`        | yes            | User is not a member                             |
| 2003 | `chat_full`              | —              | Member limit reached                             |
| 3000 | `message_not_found`      | —              | Message doesn't exist                            |
| 3001 | `message_too_large`      | yes            | Content exceeds limit                            |
| 3002 | `extra_too_large`        | yes            | Extra JSON exceeds limit                         |
| 3003 | `rate_limited`           | no (transient) | Too many messages — retry after `retry_after_ms` |
| 3004 | `content_filtered`       | yes            | Interceptor rejected                             |
| 4000 | `file_too_large`         | —              | File exceeds upload limit                        |
| 4001 | `unsupported_media_type` | yes            | File type not allowed                            |
| 4002 | `upload_failed`          | —              | Upload error                                     |
| 5000 | `internal_error`         | transient      | Server internal error                            |
| 5001 | `service_unavailable`    | transient      | Service down                                     |
| 5002 | `database_error`         | transient      | DB error                                         |
| 9000 | `malformed_frame`        | —              | Bad frame format                                 |
| 9001 | `unknown_command`        | —              | Unknown frame kind                               |
| 9002 | `frame_too_large`        | —              | Frame exceeds max size                           |

## Outbox Retry Classification

- **Permanent** — do not retry: `forbidden`, `chat_not_found`, `not_chat_member`, `message_too_large`, `extra_too_large`, `content_filtered`, `unsupported_media_type`
- **Transient** — retry with exponential backoff: `internal_error`, `service_unavailable`, `database_error`, `rate_limited`
- **Other** — handled case-by-case

## Disconnect Codes

See `DisconnectCode` in `chat_protocol::types`.

### Ranges

| Range     | Category            | Reconnect |
| --------- | ------------------- | --------- |
| 0–999     | Internal/transport  | Yes       |
| 3000–3499 | Server non-terminal | Yes       |
| 3500–3999 | Server terminal     | No        |
| 4000–4499 | Custom non-terminal | Yes       |
| 4500–4999 | Custom terminal     | No        |

### Codes

| Code | Name                 | Reconnect | When                                           |
| ---- | -------------------- | --------- | ---------------------------------------------- |
| 3000 | `ServerShutdown`     | Yes       | Graceful server restart                        |
| 3001 | `SessionExpired`     | Yes       | Token expired mid-session                      |
| 3002 | `DuplicateSession`   | Yes       | Same device_id connected from another location |
| 3003 | `ServerError`        | Yes       | Unrecoverable internal server error            |
| 3004 | `BufferOverflow`     | Yes       | Client send buffer exceeded capacity           |
| 3500 | `TokenInvalid`       | No        | Token is malformed or has invalid signature    |
| 3501 | `Banned`             | No        | User is banned                                 |
| 3502 | `UnsupportedVersion` | No        | Protocol version not supported by server       |
