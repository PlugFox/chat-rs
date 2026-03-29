// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import { describe, it, expect } from "vitest";
import {
  ProtocolWriter,
  ProtocolReader,
  ChatKind,
  ChatRole,
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
  decodeAddReactionPayload,
  decodeBlockUserPayload,
  decodeChatDeletedPayload,
  decodeChatEntry,
  decodeChatMemberEntry,
  decodeCreateChatPayload,
  decodeDeleteChatPayload,
  decodeDeleteMessagePayload,
  decodeEditMessagePayload,
  decodeErrorPayload,
  decodeForwardMessagePayload,
  decodeFrame,
  decodeFrameHeader,
  decodeGetBlockListPayload,
  decodeGetChatInfoPayload,
  decodeGetChatMembersPayload,
  decodeGetPresencePayload,
  decodeGetUserPayload,
  decodeGetUsersPayload,
  decodeHelloPayload,
  decodeInviteMembersPayload,
  decodeLastMessagePreview,
  decodeLeaveChatPayload,
  decodeLoadChatsPayload,
  decodeLoadMessagesPayload,
  decodeMemberAction,
  decodeMemberJoinedPayload,
  decodeMemberLeftPayload,
  decodeMemberUpdatedPayload,
  decodeMessage,
  decodeMessageBatch,
  decodeMessageDeletedPayload,
  decodeMuteChatPayload,
  decodePinMessagePayload,
  decodePresenceEntry,
  decodeReactionUpdatePayload,
  decodeReadReceiptPayload,
  decodeReceiptUpdatePayload,
  decodeRefreshTokenPayload,
  decodeRemoveReactionPayload,
  decodeRichSpan,
  decodeSearchPayload,
  decodeSearchScope,
  decodeSendMessagePayload,
  decodeServerLimits,
  decodeSubscribePayload,
  decodeTypingPayload,
  decodeTypingUpdatePayload,
  decodeUnblockUserPayload,
  decodeUnmuteChatPayload,
  decodeUnpinMessagePayload,
  decodeUnsubscribePayload,
  decodeUpdateChatPayload,
  decodeUpdateMemberPayload,
  decodeUpdateProfilePayload,
  decodeUserEntry,
  decodeWelcomePayload,
  encodeAddReactionPayload,
  encodeBlockUserPayload,
  encodeChatDeletedPayload,
  encodeChatEntry,
  encodeChatMemberEntry,
  encodeCreateChatPayload,
  encodeDeleteChatPayload,
  encodeDeleteMessagePayload,
  encodeEditMessagePayload,
  encodeErrorPayload,
  encodeForwardMessagePayload,
  encodeFrame,
  encodeFrameHeader,
  encodeGetBlockListPayload,
  encodeGetChatInfoPayload,
  encodeGetChatMembersPayload,
  encodeGetPresencePayload,
  encodeGetUserPayload,
  encodeGetUsersPayload,
  encodeHelloPayload,
  encodeInviteMembersPayload,
  encodeLastMessagePreview,
  encodeLeaveChatPayload,
  encodeLoadChatsPayload,
  encodeLoadMessagesPayload,
  encodeMemberAction,
  encodeMemberJoinedPayload,
  encodeMemberLeftPayload,
  encodeMemberUpdatedPayload,
  encodeMessage,
  encodeMessageBatch,
  encodeMessageDeletedPayload,
  encodeMuteChatPayload,
  encodePinMessagePayload,
  encodePresenceEntry,
  encodeReactionUpdatePayload,
  encodeReadReceiptPayload,
  encodeReceiptUpdatePayload,
  encodeRefreshTokenPayload,
  encodeRemoveReactionPayload,
  encodeRichSpan,
  encodeSearchPayload,
  encodeSearchScope,
  encodeSendMessagePayload,
  encodeServerLimits,
  encodeSubscribePayload,
  encodeTypingPayload,
  encodeTypingUpdatePayload,
  encodeUnblockUserPayload,
  encodeUnmuteChatPayload,
  encodeUnpinMessagePayload,
  encodeUnsubscribePayload,
  encodeUpdateChatPayload,
  encodeUpdateMemberPayload,
  encodeUpdateProfilePayload,
  encodeUserEntry,
  encodeWelcomePayload,
} from "../src/index.js";
import type {
  AddReactionPayload,
  BlockUserPayload,
  ChatDeletedPayload,
  ChatEntry,
  ChatMemberEntry,
  CreateChatPayload,
  DeleteChatPayload,
  DeleteMessagePayload,
  EditMessagePayload,
  ErrorPayload,
  ForwardMessagePayload,
  GetBlockListPayload,
  GetChatInfoPayload,
  GetChatMembersPayload,
  GetPresencePayload,
  GetUserPayload,
  GetUsersPayload,
  HelloPayload,
  InviteMembersPayload,
  LastMessagePreview,
  LeaveChatPayload,
  LoadChatsPayload,
  LoadMessagesPayload,
  MemberAction,
  MemberJoinedPayload,
  MemberLeftPayload,
  MemberUpdatedPayload,
  Message,
  MessageBatch,
  MessageDeletedPayload,
  MuteChatPayload,
  PinMessagePayload,
  PresenceEntry,
  ReactionUpdatePayload,
  ReadReceiptPayload,
  ReceiptUpdatePayload,
  RefreshTokenPayload,
  RemoveReactionPayload,
  RichSpan,
  SearchPayload,
  SearchScope,
  SendMessagePayload,
  ServerLimits,
  SubscribePayload,
  TypingPayload,
  TypingUpdatePayload,
  UnblockUserPayload,
  UnmuteChatPayload,
  UnpinMessagePayload,
  UnsubscribePayload,
  UpdateChatPayload,
  UpdateMemberPayload,
  UpdateProfilePayload,
  UserEntry,
  WelcomePayload,
} from "../src/index.js";

