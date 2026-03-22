// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import { CodecError } from './error.js';
import { ProtocolReader } from './reader.js';
import { ProtocolWriter } from './writer.js';

import { chatKindFromValue } from '../types/chat-kind.js';
import { chatRoleFromValue } from '../types/chat-role.js';
import { errorCodeFromValue } from '../types/error-code.js';
import { loadDirectionFromValue } from '../types/load-direction.js';
import { messageKindFromValue } from '../types/message-kind.js';
import { presenceStatusFromValue } from '../types/presence-status.js';
import { errorCodeSlug } from '../types/error-code.js';
import type { LastMessagePreview } from '../types/last-message-preview.js';
import type { MemberAction } from '../types/member-action.js';
import type { Message } from '../types/message.js';
import type { RichSpan } from '../types/rich-span.js';
import type { SearchScope } from '../types/search-scope.js';
import type { ServerLimits } from '../types/server-limits.js';
import type { AddReactionPayload } from '../types/add-reaction-payload.js';
import type { BlockUserPayload } from '../types/block-user-payload.js';
import type { ChatDeletedPayload } from '../types/chat-deleted-payload.js';
import type { ChatEntry } from '../types/chat-entry.js';
import type { ChatMemberEntry } from '../types/chat-member-entry.js';
import type { CreateChatPayload } from '../types/create-chat-payload.js';
import type { DeleteChatPayload } from '../types/delete-chat-payload.js';
import type { DeleteMessagePayload } from '../types/delete-message-payload.js';
import type { EditMessagePayload } from '../types/edit-message-payload.js';
import type { ErrorPayload } from '../types/error-payload.js';
import type { ForwardMessagePayload } from '../types/forward-message-payload.js';
import type { GetBlockListPayload } from '../types/get-block-list-payload.js';
import type { GetChatInfoPayload } from '../types/get-chat-info-payload.js';
import type { GetChatMembersPayload } from '../types/get-chat-members-payload.js';
import type { GetPresencePayload } from '../types/get-presence-payload.js';
import type { GetUserPayload } from '../types/get-user-payload.js';
import type { GetUsersPayload } from '../types/get-users-payload.js';
import type { HelloPayload } from '../types/hello-payload.js';
import type { InviteMembersPayload } from '../types/invite-members-payload.js';
import type { LeaveChatPayload } from '../types/leave-chat-payload.js';
import type { MemberJoinedPayload } from '../types/member-joined-payload.js';
import type { MemberLeftPayload } from '../types/member-left-payload.js';
import type { MemberUpdatedPayload } from '../types/member-updated-payload.js';
import type { MessageBatch } from '../types/message-batch.js';
import type { MessageDeletedPayload } from '../types/message-deleted-payload.js';
import type { MuteChatPayload } from '../types/mute-chat-payload.js';
import type { PinMessagePayload } from '../types/pin-message-payload.js';
import type { PresenceEntry } from '../types/presence-entry.js';
import type { ReactionUpdatePayload } from '../types/reaction-update-payload.js';
import type { ReadReceiptPayload } from '../types/read-receipt-payload.js';
import type { ReceiptUpdatePayload } from '../types/receipt-update-payload.js';
import type { RefreshTokenPayload } from '../types/refresh-token-payload.js';
import type { RemoveReactionPayload } from '../types/remove-reaction-payload.js';
import type { SearchPayload } from '../types/search-payload.js';
import type { SendMessagePayload } from '../types/send-message-payload.js';
import type { SubscribePayload } from '../types/subscribe-payload.js';
import type { TypingPayload } from '../types/typing-payload.js';
import type { TypingUpdatePayload } from '../types/typing-update-payload.js';
import type { UnblockUserPayload } from '../types/unblock-user-payload.js';
import type { UnmuteChatPayload } from '../types/unmute-chat-payload.js';
import type { UnpinMessagePayload } from '../types/unpin-message-payload.js';
import type { UnsubscribePayload } from '../types/unsubscribe-payload.js';
import type { UpdateChatPayload } from '../types/update-chat-payload.js';
import type { UpdateMemberPayload } from '../types/update-member-payload.js';
import type { UpdateProfilePayload } from '../types/update-profile-payload.js';
import type { UserEntry } from '../types/user-entry.js';
import type { WelcomePayload } from '../types/welcome-payload.js';
import type { LoadChatsPayload } from '../types/load-chats-payload.js';
import type { LoadMessagesPayload } from '../types/load-messages-payload.js';

