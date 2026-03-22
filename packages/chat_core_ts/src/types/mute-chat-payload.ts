// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** MuteChat frame payload (client → server, RPC). */
export interface MuteChatPayload {
  /** Target chat. */
  readonly chatId: number;
  /** Mute duration in seconds. `0` = mute forever. */
  readonly durationSecs: number;
}
