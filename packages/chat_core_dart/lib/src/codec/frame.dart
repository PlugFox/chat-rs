// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'dart:typed_data';

import 'package:chat_core/chat_core.dart';

class FrameHeader {
  const FrameHeader({
    required this.kind,
    required this.seq,
    required this.eventSeq,
  });
  final FrameKind kind;
  final int seq;
  final int eventSeq;
}

sealed class FramePayload {
  const FramePayload();
}

class FramePayloadHello extends FramePayload {
  const FramePayloadHello(this.data);
  final HelloPayload data;
}

class FramePayloadWelcome extends FramePayload {
  const FramePayloadWelcome(this.data);
  final WelcomePayload data;
}

class FramePayloadPing extends FramePayload {
  const FramePayloadPing();
}

class FramePayloadPong extends FramePayload {
  const FramePayloadPong();
}

class FramePayloadRefreshToken extends FramePayload {
  const FramePayloadRefreshToken(this.data);
  final RefreshTokenPayload data;
}

class FramePayloadSendMessage extends FramePayload {
  const FramePayloadSendMessage(this.data);
  final SendMessagePayload data;
}

class FramePayloadEditMessage extends FramePayload {
  const FramePayloadEditMessage(this.data);
  final EditMessagePayload data;
}

class FramePayloadDeleteMessage extends FramePayload {
  const FramePayloadDeleteMessage(this.data);
  final DeleteMessagePayload data;
}

class FramePayloadReadReceipt extends FramePayload {
  const FramePayloadReadReceipt(this.data);
  final ReadReceiptPayload data;
}

class FramePayloadTyping extends FramePayload {
  const FramePayloadTyping(this.data);
  final TypingPayload data;
}

class FramePayloadGetPresence extends FramePayload {
  const FramePayloadGetPresence(this.data);
  final GetPresencePayload data;
}

class FramePayloadLoadChats extends FramePayload {
  const FramePayloadLoadChats(this.data);
  final LoadChatsPayload data;
}

class FramePayloadSearch extends FramePayload {
  const FramePayloadSearch(this.data);
  final SearchPayload data;
}

class FramePayloadSubscribe extends FramePayload {
  const FramePayloadSubscribe(this.data);
  final SubscribePayload data;
}

class FramePayloadUnsubscribe extends FramePayload {
  const FramePayloadUnsubscribe(this.data);
  final UnsubscribePayload data;
}

class FramePayloadLoadMessages extends FramePayload {
  const FramePayloadLoadMessages(this.data);
  final LoadMessagesPayload data;
}

class FramePayloadAddReaction extends FramePayload {
  const FramePayloadAddReaction(this.data);
  final AddReactionPayload data;
}

class FramePayloadRemoveReaction extends FramePayload {
  const FramePayloadRemoveReaction(this.data);
  final RemoveReactionPayload data;
}

class FramePayloadPinMessage extends FramePayload {
  const FramePayloadPinMessage(this.data);
  final PinMessagePayload data;
}

class FramePayloadUnpinMessage extends FramePayload {
  const FramePayloadUnpinMessage(this.data);
  final UnpinMessagePayload data;
}

class FramePayloadForwardMessage extends FramePayload {
  const FramePayloadForwardMessage(this.data);
  final ForwardMessagePayload data;
}

class FramePayloadMessageNew extends FramePayload {
  const FramePayloadMessageNew(this.data);
  final Message data;
}

class FramePayloadMessageEdited extends FramePayload {
  const FramePayloadMessageEdited(this.data);
  final Message data;
}

class FramePayloadMessageDeleted extends FramePayload {
  const FramePayloadMessageDeleted(this.data);
  final MessageDeletedPayload data;
}

class FramePayloadReceiptUpdate extends FramePayload {
  const FramePayloadReceiptUpdate(this.data);
  final ReceiptUpdatePayload data;
}

class FramePayloadTypingUpdate extends FramePayload {
  const FramePayloadTypingUpdate(this.data);
  final TypingUpdatePayload data;
}

class FramePayloadMemberJoined extends FramePayload {
  const FramePayloadMemberJoined(this.data);
  final MemberJoinedPayload data;
}

class FramePayloadMemberLeft extends FramePayload {
  const FramePayloadMemberLeft(this.data);
  final MemberLeftPayload data;
}

class FramePayloadPresenceResult extends FramePayload {
  const FramePayloadPresenceResult(this.data);
  final List<PresenceEntry> data;
}

class FramePayloadChatUpdated extends FramePayload {
  const FramePayloadChatUpdated(this.data);
  final ChatEntry data;
}

