// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import '../../chat_core.dart';

void encodeLastMessagePreview(ProtocolWriter w, LastMessagePreview v) {
  w.writeU32(v.id);
  w.writeU32(v.senderId);
  w.writeTimestamp(v.createdAt);
  w.writeU8(v.kind.value);
  w.writeU16(v.flags.value);
  w.writeString(v.contentPreview);
}

LastMessagePreview decodeLastMessagePreview(ProtocolReader r) {
  return LastMessagePreview(
    id: r.readU32(),
    senderId: r.readU32(),
    createdAt: r.readTimestamp(),
    kind: MessageKind.fromValue(r.readU8())!,
    flags: MessageFlags(r.readU16()),
    contentPreview: r.readString(),
  );
}

void encodeChatEntry(ProtocolWriter w, ChatEntry v) {
  w.writeU32(v.id);
  w.writeU8(v.kind.value);
  w.writeOptionU32(v.parentId);
  w.writeTimestamp(v.createdAt);
  w.writeTimestamp(v.updatedAt);
  w.writeOptionalString(v.title);
  w.writeOptionalString(v.avatarUrl);
  if (v.lastMessage != null) {
    w.writeU8(1);
    encodeLastMessagePreview(w, v.lastMessage!);
  } else {
    w.writeU8(0);
  }
  w.writeU32(v.unreadCount);
  w.writeU32(v.memberCount);
}

ChatEntry decodeChatEntry(ProtocolReader r) {
  return ChatEntry(
    id: r.readU32(),
    kind: ChatKind.fromValue(r.readU8())!,
    parentId: r.readOptionU32(),
    createdAt: r.readTimestamp(),
    updatedAt: r.readTimestamp(),
    title: r.readOptionalString(),
    avatarUrl: r.readOptionalString(),
    lastMessage: r.readU8() == 1 ? decodeLastMessagePreview(r) : null,
    unreadCount: r.readU32(),
    memberCount: r.readU32(),
  );
}

void encodeChatMemberEntry(ProtocolWriter w, ChatMemberEntry v) {
  w.writeU32(v.userId);
  w.writeU8(v.role.value);
  if (v.permissions != null) {
    w.writeU8(1);
    w.writeU32(v.permissions!.value);
  } else {
    w.writeU8(0);
  }
}

ChatMemberEntry decodeChatMemberEntry(ProtocolReader r) {
  return ChatMemberEntry(
    userId: r.readU32(),
    role: ChatRole.fromValue(r.readU8())!,
    permissions: r.readU8() == 1 ? Permission(r.readU32()) : null,
  );
}

void encodeRichSpan(ProtocolWriter w, RichSpan v) {
  w.writeU32(v.start);
  w.writeU32(v.end);
  w.writeU16(v.style.value);
  w.writeOptionalString(v.meta);
}

RichSpan decodeRichSpan(ProtocolReader r) {
  return RichSpan(
    start: r.readU32(),
    end: r.readU32(),
    style: RichStyle(r.readU16()),
    meta: r.readOptionalString(),
  );
}

void encodeUserEntry(ProtocolWriter w, UserEntry v) {
  w.writeU32(v.id);
  w.writeU16(v.flags.value);
  w.writeTimestamp(v.createdAt);
  w.writeTimestamp(v.updatedAt);
  w.writeOptionalString(v.username);
  w.writeOptionalString(v.firstName);
  w.writeOptionalString(v.lastName);
  w.writeOptionalString(v.avatarUrl);
}

UserEntry decodeUserEntry(ProtocolReader r) {
  return UserEntry(
    id: r.readU32(),
    flags: UserFlags(r.readU16()),
    createdAt: r.readTimestamp(),
    updatedAt: r.readTimestamp(),
    username: r.readOptionalString(),
    firstName: r.readOptionalString(),
    lastName: r.readOptionalString(),
    avatarUrl: r.readOptionalString(),
  );
}

void encodePresenceEntry(ProtocolWriter w, PresenceEntry v) {
  w.writeU32(v.userId);
  w.writeU8(v.status.value);
  w.writeTimestamp(v.lastSeen);
}

PresenceEntry decodePresenceEntry(ProtocolReader r) {
  return PresenceEntry(
    userId: r.readU32(),
    status: PresenceStatus.fromValue(r.readU8())!,
    lastSeen: r.readTimestamp(),
  );
}

