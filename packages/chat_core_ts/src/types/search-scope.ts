// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/** Search scope selector. */
export type SearchScope =
  | { readonly type: 'chat'; readonly chatId: number }
  | { readonly type: 'global' }
  | { readonly type: 'user'; readonly userId: number };
