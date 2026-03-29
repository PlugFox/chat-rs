// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'dart:typed_data';

import 'package:test/test.dart';
import 'package:chat_core/chat_core.dart';

void main() {
  group('LastMessagePreview codec', () {
    test('roundtrip', () {
      final original = LastMessagePreview(
        id: 100000,
        senderId: 100000,
        createdAt: 1234567890,
        kind: MessageKind.text,
        flags: MessageFlags.edited,
        contentPreview: 'hello',
      );
      final w = ProtocolWriter();
      encodeLastMessagePreview(w, original);
      final decoded = decodeLastMessagePreview(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('ChatEntry codec', () {
    test('roundtrip', () {
      final original = ChatEntry(
        id: 100000,
        kind: ChatKind.direct,
        parentId: 7,
        createdAt: 1234567890,
        updatedAt: 1234567890,
        title: 'test',
        avatarUrl: 'test',
        lastMessage: LastMessagePreview(
          id: 100000,
          senderId: 100000,
          createdAt: 1234567890,
          kind: MessageKind.text,
          flags: MessageFlags.edited,
          contentPreview: 'hello',
        ),
        unreadCount: 100000,
        memberCount: 100000,
      );
      final w = ProtocolWriter();
      encodeChatEntry(w, original);
      final decoded = decodeChatEntry(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('roundtrip with nulls', () {
      final original = ChatEntry(
        id: 100000,
        kind: ChatKind.direct,
        createdAt: 1234567890,
        updatedAt: 1234567890,
        unreadCount: 100000,
        memberCount: 100000,
      );
      final w = ProtocolWriter();
      encodeChatEntry(w, original);
      final decoded = decodeChatEntry(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('ChatMemberEntry codec', () {
    test('roundtrip', () {
      final original = ChatMemberEntry(
        userId: 100000,
        role: ChatRole.member,
        permissions: Permission.sendMessages,
      );
      final w = ProtocolWriter();
      encodeChatMemberEntry(w, original);
      final decoded = decodeChatMemberEntry(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('roundtrip with nulls', () {
      final original = ChatMemberEntry(userId: 100000, role: ChatRole.member);
      final w = ProtocolWriter();
      encodeChatMemberEntry(w, original);
      final decoded = decodeChatMemberEntry(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('RichSpan codec', () {
    test('roundtrip', () {
      final original = RichSpan(
        start: 100000,
        end: 100000,
        style: RichStyle.bold,
        meta: 'test',
      );
      final w = ProtocolWriter();
      encodeRichSpan(w, original);
      final decoded = decodeRichSpan(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('roundtrip with nulls', () {
      final original = RichSpan(
        start: 100000,
        end: 100000,
        style: RichStyle.bold,
      );
      final w = ProtocolWriter();
      encodeRichSpan(w, original);
      final decoded = decodeRichSpan(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('Message codec', () {
    test('roundtrip', () {
      final original = Message(
        id: 100000,
        chatId: 100000,
        senderId: 100000,
        createdAt: 1234567890,
        updatedAt: 1234567890,
        kind: MessageKind.text,
        flags: MessageFlags.edited,
        replyToId: 7,
        content: 'hello',
        richContent: [
          RichSpan(
            start: 100000,
            end: 100000,
            style: RichStyle.bold,
            meta: 'test',
          ),
        ],
        extra: 'test',
      );
      final w = ProtocolWriter();
      encodeMessage(w, original);
      final decoded = decodeMessage(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('roundtrip with nulls', () {
      final original = Message(
        id: 100000,
        chatId: 100000,
        senderId: 100000,
        createdAt: 1234567890,
        updatedAt: 1234567890,
        kind: MessageKind.text,
        flags: MessageFlags.edited,
        content: '',
      );
      final w = ProtocolWriter();
      encodeMessage(w, original);
      final decoded = decodeMessage(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('MessageBatch codec', () {
    test('roundtrip', () {
      final original = MessageBatch(
        messages: [
          Message(
            id: 100000,
            chatId: 100000,
            senderId: 100000,
            createdAt: 1234567890,
            updatedAt: 1234567890,
            kind: MessageKind.text,
            flags: MessageFlags.edited,
            replyToId: 7,
            content: 'hello',
            richContent: [
              RichSpan(
                start: 100000,
                end: 100000,
                style: RichStyle.bold,
                meta: 'test',
              ),
            ],
            extra: 'test',
          ),
        ],
        hasMore: true,
      );
      final w = ProtocolWriter();
      encodeMessageBatch(w, original);
      final decoded = decodeMessageBatch(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('UserEntry codec', () {
    test('roundtrip', () {
      final original = UserEntry(
        id: 100000,
        flags: UserFlags.system,
        createdAt: 1234567890,
        updatedAt: 1234567890,
        username: 'test',
        firstName: 'test',
        lastName: 'test',
        avatarUrl: 'test',
      );
      final w = ProtocolWriter();
      encodeUserEntry(w, original);
      final decoded = decodeUserEntry(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('roundtrip with nulls', () {
      final original = UserEntry(
        id: 100000,
        flags: UserFlags.system,
        createdAt: 1234567890,
        updatedAt: 1234567890,
      );
      final w = ProtocolWriter();
      encodeUserEntry(w, original);
      final decoded = decodeUserEntry(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('PresenceEntry codec', () {
    test('roundtrip', () {
      final original = PresenceEntry(
        userId: 100000,
        status: PresenceStatus.offline,
        lastSeen: 1234567890,
      );
      final w = ProtocolWriter();
      encodePresenceEntry(w, original);
      final decoded = decodePresenceEntry(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('ErrorPayload codec', () {
    test('roundtrip', () {
      final original = ErrorPayload(
        code: ErrorCode.unauthorized,
        message: 'hello',
        retryAfterMs: 100000,
        extra: 'test',
      );
      final w = ProtocolWriter();
      encodeErrorPayload(w, original);
      final decoded = decodeErrorPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('roundtrip with nulls', () {
      final original = ErrorPayload(
        code: ErrorCode.unauthorized,
        message: '',
        retryAfterMs: 100000,
      );
      final w = ProtocolWriter();
      encodeErrorPayload(w, original);
      final decoded = decodeErrorPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('HelloPayload codec', () {
    test('roundtrip', () {
      final original = HelloPayload(
        protocolVersion: 42,
        sdkVersion: 'hello',
        platform: 'hello',
        token: 'hello',
        deviceId: '550e8400-e29b-41d4-a716-446655440000',
      );
      final w = ProtocolWriter();
      encodeHelloPayload(w, original);
      final decoded = decodeHelloPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('WelcomePayload codec', () {
    test('roundtrip', () {
      final original = WelcomePayload(
        sessionId: 100000,
        serverTime: 1234567890,
        userId: 100000,
        limits: ServerLimits(
          pingIntervalMs: 100000,
          pingTimeoutMs: 100000,
          maxMessageSize: 100000,
          maxExtraSize: 100000,
          maxFrameSize: 100000,
          messagesPerSec: 1000,
          connectionsPerIp: 1000,
        ),
        capabilities: ServerCapabilities.mediaUpload,
      );
      final w = ProtocolWriter();
      encodeWelcomePayload(w, original);
      final decoded = decodeWelcomePayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('ServerLimits codec', () {
    test('roundtrip', () {
      final original = ServerLimits(
        pingIntervalMs: 100000,
        pingTimeoutMs: 100000,
        maxMessageSize: 100000,
        maxExtraSize: 100000,
        maxFrameSize: 100000,
        messagesPerSec: 1000,
        connectionsPerIp: 1000,
      );
      final w = ProtocolWriter();
      encodeServerLimits(w, original);
      final decoded = decodeServerLimits(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('SendMessagePayload codec', () {
    test('roundtrip', () {
      final original = SendMessagePayload(
        chatId: 100000,
        kind: MessageKind.text,
        idempotencyKey: '550e8400-e29b-41d4-a716-446655440000',
        replyToId: 7,
        content: 'hello',
        richContent: Uint8List.fromList([1, 2]),
        extra: 'test',
        mentionedUserIds: [1, 2, 3],
      );
      final w = ProtocolWriter();
      encodeSendMessagePayload(w, original);
      final decoded = decodeSendMessagePayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('roundtrip with nulls', () {
      final original = SendMessagePayload(
        chatId: 100000,
        kind: MessageKind.text,
        idempotencyKey: '550e8400-e29b-41d4-a716-446655440000',
        content: '',
        mentionedUserIds: [],
      );
      final w = ProtocolWriter();
      encodeSendMessagePayload(w, original);
      final decoded = decodeSendMessagePayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('EditMessagePayload codec', () {
    test('roundtrip', () {
      final original = EditMessagePayload(
        chatId: 100000,
        messageId: 100000,
        content: 'hello',
        richContent: Uint8List.fromList([1, 2]),
        extra: 'test',
      );
      final w = ProtocolWriter();
      encodeEditMessagePayload(w, original);
      final decoded = decodeEditMessagePayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('roundtrip with nulls', () {
      final original = EditMessagePayload(
        chatId: 100000,
        messageId: 100000,
        content: '',
      );
      final w = ProtocolWriter();
      encodeEditMessagePayload(w, original);
      final decoded = decodeEditMessagePayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('DeleteMessagePayload codec', () {
    test('roundtrip', () {
      final original = DeleteMessagePayload(chatId: 100000, messageId: 100000);
      final w = ProtocolWriter();
      encodeDeleteMessagePayload(w, original);
      final decoded = decodeDeleteMessagePayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('ReadReceiptPayload codec', () {
    test('roundtrip', () {
      final original = ReadReceiptPayload(chatId: 100000, messageId: 100000);
      final w = ProtocolWriter();
      encodeReadReceiptPayload(w, original);
      final decoded = decodeReadReceiptPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('TypingPayload codec', () {
    test('roundtrip', () {
      final original = TypingPayload(chatId: 100000, expiresInMs: 1000);
      final w = ProtocolWriter();
      encodeTypingPayload(w, original);
      final decoded = decodeTypingPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('GetPresencePayload codec', () {
    test('roundtrip', () {
      final original = GetPresencePayload(userIds: [1, 2, 3]);
      final w = ProtocolWriter();
      encodeGetPresencePayload(w, original);
      final decoded = decodeGetPresencePayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('SearchPayload codec', () {
    test('roundtrip', () {
      final original = SearchPayload(
        scope: SearchScopeChat(chatId: 100000),
        query: 'hello',
        cursor: 100000,
        limit: 1000,
      );
      final w = ProtocolWriter();
      encodeSearchPayload(w, original);
      final decoded = decodeSearchPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('SubscribePayload codec', () {
    test('roundtrip', () {
      final original = SubscribePayload(channels: ['a', 'b']);
      final w = ProtocolWriter();
      encodeSubscribePayload(w, original);
      final decoded = decodeSubscribePayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('UnsubscribePayload codec', () {
    test('roundtrip', () {
      final original = UnsubscribePayload(channels: ['a', 'b']);
      final w = ProtocolWriter();
      encodeUnsubscribePayload(w, original);
      final decoded = decodeUnsubscribePayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('CreateChatPayload codec', () {
    test('roundtrip', () {
      final original = CreateChatPayload(
        kind: ChatKind.direct,
        parentId: 7,
        title: 'test',
        avatarUrl: 'test',
        memberIds: [1, 2, 3],
      );
      final w = ProtocolWriter();
      encodeCreateChatPayload(w, original);
      final decoded = decodeCreateChatPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('roundtrip with nulls', () {
      final original = CreateChatPayload(kind: ChatKind.direct, memberIds: []);
      final w = ProtocolWriter();
      encodeCreateChatPayload(w, original);
      final decoded = decodeCreateChatPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('UpdateChatPayload codec', () {
    test('roundtrip', () {
      final original = UpdateChatPayload(
        chatId: 100000,
        title: 'updated',
        avatarUrl: 'updated',
      );
      final w = ProtocolWriter();
      encodeUpdateChatPayload(w, original);
      final decoded = decodeUpdateChatPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('roundtrip with nulls', () {
      final original = UpdateChatPayload(chatId: 100000);
      final w = ProtocolWriter();
      encodeUpdateChatPayload(w, original);
      final decoded = decodeUpdateChatPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('DeleteChatPayload codec', () {
    test('roundtrip', () {
      final original = DeleteChatPayload(chatId: 100000);
      final w = ProtocolWriter();
      encodeDeleteChatPayload(w, original);
      final decoded = decodeDeleteChatPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('GetChatInfoPayload codec', () {
    test('roundtrip', () {
      final original = GetChatInfoPayload(chatId: 100000);
      final w = ProtocolWriter();
      encodeGetChatInfoPayload(w, original);
      final decoded = decodeGetChatInfoPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('GetChatMembersPayload codec', () {
    test('roundtrip', () {
      final original = GetChatMembersPayload(
        chatId: 100000,
        cursor: 100000,
        limit: 1000,
      );
      final w = ProtocolWriter();
      encodeGetChatMembersPayload(w, original);
      final decoded = decodeGetChatMembersPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('InviteMembersPayload codec', () {
    test('roundtrip', () {
      final original = InviteMembersPayload(chatId: 100000, userIds: [1, 2, 3]);
      final w = ProtocolWriter();
      encodeInviteMembersPayload(w, original);
      final decoded = decodeInviteMembersPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('LeaveChatPayload codec', () {
    test('roundtrip', () {
      final original = LeaveChatPayload(chatId: 100000);
      final w = ProtocolWriter();
      encodeLeaveChatPayload(w, original);
      final decoded = decodeLeaveChatPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('UpdateMemberPayload codec', () {
    test('roundtrip', () {
      final original = UpdateMemberPayload(
        chatId: 100000,
        userId: 100000,
        action: const MemberActionKick(),
      );
      final w = ProtocolWriter();
      encodeUpdateMemberPayload(w, original);
      final decoded = decodeUpdateMemberPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('MessageDeletedPayload codec', () {
    test('roundtrip', () {
      final original = MessageDeletedPayload(chatId: 100000, messageId: 100000);
      final w = ProtocolWriter();
      encodeMessageDeletedPayload(w, original);
      final decoded = decodeMessageDeletedPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('ReceiptUpdatePayload codec', () {
    test('roundtrip', () {
      final original = ReceiptUpdatePayload(
        chatId: 100000,
        userId: 100000,
        messageId: 100000,
      );
      final w = ProtocolWriter();
      encodeReceiptUpdatePayload(w, original);
      final decoded = decodeReceiptUpdatePayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('TypingUpdatePayload codec', () {
    test('roundtrip', () {
      final original = TypingUpdatePayload(
        chatId: 100000,
        userId: 100000,
        expiresInMs: 1000,
      );
      final w = ProtocolWriter();
      encodeTypingUpdatePayload(w, original);
      final decoded = decodeTypingUpdatePayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('MemberJoinedPayload codec', () {
    test('roundtrip', () {
      final original = MemberJoinedPayload(
        chatId: 100000,
        userId: 100000,
        role: ChatRole.member,
        invitedBy: 100000,
      );
      final w = ProtocolWriter();
      encodeMemberJoinedPayload(w, original);
      final decoded = decodeMemberJoinedPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('MemberLeftPayload codec', () {
    test('roundtrip', () {
      final original = MemberLeftPayload(chatId: 100000, userId: 100000);
      final w = ProtocolWriter();
      encodeMemberLeftPayload(w, original);
      final decoded = decodeMemberLeftPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('ChatDeletedPayload codec', () {
    test('roundtrip', () {
      final original = ChatDeletedPayload(chatId: 100000);
      final w = ProtocolWriter();
      encodeChatDeletedPayload(w, original);
      final decoded = decodeChatDeletedPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('MemberUpdatedPayload codec', () {
    test('roundtrip', () {
      final original = MemberUpdatedPayload(
        chatId: 100000,
        userId: 100000,
        role: ChatRole.member,
        permissions: Permission.sendMessages,
      );
      final w = ProtocolWriter();
      encodeMemberUpdatedPayload(w, original);
      final decoded = decodeMemberUpdatedPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('roundtrip with nulls', () {
      final original = MemberUpdatedPayload(
        chatId: 100000,
        userId: 100000,
        role: ChatRole.member,
      );
      final w = ProtocolWriter();
      encodeMemberUpdatedPayload(w, original);
      final decoded = decodeMemberUpdatedPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('AddReactionPayload codec', () {
    test('roundtrip', () {
      final original = AddReactionPayload(
        chatId: 100000,
        messageId: 100000,
        packId: 100000,
        emojiIndex: 42,
      );
      final w = ProtocolWriter();
      encodeAddReactionPayload(w, original);
      final decoded = decodeAddReactionPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('RemoveReactionPayload codec', () {
    test('roundtrip', () {
      final original = RemoveReactionPayload(
        chatId: 100000,
        messageId: 100000,
        packId: 100000,
        emojiIndex: 42,
      );
      final w = ProtocolWriter();
      encodeRemoveReactionPayload(w, original);
      final decoded = decodeRemoveReactionPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('ReactionUpdatePayload codec', () {
    test('roundtrip', () {
      final original = ReactionUpdatePayload(
        chatId: 100000,
        messageId: 100000,
        userId: 100000,
        packId: 100000,
        emojiIndex: 42,
        added: true,
      );
      final w = ProtocolWriter();
      encodeReactionUpdatePayload(w, original);
      final decoded = decodeReactionUpdatePayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('PinMessagePayload codec', () {
    test('roundtrip', () {
      final original = PinMessagePayload(chatId: 100000, messageId: 100000);
      final w = ProtocolWriter();
      encodePinMessagePayload(w, original);
      final decoded = decodePinMessagePayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('UnpinMessagePayload codec', () {
    test('roundtrip', () {
      final original = UnpinMessagePayload(chatId: 100000, messageId: 100000);
      final w = ProtocolWriter();
      encodeUnpinMessagePayload(w, original);
      final decoded = decodeUnpinMessagePayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('RefreshTokenPayload codec', () {
    test('roundtrip', () {
      final original = RefreshTokenPayload(token: 'hello');
      final w = ProtocolWriter();
      encodeRefreshTokenPayload(w, original);
      final decoded = decodeRefreshTokenPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('ForwardMessagePayload codec', () {
    test('roundtrip', () {
      final original = ForwardMessagePayload(
        fromChatId: 100000,
        messageId: 100000,
        toChatId: 100000,
        idempotencyKey: '550e8400-e29b-41d4-a716-446655440000',
      );
      final w = ProtocolWriter();
      encodeForwardMessagePayload(w, original);
      final decoded = decodeForwardMessagePayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('GetUserPayload codec', () {
    test('roundtrip', () {
      final original = GetUserPayload(userId: 100000);
      final w = ProtocolWriter();
      encodeGetUserPayload(w, original);
      final decoded = decodeGetUserPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('GetUsersPayload codec', () {
    test('roundtrip', () {
      final original = GetUsersPayload(userIds: [1, 2, 3]);
      final w = ProtocolWriter();
      encodeGetUsersPayload(w, original);
      final decoded = decodeGetUsersPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('UpdateProfilePayload codec', () {
    test('roundtrip', () {
      final original = UpdateProfilePayload(
        username: 'updated',
        firstName: 'updated',
        lastName: 'updated',
        avatarUrl: 'updated',
      );
      final w = ProtocolWriter();
      encodeUpdateProfilePayload(w, original);
      final decoded = decodeUpdateProfilePayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('roundtrip with nulls', () {
      final original = UpdateProfilePayload();
      final w = ProtocolWriter();
      encodeUpdateProfilePayload(w, original);
      final decoded = decodeUpdateProfilePayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('BlockUserPayload codec', () {
    test('roundtrip', () {
      final original = BlockUserPayload(userId: 100000);
      final w = ProtocolWriter();
      encodeBlockUserPayload(w, original);
      final decoded = decodeBlockUserPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('UnblockUserPayload codec', () {
    test('roundtrip', () {
      final original = UnblockUserPayload(userId: 100000);
      final w = ProtocolWriter();
      encodeUnblockUserPayload(w, original);
      final decoded = decodeUnblockUserPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('GetBlockListPayload codec', () {
    test('roundtrip', () {
      final original = GetBlockListPayload(cursor: 100000, limit: 1000);
      final w = ProtocolWriter();
      encodeGetBlockListPayload(w, original);
      final decoded = decodeGetBlockListPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('MuteChatPayload codec', () {
    test('roundtrip', () {
      final original = MuteChatPayload(chatId: 100000, durationSecs: 100000);
      final w = ProtocolWriter();
      encodeMuteChatPayload(w, original);
      final decoded = decodeMuteChatPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('UnmuteChatPayload codec', () {
    test('roundtrip', () {
      final original = UnmuteChatPayload(chatId: 100000);
      final w = ProtocolWriter();
      encodeUnmuteChatPayload(w, original);
      final decoded = decodeUnmuteChatPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('LoadChatsPayload codec', () {
    test('FirstPage roundtrip', () {
      final original = LoadChatsFirstPage(limit: 1000);
      final w = ProtocolWriter();
      encodeLoadChatsPayload(w, original);
      final decoded = decodeLoadChatsPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('After roundtrip', () {
      final original = LoadChatsAfter(cursorTs: 1234567890, limit: 1000);
      final w = ProtocolWriter();
      encodeLoadChatsPayload(w, original);
      final decoded = decodeLoadChatsPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('SearchScope codec', () {
    test('Chat roundtrip', () {
      final original = SearchScopeChat(chatId: 100000);
      final w = ProtocolWriter();
      encodeSearchScope(w, original);
      final decoded = decodeSearchScope(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('Global roundtrip', () {
      final original = const SearchScopeGlobal();
      final w = ProtocolWriter();
      encodeSearchScope(w, original);
      final decoded = decodeSearchScope(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('User roundtrip', () {
      final original = SearchScopeUser(userId: 100000);
      final w = ProtocolWriter();
      encodeSearchScope(w, original);
      final decoded = decodeSearchScope(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('LoadMessagesPayload codec', () {
    test('Paginate roundtrip', () {
      final original = LoadMessagesPaginate(
        chatId: 100000,
        direction: LoadDirection.older,
        anchorId: 100000,
        limit: 1000,
      );
      final w = ProtocolWriter();
      encodeLoadMessagesPayload(w, original);
      final decoded = decodeLoadMessagesPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('RangeCheck roundtrip', () {
      final original = LoadMessagesRangeCheck(
        chatId: 100000,
        fromId: 100000,
        toId: 100000,
        sinceTs: 1234567890,
      );
      final w = ProtocolWriter();
      encodeLoadMessagesPayload(w, original);
      final decoded = decodeLoadMessagesPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('Chunk roundtrip', () {
      final original = LoadMessagesChunk(
        chatId: 100000,
        chunkId: 100000,
        sinceTs: 1234567890,
      );
      final w = ProtocolWriter();
      encodeLoadMessagesPayload(w, original);
      final decoded = decodeLoadMessagesPayload(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('MemberAction codec', () {
    test('Kick roundtrip', () {
      final original = const MemberActionKick();
      final w = ProtocolWriter();
      encodeMemberAction(w, original);
      final decoded = decodeMemberAction(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('Ban roundtrip', () {
      final original = const MemberActionBan();
      final w = ProtocolWriter();
      encodeMemberAction(w, original);
      final decoded = decodeMemberAction(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('Mute roundtrip', () {
      final original = MemberActionMute(durationSecs: 100000);
      final w = ProtocolWriter();
      encodeMemberAction(w, original);
      final decoded = decodeMemberAction(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('ChangeRole roundtrip', () {
      final original = MemberActionChangeRole(chatRole: ChatRole.member);
      final w = ProtocolWriter();
      encodeMemberAction(w, original);
      final decoded = decodeMemberAction(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('UpdatePermissions roundtrip', () {
      final original = MemberActionUpdatePermissions(
        permission: Permission.sendMessages,
      );
      final w = ProtocolWriter();
      encodeMemberAction(w, original);
      final decoded = decodeMemberAction(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
    test('Unban roundtrip', () {
      final original = const MemberActionUnban();
      final w = ProtocolWriter();
      encodeMemberAction(w, original);
      final decoded = decodeMemberAction(ProtocolReader(w.toBytes()));
      expect(decoded, equals(original));
    });
  });

  group('FrameHeader codec', () {
    test('roundtrip', () {
      final header = FrameHeader(kind: FrameKind.hello, seq: 42, eventSeq: 7);
      final w = ProtocolWriter();
      encodeFrameHeader(w, header);
      final decoded = decodeFrameHeader(ProtocolReader(w.toBytes()));
      expect(decoded.kind, equals(header.kind));
      expect(decoded.seq, equals(header.seq));
      expect(decoded.eventSeq, equals(header.eventSeq));
    });
  });

  group('Frame codec', () {
    test('Ping frame roundtrip (no payload)', () {
      final frame = Frame(
        seq: 1,
        eventSeq: 0,
        payload: const FramePayloadPing(),
      );
      final w = ProtocolWriter();
      encodeFrame(w, frame);
      final decoded = decodeFrame(ProtocolReader(w.toBytes()));
      expect(decoded.seq, equals(1));
      expect(decoded.eventSeq, equals(0));
      expect(decoded.payload, isA<FramePayloadPing>());
    });

    test('DeleteMessage frame roundtrip (struct payload)', () {
      final payload = FramePayloadDeleteMessage(
        DeleteMessagePayload(chatId: 1, messageId: 2),
      );
      final frame = Frame(seq: 5, eventSeq: 3, payload: payload);
      final w = ProtocolWriter();
      encodeFrame(w, frame);
      final decoded = decodeFrame(ProtocolReader(w.toBytes()));
      expect(decoded.seq, equals(5));
      expect(decoded.payload, isA<FramePayloadDeleteMessage>());
      final p = decoded.payload as FramePayloadDeleteMessage;
      expect(p.data.chatId, equals(1));
      expect(p.data.messageId, equals(2));
    });

    test('LoadChats frame roundtrip (tagged enum payload)', () {
      final payload = FramePayloadLoadChats(LoadChatsFirstPage(limit: 50));
      final frame = Frame(seq: 10, eventSeq: 0, payload: payload);
      final w = ProtocolWriter();
      encodeFrame(w, frame);
      final decoded = decodeFrame(ProtocolReader(w.toBytes()));
      expect(decoded.seq, equals(10));
      expect(decoded.payload, isA<FramePayloadLoadChats>());
      final p = (decoded.payload as FramePayloadLoadChats).data;
      expect(p, isA<LoadChatsFirstPage>());
      expect((p as LoadChatsFirstPage).limit, equals(50));
    });

    test('Ack frame roundtrip (raw bytes)', () {
      final payload = FramePayloadAck(Uint8List.fromList([1, 2, 3, 4]));
      final frame = Frame(seq: 20, eventSeq: 0, payload: payload);
      final w = ProtocolWriter();
      encodeFrame(w, frame);
      final decoded = decodeFrame(ProtocolReader(w.toBytes()));
      expect(decoded.seq, equals(20));
      expect(decoded.payload, isA<FramePayloadAck>());
      expect(
        (decoded.payload as FramePayloadAck).data,
        equals(Uint8List.fromList([1, 2, 3, 4])),
      );
    });
  });
}
