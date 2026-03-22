// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/**
 * UpdateProfile frame payload (client → server, RPC).
 *
 * Uses updatable string semantics (u8 flag prefix):
 * `None` = don't change, `Some("")` = clear, `Some("value")` = set.
 */
export interface UpdateProfilePayload {
  /** New username. `None` = don't change. `Some("")` = clear. */
  readonly username: string | null;
  /** New first name. */
  readonly firstName: string | null;
  /** New last name. */
  readonly lastName: string | null;
  /** New avatar URL. */
  readonly avatarUrl: string | null;
}
