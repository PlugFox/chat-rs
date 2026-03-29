// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import { describe, it, expect } from "vitest";
import {
  ChatKind,
  ChatRole,
  DisconnectCode,
  ErrorCode,
  FrameKind,
  LoadDirection,
  MessageFlags,
  MessageKind,
  Permission,
  PresenceStatus,
  RichStyle,
  ServerCapabilities,
  UserFlags,
  chatKindFromValue,
  chatRoleFromValue,
  disconnectCodeFromValue,
  errorCodeFromValue,
  frameKindFromValue,
  loadDirectionFromValue,
  messageKindFromValue,
  presenceStatusFromValue,
} from "../src/index.js";

describe("ChatKind", () => {
  it("fromValue roundtrip", () => {
    expect(chatKindFromValue(ChatKind.Direct)).toBe(ChatKind.Direct);
    expect(chatKindFromValue(ChatKind.Group)).toBe(ChatKind.Group);
    expect(chatKindFromValue(ChatKind.Channel)).toBe(ChatKind.Channel);
  });
  it("fromValue returns undefined for invalid", () => {
    expect(chatKindFromValue(65535)).toBeUndefined();
  });
});

describe("ChatRole", () => {
  it("fromValue roundtrip", () => {
    expect(chatRoleFromValue(ChatRole.Member)).toBe(ChatRole.Member);
    expect(chatRoleFromValue(ChatRole.Moderator)).toBe(ChatRole.Moderator);
    expect(chatRoleFromValue(ChatRole.Admin)).toBe(ChatRole.Admin);
    expect(chatRoleFromValue(ChatRole.Owner)).toBe(ChatRole.Owner);
  });
  it("fromValue returns undefined for invalid", () => {
    expect(chatRoleFromValue(65535)).toBeUndefined();
  });
});

describe("MessageKind", () => {
  it("fromValue roundtrip", () => {
    expect(messageKindFromValue(MessageKind.Text)).toBe(MessageKind.Text);
    expect(messageKindFromValue(MessageKind.Image)).toBe(MessageKind.Image);
    expect(messageKindFromValue(MessageKind.File)).toBe(MessageKind.File);
    expect(messageKindFromValue(MessageKind.System)).toBe(MessageKind.System);
  });
  it("fromValue returns undefined for invalid", () => {
    expect(messageKindFromValue(65535)).toBeUndefined();
  });
});

describe("PresenceStatus", () => {
  it("fromValue roundtrip", () => {
    expect(presenceStatusFromValue(PresenceStatus.Offline)).toBe(
      PresenceStatus.Offline,
    );
    expect(presenceStatusFromValue(PresenceStatus.Online)).toBe(
      PresenceStatus.Online,
    );
  });
  it("fromValue returns undefined for invalid", () => {
    expect(presenceStatusFromValue(65535)).toBeUndefined();
  });
});