class FramePayloadChatCreated extends FramePayload {
  const FramePayloadChatCreated(this.data);
  final ChatEntry data;
}

class FramePayloadReactionUpdate extends FramePayload {
  const FramePayloadReactionUpdate(this.data);
  final ReactionUpdatePayload data;
}

class FramePayloadUserUpdated extends FramePayload {
  const FramePayloadUserUpdated(this.data);
  final UserEntry data;
}

class FramePayloadChatDeleted extends FramePayload {
  const FramePayloadChatDeleted(this.data);
  final ChatDeletedPayload data;
}

class FramePayloadMemberUpdated extends FramePayload {
  const FramePayloadMemberUpdated(this.data);
  final MemberUpdatedPayload data;
}

class FramePayloadAck extends FramePayload {
  const FramePayloadAck(this.data);
  final Uint8List data;
}

class FramePayloadError extends FramePayload {
  const FramePayloadError(this.data);
  final ErrorPayload data;
}

class FramePayloadCreateChat extends FramePayload {
  const FramePayloadCreateChat(this.data);
  final CreateChatPayload data;
}

class FramePayloadUpdateChat extends FramePayload {
  const FramePayloadUpdateChat(this.data);
  final UpdateChatPayload data;
}

class FramePayloadDeleteChat extends FramePayload {
  const FramePayloadDeleteChat(this.data);
  final DeleteChatPayload data;
}

class FramePayloadGetChatInfo extends FramePayload {
  const FramePayloadGetChatInfo(this.data);
  final GetChatInfoPayload data;
}

class FramePayloadGetChatMembers extends FramePayload {
  const FramePayloadGetChatMembers(this.data);
  final GetChatMembersPayload data;
}

class FramePayloadInviteMembers extends FramePayload {
  const FramePayloadInviteMembers(this.data);
  final InviteMembersPayload data;
}

class FramePayloadUpdateMember extends FramePayload {
  const FramePayloadUpdateMember(this.data);
  final UpdateMemberPayload data;
}

class FramePayloadLeaveChat extends FramePayload {
  const FramePayloadLeaveChat(this.data);
  final LeaveChatPayload data;
}

class FramePayloadMuteChat extends FramePayload {
  const FramePayloadMuteChat(this.data);
  final MuteChatPayload data;
}

class FramePayloadUnmuteChat extends FramePayload {
  const FramePayloadUnmuteChat(this.data);
  final UnmuteChatPayload data;
}

class FramePayloadGetUser extends FramePayload {
  const FramePayloadGetUser(this.data);
  final GetUserPayload data;
}

class FramePayloadGetUsers extends FramePayload {
  const FramePayloadGetUsers(this.data);
  final GetUsersPayload data;
}

class FramePayloadUpdateProfile extends FramePayload {
  const FramePayloadUpdateProfile(this.data);
  final UpdateProfilePayload data;
}

class FramePayloadBlockUser extends FramePayload {
  const FramePayloadBlockUser(this.data);
  final BlockUserPayload data;
}

class FramePayloadUnblockUser extends FramePayload {
  const FramePayloadUnblockUser(this.data);
  final UnblockUserPayload data;
}

class FramePayloadGetBlockList extends FramePayload {
  const FramePayloadGetBlockList(this.data);
  final GetBlockListPayload data;
}

class Frame {
  const Frame({
    required this.seq,
    required this.eventSeq,
    required this.payload,
  });
  final int seq;
  final int eventSeq;
  final FramePayload payload;
}

void encodeFrameHeader(ProtocolWriter w, FrameHeader h) {
  w.writeU8(h.kind.value);
  w.writeU32(h.seq);
  w.writeU32(h.eventSeq);
}

FrameHeader decodeFrameHeader(ProtocolReader r) {
  final kindByte = r.readU8();
  final kind = FrameKind.fromValue(kindByte);
  if (kind == null) throw CodecError('unknown FrameKind: $kindByte');
  return FrameHeader(kind: kind, seq: r.readU32(), eventSeq: r.readU32());
}

