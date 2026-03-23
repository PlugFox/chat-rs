import 'dart:typed_data';

import 'package:chat_core/chat_core.dart';
import 'package:test/test.dart';

/// True when running under dart2js / dart2wasm (no Int64 typed-data support).
final bool _isWeb = identical(0, 0.0);

void main() {
  // ---------------------------------------------------------------------------
  // ErrorCode — slug, isPermanent, isTransient
  // ---------------------------------------------------------------------------

  group('ErrorCode slug', () {
    test('every ErrorCode has a non-empty snake_case slug', () {
      for (final code in ErrorCode.values) {
        expect(code.slug, isNotEmpty, reason: '$code slug is empty');
        expect(
          code.slug,
          matches(RegExp(r'^[a-z][a-z0-9_]*$')),
          reason: '${code.slug} is not snake_case',
        );
      }
    });
  });

  group('ErrorCode isPermanent', () {
    test('permanent codes', () {
      expect(ErrorCode.forbidden.isPermanent, isTrue);
      expect(ErrorCode.chatNotFound.isPermanent, isTrue);
      expect(ErrorCode.notChatMember.isPermanent, isTrue);
      expect(ErrorCode.messageTooLarge.isPermanent, isTrue);
      expect(ErrorCode.extraTooLarge.isPermanent, isTrue);
      expect(ErrorCode.contentFiltered.isPermanent, isTrue);
      expect(ErrorCode.unsupportedMediaType.isPermanent, isTrue);
    });

    test('non-permanent codes', () {
      expect(ErrorCode.unauthorized.isPermanent, isFalse);
      expect(ErrorCode.internalError.isPermanent, isFalse);
      expect(ErrorCode.rateLimited.isPermanent, isFalse);
      expect(ErrorCode.malformedFrame.isPermanent, isFalse);
    });
  });

  group('ErrorCode isTransient', () {
    test('transient codes', () {
      expect(ErrorCode.internalError.isTransient, isTrue);
      expect(ErrorCode.serviceUnavailable.isTransient, isTrue);
      expect(ErrorCode.databaseError.isTransient, isTrue);
      expect(ErrorCode.rateLimited.isTransient, isTrue);
    });

    test('non-transient codes', () {
      expect(ErrorCode.unauthorized.isTransient, isFalse);
      expect(ErrorCode.forbidden.isTransient, isFalse);
      expect(ErrorCode.messageTooLarge.isTransient, isFalse);
    });
  });

  // ---------------------------------------------------------------------------
  // DisconnectCode — shouldReconnect
  // ---------------------------------------------------------------------------

  group('DisconnectCode shouldReconnect', () {
    test('reconnectable codes (3000-3499)', () {
      expect(DisconnectCode.serverShutdown.shouldReconnect, isTrue);
      expect(DisconnectCode.sessionExpired.shouldReconnect, isTrue);
      expect(DisconnectCode.duplicateSession.shouldReconnect, isTrue);
      expect(DisconnectCode.serverError.shouldReconnect, isTrue);
      expect(DisconnectCode.bufferOverflow.shouldReconnect, isTrue);
      expect(DisconnectCode.rateLimited.shouldReconnect, isTrue);
      expect(DisconnectCode.eventSeqOverflow.shouldReconnect, isTrue);
    });

    test('non-reconnectable codes (3500+)', () {
      expect(DisconnectCode.tokenInvalid.shouldReconnect, isFalse);
      expect(DisconnectCode.banned.shouldReconnect, isFalse);
      expect(DisconnectCode.unsupportedVersion.shouldReconnect, isFalse);
      expect(DisconnectCode.connectionLimit.shouldReconnect, isFalse);
    });
  });

  // ---------------------------------------------------------------------------
  // SearchScope — all variants toString/equality
  // ---------------------------------------------------------------------------

  group('SearchScope', () {
    test('SearchScopeGlobal equality', () {
      expect(const SearchScopeGlobal(), equals(const SearchScopeGlobal()));
    });

    test('SearchScopeUser equality', () {
      expect(
        const SearchScopeUser(userId: 1),
        equals(const SearchScopeUser(userId: 1)),
      );
      expect(
        const SearchScopeUser(userId: 1),
        isNot(equals(const SearchScopeUser(userId: 2))),
      );
    });
  });

  // ---------------------------------------------------------------------------
  // Writer — unpaired surrogate edge cases
  // ---------------------------------------------------------------------------

  group('Writer surrogate edge cases', () {
    test('unpaired high surrogate at end', () {
      final s = String.fromCharCodes([0x48, 0x69, 0xD800]);
      final w = ProtocolWriter();
      w.writeString(s);
      final r = ProtocolReader(w.toBytes());
      final decoded = r.readString();
      expect(decoded, contains('Hi'));
      expect(decoded.runes.last, 0xFFFD);
    });

    test('unpaired low surrogate', () {
      final s = String.fromCharCodes([0xDC00, 0x41]);
      final w = ProtocolWriter();
      w.writeString(s);
      final r = ProtocolReader(w.toBytes());
      final decoded = r.readString();
      expect(decoded, contains('A'));
    });

    test('high surrogate followed by non-low surrogate', () {
      final s = String.fromCharCodes([0xD800, 0x0041]);
      final w = ProtocolWriter();
      w.writeString(s);
      final r = ProtocolReader(w.toBytes());
      final decoded = r.readString();
      expect(decoded, contains('A'));
    });
  });

  // ---------------------------------------------------------------------------
  // LoadChatsAfter (second variant of sealed class)
  // ---------------------------------------------------------------------------

  group(
    'LoadChatsAfter',
    skip: _isWeb ? 'Int64 not supported by dart2js' : null,
    () {
      test('codec roundtrip', () {
        final original = LoadChatsAfter(cursorTs: 1234567890, limit: 25);
        final w = ProtocolWriter();
        encodeLoadChatsPayload(w, original);
        final decoded = decodeLoadChatsPayload(ProtocolReader(w.toBytes()));
        expect(decoded, isA<LoadChatsAfter>());
        final d = decoded as LoadChatsAfter;
        expect(d.cursorTs, 1234567890);
        expect(d.limit, 25);
      });

      test('equality', () {
        expect(
          const LoadChatsAfter(cursorTs: 100, limit: 10),
          equals(const LoadChatsAfter(cursorTs: 100, limit: 10)),
        );
      });
    },
  );

  // ---------------------------------------------------------------------------
  // LoadMessagesRangeCheck (second variant)
  // ---------------------------------------------------------------------------

  group(
    'LoadMessagesRangeCheck',
    skip: _isWeb ? 'Int64 not supported by dart2js' : null,
    () {
      test('codec roundtrip', () {
        final original = LoadMessagesRangeCheck(
          chatId: 1,
          fromId: 100,
          toId: 200,
          sinceTs: 1234567890,
        );
        final w = ProtocolWriter();
        encodeLoadMessagesPayload(w, original);
        final decoded = decodeLoadMessagesPayload(ProtocolReader(w.toBytes()));
        expect(decoded, isA<LoadMessagesRangeCheck>());
        expect(decoded, equals(original));
      });

      test('equality', () {
        expect(
          const LoadMessagesRangeCheck(
            chatId: 1,
            fromId: 10,
            toId: 20,
            sinceTs: 100,
          ),
          equals(
            const LoadMessagesRangeCheck(
              chatId: 1,
              fromId: 10,
              toId: 20,
              sinceTs: 100,
            ),
          ),
        );
      });
    },
  );

  // ---------------------------------------------------------------------------
  // Frame codec — all remaining payload variants
  // ---------------------------------------------------------------------------

  group(
    'Frame codec exhaustive',
    skip: _isWeb ? 'Int64 not supported by dart2js' : null,
    () {
      void frameRoundtrip(String name, FramePayload payload) {
        test('$name frame roundtrip', () {
          final frame = Frame(seq: 42, eventSeq: 7, payload: payload);
          final w = ProtocolWriter();
          encodeFrame(w, frame);
          final decoded = decodeFrame(ProtocolReader(w.toBytes()));
          expect(decoded.seq, 42);
          expect(decoded.eventSeq, 7);
          expect(decoded.payload.runtimeType, payload.runtimeType);
        });
      }

      // Lifecycle
      frameRoundtrip(
        'Hello',
        FramePayloadHello(
          HelloPayload(
            protocolVersion: 1,
            sdkVersion: '1.0.0',
            platform: 'dart',
            token: 'test-token',
            deviceId: '550e8400-e29b-41d4-a716-446655440000',
          ),
        ),
      );
      frameRoundtrip(
        'Welcome',
        FramePayloadWelcome(
          WelcomePayload(
            sessionId: 1,
            serverTime: 1234567890,
            userId: 1,
            capabilities: ServerCapabilities(0),
            limits: ServerLimits(
              pingIntervalMs: 30000,
              pingTimeoutMs: 10000,
              maxFrameSize: 65536,
              maxMessageSize: 4096,
              maxExtraSize: 1024,
              messagesPerSec: 10,
              connectionsPerIp: 5,
            ),
          ),
        ),
      );
      frameRoundtrip('Pong', const FramePayloadPong());
      frameRoundtrip(
        'RefreshToken',
        FramePayloadRefreshToken(RefreshTokenPayload(token: 'new-token')),
      );

      // Client requests
      frameRoundtrip(
        'SendMessage',
        FramePayloadSendMessage(
          SendMessagePayload(
            chatId: 1,
            idempotencyKey: '550e8400-e29b-41d4-a716-446655440000',
            kind: MessageKind.text,
            content: 'hello',
            mentionedUserIds: [2, 3],
          ),
        ),
      );
      frameRoundtrip(
        'EditMessage',
        FramePayloadEditMessage(
          EditMessagePayload(chatId: 1, messageId: 2, content: 'edited'),
        ),
      );
      frameRoundtrip(
        'ReadReceipt',
        FramePayloadReadReceipt(ReadReceiptPayload(chatId: 1, messageId: 10)),
      );
      frameRoundtrip(
        'Typing',
        FramePayloadTyping(TypingPayload(chatId: 1, expiresInMs: 5000)),
      );
      frameRoundtrip(
        'GetPresence',
        FramePayloadGetPresence(GetPresencePayload(userIds: [1, 2, 3])),
      );
      frameRoundtrip(
        'Search',
        FramePayloadSearch(
          SearchPayload(
            scope: const SearchScopeGlobal(),
            query: 'test',
            cursor: 0,
            limit: 10,
          ),
        ),
      );
      frameRoundtrip(
        'Subscribe',
        FramePayloadSubscribe(SubscribePayload(channels: ['chat:1', 'chat:2'])),
      );
      frameRoundtrip(
        'Unsubscribe',
        FramePayloadUnsubscribe(UnsubscribePayload(channels: ['chat:1'])),
      );
      frameRoundtrip(
        'LoadMessages',
        FramePayloadLoadMessages(
          LoadMessagesPaginate(
            chatId: 1,
            direction: LoadDirection.older,
            anchorId: 0,
            limit: 50,
          ),
        ),
      );
      frameRoundtrip(
        'AddReaction',
        FramePayloadAddReaction(
          AddReactionPayload(chatId: 1, messageId: 2, packId: 0, emojiIndex: 1),
        ),
      );
      frameRoundtrip(
        'RemoveReaction',
        FramePayloadRemoveReaction(
          RemoveReactionPayload(
            chatId: 1,
            messageId: 2,
            packId: 0,
            emojiIndex: 1,
          ),
        ),
      );
      frameRoundtrip(
        'PinMessage',
        FramePayloadPinMessage(PinMessagePayload(chatId: 1, messageId: 5)),
      );
      frameRoundtrip(
        'UnpinMessage',
        FramePayloadUnpinMessage(UnpinMessagePayload(chatId: 1, messageId: 5)),
      );
      frameRoundtrip(
        'ForwardMessage',
        FramePayloadForwardMessage(
          ForwardMessagePayload(
            fromChatId: 1,
            messageId: 10,
            toChatId: 2,
            idempotencyKey: '550e8400-e29b-41d4-a716-446655440000',
          ),
        ),
      );

      // Server events
      frameRoundtrip(
        'MessageNew',
        FramePayloadMessageNew(
          Message(
            id: 1,
            chatId: 1,
            senderId: 1,
            createdAt: 1234567890,
            updatedAt: 1234567890,
            kind: MessageKind.text,
            flags: MessageFlags(0),
            content: 'hello',
          ),
        ),
      );
      frameRoundtrip(
        'MessageEdited',
        FramePayloadMessageEdited(
          Message(
            id: 1,
            chatId: 1,
            senderId: 1,
            createdAt: 1234567890,
            updatedAt: 1234567891,
            kind: MessageKind.text,
            flags: MessageFlags.edited,
            content: 'edited',
          ),
        ),
      );
      frameRoundtrip(
        'MessageDeleted',
        FramePayloadMessageDeleted(
          MessageDeletedPayload(chatId: 1, messageId: 2),
        ),
      );
      frameRoundtrip(
        'ReceiptUpdate',
        FramePayloadReceiptUpdate(
          ReceiptUpdatePayload(chatId: 1, userId: 2, messageId: 10),
        ),
      );
      frameRoundtrip(
        'TypingUpdate',
        FramePayloadTypingUpdate(
          TypingUpdatePayload(chatId: 1, userId: 2, expiresInMs: 5000),
        ),
      );
      frameRoundtrip(
        'MemberJoined',
        FramePayloadMemberJoined(
          MemberJoinedPayload(
            chatId: 1,
            userId: 5,
            role: ChatRole.member,
            invitedBy: 1,
          ),
        ),
      );
      frameRoundtrip(
        'MemberLeft',
        FramePayloadMemberLeft(MemberLeftPayload(chatId: 1, userId: 5)),
      );
      frameRoundtrip(
        'PresenceResult',
        FramePayloadPresenceResult([
          PresenceEntry(
            userId: 1,
            status: PresenceStatus.online,
            lastSeen: 1234567890,
          ),
        ]),
      );
      frameRoundtrip(
        'ChatUpdated',
        FramePayloadChatUpdated(
          ChatEntry(
            id: 1,
            kind: ChatKind.group,
            createdAt: 1234567890,
            updatedAt: 1234567890,
            unreadCount: 0,
            memberCount: 5,
          ),
        ),
      );
      frameRoundtrip(
        'ChatCreated',
        FramePayloadChatCreated(
          ChatEntry(
            id: 2,
            kind: ChatKind.direct,
            createdAt: 1234567890,
            updatedAt: 1234567890,
            unreadCount: 0,
            memberCount: 2,
          ),
        ),
      );
      frameRoundtrip(
        'ReactionUpdate',
        FramePayloadReactionUpdate(
          ReactionUpdatePayload(
            chatId: 1,
            messageId: 2,
            userId: 3,
            packId: 0,
            emojiIndex: 1,
            added: true,
          ),
        ),
      );
      frameRoundtrip(
        'UserUpdated',
        FramePayloadUserUpdated(
          UserEntry(
            id: 1,
            username: 'alice',
            createdAt: 1234567890,
            updatedAt: 1234567890,
            flags: UserFlags(0),
          ),
        ),
      );
      frameRoundtrip(
        'ChatDeleted',
        FramePayloadChatDeleted(ChatDeletedPayload(chatId: 1)),
      );
      frameRoundtrip(
        'MemberUpdated',
        FramePayloadMemberUpdated(
          MemberUpdatedPayload(chatId: 1, userId: 2, role: ChatRole.admin),
        ),
      );
      frameRoundtrip(
        'Error',
        FramePayloadError(
          ErrorPayload(
            code: ErrorCode.unauthorized,
            message: 'bad token',
            retryAfterMs: 0,
          ),
        ),
      );

      // Chat management
      frameRoundtrip(
        'CreateChat',
        FramePayloadCreateChat(
          CreateChatPayload(
            kind: ChatKind.group,
            title: 'Test Chat',
            memberIds: [1, 2, 3],
          ),
        ),
      );
      frameRoundtrip(
        'UpdateChat',
        FramePayloadUpdateChat(UpdateChatPayload(chatId: 1, title: 'New')),
      );
      frameRoundtrip(
        'DeleteChat',
        FramePayloadDeleteChat(DeleteChatPayload(chatId: 1)),
      );
      frameRoundtrip(
        'GetChatInfo',
        FramePayloadGetChatInfo(GetChatInfoPayload(chatId: 1)),
      );
      frameRoundtrip(
        'GetChatMembers',
        FramePayloadGetChatMembers(
          GetChatMembersPayload(chatId: 1, cursor: 0, limit: 50),
        ),
      );
      frameRoundtrip(
        'InviteMembers',
        FramePayloadInviteMembers(
          InviteMembersPayload(chatId: 1, userIds: [10, 20]),
        ),
      );
      frameRoundtrip(
        'UpdateMember',
        FramePayloadUpdateMember(
          UpdateMemberPayload(
            chatId: 1,
            userId: 5,
            action: const MemberActionKick(),
          ),
        ),
      );
      frameRoundtrip(
        'LeaveChat',
        FramePayloadLeaveChat(LeaveChatPayload(chatId: 1)),
      );
      frameRoundtrip(
        'MuteChat',
        FramePayloadMuteChat(MuteChatPayload(chatId: 1, durationSecs: 3600)),
      );
      frameRoundtrip(
        'UnmuteChat',
        FramePayloadUnmuteChat(UnmuteChatPayload(chatId: 1)),
      );

      // User management
      frameRoundtrip('GetUser', FramePayloadGetUser(GetUserPayload(userId: 1)));
      frameRoundtrip(
        'GetUsers',
        FramePayloadGetUsers(GetUsersPayload(userIds: [1, 2, 3])),
      );
      frameRoundtrip(
        'UpdateProfile',
        FramePayloadUpdateProfile(UpdateProfilePayload(username: 'alice')),
      );
      frameRoundtrip(
        'BlockUser',
        FramePayloadBlockUser(BlockUserPayload(userId: 5)),
      );
      frameRoundtrip(
        'UnblockUser',
        FramePayloadUnblockUser(UnblockUserPayload(userId: 5)),
      );
      frameRoundtrip(
        'GetBlockList',
        FramePayloadGetBlockList(GetBlockListPayload(cursor: 0, limit: 50)),
      );

      // Edge cases
      test('decodeFrame throws on unknown kind', () {
        final w = ProtocolWriter();
        w.writeU8(255); // invalid frame kind
        w.writeU32(0);
        w.writeU32(0);
        expect(
          () => decodeFrame(ProtocolReader(w.toBytes())),
          throwsA(isA<CodecError>()),
        );
      });

      test('Ack with empty payload', () {
        final frame = Frame(
          seq: 1,
          eventSeq: 0,
          payload: FramePayloadAck(Uint8List(0)),
        );
        final w = ProtocolWriter();
        encodeFrame(w, frame);
        final decoded = decodeFrame(ProtocolReader(w.toBytes()));
        expect(decoded.payload, isA<FramePayloadAck>());
        expect((decoded.payload as FramePayloadAck).data, isEmpty);
      });
    },
  );
}