describe("ErrorCode", () => {
  it("fromValue roundtrip", () => {
    expect(errorCodeFromValue(ErrorCode.Unauthorized)).toBe(
      ErrorCode.Unauthorized,
    );
    expect(errorCodeFromValue(ErrorCode.TokenExpired)).toBe(
      ErrorCode.TokenExpired,
    );
    expect(errorCodeFromValue(ErrorCode.Forbidden)).toBe(ErrorCode.Forbidden);
    expect(errorCodeFromValue(ErrorCode.SessionRevoked)).toBe(
      ErrorCode.SessionRevoked,
    );
    expect(errorCodeFromValue(ErrorCode.UnsupportedVersion)).toBe(
      ErrorCode.UnsupportedVersion,
    );
    expect(errorCodeFromValue(ErrorCode.ChatNotFound)).toBe(
      ErrorCode.ChatNotFound,
    );
    expect(errorCodeFromValue(ErrorCode.ChatAlreadyExists)).toBe(
      ErrorCode.ChatAlreadyExists,
    );
    expect(errorCodeFromValue(ErrorCode.NotChatMember)).toBe(
      ErrorCode.NotChatMember,
    );
    expect(errorCodeFromValue(ErrorCode.ChatFull)).toBe(ErrorCode.ChatFull);
    expect(errorCodeFromValue(ErrorCode.MessageNotFound)).toBe(
      ErrorCode.MessageNotFound,
    );
    expect(errorCodeFromValue(ErrorCode.MessageTooLarge)).toBe(
      ErrorCode.MessageTooLarge,
    );
    expect(errorCodeFromValue(ErrorCode.ExtraTooLarge)).toBe(
      ErrorCode.ExtraTooLarge,
    );
    expect(errorCodeFromValue(ErrorCode.RateLimited)).toBe(
      ErrorCode.RateLimited,
    );
    expect(errorCodeFromValue(ErrorCode.ContentFiltered)).toBe(
      ErrorCode.ContentFiltered,
    );
    expect(errorCodeFromValue(ErrorCode.FileTooLarge)).toBe(
      ErrorCode.FileTooLarge,
    );
    expect(errorCodeFromValue(ErrorCode.UnsupportedMediaType)).toBe(
      ErrorCode.UnsupportedMediaType,
    );
    expect(errorCodeFromValue(ErrorCode.UploadFailed)).toBe(
      ErrorCode.UploadFailed,
    );
    expect(errorCodeFromValue(ErrorCode.InternalError)).toBe(
      ErrorCode.InternalError,
    );
    expect(errorCodeFromValue(ErrorCode.ServiceUnavailable)).toBe(
      ErrorCode.ServiceUnavailable,
    );
    expect(errorCodeFromValue(ErrorCode.DatabaseError)).toBe(
      ErrorCode.DatabaseError,
    );
    expect(errorCodeFromValue(ErrorCode.MalformedFrame)).toBe(
      ErrorCode.MalformedFrame,
    );
    expect(errorCodeFromValue(ErrorCode.UnknownCommand)).toBe(
      ErrorCode.UnknownCommand,
    );
    expect(errorCodeFromValue(ErrorCode.FrameTooLarge)).toBe(
      ErrorCode.FrameTooLarge,
    );
  });
  it("fromValue returns undefined for invalid", () => {
    expect(errorCodeFromValue(65535)).toBeUndefined();
  });
});

describe("DisconnectCode", () => {
  it("fromValue roundtrip", () => {
    expect(disconnectCodeFromValue(DisconnectCode.ServerShutdown)).toBe(
      DisconnectCode.ServerShutdown,
    );
    expect(disconnectCodeFromValue(DisconnectCode.SessionExpired)).toBe(
      DisconnectCode.SessionExpired,
    );
    expect(disconnectCodeFromValue(DisconnectCode.DuplicateSession)).toBe(
      DisconnectCode.DuplicateSession,
    );
    expect(disconnectCodeFromValue(DisconnectCode.ServerError)).toBe(
      DisconnectCode.ServerError,
    );
    expect(disconnectCodeFromValue(DisconnectCode.BufferOverflow)).toBe(
      DisconnectCode.BufferOverflow,
    );
    expect(disconnectCodeFromValue(DisconnectCode.RateLimited)).toBe(
      DisconnectCode.RateLimited,
    );
    expect(disconnectCodeFromValue(DisconnectCode.EventSeqOverflow)).toBe(
      DisconnectCode.EventSeqOverflow,
    );
    expect(disconnectCodeFromValue(DisconnectCode.TokenInvalid)).toBe(
      DisconnectCode.TokenInvalid,
    );
    expect(disconnectCodeFromValue(DisconnectCode.Banned)).toBe(
      DisconnectCode.Banned,
    );
    expect(disconnectCodeFromValue(DisconnectCode.UnsupportedVersion)).toBe(
      DisconnectCode.UnsupportedVersion,
    );
    expect(disconnectCodeFromValue(DisconnectCode.ConnectionLimit)).toBe(
      DisconnectCode.ConnectionLimit,
    );
  });
  it("fromValue returns undefined for invalid", () => {
    expect(disconnectCodeFromValue(65535)).toBeUndefined();
  });
});