FrameKind _framePayloadKind(FramePayload p) {
  return switch (p) {
    FramePayloadHello() => FrameKind.hello,
    FramePayloadWelcome() => FrameKind.welcome,
    FramePayloadPing() => FrameKind.ping,
    FramePayloadPong() => FrameKind.pong,
    FramePayloadRefreshToken() => FrameKind.refreshToken,
    FramePayloadSendMessage() => FrameKind.sendMessage,
    FramePayloadEditMessage() => FrameKind.editMessage,
    FramePayloadDeleteMessage() => FrameKind.deleteMessage,
    FramePayloadReadReceipt() => FrameKind.readReceipt,
    FramePayloadTyping() => FrameKind.typing,
    FramePayloadGetPresence() => FrameKind.getPresence,
    FramePayloadLoadChats() => FrameKind.loadChats,
    FramePayloadSearch() => FrameKind.search,
    FramePayloadSubscribe() => FrameKind.subscribe,
    FramePayloadUnsubscribe() => FrameKind.unsubscribe,
    FramePayloadLoadMessages() => FrameKind.loadMessages,
    FramePayloadAddReaction() => FrameKind.addReaction,
    FramePayloadRemoveReaction() => FrameKind.removeReaction,
    FramePayloadPinMessage() => FrameKind.pinMessage,
    FramePayloadUnpinMessage() => FrameKind.unpinMessage,
    FramePayloadForwardMessage() => FrameKind.forwardMessage,
    FramePayloadMessageNew() => FrameKind.messageNew,
    FramePayloadMessageEdited() => FrameKind.messageEdited,
    FramePayloadMessageDeleted() => FrameKind.messageDeleted,
    FramePayloadReceiptUpdate() => FrameKind.receiptUpdate,
    FramePayloadTypingUpdate() => FrameKind.typingUpdate,
    FramePayloadMemberJoined() => FrameKind.memberJoined,
    FramePayloadMemberLeft() => FrameKind.memberLeft,
    FramePayloadPresenceResult() => FrameKind.presenceResult,
    FramePayloadChatUpdated() => FrameKind.chatUpdated,
    FramePayloadChatCreated() => FrameKind.chatCreated,
    FramePayloadReactionUpdate() => FrameKind.reactionUpdate,
    FramePayloadUserUpdated() => FrameKind.userUpdated,
    FramePayloadChatDeleted() => FrameKind.chatDeleted,
    FramePayloadMemberUpdated() => FrameKind.memberUpdated,
    FramePayloadAck() => FrameKind.ack,
    FramePayloadError() => FrameKind.error,
    FramePayloadCreateChat() => FrameKind.createChat,
    FramePayloadUpdateChat() => FrameKind.updateChat,
    FramePayloadDeleteChat() => FrameKind.deleteChat,
    FramePayloadGetChatInfo() => FrameKind.getChatInfo,
    FramePayloadGetChatMembers() => FrameKind.getChatMembers,
    FramePayloadInviteMembers() => FrameKind.inviteMembers,
    FramePayloadUpdateMember() => FrameKind.updateMember,
    FramePayloadLeaveChat() => FrameKind.leaveChat,
    FramePayloadMuteChat() => FrameKind.muteChat,
    FramePayloadUnmuteChat() => FrameKind.unmuteChat,
    FramePayloadGetUser() => FrameKind.getUser,
    FramePayloadGetUsers() => FrameKind.getUsers,
    FramePayloadUpdateProfile() => FrameKind.updateProfile,
    FramePayloadBlockUser() => FrameKind.blockUser,
    FramePayloadUnblockUser() => FrameKind.unblockUser,
    FramePayloadGetBlockList() => FrameKind.getBlockList,
  };
}