void encodeHelloPayload(ProtocolWriter w, HelloPayload v) {
  w.writeU8(v.protocolVersion);
  w.writeString(v.sdkVersion);
  w.writeString(v.platform);
  w.writeString(v.token);
  w.writeUuid(v.deviceId);
}

HelloPayload decodeHelloPayload(ProtocolReader r) {
  return HelloPayload(
    protocolVersion: r.readU8(),
    sdkVersion: r.readString(),
    platform: r.readString(),
    token: r.readString(),
    deviceId: r.readUuid(),
  );
}

void encodeWelcomePayload(ProtocolWriter w, WelcomePayload v) {
  w.writeU32(v.sessionId);
  w.writeTimestamp(v.serverTime);
  w.writeU32(v.userId);
  encodeServerLimits(w, v.limits);
  w.writeU32(v.capabilities.value);
}

WelcomePayload decodeWelcomePayload(ProtocolReader r) {
  return WelcomePayload(
    sessionId: r.readU32(),
    serverTime: r.readTimestamp(),
    userId: r.readU32(),
    limits: decodeServerLimits(r),
    capabilities: ServerCapabilities(r.readU32()),
  );
}

void encodeServerLimits(ProtocolWriter w, ServerLimits v) {
  w.writeU32(v.pingIntervalMs);
  w.writeU32(v.pingTimeoutMs);
  w.writeU32(v.maxMessageSize);
  w.writeU32(v.maxExtraSize);
  w.writeU32(v.maxFrameSize);
  w.writeU16(v.messagesPerSec);
  w.writeU16(v.connectionsPerIp);
}

ServerLimits decodeServerLimits(ProtocolReader r) {
  return ServerLimits(
    pingIntervalMs: r.readU32(),
    pingTimeoutMs: r.readU32(),
    maxMessageSize: r.readU32(),
    maxExtraSize: r.readU32(),
    maxFrameSize: r.readU32(),
    messagesPerSec: r.readU16(),
    connectionsPerIp: r.readU16(),
  );
}

void encodeSendMessagePayload(ProtocolWriter w, SendMessagePayload v) {
  w.writeU32(v.chatId);
  w.writeU8(v.kind.value);
  w.writeUuid(v.idempotencyKey);
  w.writeOptionU32(v.replyToId);
  w.writeString(v.content);
  w.writeOptionalBytes(v.richContent);
  w.writeOptionalString(v.extra);
  w.writeU16(v.mentionedUserIds.length);
  for (final v in v.mentionedUserIds) {
    w.writeU32(v);
  }
}

SendMessagePayload decodeSendMessagePayload(ProtocolReader r) {
  return SendMessagePayload(
    chatId: r.readU32(),
    kind: MessageKind.fromValue(r.readU8())!,
    idempotencyKey: r.readUuid(),
    replyToId: r.readOptionU32(),
    content: r.readString(),
    richContent: r.readOptionalBytes(),
    extra: r.readOptionalString(),
    mentionedUserIds: r.readVecU32(),
  );
}

void encodeEditMessagePayload(ProtocolWriter w, EditMessagePayload v) {
  w.writeU32(v.chatId);
  w.writeU32(v.messageId);
  w.writeString(v.content);
  w.writeOptionalBytes(v.richContent);
  w.writeOptionalString(v.extra);
}

EditMessagePayload decodeEditMessagePayload(ProtocolReader r) {
  return EditMessagePayload(
    chatId: r.readU32(),
    messageId: r.readU32(),
    content: r.readString(),
    richContent: r.readOptionalBytes(),
    extra: r.readOptionalString(),
  );
}

void encodeDeleteMessagePayload(ProtocolWriter w, DeleteMessagePayload v) {
  w.writeU32(v.chatId);
  w.writeU32(v.messageId);
}

DeleteMessagePayload decodeDeleteMessagePayload(ProtocolReader r) {
  return DeleteMessagePayload(chatId: r.readU32(), messageId: r.readU32());
}

void encodeReadReceiptPayload(ProtocolWriter w, ReadReceiptPayload v) {
  w.writeU32(v.chatId);
  w.writeU32(v.messageId);
}

ReadReceiptPayload decodeReadReceiptPayload(ProtocolReader r) {
  return ReadReceiptPayload(chatId: r.readU32(), messageId: r.readU32());
}

