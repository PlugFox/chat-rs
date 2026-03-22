// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/**
 * UpdateChat frame payload (client → server).
 *
 * **Clear semantics**: an empty string means "clear this field" (set to NULL on server).
 * `None` means "don't change". On the wire, `None` = `len 0` and empty string is not
 * distinguishable from `None`, so we use a `u8 flag` prefix:
 * `0` = don't change, `1` = set to following string (empty string = clear).
 */
export interface UpdateChatPayload {
  /** Target chat. */
  readonly chatId: number;
  /** New title. `None` = don't change. `Some("")` = clear. */
  readonly title: string | null;
  /** New avatar URL. `None` = don't change. `Some("")` = clear. */
  readonly avatarUrl: string | null;
}