export function encodeLastMessagePreview(w: ProtocolWriter, v: LastMessagePreview): void {
  w.writeU32(v.id);
  w.writeU32(v.senderId);
  w.writeTimestamp(v.createdAt);
  w.writeU8(v.kind);
  w.writeU16(v.flags);
  w.writeString(v.contentPreview);
}

export function decodeLastMessagePreview(r: ProtocolReader): LastMessagePreview {
  return {
    id: r.readU32(),
    senderId: r.readU32(),
    createdAt: r.readTimestamp(),
    kind: r.readEnum(r.readU8(), messageKindFromValue, 'MessageKind'),
    flags: r.readU16(),
    contentPreview: r.readString(),
  };
}

export function encodeChatEntry(w: ProtocolWriter, v: ChatEntry): void {
  w.writeU32(v.id);
  w.writeU8(v.kind);
  w.writeOptionU32(v.parentId);
  w.writeTimestamp(v.createdAt);
  w.writeTimestamp(v.updatedAt);
  w.writeOptionalString(v.title);
  w.writeOptionalString(v.avatarUrl);
  if (v.lastMessage !== null) { w.writeU8(1); encodeLastMessagePreview(w, v.lastMessage); } else { w.writeU8(0); }
  w.writeU32(v.unreadCount);
  w.writeU32(v.memberCount);
}

export function decodeChatEntry(r: ProtocolReader): ChatEntry {
  return {
    id: r.readU32(),
    kind: r.readEnum(r.readU8(), chatKindFromValue, 'ChatKind'),
    parentId: r.readOptionU32(),
    createdAt: r.readTimestamp(),
    updatedAt: r.readTimestamp(),
    title: r.readOptionalString(),
    avatarUrl: r.readOptionalString(),
    lastMessage: r.readU8() === 1 ? decodeLastMessagePreview(r) : null,
    unreadCount: r.readU32(),
    memberCount: r.readU32(),
  };
}

export function encodeChatMemberEntry(w: ProtocolWriter, v: ChatMemberEntry): void {
  w.writeU32(v.userId);
  w.writeU8(v.role);
  if (v.permissions !== null) { w.writeU8(1); w.writeU32(v.permissions); } else { w.writeU8(0); }
}

export function decodeChatMemberEntry(r: ProtocolReader): ChatMemberEntry {
  return {
    userId: r.readU32(),
    role: r.readEnum(r.readU8(), chatRoleFromValue, 'ChatRole'),
    permissions: r.readU8() === 1 ? r.readU32() : null,
  };
}

export function encodeRichSpan(w: ProtocolWriter, v: RichSpan): void {
  w.writeU32(v.start);
  w.writeU32(v.end);
  w.writeU16(v.style);
  w.writeOptionalString(v.meta);
}

export function decodeRichSpan(r: ProtocolReader): RichSpan {
  return {
    start: r.readU32(),
    end: r.readU32(),
    style: r.readU16(),
    meta: r.readOptionalString(),
  };
}

export function encodeUserEntry(w: ProtocolWriter, v: UserEntry): void {
  w.writeU32(v.id);
  w.writeU16(v.flags);
  w.writeTimestamp(v.createdAt);
  w.writeTimestamp(v.updatedAt);
  w.writeOptionalString(v.username);
  w.writeOptionalString(v.firstName);
  w.writeOptionalString(v.lastName);
  w.writeOptionalString(v.avatarUrl);
}

export function decodeUserEntry(r: ProtocolReader): UserEntry {
  return {
    id: r.readU32(),
    flags: r.readU16(),
    createdAt: r.readTimestamp(),
    updatedAt: r.readTimestamp(),
    username: r.readOptionalString(),
    firstName: r.readOptionalString(),
    lastName: r.readOptionalString(),
    avatarUrl: r.readOptionalString(),
  };
}

export function encodePresenceEntry(w: ProtocolWriter, v: PresenceEntry): void {
  w.writeU32(v.userId);
  w.writeU8(v.status);
  w.writeTimestamp(v.lastSeen);
}

export function decodePresenceEntry(r: ProtocolReader): PresenceEntry {
  return {
    userId: r.readU32(),
    status: r.readEnum(r.readU8(), presenceStatusFromValue, 'PresenceStatus'),
    lastSeen: r.readTimestamp(),
  };
}