void encodeTypingPayload(ProtocolWriter w, TypingPayload v) {
  w.writeU32(v.chatId);
  w.writeU16(v.expiresInMs);
}

TypingPayload decodeTypingPayload(ProtocolReader r) {
  return TypingPayload(chatId: r.readU32(), expiresInMs: r.readU16());
}

void encodeGetPresencePayload(ProtocolWriter w, GetPresencePayload v) {
  w.writeU16(v.userIds.length);
  for (final v in v.userIds) {
    w.writeU32(v);
  }
}

GetPresencePayload decodeGetPresencePayload(ProtocolReader r) {
  return GetPresencePayload(userIds: r.readVecU32());
}

void encodeSearchPayload(ProtocolWriter w, SearchPayload v) {
  encodeSearchScope(w, v.scope);
  w.writeString(v.query);
  w.writeU32(v.cursor);
  w.writeU16(v.limit);
}

SearchPayload decodeSearchPayload(ProtocolReader r) {
  return SearchPayload(
    scope: decodeSearchScope(r),
    query: r.readString(),
    cursor: r.readU32(),
    limit: r.readU16(),
  );
}

void encodeSubscribePayload(ProtocolWriter w, SubscribePayload v) {
  w.writeU16(v.channels.length);
  for (final v in v.channels) {
    w.writeString(v);
  }
}

SubscribePayload decodeSubscribePayload(ProtocolReader r) {
  return SubscribePayload(channels: r.readVecString());
}

void encodeUnsubscribePayload(ProtocolWriter w, UnsubscribePayload v) {
  w.writeU16(v.channels.length);
  for (final v in v.channels) {
    w.writeString(v);
  }
}

UnsubscribePayload decodeUnsubscribePayload(ProtocolReader r) {
  return UnsubscribePayload(channels: r.readVecString());
}

void encodeCreateChatPayload(ProtocolWriter w, CreateChatPayload v) {
  w.writeU8(v.kind.value);
  w.writeOptionU32(v.parentId);
  w.writeOptionalString(v.title);
  w.writeOptionalString(v.avatarUrl);
  w.writeU16(v.memberIds.length);
  for (final v in v.memberIds) {
    w.writeU32(v);
  }
}

CreateChatPayload decodeCreateChatPayload(ProtocolReader r) {
  return CreateChatPayload(
    kind: ChatKind.fromValue(r.readU8())!,
    parentId: r.readOptionU32(),
    title: r.readOptionalString(),
    avatarUrl: r.readOptionalString(),
    memberIds: r.readVecU32(),
  );
}

void encodeUpdateChatPayload(ProtocolWriter w, UpdateChatPayload v) {
  w.writeU32(v.chatId);
  w.writeUpdatableString(v.title);
  w.writeUpdatableString(v.avatarUrl);
}

UpdateChatPayload decodeUpdateChatPayload(ProtocolReader r) {
  return UpdateChatPayload(
    chatId: r.readU32(),
    title: r.readUpdatableString(),
    avatarUrl: r.readUpdatableString(),
  );
}

void encodeDeleteChatPayload(ProtocolWriter w, DeleteChatPayload v) {
  w.writeU32(v.chatId);
}

DeleteChatPayload decodeDeleteChatPayload(ProtocolReader r) {
  return DeleteChatPayload(chatId: r.readU32());
}

void encodeGetChatInfoPayload(ProtocolWriter w, GetChatInfoPayload v) {
  w.writeU32(v.chatId);
}

GetChatInfoPayload decodeGetChatInfoPayload(ProtocolReader r) {
  return GetChatInfoPayload(chatId: r.readU32());
}

void encodeGetChatMembersPayload(ProtocolWriter w, GetChatMembersPayload v) {
  w.writeU32(v.chatId);
  w.writeU32(v.cursor);
  w.writeU16(v.limit);
}

GetChatMembersPayload decodeGetChatMembersPayload(ProtocolReader r) {
  return GetChatMembersPayload(
    chatId: r.readU32(),
    cursor: r.readU32(),
    limit: r.readU16(),
  );
}

void encodeInviteMembersPayload(ProtocolWriter w, InviteMembersPayload v) {
  w.writeU32(v.chatId);
  w.writeU16(v.userIds.length);
  for (final v in v.userIds) {
    w.writeU32(v);
  }
}