describe("FrameKind", () => {
  it("fromValue roundtrip", () => {
    expect(frameKindFromValue(FrameKind.Hello)).toBe(FrameKind.Hello);
    expect(frameKindFromValue(FrameKind.Welcome)).toBe(FrameKind.Welcome);
    expect(frameKindFromValue(FrameKind.Ping)).toBe(FrameKind.Ping);
    expect(frameKindFromValue(FrameKind.Pong)).toBe(FrameKind.Pong);
    expect(frameKindFromValue(FrameKind.RefreshToken)).toBe(
      FrameKind.RefreshToken,
    );
    expect(frameKindFromValue(FrameKind.SendMessage)).toBe(
      FrameKind.SendMessage,
    );
    expect(frameKindFromValue(FrameKind.EditMessage)).toBe(
      FrameKind.EditMessage,
    );
    expect(frameKindFromValue(FrameKind.DeleteMessage)).toBe(
      FrameKind.DeleteMessage,
    );
    expect(frameKindFromValue(FrameKind.ReadReceipt)).toBe(
      FrameKind.ReadReceipt,
    );
    expect(frameKindFromValue(FrameKind.Typing)).toBe(FrameKind.Typing);
    expect(frameKindFromValue(FrameKind.GetPresence)).toBe(
      FrameKind.GetPresence,
    );
    expect(frameKindFromValue(FrameKind.LoadChats)).toBe(FrameKind.LoadChats);
    expect(frameKindFromValue(FrameKind.Search)).toBe(FrameKind.Search);
    expect(frameKindFromValue(FrameKind.Subscribe)).toBe(FrameKind.Subscribe);
    expect(frameKindFromValue(FrameKind.Unsubscribe)).toBe(
      FrameKind.Unsubscribe,
    );
    expect(frameKindFromValue(FrameKind.LoadMessages)).toBe(
      FrameKind.LoadMessages,
    );
    expect(frameKindFromValue(FrameKind.AddReaction)).toBe(
      FrameKind.AddReaction,
    );
    expect(frameKindFromValue(FrameKind.RemoveReaction)).toBe(
      FrameKind.RemoveReaction,
    );
    expect(frameKindFromValue(FrameKind.PinMessage)).toBe(FrameKind.PinMessage);
    expect(frameKindFromValue(FrameKind.UnpinMessage)).toBe(
      FrameKind.UnpinMessage,
    );
    expect(frameKindFromValue(FrameKind.ForwardMessage)).toBe(
      FrameKind.ForwardMessage,
    );
    expect(frameKindFromValue(FrameKind.MessageNew)).toBe(FrameKind.MessageNew);
    expect(frameKindFromValue(FrameKind.MessageEdited)).toBe(
      FrameKind.MessageEdited,
    );
    expect(frameKindFromValue(FrameKind.MessageDeleted)).toBe(
      FrameKind.MessageDeleted,
    );
    expect(frameKindFromValue(FrameKind.ReceiptUpdate)).toBe(
      FrameKind.ReceiptUpdate,
    );
    expect(frameKindFromValue(FrameKind.TypingUpdate)).toBe(
      FrameKind.TypingUpdate,
    );
    expect(frameKindFromValue(FrameKind.MemberJoined)).toBe(
      FrameKind.MemberJoined,
    );
    expect(frameKindFromValue(FrameKind.MemberLeft)).toBe(FrameKind.MemberLeft);
    expect(frameKindFromValue(FrameKind.PresenceResult)).toBe(
      FrameKind.PresenceResult,
    );
    expect(frameKindFromValue(FrameKind.ChatUpdated)).toBe(
      FrameKind.ChatUpdated,
    );
    expect(frameKindFromValue(FrameKind.ChatCreated)).toBe(
      FrameKind.ChatCreated,
    );
    expect(frameKindFromValue(FrameKind.ReactionUpdate)).toBe(
      FrameKind.ReactionUpdate,
    );
    expect(frameKindFromValue(FrameKind.UserUpdated)).toBe(
      FrameKind.UserUpdated,
    );
    expect(frameKindFromValue(FrameKind.ChatDeleted)).toBe(
      FrameKind.ChatDeleted,
    );
    expect(frameKindFromValue(FrameKind.MemberUpdated)).toBe(
      FrameKind.MemberUpdated,
    );
    expect(frameKindFromValue(FrameKind.Ack)).toBe(FrameKind.Ack);
    expect(frameKindFromValue(FrameKind.Error)).toBe(FrameKind.Error);
    expect(frameKindFromValue(FrameKind.CreateChat)).toBe(FrameKind.CreateChat);
    expect(frameKindFromValue(FrameKind.UpdateChat)).toBe(FrameKind.UpdateChat);
    expect(frameKindFromValue(FrameKind.DeleteChat)).toBe(FrameKind.DeleteChat);
    expect(frameKindFromValue(FrameKind.GetChatInfo)).toBe(
      FrameKind.GetChatInfo,
    );
    expect(frameKindFromValue(FrameKind.GetChatMembers)).toBe(
      FrameKind.GetChatMembers,
    );
    expect(frameKindFromValue(FrameKind.InviteMembers)).toBe(
      FrameKind.InviteMembers,
    );
    expect(frameKindFromValue(FrameKind.UpdateMember)).toBe(
      FrameKind.UpdateMember,
    );
    expect(frameKindFromValue(FrameKind.LeaveChat)).toBe(FrameKind.LeaveChat);
    expect(frameKindFromValue(FrameKind.MuteChat)).toBe(FrameKind.MuteChat);
    expect(frameKindFromValue(FrameKind.UnmuteChat)).toBe(FrameKind.UnmuteChat);
    expect(frameKindFromValue(FrameKind.GetUser)).toBe(FrameKind.GetUser);
    expect(frameKindFromValue(FrameKind.GetUsers)).toBe(FrameKind.GetUsers);
    expect(frameKindFromValue(FrameKind.UpdateProfile)).toBe(
      FrameKind.UpdateProfile,
    );
    expect(frameKindFromValue(FrameKind.BlockUser)).toBe(FrameKind.BlockUser);
    expect(frameKindFromValue(FrameKind.UnblockUser)).toBe(
      FrameKind.UnblockUser,
    );
    expect(frameKindFromValue(FrameKind.GetBlockList)).toBe(
      FrameKind.GetBlockList,
    );
  });
  it("fromValue returns undefined for invalid", () => {
    expect(frameKindFromValue(65535)).toBeUndefined();
  });
});