describe("LastMessagePreview codec", () => {
  it("roundtrip", () => {
    const original: LastMessagePreview = {
      id: 100000,
      senderId: 100000,
      createdAt: 1234567890,
      kind: MessageKind.Text,
      flags: MessageFlags.EDITED,
      contentPreview: "hello",
    };
    const w = new ProtocolWriter();
    encodeLastMessagePreview(w, original);
    const decoded = decodeLastMessagePreview(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("ChatEntry codec", () => {
  it("roundtrip", () => {
    const original: ChatEntry = {
      id: 100000,
      kind: ChatKind.Direct,
      parentId: 7,
      createdAt: 1234567890,
      updatedAt: 1234567890,
      title: "test",
      avatarUrl: "test",
      lastMessage: {
        id: 100000,
        senderId: 100000,
        createdAt: 1234567890,
        kind: MessageKind.Text,
        flags: MessageFlags.EDITED,
        contentPreview: "hello",
      },
      unreadCount: 100000,
      memberCount: 100000,
    };
    const w = new ProtocolWriter();
    encodeChatEntry(w, original);
    const decoded = decodeChatEntry(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("roundtrip with nulls", () => {
    const original: ChatEntry = {
      id: 100000,
      kind: ChatKind.Direct,
      parentId: null,
      createdAt: 1234567890,
      updatedAt: 1234567890,
      title: null,
      avatarUrl: null,
      lastMessage: null,
      unreadCount: 100000,
      memberCount: 100000,
    };
    const w = new ProtocolWriter();
    encodeChatEntry(w, original);
    const decoded = decodeChatEntry(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("ChatMemberEntry codec", () => {
  it("roundtrip", () => {
    const original: ChatMemberEntry = {
      userId: 100000,
      role: ChatRole.Member,
      permissions: Permission.SEND_MESSAGES,
    };
    const w = new ProtocolWriter();
    encodeChatMemberEntry(w, original);
    const decoded = decodeChatMemberEntry(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("roundtrip with nulls", () => {
    const original: ChatMemberEntry = {
      userId: 100000,
      role: ChatRole.Member,
      permissions: null,
    };
    const w = new ProtocolWriter();
    encodeChatMemberEntry(w, original);
    const decoded = decodeChatMemberEntry(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("RichSpan codec", () => {
  it("roundtrip", () => {
    const original: RichSpan = {
      start: 100000,
      end: 100000,
      style: RichStyle.BOLD,
      meta: "test",
    };
    const w = new ProtocolWriter();
    encodeRichSpan(w, original);
    const decoded = decodeRichSpan(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("roundtrip with nulls", () => {
    const original: RichSpan = {
      start: 100000,
      end: 100000,
      style: RichStyle.BOLD,
      meta: null,
    };
    const w = new ProtocolWriter();
    encodeRichSpan(w, original);
    const decoded = decodeRichSpan(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("Message codec", () => {
  it("roundtrip", () => {
    const original: Message = {
      id: 100000,
      chatId: 100000,
      senderId: 100000,
      createdAt: 1234567890,
      updatedAt: 1234567890,
      kind: MessageKind.Text,
      flags: MessageFlags.EDITED,
      replyToId: 7,
      content: "hello",
      richContent: [
        { start: 100000, end: 100000, style: RichStyle.BOLD, meta: "test" },
      ],
      extra: "test",
    };
    const w = new ProtocolWriter();
    encodeMessage(w, original);
    const decoded = decodeMessage(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("roundtrip with nulls", () => {
    const original: Message = {
      id: 100000,
      chatId: 100000,
      senderId: 100000,
      createdAt: 1234567890,
      updatedAt: 1234567890,
      kind: MessageKind.Text,
      flags: MessageFlags.EDITED,
      replyToId: null,
      content: "",
      richContent: null,
      extra: null,
    };
    const w = new ProtocolWriter();
    encodeMessage(w, original);
    const decoded = decodeMessage(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("MessageBatch codec", () => {
  it("roundtrip", () => {
    const original: MessageBatch = {
      messages: [
        {
          id: 100000,
          chatId: 100000,
          senderId: 100000,
          createdAt: 1234567890,
          updatedAt: 1234567890,
          kind: MessageKind.Text,
          flags: MessageFlags.EDITED,
          replyToId: 7,
          content: "hello",
          richContent: [
            { start: 100000, end: 100000, style: RichStyle.BOLD, meta: "test" },
          ],
          extra: "test",
        },
      ],
      hasMore: true,
    };
    const w = new ProtocolWriter();
    encodeMessageBatch(w, original);
    const decoded = decodeMessageBatch(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("UserEntry codec", () => {
  it("roundtrip", () => {
    const original: UserEntry = {
      id: 100000,
      flags: UserFlags.SYSTEM,
      createdAt: 1234567890,
      updatedAt: 1234567890,
      username: "test",
      firstName: "test",
      lastName: "test",
      avatarUrl: "test",
    };
    const w = new ProtocolWriter();
    encodeUserEntry(w, original);
    const decoded = decodeUserEntry(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("roundtrip with nulls", () => {
    const original: UserEntry = {
      id: 100000,
      flags: UserFlags.SYSTEM,
      createdAt: 1234567890,
      updatedAt: 1234567890,
      username: null,
      firstName: null,
      lastName: null,
      avatarUrl: null,
    };
    const w = new ProtocolWriter();
    encodeUserEntry(w, original);
    const decoded = decodeUserEntry(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("PresenceEntry codec", () => {
  it("roundtrip", () => {
    const original: PresenceEntry = {
      userId: 100000,
      status: PresenceStatus.Offline,
      lastSeen: 1234567890,
    };
    const w = new ProtocolWriter();
    encodePresenceEntry(w, original);
    const decoded = decodePresenceEntry(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("ErrorPayload codec", () => {
  it("roundtrip", () => {
    const original: ErrorPayload = {
      code: ErrorCode.Unauthorized,
      message: "hello",
      retryAfterMs: 100000,
      extra: "test",
    };
    const w = new ProtocolWriter();
    encodeErrorPayload(w, original);
    const decoded = decodeErrorPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("roundtrip with nulls", () => {
    const original: ErrorPayload = {
      code: ErrorCode.Unauthorized,
      message: "",
      retryAfterMs: 100000,
      extra: null,
    };
    const w = new ProtocolWriter();
    encodeErrorPayload(w, original);
    const decoded = decodeErrorPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("HelloPayload codec", () => {
  it("roundtrip", () => {
    const original: HelloPayload = {
      protocolVersion: 42,
      sdkVersion: "hello",
      platform: "hello",
      token: "hello",
      deviceId: "550e8400-e29b-41d4-a716-446655440000",
    };
    const w = new ProtocolWriter();
    encodeHelloPayload(w, original);
    const decoded = decodeHelloPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("WelcomePayload codec", () => {
  it("roundtrip", () => {
    const original: WelcomePayload = {
      sessionId: 100000,
      serverTime: 1234567890,
      userId: 100000,
      limits: {
        pingIntervalMs: 100000,
        pingTimeoutMs: 100000,
        maxMessageSize: 100000,
        maxExtraSize: 100000,
        maxFrameSize: 100000,
        messagesPerSec: 1000,
        connectionsPerIp: 1000,
      },
      capabilities: ServerCapabilities.MEDIA_UPLOAD,
    };
    const w = new ProtocolWriter();
    encodeWelcomePayload(w, original);
    const decoded = decodeWelcomePayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("ServerLimits codec", () => {
  it("roundtrip", () => {
    const original: ServerLimits = {
      pingIntervalMs: 100000,
      pingTimeoutMs: 100000,
      maxMessageSize: 100000,
      maxExtraSize: 100000,
      maxFrameSize: 100000,
      messagesPerSec: 1000,
      connectionsPerIp: 1000,
    };
    const w = new ProtocolWriter();
    encodeServerLimits(w, original);
    const decoded = decodeServerLimits(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("SendMessagePayload codec", () => {
  it("roundtrip", () => {
    const original: SendMessagePayload = {
      chatId: 100000,
      kind: MessageKind.Text,
      idempotencyKey: "550e8400-e29b-41d4-a716-446655440000",
      replyToId: 7,
      content: "hello",
      richContent: new Uint8Array([1, 2]),
      extra: "test",
      mentionedUserIds: [1, 2, 3],
    };
    const w = new ProtocolWriter();
    encodeSendMessagePayload(w, original);
    const decoded = decodeSendMessagePayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("roundtrip with nulls", () => {
    const original: SendMessagePayload = {
      chatId: 100000,
      kind: MessageKind.Text,
      idempotencyKey: "550e8400-e29b-41d4-a716-446655440000",
      replyToId: null,
      content: "",
      richContent: null,
      extra: null,
      mentionedUserIds: [],
    };
    const w = new ProtocolWriter();
    encodeSendMessagePayload(w, original);
    const decoded = decodeSendMessagePayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("EditMessagePayload codec", () => {
  it("roundtrip", () => {
    const original: EditMessagePayload = {
      chatId: 100000,
      messageId: 100000,
      content: "hello",
      richContent: new Uint8Array([1, 2]),
      extra: "test",
    };
    const w = new ProtocolWriter();
    encodeEditMessagePayload(w, original);
    const decoded = decodeEditMessagePayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("roundtrip with nulls", () => {
    const original: EditMessagePayload = {
      chatId: 100000,
      messageId: 100000,
      content: "",
      richContent: null,
      extra: null,
    };
    const w = new ProtocolWriter();
    encodeEditMessagePayload(w, original);
    const decoded = decodeEditMessagePayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("DeleteMessagePayload codec", () => {
  it("roundtrip", () => {
    const original: DeleteMessagePayload = {
      chatId: 100000,
      messageId: 100000,
    };
    const w = new ProtocolWriter();
    encodeDeleteMessagePayload(w, original);
    const decoded = decodeDeleteMessagePayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("ReadReceiptPayload codec", () => {
  it("roundtrip", () => {
    const original: ReadReceiptPayload = { chatId: 100000, messageId: 100000 };
    const w = new ProtocolWriter();
    encodeReadReceiptPayload(w, original);
    const decoded = decodeReadReceiptPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("TypingPayload codec", () => {
  it("roundtrip", () => {
    const original: TypingPayload = { chatId: 100000, expiresInMs: 1000 };
    const w = new ProtocolWriter();
    encodeTypingPayload(w, original);
    const decoded = decodeTypingPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("GetPresencePayload codec", () => {
  it("roundtrip", () => {
    const original: GetPresencePayload = { userIds: [1, 2, 3] };
    const w = new ProtocolWriter();
    encodeGetPresencePayload(w, original);
    const decoded = decodeGetPresencePayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("SearchPayload codec", () => {
  it("roundtrip", () => {
    const original: SearchPayload = {
      scope: { type: "chat" as const, chatId: 100000 },
      query: "hello",
      cursor: 100000,
      limit: 1000,
    };
    const w = new ProtocolWriter();
    encodeSearchPayload(w, original);
    const decoded = decodeSearchPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("SubscribePayload codec", () => {
  it("roundtrip", () => {
    const original: SubscribePayload = { channels: ["a", "b"] };
    const w = new ProtocolWriter();
    encodeSubscribePayload(w, original);
    const decoded = decodeSubscribePayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("UnsubscribePayload codec", () => {
  it("roundtrip", () => {
    const original: UnsubscribePayload = { channels: ["a", "b"] };
    const w = new ProtocolWriter();
    encodeUnsubscribePayload(w, original);
    const decoded = decodeUnsubscribePayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("CreateChatPayload codec", () => {
  it("roundtrip", () => {
    const original: CreateChatPayload = {
      kind: ChatKind.Direct,
      parentId: 7,
      title: "test",
      avatarUrl: "test",
      memberIds: [1, 2, 3],
    };
    const w = new ProtocolWriter();
    encodeCreateChatPayload(w, original);
    const decoded = decodeCreateChatPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("roundtrip with nulls", () => {
    const original: CreateChatPayload = {
      kind: ChatKind.Direct,
      parentId: null,
      title: null,
      avatarUrl: null,
      memberIds: [],
    };
    const w = new ProtocolWriter();
    encodeCreateChatPayload(w, original);
    const decoded = decodeCreateChatPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("UpdateChatPayload codec", () => {
  it("roundtrip", () => {
    const original: UpdateChatPayload = {
      chatId: 100000,
      title: "updated",
      avatarUrl: "updated",
    };
    const w = new ProtocolWriter();
    encodeUpdateChatPayload(w, original);
    const decoded = decodeUpdateChatPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("roundtrip with nulls", () => {
    const original: UpdateChatPayload = {
      chatId: 100000,
      title: null,
      avatarUrl: null,
    };
    const w = new ProtocolWriter();
    encodeUpdateChatPayload(w, original);
    const decoded = decodeUpdateChatPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("DeleteChatPayload codec", () => {
  it("roundtrip", () => {
    const original: DeleteChatPayload = { chatId: 100000 };
    const w = new ProtocolWriter();
    encodeDeleteChatPayload(w, original);
    const decoded = decodeDeleteChatPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("GetChatInfoPayload codec", () => {
  it("roundtrip", () => {
    const original: GetChatInfoPayload = { chatId: 100000 };
    const w = new ProtocolWriter();
    encodeGetChatInfoPayload(w, original);
    const decoded = decodeGetChatInfoPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("GetChatMembersPayload codec", () => {
  it("roundtrip", () => {
    const original: GetChatMembersPayload = {
      chatId: 100000,
      cursor: 100000,
      limit: 1000,
    };
    const w = new ProtocolWriter();
    encodeGetChatMembersPayload(w, original);
    const decoded = decodeGetChatMembersPayload(
      new ProtocolReader(w.toBytes()),
    );
    expect(decoded).toEqual(original);
  });
});

describe("InviteMembersPayload codec", () => {
  it("roundtrip", () => {
    const original: InviteMembersPayload = {
      chatId: 100000,
      userIds: [1, 2, 3],
    };
    const w = new ProtocolWriter();
    encodeInviteMembersPayload(w, original);
    const decoded = decodeInviteMembersPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("LeaveChatPayload codec", () => {
  it("roundtrip", () => {
    const original: LeaveChatPayload = { chatId: 100000 };
    const w = new ProtocolWriter();
    encodeLeaveChatPayload(w, original);
    const decoded = decodeLeaveChatPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("UpdateMemberPayload codec", () => {
  it("roundtrip", () => {
    const original: UpdateMemberPayload = {
      chatId: 100000,
      userId: 100000,
      action: { type: "kick" as const },
    };
    const w = new ProtocolWriter();
    encodeUpdateMemberPayload(w, original);
    const decoded = decodeUpdateMemberPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("MessageDeletedPayload codec", () => {
  it("roundtrip", () => {
    const original: MessageDeletedPayload = {
      chatId: 100000,
      messageId: 100000,
    };
    const w = new ProtocolWriter();
    encodeMessageDeletedPayload(w, original);
    const decoded = decodeMessageDeletedPayload(
      new ProtocolReader(w.toBytes()),
    );
    expect(decoded).toEqual(original);
  });
});

describe("ReceiptUpdatePayload codec", () => {
  it("roundtrip", () => {
    const original: ReceiptUpdatePayload = {
      chatId: 100000,
      userId: 100000,
      messageId: 100000,
    };
    const w = new ProtocolWriter();
    encodeReceiptUpdatePayload(w, original);
    const decoded = decodeReceiptUpdatePayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("TypingUpdatePayload codec", () => {
  it("roundtrip", () => {
    const original: TypingUpdatePayload = {
      chatId: 100000,
      userId: 100000,
      expiresInMs: 1000,
    };
    const w = new ProtocolWriter();
    encodeTypingUpdatePayload(w, original);
    const decoded = decodeTypingUpdatePayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("MemberJoinedPayload codec", () => {
  it("roundtrip", () => {
    const original: MemberJoinedPayload = {
      chatId: 100000,
      userId: 100000,
      role: ChatRole.Member,
      invitedBy: 100000,
    };
    const w = new ProtocolWriter();
    encodeMemberJoinedPayload(w, original);
    const decoded = decodeMemberJoinedPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("MemberLeftPayload codec", () => {
  it("roundtrip", () => {
    const original: MemberLeftPayload = { chatId: 100000, userId: 100000 };
    const w = new ProtocolWriter();
    encodeMemberLeftPayload(w, original);
    const decoded = decodeMemberLeftPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("ChatDeletedPayload codec", () => {
  it("roundtrip", () => {
    const original: ChatDeletedPayload = { chatId: 100000 };
    const w = new ProtocolWriter();
    encodeChatDeletedPayload(w, original);
    const decoded = decodeChatDeletedPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("MemberUpdatedPayload codec", () => {
  it("roundtrip", () => {
    const original: MemberUpdatedPayload = {
      chatId: 100000,
      userId: 100000,
      role: ChatRole.Member,
      permissions: Permission.SEND_MESSAGES,
    };
    const w = new ProtocolWriter();
    encodeMemberUpdatedPayload(w, original);
    const decoded = decodeMemberUpdatedPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("roundtrip with nulls", () => {
    const original: MemberUpdatedPayload = {
      chatId: 100000,
      userId: 100000,
      role: ChatRole.Member,
      permissions: null,
    };
    const w = new ProtocolWriter();
    encodeMemberUpdatedPayload(w, original);
    const decoded = decodeMemberUpdatedPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("AddReactionPayload codec", () => {
  it("roundtrip", () => {
    const original: AddReactionPayload = {
      chatId: 100000,
      messageId: 100000,
      packId: 100000,
      emojiIndex: 42,
    };
    const w = new ProtocolWriter();
    encodeAddReactionPayload(w, original);
    const decoded = decodeAddReactionPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("RemoveReactionPayload codec", () => {
  it("roundtrip", () => {
    const original: RemoveReactionPayload = {
      chatId: 100000,
      messageId: 100000,
      packId: 100000,
      emojiIndex: 42,
    };
    const w = new ProtocolWriter();
    encodeRemoveReactionPayload(w, original);
    const decoded = decodeRemoveReactionPayload(
      new ProtocolReader(w.toBytes()),
    );
    expect(decoded).toEqual(original);
  });
});

describe("ReactionUpdatePayload codec", () => {
  it("roundtrip", () => {
    const original: ReactionUpdatePayload = {
      chatId: 100000,
      messageId: 100000,
      userId: 100000,
      packId: 100000,
      emojiIndex: 42,
      added: true,
    };
    const w = new ProtocolWriter();
    encodeReactionUpdatePayload(w, original);
    const decoded = decodeReactionUpdatePayload(
      new ProtocolReader(w.toBytes()),
    );
    expect(decoded).toEqual(original);
  });
});

describe("PinMessagePayload codec", () => {
  it("roundtrip", () => {
    const original: PinMessagePayload = { chatId: 100000, messageId: 100000 };
    const w = new ProtocolWriter();
    encodePinMessagePayload(w, original);
    const decoded = decodePinMessagePayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("UnpinMessagePayload codec", () => {
  it("roundtrip", () => {
    const original: UnpinMessagePayload = { chatId: 100000, messageId: 100000 };
    const w = new ProtocolWriter();
    encodeUnpinMessagePayload(w, original);
    const decoded = decodeUnpinMessagePayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("RefreshTokenPayload codec", () => {
  it("roundtrip", () => {
    const original: RefreshTokenPayload = { token: "hello" };
    const w = new ProtocolWriter();
    encodeRefreshTokenPayload(w, original);
    const decoded = decodeRefreshTokenPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("ForwardMessagePayload codec", () => {
  it("roundtrip", () => {
    const original: ForwardMessagePayload = {
      fromChatId: 100000,
      messageId: 100000,
      toChatId: 100000,
      idempotencyKey: "550e8400-e29b-41d4-a716-446655440000",
    };
    const w = new ProtocolWriter();
    encodeForwardMessagePayload(w, original);
    const decoded = decodeForwardMessagePayload(
      new ProtocolReader(w.toBytes()),
    );
    expect(decoded).toEqual(original);
  });
});

describe("GetUserPayload codec", () => {
  it("roundtrip", () => {
    const original: GetUserPayload = { userId: 100000 };
    const w = new ProtocolWriter();
    encodeGetUserPayload(w, original);
    const decoded = decodeGetUserPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("GetUsersPayload codec", () => {
  it("roundtrip", () => {
    const original: GetUsersPayload = { userIds: [1, 2, 3] };
    const w = new ProtocolWriter();
    encodeGetUsersPayload(w, original);
    const decoded = decodeGetUsersPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("UpdateProfilePayload codec", () => {
  it("roundtrip", () => {
    const original: UpdateProfilePayload = {
      username: "updated",
      firstName: "updated",
      lastName: "updated",
      avatarUrl: "updated",
    };
    const w = new ProtocolWriter();
    encodeUpdateProfilePayload(w, original);
    const decoded = decodeUpdateProfilePayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("roundtrip with nulls", () => {
    const original: UpdateProfilePayload = {
      username: null,
      firstName: null,
      lastName: null,
      avatarUrl: null,
    };
    const w = new ProtocolWriter();
    encodeUpdateProfilePayload(w, original);
    const decoded = decodeUpdateProfilePayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("BlockUserPayload codec", () => {
  it("roundtrip", () => {
    const original: BlockUserPayload = { userId: 100000 };
    const w = new ProtocolWriter();
    encodeBlockUserPayload(w, original);
    const decoded = decodeBlockUserPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("UnblockUserPayload codec", () => {
  it("roundtrip", () => {
    const original: UnblockUserPayload = { userId: 100000 };
    const w = new ProtocolWriter();
    encodeUnblockUserPayload(w, original);
    const decoded = decodeUnblockUserPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("GetBlockListPayload codec", () => {
  it("roundtrip", () => {
    const original: GetBlockListPayload = { cursor: 100000, limit: 1000 };
    const w = new ProtocolWriter();
    encodeGetBlockListPayload(w, original);
    const decoded = decodeGetBlockListPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("MuteChatPayload codec", () => {
  it("roundtrip", () => {
    const original: MuteChatPayload = { chatId: 100000, durationSecs: 100000 };
    const w = new ProtocolWriter();
    encodeMuteChatPayload(w, original);
    const decoded = decodeMuteChatPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("UnmuteChatPayload codec", () => {
  it("roundtrip", () => {
    const original: UnmuteChatPayload = { chatId: 100000 };
    const w = new ProtocolWriter();
    encodeUnmuteChatPayload(w, original);
    const decoded = decodeUnmuteChatPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("LoadChatsPayload codec", () => {
  it("FirstPage roundtrip", () => {
    const original: LoadChatsPayload = {
      type: "firstPage" as const,
      limit: 1000,
    };
    const w = new ProtocolWriter();
    encodeLoadChatsPayload(w, original);
    const decoded = decodeLoadChatsPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("After roundtrip", () => {
    const original: LoadChatsPayload = {
      type: "after" as const,
      cursorTs: 1234567890,
      limit: 1000,
    };
    const w = new ProtocolWriter();
    encodeLoadChatsPayload(w, original);
    const decoded = decodeLoadChatsPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("SearchScope codec", () => {
  it("Chat roundtrip", () => {
    const original: SearchScope = { type: "chat" as const, chatId: 100000 };
    const w = new ProtocolWriter();
    encodeSearchScope(w, original);
    const decoded = decodeSearchScope(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("Global roundtrip", () => {
    const original: SearchScope = { type: "global" as const };
    const w = new ProtocolWriter();
    encodeSearchScope(w, original);
    const decoded = decodeSearchScope(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("User roundtrip", () => {
    const original: SearchScope = { type: "user" as const, userId: 100000 };
    const w = new ProtocolWriter();
    encodeSearchScope(w, original);
    const decoded = decodeSearchScope(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("LoadMessagesPayload codec", () => {
  it("Paginate roundtrip", () => {
    const original: LoadMessagesPayload = {
      type: "paginate" as const,
      chatId: 100000,
      direction: LoadDirection.Older,
      anchorId: 100000,
      limit: 1000,
    };
    const w = new ProtocolWriter();
    encodeLoadMessagesPayload(w, original);
    const decoded = decodeLoadMessagesPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("RangeCheck roundtrip", () => {
    const original: LoadMessagesPayload = {
      type: "rangeCheck" as const,
      chatId: 100000,
      fromId: 100000,
      toId: 100000,
      sinceTs: 1234567890,
    };
    const w = new ProtocolWriter();
    encodeLoadMessagesPayload(w, original);
    const decoded = decodeLoadMessagesPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("Chunk roundtrip", () => {
    const original: LoadMessagesPayload = {
      type: "chunk" as const,
      chatId: 100000,
      chunkId: 100000,
      sinceTs: 1234567890,
    };
    const w = new ProtocolWriter();
    encodeLoadMessagesPayload(w, original);
    const decoded = decodeLoadMessagesPayload(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("MemberAction codec", () => {
  it("Kick roundtrip", () => {
    const original: MemberAction = { type: "kick" as const };
    const w = new ProtocolWriter();
    encodeMemberAction(w, original);
    const decoded = decodeMemberAction(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("Ban roundtrip", () => {
    const original: MemberAction = { type: "ban" as const };
    const w = new ProtocolWriter();
    encodeMemberAction(w, original);
    const decoded = decodeMemberAction(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("Mute roundtrip", () => {
    const original: MemberAction = {
      type: "mute" as const,
      durationSecs: 100000,
    };
    const w = new ProtocolWriter();
    encodeMemberAction(w, original);
    const decoded = decodeMemberAction(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("ChangeRole roundtrip", () => {
    const original: MemberAction = {
      type: "changeRole" as const,
      chatRole: ChatRole.Member,
    };
    const w = new ProtocolWriter();
    encodeMemberAction(w, original);
    const decoded = decodeMemberAction(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("UpdatePermissions roundtrip", () => {
    const original: MemberAction = {
      type: "updatePermissions" as const,
      permission: Permission.SEND_MESSAGES,
    };
    const w = new ProtocolWriter();
    encodeMemberAction(w, original);
    const decoded = decodeMemberAction(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
  it("Unban roundtrip", () => {
    const original: MemberAction = { type: "unban" as const };
    const w = new ProtocolWriter();
    encodeMemberAction(w, original);
    const decoded = decodeMemberAction(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(original);
  });
});

describe("FrameHeader codec", () => {
  it("roundtrip", () => {
    const header = { kind: FrameKind.Hello, seq: 42, eventSeq: 7 };
    const w = new ProtocolWriter();
    encodeFrameHeader(w, header);
    const decoded = decodeFrameHeader(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(header);
  });
});

describe("Frame codec", () => {
  it("Ping frame roundtrip (no payload)", () => {
    const frame = { seq: 1, eventSeq: 0, payload: { type: "ping" as const } };
    const w = new ProtocolWriter();
    encodeFrame(w, frame);
    const decoded = decodeFrame(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(frame);
  });

  it("DeleteMessage frame roundtrip (struct payload)", () => {
    const frame = {
      seq: 5,
      eventSeq: 3,
      payload: {
        type: "deleteMessage" as const,
        data: { chatId: 1, messageId: 2 },
      },
    };
    const w = new ProtocolWriter();
    encodeFrame(w, frame);
    const decoded = decodeFrame(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(frame);
  });

  it("LoadChats frame roundtrip (tagged enum payload)", () => {
    const frame = {
      seq: 10,
      eventSeq: 0,
      payload: {
        type: "loadChats" as const,
        data: { type: "firstPage" as const, limit: 50 },
      },
    };
    const w = new ProtocolWriter();
    encodeFrame(w, frame);
    const decoded = decodeFrame(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(frame);
  });

  it("Ack frame roundtrip (raw bytes)", () => {
    const frame = {
      seq: 20,
      eventSeq: 0,
      payload: { type: "ack" as const, data: new Uint8Array([1, 2, 3, 4]) },
    };
    const w = new ProtocolWriter();
    encodeFrame(w, frame);
    const decoded = decodeFrame(new ProtocolReader(w.toBytes()));
    expect(decoded).toEqual(frame);
  });
});