export function encodeHelloPayload(w: ProtocolWriter, v: HelloPayload): void {
  w.writeU8(v.protocolVersion);
  w.writeString(v.sdkVersion);
  w.writeString(v.platform);
  w.writeString(v.token);
  w.writeUuid(v.deviceId);
}

export function decodeHelloPayload(r: ProtocolReader): HelloPayload {
  return {
    protocolVersion: r.readU8(),
    sdkVersion: r.readString(),
    platform: r.readString(),
    token: r.readString(),
    deviceId: r.readUuid(),
  };
}

export function encodeWelcomePayload(w: ProtocolWriter, v: WelcomePayload): void {
  w.writeU32(v.sessionId);
  w.writeTimestamp(v.serverTime);
  w.writeU32(v.userId);
  encodeServerLimits(w, v.limits);
  w.writeU32(v.capabilities);
}

export function decodeWelcomePayload(r: ProtocolReader): WelcomePayload {
  return {
    sessionId: r.readU32(),
    serverTime: r.readTimestamp(),
    userId: r.readU32(),
    limits: decodeServerLimits(r),
    capabilities: r.readU32(),
  };
}

export function encodeServerLimits(w: ProtocolWriter, v: ServerLimits): void {
  w.writeU32(v.pingIntervalMs);
  w.writeU32(v.pingTimeoutMs);
  w.writeU32(v.maxMessageSize);
  w.writeU32(v.maxExtraSize);
  w.writeU32(v.maxFrameSize);
  w.writeU16(v.messagesPerSec);
  w.writeU16(v.connectionsPerIp);
}

export function decodeServerLimits(r: ProtocolReader): ServerLimits {
  return {
    pingIntervalMs: r.readU32(),
    pingTimeoutMs: r.readU32(),
    maxMessageSize: r.readU32(),
    maxExtraSize: r.readU32(),
    maxFrameSize: r.readU32(),
    messagesPerSec: r.readU16(),
    connectionsPerIp: r.readU16(),
  };
}

export function encodeSendMessagePayload(w: ProtocolWriter, v: SendMessagePayload): void {
  w.writeU32(v.chatId);
  w.writeU8(v.kind);
  w.writeUuid(v.idempotencyKey);
  w.writeOptionU32(v.replyToId);
  w.writeString(v.content);
  w.writeOptionalBytes(v.richContent);
  w.writeOptionalString(v.extra);
  w.writeU16(v.mentionedUserIds.length);
  for (const _v of v.mentionedUserIds) w.writeU32(_v);
}

export function decodeSendMessagePayload(r: ProtocolReader): SendMessagePayload {
  return {
    chatId: r.readU32(),
    kind: r.readEnum(r.readU8(), messageKindFromValue, 'MessageKind'),
    idempotencyKey: r.readUuid(),
    replyToId: r.readOptionU32(),
    content: r.readString(),
    richContent: r.readOptionalBytes(),
    extra: r.readOptionalString(),
    mentionedUserIds: r.readVecU32(),
  };
}

export function encodeEditMessagePayload(w: ProtocolWriter, v: EditMessagePayload): void {
  w.writeU32(v.chatId);
  w.writeU32(v.messageId);
  w.writeString(v.content);
  w.writeOptionalBytes(v.richContent);
  w.writeOptionalString(v.extra);
}

export function decodeEditMessagePayload(r: ProtocolReader): EditMessagePayload {
  return {
    chatId: r.readU32(),
    messageId: r.readU32(),
    content: r.readString(),
    richContent: r.readOptionalBytes(),
    extra: r.readOptionalString(),
  };
}

export function encodeDeleteMessagePayload(w: ProtocolWriter, v: DeleteMessagePayload): void {
  w.writeU32(v.chatId);
  w.writeU32(v.messageId);
}

export function decodeDeleteMessagePayload(r: ProtocolReader): DeleteMessagePayload {
  return {
    chatId: r.readU32(),
    messageId: r.readU32(),
  };
}

export function encodeReadReceiptPayload(w: ProtocolWriter, v: ReadReceiptPayload): void {
  w.writeU32(v.chatId);
  w.writeU32(v.messageId);
}

export function decodeReadReceiptPayload(r: ProtocolReader): ReadReceiptPayload {
  return {
    chatId: r.readU32(),
    messageId: r.readU32(),
  };
}