describe("LoadDirection", () => {
  it("fromValue roundtrip", () => {
    expect(loadDirectionFromValue(LoadDirection.Older)).toBe(
      LoadDirection.Older,
    );
    expect(loadDirectionFromValue(LoadDirection.Newer)).toBe(
      LoadDirection.Newer,
    );
  });
  it("fromValue returns undefined for invalid", () => {
    expect(loadDirectionFromValue(65535)).toBeUndefined();
  });
});

describe("Permission", () => {
  it("contains", () => {
    expect(
      Permission.contains(Permission.SEND_MESSAGES, Permission.SEND_MESSAGES),
    ).toBe(true);
    expect(
      Permission.contains(Permission.SEND_MESSAGES, Permission.SEND_MEDIA),
    ).toBe(false);
  });
  it("add and remove", () => {
    let flags = Permission.add(Permission.SEND_MESSAGES, Permission.SEND_MEDIA);
    expect(Permission.contains(flags, Permission.SEND_MESSAGES)).toBe(true);
    expect(Permission.contains(flags, Permission.SEND_MEDIA)).toBe(true);
    flags = Permission.remove(flags, Permission.SEND_MESSAGES);
    expect(Permission.contains(flags, Permission.SEND_MESSAGES)).toBe(false);
    expect(Permission.contains(flags, Permission.SEND_MEDIA)).toBe(true);
  });
  it("toggle", () => {
    let flags = Permission.SEND_MESSAGES;
    flags = Permission.toggle(flags, Permission.SEND_MESSAGES);
    expect(flags).toBe(0);
    flags = Permission.toggle(flags, Permission.SEND_MESSAGES);
    expect(flags).toBe(Permission.SEND_MESSAGES);
  });
});

describe("MessageFlags", () => {
  it("contains", () => {
    expect(
      MessageFlags.contains(MessageFlags.EDITED, MessageFlags.EDITED),
    ).toBe(true);
    expect(
      MessageFlags.contains(MessageFlags.EDITED, MessageFlags.DELETED),
    ).toBe(false);
  });
  it("add and remove", () => {
    let flags = MessageFlags.add(MessageFlags.EDITED, MessageFlags.DELETED);
    expect(MessageFlags.contains(flags, MessageFlags.EDITED)).toBe(true);
    expect(MessageFlags.contains(flags, MessageFlags.DELETED)).toBe(true);
    flags = MessageFlags.remove(flags, MessageFlags.EDITED);
    expect(MessageFlags.contains(flags, MessageFlags.EDITED)).toBe(false);
    expect(MessageFlags.contains(flags, MessageFlags.DELETED)).toBe(true);
  });
  it("toggle", () => {
    let flags = MessageFlags.EDITED;
    flags = MessageFlags.toggle(flags, MessageFlags.EDITED);
    expect(flags).toBe(0);
    flags = MessageFlags.toggle(flags, MessageFlags.EDITED);
    expect(flags).toBe(MessageFlags.EDITED);
  });
});