InviteMembersPayload decodeInviteMembersPayload(ProtocolReader r) {
  return InviteMembersPayload(chatId: r.readU32(), userIds: r.readVecU32());
}

void encodeLeaveChatPayload(ProtocolWriter w, LeaveChatPayload v) {
  w.writeU32(v.chatId);
}

LeaveChatPayload decodeLeaveChatPayload(ProtocolReader r) {
  return LeaveChatPayload(chatId: r.readU32());
}

void encodeUpdateMemberPayload(ProtocolWriter w, UpdateMemberPayload v) {
  w.writeU32(v.chatId);
  w.writeU32(v.userId);
  encodeMemberAction(w, v.action);
}

UpdateMemberPayload decodeUpdateMemberPayload(ProtocolReader r) {
  return UpdateMemberPayload(
    chatId: r.readU32(),
    userId: r.readU32(),
    action: decodeMemberAction(r),
  );
}

void encodeMessageDeletedPayload(ProtocolWriter w, MessageDeletedPayload v) {
  w.writeU32(v.chatId);
  w.writeU32(v.messageId);
}

MessageDeletedPayload decodeMessageDeletedPayload(ProtocolReader r) {
  return MessageDeletedPayload(chatId: r.readU32(), messageId: r.readU32());
}

void encodeReceiptUpdatePayload(ProtocolWriter w, ReceiptUpdatePayload v) {
  w.writeU32(v.chatId);
  w.writeU32(v.userId);
  w.writeU32(v.messageId);
}

ReceiptUpdatePayload decodeReceiptUpdatePayload(ProtocolReader r) {
  return ReceiptUpdatePayload(
    chatId: r.readU32(),
    userId: r.readU32(),
    messageId: r.readU32(),
  );
}

void encodeTypingUpdatePayload(ProtocolWriter w, TypingUpdatePayload v) {
  w.writeU32(v.chatId);
  w.writeU32(v.userId);
  w.writeU16(v.expiresInMs);
}

TypingUpdatePayload decodeTypingUpdatePayload(ProtocolReader r) {
  return TypingUpdatePayload(
    chatId: r.readU32(),
    userId: r.readU32(),
    expiresInMs: r.readU16(),
  );
}

void encodeMemberJoinedPayload(ProtocolWriter w, MemberJoinedPayload v) {
  w.writeU32(v.chatId);
  w.writeU32(v.userId);
  w.writeU8(v.role.value);
  w.writeU32(v.invitedBy);
}

MemberJoinedPayload decodeMemberJoinedPayload(ProtocolReader r) {
  return MemberJoinedPayload(
    chatId: r.readU32(),
    userId: r.readU32(),
    role: ChatRole.fromValue(r.readU8())!,
    invitedBy: r.readU32(),
  );
}

void encodeMemberLeftPayload(ProtocolWriter w, MemberLeftPayload v) {
  w.writeU32(v.chatId);
  w.writeU32(v.userId);
}

MemberLeftPayload decodeMemberLeftPayload(ProtocolReader r) {
  return MemberLeftPayload(chatId: r.readU32(), userId: r.readU32());
}

void encodeChatDeletedPayload(ProtocolWriter w, ChatDeletedPayload v) {
  w.writeU32(v.chatId);
}

ChatDeletedPayload decodeChatDeletedPayload(ProtocolReader r) {
  return ChatDeletedPayload(chatId: r.readU32());
}

void encodeMemberUpdatedPayload(ProtocolWriter w, MemberUpdatedPayload v) {
  w.writeU32(v.chatId);
  w.writeU32(v.userId);
  w.writeU8(v.role.value);
  if (v.permissions != null) {
    w.writeU8(1);
    w.writeU32(v.permissions!.value);
  } else {
    w.writeU8(0);
  }
}

MemberUpdatedPayload decodeMemberUpdatedPayload(ProtocolReader r) {
  return MemberUpdatedPayload(
    chatId: r.readU32(),
    userId: r.readU32(),
    role: ChatRole.fromValue(r.readU8())!,
    permissions: r.readU8() == 1 ? Permission(r.readU32()) : null,
  );
}

void encodeAddReactionPayload(ProtocolWriter w, AddReactionPayload v) {
  w.writeU32(v.chatId);
  w.writeU32(v.messageId);
  w.writeU32(v.packId);
  w.writeU8(v.emojiIndex);
}