export function encodeTypingPayload(w: ProtocolWriter, v: TypingPayload): void {
  w.writeU32(v.chatId);
  w.writeU16(v.expiresInMs);
}

export function decodeTypingPayload(r: ProtocolReader): TypingPayload {
  return {
    chatId: r.readU32(),
    expiresInMs: r.readU16(),
  };
}

export function encodeGetPresencePayload(w: ProtocolWriter, v: GetPresencePayload): void {
  w.writeU16(v.userIds.length);
  for (const _v of v.userIds) w.writeU32(_v);
}

export function decodeGetPresencePayload(r: ProtocolReader): GetPresencePayload {
  return {
    userIds: r.readVecU32(),
  };
}

export function encodeSearchPayload(w: ProtocolWriter, v: SearchPayload): void {
  encodeSearchScope(w, v.scope);
  w.writeString(v.query);
  w.writeU32(v.cursor);
  w.writeU16(v.limit);
}

export function decodeSearchPayload(r: ProtocolReader): SearchPayload {
  return {
    scope: decodeSearchScope(r),
    query: r.readString(),
    cursor: r.readU32(),
    limit: r.readU16(),
  };
}

export function encodeSubscribePayload(w: ProtocolWriter, v: SubscribePayload): void {
  w.writeU16(v.channels.length);
  for (const _v of v.channels) w.writeString(_v);
}

export function decodeSubscribePayload(r: ProtocolReader): SubscribePayload {
  return {
    channels: r.readVecString(),
  };
}

export function encodeUnsubscribePayload(w: ProtocolWriter, v: UnsubscribePayload): void {
  w.writeU16(v.channels.length);
  for (const _v of v.channels) w.writeString(_v);
}

export function decodeUnsubscribePayload(r: ProtocolReader): UnsubscribePayload {
  return {
    channels: r.readVecString(),
  };
}

export function encodeCreateChatPayload(w: ProtocolWriter, v: CreateChatPayload): void {
  w.writeU8(v.kind);
  w.writeOptionU32(v.parentId);
  w.writeOptionalString(v.title);
  w.writeOptionalString(v.avatarUrl);
  w.writeU16(v.memberIds.length);
  for (const _v of v.memberIds) w.writeU32(_v);
}

export function decodeCreateChatPayload(r: ProtocolReader): CreateChatPayload {
  return {
    kind: r.readEnum(r.readU8(), chatKindFromValue, 'ChatKind'),
    parentId: r.readOptionU32(),
    title: r.readOptionalString(),
    avatarUrl: r.readOptionalString(),
    memberIds: r.readVecU32(),
  };
}

export function encodeUpdateChatPayload(w: ProtocolWriter, v: UpdateChatPayload): void {
  w.writeU32(v.chatId);
  w.writeUpdatableString(v.title);
  w.writeUpdatableString(v.avatarUrl);
}

export function decodeUpdateChatPayload(r: ProtocolReader): UpdateChatPayload {
  return {
    chatId: r.readU32(),
    title: r.readUpdatableString(),
    avatarUrl: r.readUpdatableString(),
  };
}

export function encodeDeleteChatPayload(w: ProtocolWriter, v: DeleteChatPayload): void {
  w.writeU32(v.chatId);
}

export function decodeDeleteChatPayload(r: ProtocolReader): DeleteChatPayload {
  return {
    chatId: r.readU32(),
  };
}

export function encodeGetChatInfoPayload(w: ProtocolWriter, v: GetChatInfoPayload): void {
  w.writeU32(v.chatId);
}

export function decodeGetChatInfoPayload(r: ProtocolReader): GetChatInfoPayload {
  return {
    chatId: r.readU32(),
  };
}

export function encodeGetChatMembersPayload(w: ProtocolWriter, v: GetChatMembersPayload): void {
  w.writeU32(v.chatId);
  w.writeU32(v.cursor);
  w.writeU16(v.limit);
}

export function decodeGetChatMembersPayload(r: ProtocolReader): GetChatMembersPayload {
  return {
    chatId: r.readU32(),
    cursor: r.readU32(),
    limit: r.readU16(),
  };
}

export function encodeInviteMembersPayload(w: ProtocolWriter, v: InviteMembersPayload): void {
  w.writeU32(v.chatId);
  w.writeU16(v.userIds.length);
  for (const _v of v.userIds) w.writeU32(_v);
}