describe("RichStyle", () => {
  it("contains", () => {
    expect(RichStyle.contains(RichStyle.BOLD, RichStyle.BOLD)).toBe(true);
    expect(RichStyle.contains(RichStyle.BOLD, RichStyle.ITALIC)).toBe(false);
  });
  it("add and remove", () => {
    let flags = RichStyle.add(RichStyle.BOLD, RichStyle.ITALIC);
    expect(RichStyle.contains(flags, RichStyle.BOLD)).toBe(true);
    expect(RichStyle.contains(flags, RichStyle.ITALIC)).toBe(true);
    flags = RichStyle.remove(flags, RichStyle.BOLD);
    expect(RichStyle.contains(flags, RichStyle.BOLD)).toBe(false);
    expect(RichStyle.contains(flags, RichStyle.ITALIC)).toBe(true);
  });
  it("toggle", () => {
    let flags = RichStyle.BOLD;
    flags = RichStyle.toggle(flags, RichStyle.BOLD);
    expect(flags).toBe(0);
    flags = RichStyle.toggle(flags, RichStyle.BOLD);
    expect(flags).toBe(RichStyle.BOLD);
  });
});

describe("UserFlags", () => {
  it("contains", () => {
    expect(UserFlags.contains(UserFlags.SYSTEM, UserFlags.SYSTEM)).toBe(true);
    expect(UserFlags.contains(UserFlags.SYSTEM, UserFlags.BOT)).toBe(false);
  });
  it("add and remove", () => {
    let flags = UserFlags.add(UserFlags.SYSTEM, UserFlags.BOT);
    expect(UserFlags.contains(flags, UserFlags.SYSTEM)).toBe(true);
    expect(UserFlags.contains(flags, UserFlags.BOT)).toBe(true);
    flags = UserFlags.remove(flags, UserFlags.SYSTEM);
    expect(UserFlags.contains(flags, UserFlags.SYSTEM)).toBe(false);
    expect(UserFlags.contains(flags, UserFlags.BOT)).toBe(true);
  });
  it("toggle", () => {
    let flags = UserFlags.SYSTEM;
    flags = UserFlags.toggle(flags, UserFlags.SYSTEM);
    expect(flags).toBe(0);
    flags = UserFlags.toggle(flags, UserFlags.SYSTEM);
    expect(flags).toBe(UserFlags.SYSTEM);
  });
});

describe("ServerCapabilities", () => {
  it("contains", () => {
    expect(
      ServerCapabilities.contains(
        ServerCapabilities.MEDIA_UPLOAD,
        ServerCapabilities.MEDIA_UPLOAD,
      ),
    ).toBe(true);
    expect(
      ServerCapabilities.contains(
        ServerCapabilities.MEDIA_UPLOAD,
        ServerCapabilities.SEARCH,
      ),
    ).toBe(false);
  });
  it("add and remove", () => {
    let flags = ServerCapabilities.add(
      ServerCapabilities.MEDIA_UPLOAD,
      ServerCapabilities.SEARCH,
    );
    expect(
      ServerCapabilities.contains(flags, ServerCapabilities.MEDIA_UPLOAD),
    ).toBe(true);
    expect(ServerCapabilities.contains(flags, ServerCapabilities.SEARCH)).toBe(
      true,
    );
    flags = ServerCapabilities.remove(flags, ServerCapabilities.MEDIA_UPLOAD);
    expect(
      ServerCapabilities.contains(flags, ServerCapabilities.MEDIA_UPLOAD),
    ).toBe(false);
    expect(ServerCapabilities.contains(flags, ServerCapabilities.SEARCH)).toBe(
      true,
    );
  });
  it("toggle", () => {
    let flags = ServerCapabilities.MEDIA_UPLOAD;
    flags = ServerCapabilities.toggle(flags, ServerCapabilities.MEDIA_UPLOAD);
    expect(flags).toBe(0);
    flags = ServerCapabilities.toggle(flags, ServerCapabilities.MEDIA_UPLOAD);
    expect(flags).toBe(ServerCapabilities.MEDIA_UPLOAD);
  });
});