void encodeFrame(ProtocolWriter w, Frame frame) {
  final kind = _framePayloadKind(frame.payload);
  w.writeU8(kind.value);
  w.writeU32(frame.seq);
  w.writeU32(frame.eventSeq);
  switch (frame.payload) {
    case FramePayloadHello p:
      encodeHelloPayload(w, p.data);
    case FramePayloadWelcome p:
      encodeWelcomePayload(w, p.data);
    case FramePayloadPing():
      break;
    case FramePayloadPong():
      break;
    case FramePayloadRefreshToken p:
      encodeRefreshTokenPayload(w, p.data);
    case FramePayloadSendMessage p:
      encodeSendMessagePayload(w, p.data);
    case FramePayloadEditMessage p:
      encodeEditMessagePayload(w, p.data);
    case FramePayloadDeleteMessage p:
      encodeDeleteMessagePayload(w, p.data);
    case FramePayloadReadReceipt p:
      encodeReadReceiptPayload(w, p.data);
    case FramePayloadTyping p:
      encodeTypingPayload(w, p.data);
    case FramePayloadGetPresence p:
      encodeGetPresencePayload(w, p.data);
    case FramePayloadLoadChats p:
      encodeLoadChatsPayload(w, p.data);
    case FramePayloadSearch p:
      encodeSearchPayload(w, p.data);
    case FramePayloadSubscribe p:
      encodeSubscribePayload(w, p.data);
    case FramePayloadUnsubscribe p:
      encodeUnsubscribePayload(w, p.data);
    case FramePayloadLoadMessages p:
      encodeLoadMessagesPayload(w, p.data);
    case FramePayloadAddReaction p:
      encodeAddReactionPayload(w, p.data);
    case FramePayloadRemoveReaction p:
      encodeRemoveReactionPayload(w, p.data);
    case FramePayloadPinMessage p:
      encodePinMessagePayload(w, p.data);
    case FramePayloadUnpinMessage p:
      encodeUnpinMessagePayload(w, p.data);
    case FramePayloadForwardMessage p:
      encodeForwardMessagePayload(w, p.data);
    case FramePayloadMessageNew p:
      encodeMessage(w, p.data);
    case FramePayloadMessageEdited p:
      encodeMessage(w, p.data);
    case FramePayloadMessageDeleted p:
      encodeMessageDeletedPayload(w, p.data);
    case FramePayloadReceiptUpdate p:
      encodeReceiptUpdatePayload(w, p.data);
    case FramePayloadTypingUpdate p:
      encodeTypingUpdatePayload(w, p.data);
    case FramePayloadMemberJoined p:
      encodeMemberJoinedPayload(w, p.data);
    case FramePayloadMemberLeft p:
      encodeMemberLeftPayload(w, p.data);
    case FramePayloadPresenceResult p:
      w.writeU16(p.data.length);
      for (final e in p.data) {
        encodePresenceEntry(w, e);
      }
    case FramePayloadChatUpdated p:
      encodeChatEntry(w, p.data);
    case FramePayloadChatCreated p:
      encodeChatEntry(w, p.data);
    case FramePayloadReactionUpdate p:
      encodeReactionUpdatePayload(w, p.data);
    case FramePayloadUserUpdated p:
      encodeUserEntry(w, p.data);
    case FramePayloadChatDeleted p:
      encodeChatDeletedPayload(w, p.data);
    case FramePayloadMemberUpdated p:
      encodeMemberUpdatedPayload(w, p.data);
    case FramePayloadAck p:
      w.writeRawBytes(p.data);
    case FramePayloadError p:
      encodeErrorPayload(w, p.data);
    case FramePayloadCreateChat p:
      encodeCreateChatPayload(w, p.data);
    case FramePayloadUpdateChat p:
      encodeUpdateChatPayload(w, p.data);
    case FramePayloadDeleteChat p:
      encodeDeleteChatPayload(w, p.data);
    case FramePayloadGetChatInfo p:
      encodeGetChatInfoPayload(w, p.data);
    case FramePayloadGetChatMembers p:
      encodeGetChatMembersPayload(w, p.data);
    case FramePayloadInviteMembers p:
      encodeInviteMembersPayload(w, p.data);
    case FramePayloadUpdateMember p:
      encodeUpdateMemberPayload(w, p.data);
    case FramePayloadLeaveChat p:
      encodeLeaveChatPayload(w, p.data);
    case FramePayloadMuteChat p:
      encodeMuteChatPayload(w, p.data);
    case FramePayloadUnmuteChat p:
      encodeUnmuteChatPayload(w, p.data);
    case FramePayloadGetUser p:
      encodeGetUserPayload(w, p.data);
    case FramePayloadGetUsers p:
      encodeGetUsersPayload(w, p.data);
    case FramePayloadUpdateProfile p:
      encodeUpdateProfilePayload(w, p.data);
    case FramePayloadBlockUser p:
      encodeBlockUserPayload(w, p.data);
    case FramePayloadUnblockUser p:
      encodeUnblockUserPayload(w, p.data);
    case FramePayloadGetBlockList p:
      encodeGetBlockListPayload(w, p.data);
  }
}

