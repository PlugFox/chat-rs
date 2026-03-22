// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/**
 * Ack payload — command-specific response data.
 *
 * The variant is determined by the `FrameKind` of the original request.
 * Some variants carry raw bytes that must be decoded with the appropriate
 * codec function (e.g. `decode_message_batch` for `MessageBatch`).
 * This is intentional: the codec layer does not track which request
 * generated the Ack, so the caller provides the context.
 */
export type AckPayload =
  | { readonly type: "empty" }
  | { readonly type: "messageId"; readonly value: number }
  | { readonly type: "chatId"; readonly value: number }
  | { readonly type: "messageBatch"; readonly value: Uint8Array }
  | { readonly type: "chatList"; readonly value: Uint8Array }
  | { readonly type: "chatInfo"; readonly value: Uint8Array }
  | { readonly type: "memberList"; readonly value: Uint8Array }
  | { readonly type: "searchResults"; readonly value: Uint8Array }
  | { readonly type: "userInfo"; readonly value: Uint8Array }
  | { readonly type: "userList"; readonly value: Uint8Array }
  | { readonly type: "blockList"; readonly value: Uint8Array };