export function decodeInviteMembersPayload(r: ProtocolReader): InviteMembersPayload {
  return {
    chatId: r.readU32(),
    userIds: r.readVecU32(),
  };
}

export function encodeLeaveChatPayload(w: ProtocolWriter, v: LeaveChatPayload): void {
  w.writeU32(v.chatId);
}

export function decodeLeaveChatPayload(r: ProtocolReader): LeaveChatPayload {
  return {
    chatId: r.readU32(),
  };
}

export function encodeUpdateMemberPayload(w: ProtocolWriter, v: UpdateMemberPayload): void {
  w.writeU32(v.chatId);
  w.writeU32(v.userId);
  encodeMemberAction(w, v.action);
}

export function decodeUpdateMemberPayload(r: ProtocolReader): UpdateMemberPayload {
  return {
    chatId: r.readU32(),
    userId: r.readU32(),
    action: decodeMemberAction(r),
  };
}

export function encodeMessageDeletedPayload(w: ProtocolWriter, v: MessageDeletedPayload): void {
  w.writeU32(v.chatId);
  w.writeU32(v.messageId);
}

export function decodeMessageDeletedPayload(r: ProtocolReader): MessageDeletedPayload {
  return {
    chatId: r.readU32(),
    messageId: r.readU32(),
  };
}

export function encodeReceiptUpdatePayload(w: ProtocolWriter, v: ReceiptUpdatePayload): void {
  w.writeU32(v.chatId);
  w.writeU32(v.userId);
  w.writeU32(v.messageId);
}

export function decodeReceiptUpdatePayload(r: ProtocolReader): ReceiptUpdatePayload {
  return {
    chatId: r.readU32(),
    userId: r.readU32(),
    messageId: r.readU32(),
  };
}

export function encodeTypingUpdatePayload(w: ProtocolWriter, v: TypingUpdatePayload): void {
  w.writeU32(v.chatId);
  w.writeU32(v.userId);
  w.writeU16(v.expiresInMs);
}

export function decodeTypingUpdatePayload(r: ProtocolReader): TypingUpdatePayload {
  return {
    chatId: r.readU32(),
    userId: r.readU32(),
    expiresInMs: r.readU16(),
  };
}

export function encodeMemberJoinedPayload(w: ProtocolWriter, v: MemberJoinedPayload): void {
  w.writeU32(v.chatId);
  w.writeU32(v.userId);
  w.writeU8(v.role);
  w.writeU32(v.invitedBy);
}

export function decodeMemberJoinedPayload(r: ProtocolReader): MemberJoinedPayload {
  return {
    chatId: r.readU32(),
    userId: r.readU32(),
    role: r.readEnum(r.readU8(), chatRoleFromValue, 'ChatRole'),
    invitedBy: r.readU32(),
  };
}

export function encodeMemberLeftPayload(w: ProtocolWriter, v: MemberLeftPayload): void {
  w.writeU32(v.chatId);
  w.writeU32(v.userId);
}

export function decodeMemberLeftPayload(r: ProtocolReader): MemberLeftPayload {
  return {
    chatId: r.readU32(),
    userId: r.readU32(),
  };
}

export function encodeChatDeletedPayload(w: ProtocolWriter, v: ChatDeletedPayload): void {
  w.writeU32(v.chatId);
}

export function decodeChatDeletedPayload(r: ProtocolReader): ChatDeletedPayload {
  return {
    chatId: r.readU32(),
  };
}

export function encodeMemberUpdatedPayload(w: ProtocolWriter, v: MemberUpdatedPayload): void {
  w.writeU32(v.chatId);
  w.writeU32(v.userId);
  w.writeU8(v.role);
  if (v.permissions !== null) { w.writeU8(1); w.writeU32(v.permissions); } else { w.writeU8(0); }
}

export function decodeMemberUpdatedPayload(r: ProtocolReader): MemberUpdatedPayload {
  return {
    chatId: r.readU32(),
    userId: r.readU32(),
    role: r.readEnum(r.readU8(), chatRoleFromValue, 'ChatRole'),
    permissions: r.readU8() === 1 ? r.readU32() : null,
  };
}

export function encodeAddReactionPayload(w: ProtocolWriter, v: AddReactionPayload): void {
  w.writeU32(v.chatId);
  w.writeU32(v.messageId);
  w.writeU32(v.packId);
  w.writeU8(v.emojiIndex);
}