AddReactionPayload decodeAddReactionPayload(ProtocolReader r) {
  return AddReactionPayload(
    chatId: r.readU32(),
    messageId: r.readU32(),
    packId: r.readU32(),
    emojiIndex: r.readU8(),
  );
}

void encodeRemoveReactionPayload(ProtocolWriter w, RemoveReactionPayload v) {
  w.writeU32(v.chatId);
  w.writeU32(v.messageId);
  w.writeU32(v.packId);
  w.writeU8(v.emojiIndex);
}

RemoveReactionPayload decodeRemoveReactionPayload(ProtocolReader r) {
  return RemoveReactionPayload(
    chatId: r.readU32(),
    messageId: r.readU32(),
    packId: r.readU32(),
    emojiIndex: r.readU8(),
  );
}

void encodeReactionUpdatePayload(ProtocolWriter w, ReactionUpdatePayload v) {
  w.writeU32(v.chatId);
  w.writeU32(v.messageId);
  w.writeU32(v.userId);
  w.writeU32(v.packId);
  w.writeU8(v.emojiIndex);
  w.writeU8(v.added ? 1 : 0);
}

ReactionUpdatePayload decodeReactionUpdatePayload(ProtocolReader r) {
  return ReactionUpdatePayload(
    chatId: r.readU32(),
    messageId: r.readU32(),
    userId: r.readU32(),
    packId: r.readU32(),
    emojiIndex: r.readU8(),
    added: r.readU8() != 0,
  );
}

void encodePinMessagePayload(ProtocolWriter w, PinMessagePayload v) {
  w.writeU32(v.chatId);
  w.writeU32(v.messageId);
}

PinMessagePayload decodePinMessagePayload(ProtocolReader r) {
  return PinMessagePayload(chatId: r.readU32(), messageId: r.readU32());
}

void encodeUnpinMessagePayload(ProtocolWriter w, UnpinMessagePayload v) {
  w.writeU32(v.chatId);
  w.writeU32(v.messageId);
}

UnpinMessagePayload decodeUnpinMessagePayload(ProtocolReader r) {
  return UnpinMessagePayload(chatId: r.readU32(), messageId: r.readU32());
}

void encodeRefreshTokenPayload(ProtocolWriter w, RefreshTokenPayload v) {
  w.writeString(v.token);
}

RefreshTokenPayload decodeRefreshTokenPayload(ProtocolReader r) {
  return RefreshTokenPayload(token: r.readString());
}

void encodeForwardMessagePayload(ProtocolWriter w, ForwardMessagePayload v) {
  w.writeU32(v.fromChatId);
  w.writeU32(v.messageId);
  w.writeU32(v.toChatId);
  w.writeUuid(v.idempotencyKey);
}

ForwardMessagePayload decodeForwardMessagePayload(ProtocolReader r) {
  return ForwardMessagePayload(
    fromChatId: r.readU32(),
    messageId: r.readU32(),
    toChatId: r.readU32(),
    idempotencyKey: r.readUuid(),
  );
}

void encodeGetUserPayload(ProtocolWriter w, GetUserPayload v) {
  w.writeU32(v.userId);
}

GetUserPayload decodeGetUserPayload(ProtocolReader r) {
  return GetUserPayload(userId: r.readU32());
}

void encodeGetUsersPayload(ProtocolWriter w, GetUsersPayload v) {
  w.writeU16(v.userIds.length);
  for (final v in v.userIds) {
    w.writeU32(v);
  }
}

GetUsersPayload decodeGetUsersPayload(ProtocolReader r) {
  return GetUsersPayload(userIds: r.readVecU32());
}

void encodeUpdateProfilePayload(ProtocolWriter w, UpdateProfilePayload v) {
  w.writeUpdatableString(v.username);
  w.writeUpdatableString(v.firstName);
  w.writeUpdatableString(v.lastName);
  w.writeUpdatableString(v.avatarUrl);
}

UpdateProfilePayload decodeUpdateProfilePayload(ProtocolReader r) {
  return UpdateProfilePayload(
    username: r.readUpdatableString(),
    firstName: r.readUpdatableString(),
    lastName: r.readUpdatableString(),
    avatarUrl: r.readUpdatableString(),
  );
}