Frame decodeFrame(ProtocolReader r) {
  final header = decodeFrameHeader(r);
  late FramePayload payload;
  switch (header.kind.value) {
    case 1:
      payload = FramePayloadHello(decodeHelloPayload(r));
    case 2:
      payload = FramePayloadWelcome(decodeWelcomePayload(r));
    case 3:
      payload = FramePayloadPing();
    case 4:
      payload = FramePayloadPong();
    case 5:
      payload = FramePayloadRefreshToken(decodeRefreshTokenPayload(r));
    case 16:
      payload = FramePayloadSendMessage(decodeSendMessagePayload(r));
    case 17:
      payload = FramePayloadEditMessage(decodeEditMessagePayload(r));
    case 18:
      payload = FramePayloadDeleteMessage(decodeDeleteMessagePayload(r));
    case 19:
      payload = FramePayloadReadReceipt(decodeReadReceiptPayload(r));
    case 20:
      payload = FramePayloadTyping(decodeTypingPayload(r));
    case 21:
      payload = FramePayloadGetPresence(decodeGetPresencePayload(r));
    case 22:
      payload = FramePayloadLoadChats(decodeLoadChatsPayload(r));
    case 23:
      payload = FramePayloadSearch(decodeSearchPayload(r));
    case 24:
      payload = FramePayloadSubscribe(decodeSubscribePayload(r));
    case 25:
      payload = FramePayloadUnsubscribe(decodeUnsubscribePayload(r));
    case 26:
      payload = FramePayloadLoadMessages(decodeLoadMessagesPayload(r));
    case 27:
      payload = FramePayloadAddReaction(decodeAddReactionPayload(r));
    case 28:
      payload = FramePayloadRemoveReaction(decodeRemoveReactionPayload(r));
    case 29:
      payload = FramePayloadPinMessage(decodePinMessagePayload(r));
    case 30:
      payload = FramePayloadUnpinMessage(decodeUnpinMessagePayload(r));
    case 31:
      payload = FramePayloadForwardMessage(decodeForwardMessagePayload(r));
    case 32:
      payload = FramePayloadMessageNew(decodeMessage(r));
    case 33:
      payload = FramePayloadMessageEdited(decodeMessage(r));
    case 34:
      payload = FramePayloadMessageDeleted(decodeMessageDeletedPayload(r));
    case 35:
      payload = FramePayloadReceiptUpdate(decodeReceiptUpdatePayload(r));
    case 36:
      payload = FramePayloadTypingUpdate(decodeTypingUpdatePayload(r));
    case 37:
      payload = FramePayloadMemberJoined(decodeMemberJoinedPayload(r));
    case 38:
      payload = FramePayloadMemberLeft(decodeMemberLeftPayload(r));
    case 39:
      payload = FramePayloadPresenceResult(
        r.readArray(r.readU16(), () => decodePresenceEntry(r)),
      );
    case 40:
      payload = FramePayloadChatUpdated(decodeChatEntry(r));
    case 41:
      payload = FramePayloadChatCreated(decodeChatEntry(r));
    case 42:
      payload = FramePayloadReactionUpdate(decodeReactionUpdatePayload(r));
    case 43:
      payload = FramePayloadUserUpdated(decodeUserEntry(r));
    case 44:
      payload = FramePayloadChatDeleted(decodeChatDeletedPayload(r));
    case 45:
      payload = FramePayloadMemberUpdated(decodeMemberUpdatedPayload(r));
    case 48:
      payload = FramePayloadAck(
        r.remaining > 0 ? r.readBytes(r.remaining) : Uint8List(0),
      );
    case 49:
      payload = FramePayloadError(decodeErrorPayload(r));
    case 64:
      payload = FramePayloadCreateChat(decodeCreateChatPayload(r));
    case 65:
      payload = FramePayloadUpdateChat(decodeUpdateChatPayload(r));
    case 66:
      payload = FramePayloadDeleteChat(decodeDeleteChatPayload(r));
    case 67:
      payload = FramePayloadGetChatInfo(decodeGetChatInfoPayload(r));
    case 68:
      payload = FramePayloadGetChatMembers(decodeGetChatMembersPayload(r));
    case 69:
      payload = FramePayloadInviteMembers(decodeInviteMembersPayload(r));
    case 70:
      payload = FramePayloadUpdateMember(decodeUpdateMemberPayload(r));
    case 71:
      payload = FramePayloadLeaveChat(decodeLeaveChatPayload(r));
    case 72:
      payload = FramePayloadMuteChat(decodeMuteChatPayload(r));
    case 73:
      payload = FramePayloadUnmuteChat(decodeUnmuteChatPayload(r));
    case 80:
      payload = FramePayloadGetUser(decodeGetUserPayload(r));
    case 81:
      payload = FramePayloadGetUsers(decodeGetUsersPayload(r));
    case 82:
      payload = FramePayloadUpdateProfile(decodeUpdateProfilePayload(r));
    case 83:
      payload = FramePayloadBlockUser(decodeBlockUserPayload(r));
    case 84:
      payload = FramePayloadUnblockUser(decodeUnblockUserPayload(r));
    case 85:
      payload = FramePayloadGetBlockList(decodeGetBlockListPayload(r));
    default:
      throw CodecError('unhandled FrameKind: ${header.kind.value}');
  }
  return Frame(seq: header.seq, eventSeq: header.eventSeq, payload: payload);
}