export function decodeAddReactionPayload(r: ProtocolReader): AddReactionPayload {
  return {
    chatId: r.readU32(),
    messageId: r.readU32(),
    packId: r.readU32(),
    emojiIndex: r.readU8(),
  };
}

export function encodeRemoveReactionPayload(w: ProtocolWriter, v: RemoveReactionPayload): void {
  w.writeU32(v.chatId);
  w.writeU32(v.messageId);
  w.writeU32(v.packId);
  w.writeU8(v.emojiIndex);
}

export function decodeRemoveReactionPayload(r: ProtocolReader): RemoveReactionPayload {
  return {
    chatId: r.readU32(),
    messageId: r.readU32(),
    packId: r.readU32(),
    emojiIndex: r.readU8(),
  };
}

export function encodeReactionUpdatePayload(w: ProtocolWriter, v: ReactionUpdatePayload): void {
  w.writeU32(v.chatId);
  w.writeU32(v.messageId);
  w.writeU32(v.userId);
  w.writeU32(v.packId);
  w.writeU8(v.emojiIndex);
  w.writeU8(v.added ? 1 : 0);
}

export function decodeReactionUpdatePayload(r: ProtocolReader): ReactionUpdatePayload {
  return {
    chatId: r.readU32(),
    messageId: r.readU32(),
    userId: r.readU32(),
    packId: r.readU32(),
    emojiIndex: r.readU8(),
    added: r.readU8() !== 0,
  };
}

export function encodePinMessagePayload(w: ProtocolWriter, v: PinMessagePayload): void {
  w.writeU32(v.chatId);
  w.writeU32(v.messageId);
}

export function decodePinMessagePayload(r: ProtocolReader): PinMessagePayload {
  return {
    chatId: r.readU32(),
    messageId: r.readU32(),
  };
}

export function encodeUnpinMessagePayload(w: ProtocolWriter, v: UnpinMessagePayload): void {
  w.writeU32(v.chatId);
  w.writeU32(v.messageId);
}

export function decodeUnpinMessagePayload(r: ProtocolReader): UnpinMessagePayload {
  return {
    chatId: r.readU32(),
    messageId: r.readU32(),
  };
}

export function encodeRefreshTokenPayload(w: ProtocolWriter, v: RefreshTokenPayload): void {
  w.writeString(v.token);
}

export function decodeRefreshTokenPayload(r: ProtocolReader): RefreshTokenPayload {
  return {
    token: r.readString(),
  };
}

export function encodeForwardMessagePayload(w: ProtocolWriter, v: ForwardMessagePayload): void {
  w.writeU32(v.fromChatId);
  w.writeU32(v.messageId);
  w.writeU32(v.toChatId);
  w.writeUuid(v.idempotencyKey);
}

export function decodeForwardMessagePayload(r: ProtocolReader): ForwardMessagePayload {
  return {
    fromChatId: r.readU32(),
    messageId: r.readU32(),
    toChatId: r.readU32(),
    idempotencyKey: r.readUuid(),
  };
}

export function encodeGetUserPayload(w: ProtocolWriter, v: GetUserPayload): void {
  w.writeU32(v.userId);
}

export function decodeGetUserPayload(r: ProtocolReader): GetUserPayload {
  return {
    userId: r.readU32(),
  };
}

export function encodeGetUsersPayload(w: ProtocolWriter, v: GetUsersPayload): void {
  w.writeU16(v.userIds.length);
  for (const _v of v.userIds) w.writeU32(_v);
}

export function decodeGetUsersPayload(r: ProtocolReader): GetUsersPayload {
  return {
    userIds: r.readVecU32(),
  };
}

export function encodeUpdateProfilePayload(w: ProtocolWriter, v: UpdateProfilePayload): void {
  w.writeUpdatableString(v.username);
  w.writeUpdatableString(v.firstName);
  w.writeUpdatableString(v.lastName);
  w.writeUpdatableString(v.avatarUrl);
}

export function decodeUpdateProfilePayload(r: ProtocolReader): UpdateProfilePayload {
  return {
    username: r.readUpdatableString(),
    firstName: r.readUpdatableString(),
    lastName: r.readUpdatableString(),
    avatarUrl: r.readUpdatableString(),
  };
}

