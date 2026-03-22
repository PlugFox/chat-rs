// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/**
 * Frame type identifier — first byte of every WS binary frame.
 *
 * Values are stable and must never be renumbered.
 */
export const enum FrameKind {
  /** Client → server: protocol version, token, device_id. */
  Hello = 1,
  /** Server → client: session_id, server_time, limits. */
  Welcome = 2,
  /** Keepalive ping (both directions). */
  Ping = 3,
  /** Keepalive pong (both directions). */
  Pong = 4,
  /** Refresh JWT token without disconnecting (client → server). */
  RefreshToken = 5,
  /** Send a new message (persist, needs Ack). */
  SendMessage = 16,
  /** Edit an existing message (persist, needs Ack). */
  EditMessage = 17,
  /** Soft-delete a message (persist, needs Ack). */
  DeleteMessage = 18,
  /** Mark messages as read (persist, fire-and-forget). */
  ReadReceipt = 19,
  /** Typing indicator (ephemeral, fire-and-forget). */
  Typing = 20,
  /** Request online/offline status (RPC). */
  GetPresence = 21,
  /** Load chat list (RPC). */
  LoadChats = 22,
  /** Full-text message search (RPC). */
  Search = 23,
  /** Subscribe to real-time events for a chat (RPC). */
  Subscribe = 24,
  /** Unsubscribe from a chat (fire-and-forget). */
  Unsubscribe = 25,
  /** Load message history (RPC). */
  LoadMessages = 26,
  /** Add a reaction to a message (needs Ack). */
  AddReaction = 27,
  /** Remove a reaction from a message (needs Ack). */
  RemoveReaction = 28,
  /** Pin a message in a chat (needs Ack). */
  PinMessage = 29,
  /** Unpin a message in a chat (needs Ack). */
  UnpinMessage = 30,
  /** Forward a message to another chat (persist, needs Ack). */
  ForwardMessage = 31,
  /** New message delivered in real-time. Payload: single `Message`. */
  MessageNew = 32,
  /** Message content changed. Payload: single `Message` with updated fields. */
  MessageEdited = 33,
  /** Message marked deleted. Payload: `chat_id: u32, message_id: u32`. */
  MessageDeleted = 34,
  /** Read receipt update. */
  ReceiptUpdate = 35,
  /** Typing indicator broadcast. */
  TypingUpdate = 36,
  /** Member joined chat. */
  MemberJoined = 37,
  /** Member left chat. */
  MemberLeft = 38,
  /** Response to GetPresence. */
  PresenceResult = 39,
  /** Chat metadata changed (title, avatar). Payload: full `ChatEntry`. */
  ChatUpdated = 40,
  /** New chat the user is a member of. Payload: full `ChatEntry`. */
  ChatCreated = 41,
  /** Reaction added or removed on a message. */
  ReactionUpdate = 42,
  /** User profile changed (server → client push). */
  UserUpdated = 43,
  /** Chat was deleted (server → client push). */
  ChatDeleted = 44,
  /** Chat member's role or permissions changed (server → client push). */
  MemberUpdated = 45,
  /** Command acknowledged. */
  Ack = 48,
  /** Error response. */
  Error = 49,
  /** Create a new chat. */
  CreateChat = 64,
  /** Update chat info (title, avatar). */
  UpdateChat = 65,
  /** Delete a chat. */
  DeleteChat = 66,
  /** Get chat details. */
  GetChatInfo = 67,
  /** List chat members. */
  GetChatMembers = 68,
  /** Invite users to a chat. */
  InviteMembers = 69,
  /** Kick, ban, mute, change role, or update permissions for a member. */
  UpdateMember = 70,
  /** Leave a chat. */
  LeaveChat = 71,
  /** Mute chat notifications (client → server, RPC). */
  MuteChat = 72,
  /** Unmute chat notifications (client → server, RPC). */
  UnmuteChat = 73,
  /** Get a single user's profile. */
  GetUser = 80,
  /** Get multiple users' profiles. */
  GetUsers = 81,
  /** Update own profile. */
  UpdateProfile = 82,
  /** Block a user. */
  BlockUser = 83,
  /** Unblock a user. */
  UnblockUser = 84,
  /** Get block list. */
  GetBlockList = 85,
}

/** Convert wire value to FrameKind, or undefined if unknown. */
export function frameKindFromValue(value: number): FrameKind | undefined {
  switch (value) {
    case 1:
      return FrameKind.Hello;
    case 2:
      return FrameKind.Welcome;
    case 3:
      return FrameKind.Ping;
    case 4:
      return FrameKind.Pong;
    case 5:
      return FrameKind.RefreshToken;
    case 16:
      return FrameKind.SendMessage;
    case 17:
      return FrameKind.EditMessage;
    case 18:
      return FrameKind.DeleteMessage;
    case 19:
      return FrameKind.ReadReceipt;
    case 20:
      return FrameKind.Typing;
    case 21:
      return FrameKind.GetPresence;
    case 22:
      return FrameKind.LoadChats;
    case 23:
      return FrameKind.Search;
    case 24:
      return FrameKind.Subscribe;
    case 25:
      return FrameKind.Unsubscribe;
    case 26:
      return FrameKind.LoadMessages;
    case 27:
      return FrameKind.AddReaction;
    case 28:
      return FrameKind.RemoveReaction;
    case 29:
      return FrameKind.PinMessage;
    case 30:
      return FrameKind.UnpinMessage;
    case 31:
      return FrameKind.ForwardMessage;
    case 32:
      return FrameKind.MessageNew;
    case 33:
      return FrameKind.MessageEdited;
    case 34:
      return FrameKind.MessageDeleted;
    case 35:
      return FrameKind.ReceiptUpdate;
    case 36:
      return FrameKind.TypingUpdate;
    case 37:
      return FrameKind.MemberJoined;
    case 38:
      return FrameKind.MemberLeft;
    case 39:
      return FrameKind.PresenceResult;
    case 40:
      return FrameKind.ChatUpdated;
    case 41:
      return FrameKind.ChatCreated;
    case 42:
      return FrameKind.ReactionUpdate;
    case 43:
      return FrameKind.UserUpdated;
    case 44:
      return FrameKind.ChatDeleted;
    case 45:
      return FrameKind.MemberUpdated;
    case 48:
      return FrameKind.Ack;
    case 49:
      return FrameKind.Error;
    case 64:
      return FrameKind.CreateChat;
    case 65:
      return FrameKind.UpdateChat;
    case 66:
      return FrameKind.DeleteChat;
    case 67:
      return FrameKind.GetChatInfo;
    case 68:
      return FrameKind.GetChatMembers;
    case 69:
      return FrameKind.InviteMembers;
    case 70:
      return FrameKind.UpdateMember;
    case 71:
      return FrameKind.LeaveChat;
    case 72:
      return FrameKind.MuteChat;
    case 73:
      return FrameKind.UnmuteChat;
    case 80:
      return FrameKind.GetUser;
    case 81:
      return FrameKind.GetUsers;
    case 82:
      return FrameKind.UpdateProfile;
    case 83:
      return FrameKind.BlockUser;
    case 84:
      return FrameKind.UnblockUser;
    case 85:
      return FrameKind.GetBlockList;
    default:
      return undefined;
  }
}