void encodeBlockUserPayload(ProtocolWriter w, BlockUserPayload v) {
  w.writeU32(v.userId);
}

BlockUserPayload decodeBlockUserPayload(ProtocolReader r) {
  return BlockUserPayload(userId: r.readU32());
}

void encodeUnblockUserPayload(ProtocolWriter w, UnblockUserPayload v) {
  w.writeU32(v.userId);
}

UnblockUserPayload decodeUnblockUserPayload(ProtocolReader r) {
  return UnblockUserPayload(userId: r.readU32());
}

void encodeGetBlockListPayload(ProtocolWriter w, GetBlockListPayload v) {
  w.writeU32(v.cursor);
  w.writeU16(v.limit);
}

GetBlockListPayload decodeGetBlockListPayload(ProtocolReader r) {
  return GetBlockListPayload(cursor: r.readU32(), limit: r.readU16());
}

void encodeMuteChatPayload(ProtocolWriter w, MuteChatPayload v) {
  w.writeU32(v.chatId);
  w.writeU32(v.durationSecs);
}

MuteChatPayload decodeMuteChatPayload(ProtocolReader r) {
  return MuteChatPayload(chatId: r.readU32(), durationSecs: r.readU32());
}

void encodeUnmuteChatPayload(ProtocolWriter w, UnmuteChatPayload v) {
  w.writeU32(v.chatId);
}

UnmuteChatPayload decodeUnmuteChatPayload(ProtocolReader r) {
  return UnmuteChatPayload(chatId: r.readU32());
}

void encodeErrorPayload(ProtocolWriter w, ErrorPayload v) {
  w.writeU16(v.code.value);
  final slug = v.code.slug;
  w.writeU8(slug.length);
  for (var i = 0; i < slug.length; i++) {
    w.writeU8(slug.codeUnitAt(i));
  }
  w.writeString(v.message);
  w.writeU32(v.retryAfterMs);
  w.writeOptionalString(v.extra);
}

ErrorPayload decodeErrorPayload(ProtocolReader r) {
  final codeRaw = r.readU16();
  final code = ErrorCode.fromValue(codeRaw);
  if (code == null) throw CodecError('unknown ErrorCode: $codeRaw');
  r.skip(r.readU8());
  return ErrorPayload(
    code: code,
    message: r.readString(),
    retryAfterMs: r.readU32(),
    extra: r.readOptionalString(),
  );
}

void encodeMessage(ProtocolWriter w, Message v) {
  w.writeU32(v.id);
  w.writeU32(v.chatId);
  w.writeU32(v.senderId);
  w.writeTimestamp(v.createdAt);
  w.writeTimestamp(v.updatedAt);
  w.writeU8(v.kind.value);
  w.writeU16(v.flags.value);
  w.writeOptionU32(v.replyToId);
  w.writeString(v.content);
  if (v.richContent != null) {
    final lenOffset = w.reserve(4);
    final blobStart = w.length;
    w.writeU16(v.richContent!.length);
    for (final span in v.richContent!) {
      encodeRichSpan(w, span);
    }
    w.patchU32(lenOffset, w.length - blobStart);
  } else {
    w.writeU32(0);
  }
  w.writeOptionalString(v.extra);
}

Message decodeMessage(ProtocolReader r) {
  final id = r.readU32();
  final chatId = r.readU32();
  final senderId = r.readU32();
  final createdAt = r.readTimestamp();
  final updatedAt = r.readTimestamp();
  final kind = MessageKind.fromValue(r.readU8())!;
  final flags = MessageFlags(r.readU16());
  final replyToId = r.readOptionU32();
  final content = r.readString();
  final richLen = r.readU32();
  List<RichSpan>? richContent;
  if (richLen > 0) {
    final richData = r.readBytes(richLen);
    final rr = ProtocolReader(richData);
    richContent = rr.readArray(rr.readU16(), () => decodeRichSpan(rr));
  }
  final extra = r.readOptionalString();
  return Message(
    id: id,
    chatId: chatId,
    senderId: senderId,
    createdAt: createdAt,
    updatedAt: updatedAt,
    kind: kind,
    flags: flags,
    replyToId: replyToId,
    content: content,
    richContent: richContent,
    extra: extra,
  );
}