export function encodeBlockUserPayload(w: ProtocolWriter, v: BlockUserPayload): void {
  w.writeU32(v.userId);
}

export function decodeBlockUserPayload(r: ProtocolReader): BlockUserPayload {
  return {
    userId: r.readU32(),
  };
}

export function encodeUnblockUserPayload(w: ProtocolWriter, v: UnblockUserPayload): void {
  w.writeU32(v.userId);
}

export function decodeUnblockUserPayload(r: ProtocolReader): UnblockUserPayload {
  return {
    userId: r.readU32(),
  };
}

export function encodeGetBlockListPayload(w: ProtocolWriter, v: GetBlockListPayload): void {
  w.writeU32(v.cursor);
  w.writeU16(v.limit);
}

export function decodeGetBlockListPayload(r: ProtocolReader): GetBlockListPayload {
  return {
    cursor: r.readU32(),
    limit: r.readU16(),
  };
}

export function encodeMuteChatPayload(w: ProtocolWriter, v: MuteChatPayload): void {
  w.writeU32(v.chatId);
  w.writeU32(v.durationSecs);
}

export function decodeMuteChatPayload(r: ProtocolReader): MuteChatPayload {
  return {
    chatId: r.readU32(),
    durationSecs: r.readU32(),
  };
}

export function encodeUnmuteChatPayload(w: ProtocolWriter, v: UnmuteChatPayload): void {
  w.writeU32(v.chatId);
}

export function decodeUnmuteChatPayload(r: ProtocolReader): UnmuteChatPayload {
  return {
    chatId: r.readU32(),
  };
}

export function encodeErrorPayload(w: ProtocolWriter, v: ErrorPayload): void {
  w.writeU16(v.code);
  const slug = errorCodeSlug(v.code);
  w.writeU8(slug.length);
  for (let i = 0; i < slug.length; i++) w.writeU8(slug.charCodeAt(i));
  w.writeString(v.message);
  w.writeU32(v.retryAfterMs);
  w.writeOptionalString(v.extra);
}

export function decodeErrorPayload(r: ProtocolReader): ErrorPayload {
  const codeRaw = r.readU16();
  const code = r.readEnum(codeRaw, errorCodeFromValue, 'ErrorCode');
  r.skip(r.readU8());
  return {
    code,
    message: r.readString(),
    retryAfterMs: r.readU32(),
    extra: r.readOptionalString(),
  };
}

export function encodeMessage(w: ProtocolWriter, v: Message): void {
  w.writeU32(v.id);
  w.writeU32(v.chatId);
  w.writeU32(v.senderId);
  w.writeTimestamp(v.createdAt);
  w.writeTimestamp(v.updatedAt);
  w.writeU8(v.kind);
  w.writeU16(v.flags);
  w.writeOptionU32(v.replyToId);
  w.writeString(v.content);
  if (v.richContent !== null) {
    const tmp = new ProtocolWriter();
    tmp.writeU16(v.richContent.length);
    for (const span of v.richContent) encodeRichSpan(tmp, span);
    const blob = tmp.toBytes();
    w.writeU32(blob.length);
    w.writeRawBytes(blob);
  } else {
    w.writeU32(0);
  }
  w.writeOptionalString(v.extra);
}

export function decodeMessage(r: ProtocolReader): Message {
  const id = r.readU32();
  const chatId = r.readU32();
  const senderId = r.readU32();
  const createdAt = r.readTimestamp();
  const updatedAt = r.readTimestamp();
  const kind = r.readEnum(r.readU8(), messageKindFromValue, 'MessageKind');
  const flags = r.readU16();
  const replyToId = r.readOptionU32();
  const content = r.readString();
  const richLen = r.readU32();
  let richContent: RichSpan[] | null = null;
  if (richLen > 0) {
    const richData = r.readBytes(richLen);
    const rr = new ProtocolReader(richData);
    richContent = rr.readArray(rr.readU16(), () => decodeRichSpan(rr));
  }
  const extra = r.readOptionalString();
  return { id, chatId, senderId, createdAt, updatedAt, kind, flags, replyToId, content, richContent, extra };
}

export function encodeMessageBatch(w: ProtocolWriter, v: MessageBatch): void {
  w.writeU8(v.hasMore ? 1 : 0);
  w.writeU32(v.messages.length);
  for (const msg of v.messages) encodeMessage(w, msg);
}