void encodeMessageBatch(ProtocolWriter w, MessageBatch v) {
  w.writeU8(v.hasMore ? 1 : 0);
  w.writeU32(v.messages.length);
  for (final msg in v.messages) {
    encodeMessage(w, msg);
  }
}

MessageBatch decodeMessageBatch(ProtocolReader r) {
  final hasMore = r.readU8() != 0;
  final messages = r.readArray(r.readU32(), () => decodeMessage(r));
  return MessageBatch(messages: messages, hasMore: hasMore);
}

void encodeLoadChatsPayload(ProtocolWriter w, LoadChatsPayload v) {
  switch (v) {
    case LoadChatsFirstPage p:
      w.writeU8(0);
      w.writeU16(p.limit);
    case LoadChatsAfter p:
      w.writeU8(1);
      w.writeTimestamp(p.cursorTs);
      w.writeU16(p.limit);
  }
}

LoadChatsPayload decodeLoadChatsPayload(ProtocolReader r) {
  final d = r.readU8();
  return switch (d) {
    0 => LoadChatsFirstPage(limit: r.readU16()),
    1 => LoadChatsAfter(cursorTs: r.readTimestamp(), limit: r.readU16()),
    _ => throw CodecError('unknown LoadChatsPayload discriminant: $d'),
  };
}

void encodeSearchScope(ProtocolWriter w, SearchScope v) {
  switch (v) {
    case SearchScopeChat p:
      w.writeU8(0);
      w.writeU32(p.chatId);
    case SearchScopeGlobal():
      w.writeU8(1);
    case SearchScopeUser p:
      w.writeU8(2);
      w.writeU32(p.userId);
  }
}

SearchScope decodeSearchScope(ProtocolReader r) {
  final d = r.readU8();
  return switch (d) {
    0 => SearchScopeChat(chatId: r.readU32()),
    1 => SearchScopeGlobal(),
    2 => SearchScopeUser(userId: r.readU32()),
    _ => throw CodecError('unknown SearchScope discriminant: $d'),
  };
}

void encodeLoadMessagesPayload(ProtocolWriter w, LoadMessagesPayload v) {
  switch (v) {
    case LoadMessagesPaginate p:
      w.writeU32(p.chatId);
      w.writeU8(0);
      w.writeU8(p.direction.value);
      w.writeU32(p.anchorId);
      w.writeU16(p.limit);
    case LoadMessagesRangeCheck p:
      w.writeU32(p.chatId);
      w.writeU8(1);
      w.writeU32(p.fromId);
      w.writeU32(p.toId);
      w.writeTimestamp(p.sinceTs);
  }
}

LoadMessagesPayload decodeLoadMessagesPayload(ProtocolReader r) {
  final chatId = r.readU32();
  final d = r.readU8();
  return switch (d) {
    0 => LoadMessagesPaginate(
      chatId: chatId,
      direction: LoadDirection.fromValue(r.readU8())!,
      anchorId: r.readU32(),
      limit: r.readU16(),
    ),
    1 => LoadMessagesRangeCheck(
      chatId: chatId,
      fromId: r.readU32(),
      toId: r.readU32(),
      sinceTs: r.readTimestamp(),
    ),
    _ => throw CodecError('unknown LoadMessagesPayload mode: $d'),
  };
}

void encodeMemberAction(ProtocolWriter w, MemberAction v) {
  switch (v) {
    case MemberActionKick():
      w.writeU8(0);
    case MemberActionBan():
      w.writeU8(1);
    case MemberActionMute p:
      w.writeU8(2);
      w.writeU32(p.durationSecs);
    case MemberActionChangeRole p:
      w.writeU8(3);
      w.writeU8(p.chatRole.value);
    case MemberActionUpdatePermissions p:
      w.writeU8(4);
      w.writeU32(p.permission.value);
    case MemberActionUnban():
      w.writeU8(5);
  }
}

MemberAction decodeMemberAction(ProtocolReader r) {
  final d = r.readU8();
  return switch (d) {
    0 => MemberActionKick(),
    1 => MemberActionBan(),
    2 => MemberActionMute(durationSecs: r.readU32()),
    3 => MemberActionChangeRole(chatRole: ChatRole.fromValue(r.readU8())!),
    4 => MemberActionUpdatePermissions(permission: Permission(r.readU32())),
    5 => MemberActionUnban(),
    _ => throw CodecError('unknown MemberAction discriminant: $d'),
  };
}