export function decodeMessageBatch(r: ProtocolReader): MessageBatch {
  const hasMore = r.readU8() !== 0;
  const messages = r.readArray(r.readU32(), () => decodeMessage(r));
  return { messages, hasMore };
}

export function encodeLoadChatsPayload(w: ProtocolWriter, v: LoadChatsPayload): void {
  switch (v.type) {
    case 'firstPage':
      w.writeU8(0);
      w.writeU16(v.limit);
      break;
    case 'after':
      w.writeU8(1);
      w.writeTimestamp(v.cursorTs);
      w.writeU16(v.limit);
      break;
  }
}

export function decodeLoadChatsPayload(r: ProtocolReader): LoadChatsPayload {
  const _d = r.readU8();
  switch (_d) {
    case 0:
      return { type: 'firstPage', limit: r.readU16() };
    case 1:
      return { type: 'after', cursorTs: r.readTimestamp(), limit: r.readU16() };
    default: throw new CodecError(`unknown LoadChatsPayload discriminant: ${_d}`);
  }
}

export function encodeSearchScope(w: ProtocolWriter, v: SearchScope): void {
  switch (v.type) {
    case 'chat':
      w.writeU8(0);
      w.writeU32(v.chatId);
      break;
    case 'global':
      w.writeU8(1);
      break;
    case 'user':
      w.writeU8(2);
      w.writeU32(v.userId);
      break;
  }
}

export function decodeSearchScope(r: ProtocolReader): SearchScope {
  const _d = r.readU8();
  switch (_d) {
    case 0:
      return { type: 'chat', chatId: r.readU32() };
    case 1:
      return { type: 'global' };
    case 2:
      return { type: 'user', userId: r.readU32() };
    default: throw new CodecError(`unknown SearchScope discriminant: ${_d}`);
  }
}

export function encodeLoadMessagesPayload(w: ProtocolWriter, v: LoadMessagesPayload): void {
  switch (v.type) {
    case 'paginate':
      w.writeU32(v.chatId);
      w.writeU8(0);
      w.writeU8(v.direction);
      w.writeU32(v.anchorId);
      w.writeU16(v.limit);
      break;
    case 'rangeCheck':
      w.writeU32(v.chatId);
      w.writeU8(1);
      w.writeU32(v.fromId);
      w.writeU32(v.toId);
      w.writeTimestamp(v.sinceTs);
      break;
  }
}

export function decodeLoadMessagesPayload(r: ProtocolReader): LoadMessagesPayload {
  const chatId = r.readU32();
  const _d = r.readU8();
  switch (_d) {
    case 0:
      return { type: 'paginate', chatId: chatId, direction: r.readEnum(r.readU8(), loadDirectionFromValue, 'LoadDirection'), anchorId: r.readU32(), limit: r.readU16() };
    case 1:
      return { type: 'rangeCheck', chatId: chatId, fromId: r.readU32(), toId: r.readU32(), sinceTs: r.readTimestamp() };
    default: throw new CodecError(`unknown LoadMessagesPayload mode: ${_d}`);
  }
}

export function encodeMemberAction(w: ProtocolWriter, v: MemberAction): void {
  switch (v.type) {
    case 'kick':
      w.writeU8(0);
      break;
    case 'ban':
      w.writeU8(1);
      break;
    case 'mute':
      w.writeU8(2);
      w.writeU32(v.durationSecs);
      break;
    case 'changeRole':
      w.writeU8(3);
      w.writeU8(v.chatRole);
      break;
    case 'updatePermissions':
      w.writeU8(4);
      w.writeU32(v.permission);
      break;
    case 'unban':
      w.writeU8(5);
      break;
  }
}

export function decodeMemberAction(r: ProtocolReader): MemberAction {
  const _d = r.readU8();
  switch (_d) {
    case 0:
      return { type: 'kick' };
    case 1:
      return { type: 'ban' };
    case 2:
      return { type: 'mute', durationSecs: r.readU32() };
    case 3:
      return { type: 'changeRole', chatRole: r.readEnum(r.readU8(), chatRoleFromValue, 'ChatRole') };
    case 4:
      return { type: 'updatePermissions', permission: r.readU32() };
    case 5:
      return { type: 'unban' };
    default: throw new CodecError(`unknown MemberAction discriminant: ${_d}`);
  }
}

