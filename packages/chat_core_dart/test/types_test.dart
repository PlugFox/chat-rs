// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'dart:typed_data';

import 'package:test/test.dart';
import 'package:chat_core/chat_core.dart';

void main() {
  group('ChatKind', () {
    test('fromValue roundtrip', () {
      expect(ChatKind.fromValue(ChatKind.direct.value), ChatKind.direct);
      expect(ChatKind.fromValue(ChatKind.group.value), ChatKind.group);
      expect(ChatKind.fromValue(ChatKind.channel.value), ChatKind.channel);
    });
    test('fromValue returns null for invalid', () {
      expect(ChatKind.fromValue(255), isNull);
    });
  });

  group('ChatRole', () {
    test('fromValue roundtrip', () {
      expect(ChatRole.fromValue(ChatRole.member.value), ChatRole.member);
      expect(ChatRole.fromValue(ChatRole.moderator.value), ChatRole.moderator);
      expect(ChatRole.fromValue(ChatRole.admin.value), ChatRole.admin);
      expect(ChatRole.fromValue(ChatRole.owner.value), ChatRole.owner);
    });
    test('fromValue returns null for invalid', () {
      expect(ChatRole.fromValue(255), isNull);
    });
  });

  group('MessageKind', () {
    test('fromValue roundtrip', () {
      expect(MessageKind.fromValue(MessageKind.text.value), MessageKind.text);
      expect(MessageKind.fromValue(MessageKind.image.value), MessageKind.image);
      expect(MessageKind.fromValue(MessageKind.file.value), MessageKind.file);
      expect(
        MessageKind.fromValue(MessageKind.system.value),
        MessageKind.system,
      );
    });
    test('fromValue returns null for invalid', () {
      expect(MessageKind.fromValue(255), isNull);
    });
  });

  group('PresenceStatus', () {
    test('fromValue roundtrip', () {
      expect(
        PresenceStatus.fromValue(PresenceStatus.offline.value),
        PresenceStatus.offline,
      );
      expect(
        PresenceStatus.fromValue(PresenceStatus.online.value),
        PresenceStatus.online,
      );
    });
    test('fromValue returns null for invalid', () {
      expect(PresenceStatus.fromValue(255), isNull);
    });
  });

  group('ErrorCode', () {
    test('fromValue roundtrip', () {
      expect(
        ErrorCode.fromValue(ErrorCode.unauthorized.value),
        ErrorCode.unauthorized,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.tokenExpired.value),
        ErrorCode.tokenExpired,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.forbidden.value),
        ErrorCode.forbidden,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.sessionRevoked.value),
        ErrorCode.sessionRevoked,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.unsupportedVersion.value),
        ErrorCode.unsupportedVersion,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.chatNotFound.value),
        ErrorCode.chatNotFound,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.chatAlreadyExists.value),
        ErrorCode.chatAlreadyExists,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.notChatMember.value),
        ErrorCode.notChatMember,
      );
      expect(ErrorCode.fromValue(ErrorCode.chatFull.value), ErrorCode.chatFull);
      expect(
        ErrorCode.fromValue(ErrorCode.messageNotFound.value),
        ErrorCode.messageNotFound,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.messageTooLarge.value),
        ErrorCode.messageTooLarge,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.extraTooLarge.value),
        ErrorCode.extraTooLarge,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.rateLimited.value),
        ErrorCode.rateLimited,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.contentFiltered.value),
        ErrorCode.contentFiltered,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.fileTooLarge.value),
        ErrorCode.fileTooLarge,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.unsupportedMediaType.value),
        ErrorCode.unsupportedMediaType,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.uploadFailed.value),
        ErrorCode.uploadFailed,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.internalError.value),
        ErrorCode.internalError,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.serviceUnavailable.value),
        ErrorCode.serviceUnavailable,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.databaseError.value),
        ErrorCode.databaseError,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.malformedFrame.value),
        ErrorCode.malformedFrame,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.unknownCommand.value),
        ErrorCode.unknownCommand,
      );
      expect(
        ErrorCode.fromValue(ErrorCode.frameTooLarge.value),
        ErrorCode.frameTooLarge,
      );
    });
    test('fromValue returns null for invalid', () {
      expect(ErrorCode.fromValue(255), isNull);
    });
  });

  group('DisconnectCode', () {
    test('fromValue roundtrip', () {
      expect(
        DisconnectCode.fromValue(DisconnectCode.serverShutdown.value),
        DisconnectCode.serverShutdown,
      );
      expect(
        DisconnectCode.fromValue(DisconnectCode.sessionExpired.value),
        DisconnectCode.sessionExpired,
      );
      expect(
        DisconnectCode.fromValue(DisconnectCode.duplicateSession.value),
        DisconnectCode.duplicateSession,
      );
      expect(
        DisconnectCode.fromValue(DisconnectCode.serverError.value),
        DisconnectCode.serverError,
      );
      expect(
        DisconnectCode.fromValue(DisconnectCode.bufferOverflow.value),
        DisconnectCode.bufferOverflow,
      );
      expect(
        DisconnectCode.fromValue(DisconnectCode.rateLimited.value),
        DisconnectCode.rateLimited,
      );
      expect(
        DisconnectCode.fromValue(DisconnectCode.eventSeqOverflow.value),
        DisconnectCode.eventSeqOverflow,
      );
      expect(
        DisconnectCode.fromValue(DisconnectCode.tokenInvalid.value),
        DisconnectCode.tokenInvalid,
      );
      expect(
        DisconnectCode.fromValue(DisconnectCode.banned.value),
        DisconnectCode.banned,
      );
      expect(
        DisconnectCode.fromValue(DisconnectCode.unsupportedVersion.value),
        DisconnectCode.unsupportedVersion,
      );
      expect(
        DisconnectCode.fromValue(DisconnectCode.connectionLimit.value),
        DisconnectCode.connectionLimit,
      );
    });
    test('fromValue returns null for invalid', () {
      expect(DisconnectCode.fromValue(255), isNull);
    });
  });

  group('FrameKind', () {
    test('fromValue roundtrip', () {
      expect(FrameKind.fromValue(FrameKind.hello.value), FrameKind.hello);
      expect(FrameKind.fromValue(FrameKind.welcome.value), FrameKind.welcome);
      expect(FrameKind.fromValue(FrameKind.ping.value), FrameKind.ping);
      expect(FrameKind.fromValue(FrameKind.pong.value), FrameKind.pong);
      expect(
        FrameKind.fromValue(FrameKind.refreshToken.value),
        FrameKind.refreshToken,
      );
      expect(
        FrameKind.fromValue(FrameKind.sendMessage.value),
        FrameKind.sendMessage,
      );
      expect(
        FrameKind.fromValue(FrameKind.editMessage.value),
        FrameKind.editMessage,
      );
      expect(
        FrameKind.fromValue(FrameKind.deleteMessage.value),
        FrameKind.deleteMessage,
      );
      expect(
        FrameKind.fromValue(FrameKind.readReceipt.value),
        FrameKind.readReceipt,
      );
      expect(FrameKind.fromValue(FrameKind.typing.value), FrameKind.typing);
      expect(
        FrameKind.fromValue(FrameKind.getPresence.value),
        FrameKind.getPresence,
      );
      expect(
        FrameKind.fromValue(FrameKind.loadChats.value),
        FrameKind.loadChats,
      );
      expect(FrameKind.fromValue(FrameKind.search.value), FrameKind.search);
      expect(
        FrameKind.fromValue(FrameKind.subscribe.value),
        FrameKind.subscribe,
      );
      expect(
        FrameKind.fromValue(FrameKind.unsubscribe.value),
        FrameKind.unsubscribe,
      );
      expect(
        FrameKind.fromValue(FrameKind.loadMessages.value),
        FrameKind.loadMessages,
      );
      expect(
        FrameKind.fromValue(FrameKind.addReaction.value),
        FrameKind.addReaction,
      );
      expect(
        FrameKind.fromValue(FrameKind.removeReaction.value),
        FrameKind.removeReaction,
      );
      expect(
        FrameKind.fromValue(FrameKind.pinMessage.value),
        FrameKind.pinMessage,
      );
      expect(
        FrameKind.fromValue(FrameKind.unpinMessage.value),
        FrameKind.unpinMessage,
      );
      expect(
        FrameKind.fromValue(FrameKind.forwardMessage.value),
        FrameKind.forwardMessage,
      );
      expect(
        FrameKind.fromValue(FrameKind.messageNew.value),
        FrameKind.messageNew,
      );
      expect(
        FrameKind.fromValue(FrameKind.messageEdited.value),
        FrameKind.messageEdited,
      );
      expect(
        FrameKind.fromValue(FrameKind.messageDeleted.value),
        FrameKind.messageDeleted,
      );
      expect(
        FrameKind.fromValue(FrameKind.receiptUpdate.value),
        FrameKind.receiptUpdate,
      );
      expect(
        FrameKind.fromValue(FrameKind.typingUpdate.value),
        FrameKind.typingUpdate,
      );
      expect(
        FrameKind.fromValue(FrameKind.memberJoined.value),
        FrameKind.memberJoined,
      );
      expect(
        FrameKind.fromValue(FrameKind.memberLeft.value),
        FrameKind.memberLeft,
      );
      expect(
        FrameKind.fromValue(FrameKind.presenceResult.value),
        FrameKind.presenceResult,
      );
      expect(
        FrameKind.fromValue(FrameKind.chatUpdated.value),
        FrameKind.chatUpdated,
      );
      expect(
        FrameKind.fromValue(FrameKind.chatCreated.value),
        FrameKind.chatCreated,
      );
      expect(
        FrameKind.fromValue(FrameKind.reactionUpdate.value),
        FrameKind.reactionUpdate,
      );
      expect(
        FrameKind.fromValue(FrameKind.userUpdated.value),
        FrameKind.userUpdated,
      );
      expect(
        FrameKind.fromValue(FrameKind.chatDeleted.value),
        FrameKind.chatDeleted,
      );
      expect(
        FrameKind.fromValue(FrameKind.memberUpdated.value),
        FrameKind.memberUpdated,
      );
      expect(FrameKind.fromValue(FrameKind.ack.value), FrameKind.ack);
      expect(FrameKind.fromValue(FrameKind.error.value), FrameKind.error);
      expect(
        FrameKind.fromValue(FrameKind.createChat.value),
        FrameKind.createChat,
      );
      expect(
        FrameKind.fromValue(FrameKind.updateChat.value),
        FrameKind.updateChat,
      );
      expect(
        FrameKind.fromValue(FrameKind.deleteChat.value),
        FrameKind.deleteChat,
      );
      expect(
        FrameKind.fromValue(FrameKind.getChatInfo.value),
        FrameKind.getChatInfo,
      );
      expect(
        FrameKind.fromValue(FrameKind.getChatMembers.value),
        FrameKind.getChatMembers,
      );
      expect(
        FrameKind.fromValue(FrameKind.inviteMembers.value),
        FrameKind.inviteMembers,
      );
      expect(
        FrameKind.fromValue(FrameKind.updateMember.value),
        FrameKind.updateMember,
      );
      expect(
        FrameKind.fromValue(FrameKind.leaveChat.value),
        FrameKind.leaveChat,
      );
      expect(FrameKind.fromValue(FrameKind.muteChat.value), FrameKind.muteChat);
      expect(
        FrameKind.fromValue(FrameKind.unmuteChat.value),
        FrameKind.unmuteChat,
      );
      expect(FrameKind.fromValue(FrameKind.getUser.value), FrameKind.getUser);
      expect(FrameKind.fromValue(FrameKind.getUsers.value), FrameKind.getUsers);
      expect(
        FrameKind.fromValue(FrameKind.updateProfile.value),
        FrameKind.updateProfile,
      );
      expect(
        FrameKind.fromValue(FrameKind.blockUser.value),
        FrameKind.blockUser,
      );
      expect(
        FrameKind.fromValue(FrameKind.unblockUser.value),
        FrameKind.unblockUser,
      );
      expect(
        FrameKind.fromValue(FrameKind.getBlockList.value),
        FrameKind.getBlockList,
      );
    });
    test('fromValue returns null for invalid', () {
      expect(FrameKind.fromValue(255), isNull);
    });
  });

  group('LoadDirection', () {
    test('fromValue roundtrip', () {
      expect(
        LoadDirection.fromValue(LoadDirection.older.value),
        LoadDirection.older,
      );
      expect(
        LoadDirection.fromValue(LoadDirection.newer.value),
        LoadDirection.newer,
      );
    });
    test('fromValue returns null for invalid', () {
      expect(LoadDirection.fromValue(255), isNull);
    });
  });

  group('Permission', () {
    test('contains', () {
      final flags = Permission.sendMessages;
      expect(flags.contains(Permission.sendMessages), isTrue);
      expect(flags.contains(Permission.sendMedia), isFalse);
    });
    test('add and remove', () {
      var flags = Permission.sendMessages;
      flags = flags.add(Permission.sendMedia);
      expect(flags.contains(Permission.sendMessages), isTrue);
      expect(flags.contains(Permission.sendMedia), isTrue);
      flags = flags.remove(Permission.sendMessages);
      expect(flags.contains(Permission.sendMessages), isFalse);
      expect(flags.contains(Permission.sendMedia), isTrue);
    });
    test('toggle', () {
      var flags = Permission.sendMessages;
      flags = flags.toggle(Permission.sendMessages);
      expect(flags.isEmpty, isTrue);
      flags = flags.toggle(Permission.sendMessages);
      expect(flags.contains(Permission.sendMessages), isTrue);
    });
    test('isEmpty', () {
      expect(const Permission(0).isEmpty, isTrue);
      expect(Permission.sendMessages.isEmpty, isFalse);
      expect(Permission.sendMessages.isNotEmpty, isTrue);
    });
  });

  group('MessageFlags', () {
    test('contains', () {
      final flags = MessageFlags.edited;
      expect(flags.contains(MessageFlags.edited), isTrue);
      expect(flags.contains(MessageFlags.deleted), isFalse);
    });
    test('add and remove', () {
      var flags = MessageFlags.edited;
      flags = flags.add(MessageFlags.deleted);
      expect(flags.contains(MessageFlags.edited), isTrue);
      expect(flags.contains(MessageFlags.deleted), isTrue);
      flags = flags.remove(MessageFlags.edited);
      expect(flags.contains(MessageFlags.edited), isFalse);
      expect(flags.contains(MessageFlags.deleted), isTrue);
    });
    test('toggle', () {
      var flags = MessageFlags.edited;
      flags = flags.toggle(MessageFlags.edited);
      expect(flags.isEmpty, isTrue);
      flags = flags.toggle(MessageFlags.edited);
      expect(flags.contains(MessageFlags.edited), isTrue);
    });
    test('isEmpty', () {
      expect(const MessageFlags(0).isEmpty, isTrue);
      expect(MessageFlags.edited.isEmpty, isFalse);
      expect(MessageFlags.edited.isNotEmpty, isTrue);
    });
  });

  group('RichStyle', () {
    test('contains', () {
      final flags = RichStyle.bold;
      expect(flags.contains(RichStyle.bold), isTrue);
      expect(flags.contains(RichStyle.italic), isFalse);
    });
    test('add and remove', () {
      var flags = RichStyle.bold;
      flags = flags.add(RichStyle.italic);
      expect(flags.contains(RichStyle.bold), isTrue);
      expect(flags.contains(RichStyle.italic), isTrue);
      flags = flags.remove(RichStyle.bold);
      expect(flags.contains(RichStyle.bold), isFalse);
      expect(flags.contains(RichStyle.italic), isTrue);
    });
    test('toggle', () {
      var flags = RichStyle.bold;
      flags = flags.toggle(RichStyle.bold);
      expect(flags.isEmpty, isTrue);
      flags = flags.toggle(RichStyle.bold);
      expect(flags.contains(RichStyle.bold), isTrue);
    });
    test('isEmpty', () {
      expect(const RichStyle(0).isEmpty, isTrue);
      expect(RichStyle.bold.isEmpty, isFalse);
      expect(RichStyle.bold.isNotEmpty, isTrue);
    });
  });

  group('UserFlags', () {
    test('contains', () {
      final flags = UserFlags.system;
      expect(flags.contains(UserFlags.system), isTrue);
      expect(flags.contains(UserFlags.bot), isFalse);
    });
    test('add and remove', () {
      var flags = UserFlags.system;
      flags = flags.add(UserFlags.bot);
      expect(flags.contains(UserFlags.system), isTrue);
      expect(flags.contains(UserFlags.bot), isTrue);
      flags = flags.remove(UserFlags.system);
      expect(flags.contains(UserFlags.system), isFalse);
      expect(flags.contains(UserFlags.bot), isTrue);
    });
    test('toggle', () {
      var flags = UserFlags.system;
      flags = flags.toggle(UserFlags.system);
      expect(flags.isEmpty, isTrue);
      flags = flags.toggle(UserFlags.system);
      expect(flags.contains(UserFlags.system), isTrue);
    });
    test('isEmpty', () {
      expect(const UserFlags(0).isEmpty, isTrue);
      expect(UserFlags.system.isEmpty, isFalse);
      expect(UserFlags.system.isNotEmpty, isTrue);
    });
  });

  group('ServerCapabilities', () {
    test('contains', () {
      final flags = ServerCapabilities.mediaUpload;
      expect(flags.contains(ServerCapabilities.mediaUpload), isTrue);
      expect(flags.contains(ServerCapabilities.search), isFalse);
    });
    test('add and remove', () {
      var flags = ServerCapabilities.mediaUpload;
      flags = flags.add(ServerCapabilities.search);
      expect(flags.contains(ServerCapabilities.mediaUpload), isTrue);
      expect(flags.contains(ServerCapabilities.search), isTrue);
      flags = flags.remove(ServerCapabilities.mediaUpload);
      expect(flags.contains(ServerCapabilities.mediaUpload), isFalse);
      expect(flags.contains(ServerCapabilities.search), isTrue);
    });
    test('toggle', () {
      var flags = ServerCapabilities.mediaUpload;
      flags = flags.toggle(ServerCapabilities.mediaUpload);
      expect(flags.isEmpty, isTrue);
      flags = flags.toggle(ServerCapabilities.mediaUpload);
      expect(flags.contains(ServerCapabilities.mediaUpload), isTrue);
    });
    test('isEmpty', () {
      expect(const ServerCapabilities(0).isEmpty, isTrue);
      expect(ServerCapabilities.mediaUpload.isEmpty, isFalse);
      expect(ServerCapabilities.mediaUpload.isNotEmpty, isTrue);
    });
  });

  group('LastMessagePreview', () {
    test('equality', () {
      final a = LastMessagePreview(
        id: 100000,
        senderId: 100000,
        createdAt: 1234567890,
        kind: MessageKind.text,
        flags: MessageFlags.edited,
        contentPreview: 'hello',
      );
      final b = LastMessagePreview(
        id: 100000,
        senderId: 100000,
        createdAt: 1234567890,
        kind: MessageKind.text,
        flags: MessageFlags.edited,
        contentPreview: 'hello',
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('ChatEntry', () {
    test('equality', () {
      final a = ChatEntry(
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
      final b = ChatEntry(
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
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('ChatMemberEntry', () {
    test('equality', () {
      final a = ChatMemberEntry(
        userId: 100000,
        role: ChatRole.member,
        permissions: Permission.sendMessages,
      );
      final b = ChatMemberEntry(
        userId: 100000,
        role: ChatRole.member,
        permissions: Permission.sendMessages,
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('RichSpan', () {
    test('equality', () {
      final a = RichSpan(
        start: 100000,
        end: 100000,
        style: RichStyle.bold,
        meta: 'test',
      );
      final b = RichSpan(
        start: 100000,
        end: 100000,
        style: RichStyle.bold,
        meta: 'test',
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('Message', () {
    test('equality', () {
      final a = Message(
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
      final b = Message(
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
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('MessageBatch', () {
    test('equality', () {
      final a = MessageBatch(
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
      final b = MessageBatch(
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
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('UserEntry', () {
    test('equality', () {
      final a = UserEntry(
        id: 100000,
        flags: UserFlags.system,
        createdAt: 1234567890,
        updatedAt: 1234567890,
        username: 'test',
        firstName: 'test',
        lastName: 'test',
        avatarUrl: 'test',
      );
      final b = UserEntry(
        id: 100000,
        flags: UserFlags.system,
        createdAt: 1234567890,
        updatedAt: 1234567890,
        username: 'test',
        firstName: 'test',
        lastName: 'test',
        avatarUrl: 'test',
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('PresenceEntry', () {
    test('equality', () {
      final a = PresenceEntry(
        userId: 100000,
        status: PresenceStatus.offline,
        lastSeen: 1234567890,
      );
      final b = PresenceEntry(
        userId: 100000,
        status: PresenceStatus.offline,
        lastSeen: 1234567890,
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('ErrorPayload', () {
    test('equality', () {
      final a = ErrorPayload(
        code: ErrorCode.unauthorized,
        message: 'hello',
        retryAfterMs: 100000,
        extra: 'test',
      );
      final b = ErrorPayload(
        code: ErrorCode.unauthorized,
        message: 'hello',
        retryAfterMs: 100000,
        extra: 'test',
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('HelloPayload', () {
    test('equality', () {
      final a = HelloPayload(
        protocolVersion: 42,
        sdkVersion: 'hello',
        platform: 'hello',
        token: 'hello',
        deviceId: '550e8400-e29b-41d4-a716-446655440000',
      );
      final b = HelloPayload(
        protocolVersion: 42,
        sdkVersion: 'hello',
        platform: 'hello',
        token: 'hello',
        deviceId: '550e8400-e29b-41d4-a716-446655440000',
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('WelcomePayload', () {
    test('equality', () {
      final a = WelcomePayload(
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
      final b = WelcomePayload(
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
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('ServerLimits', () {
    test('equality', () {
      final a = ServerLimits(
        pingIntervalMs: 100000,
        pingTimeoutMs: 100000,
        maxMessageSize: 100000,
        maxExtraSize: 100000,
        maxFrameSize: 100000,
        messagesPerSec: 1000,
        connectionsPerIp: 1000,
      );
      final b = ServerLimits(
        pingIntervalMs: 100000,
        pingTimeoutMs: 100000,
        maxMessageSize: 100000,
        maxExtraSize: 100000,
        maxFrameSize: 100000,
        messagesPerSec: 1000,
        connectionsPerIp: 1000,
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('SendMessagePayload', () {
    test('equality', () {
      final a = SendMessagePayload(
        chatId: 100000,
        kind: MessageKind.text,
        idempotencyKey: '550e8400-e29b-41d4-a716-446655440000',
        replyToId: 7,
        content: 'hello',
        richContent: Uint8List.fromList([1, 2]),
        extra: 'test',
        mentionedUserIds: [1, 2, 3],
      );
      final b = SendMessagePayload(
        chatId: 100000,
        kind: MessageKind.text,
        idempotencyKey: '550e8400-e29b-41d4-a716-446655440000',
        replyToId: 7,
        content: 'hello',
        richContent: Uint8List.fromList([1, 2]),
        extra: 'test',
        mentionedUserIds: [1, 2, 3],
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('EditMessagePayload', () {
    test('equality', () {
      final a = EditMessagePayload(
        chatId: 100000,
        messageId: 100000,
        content: 'hello',
        richContent: Uint8List.fromList([1, 2]),
        extra: 'test',
      );
      final b = EditMessagePayload(
        chatId: 100000,
        messageId: 100000,
        content: 'hello',
        richContent: Uint8List.fromList([1, 2]),
        extra: 'test',
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('DeleteMessagePayload', () {
    test('equality', () {
      final a = DeleteMessagePayload(chatId: 100000, messageId: 100000);
      final b = DeleteMessagePayload(chatId: 100000, messageId: 100000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('ReadReceiptPayload', () {
    test('equality', () {
      final a = ReadReceiptPayload(chatId: 100000, messageId: 100000);
      final b = ReadReceiptPayload(chatId: 100000, messageId: 100000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('TypingPayload', () {
    test('equality', () {
      final a = TypingPayload(chatId: 100000, expiresInMs: 1000);
      final b = TypingPayload(chatId: 100000, expiresInMs: 1000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('GetPresencePayload', () {
    test('equality', () {
      final a = GetPresencePayload(userIds: [1, 2, 3]);
      final b = GetPresencePayload(userIds: [1, 2, 3]);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('SearchPayload', () {
    test('equality', () {
      final a = SearchPayload(
        scope: SearchScopeChat(chatId: 100000),
        query: 'hello',
        cursor: 100000,
        limit: 1000,
      );
      final b = SearchPayload(
        scope: SearchScopeChat(chatId: 100000),
        query: 'hello',
        cursor: 100000,
        limit: 1000,
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('SubscribePayload', () {
    test('equality', () {
      final a = SubscribePayload(channels: ['a', 'b']);
      final b = SubscribePayload(channels: ['a', 'b']);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('UnsubscribePayload', () {
    test('equality', () {
      final a = UnsubscribePayload(channels: ['a', 'b']);
      final b = UnsubscribePayload(channels: ['a', 'b']);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('CreateChatPayload', () {
    test('equality', () {
      final a = CreateChatPayload(
        kind: ChatKind.direct,
        parentId: 7,
        title: 'test',
        avatarUrl: 'test',
        memberIds: [1, 2, 3],
      );
      final b = CreateChatPayload(
        kind: ChatKind.direct,
        parentId: 7,
        title: 'test',
        avatarUrl: 'test',
        memberIds: [1, 2, 3],
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('UpdateChatPayload', () {
    test('equality', () {
      final a = UpdateChatPayload(
        chatId: 100000,
        title: 'updated',
        avatarUrl: 'updated',
      );
      final b = UpdateChatPayload(
        chatId: 100000,
        title: 'updated',
        avatarUrl: 'updated',
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('DeleteChatPayload', () {
    test('equality', () {
      final a = DeleteChatPayload(chatId: 100000);
      final b = DeleteChatPayload(chatId: 100000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('GetChatInfoPayload', () {
    test('equality', () {
      final a = GetChatInfoPayload(chatId: 100000);
      final b = GetChatInfoPayload(chatId: 100000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('GetChatMembersPayload', () {
    test('equality', () {
      final a = GetChatMembersPayload(
        chatId: 100000,
        cursor: 100000,
        limit: 1000,
      );
      final b = GetChatMembersPayload(
        chatId: 100000,
        cursor: 100000,
        limit: 1000,
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('InviteMembersPayload', () {
    test('equality', () {
      final a = InviteMembersPayload(chatId: 100000, userIds: [1, 2, 3]);
      final b = InviteMembersPayload(chatId: 100000, userIds: [1, 2, 3]);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('LeaveChatPayload', () {
    test('equality', () {
      final a = LeaveChatPayload(chatId: 100000);
      final b = LeaveChatPayload(chatId: 100000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('UpdateMemberPayload', () {
    test('equality', () {
      final a = UpdateMemberPayload(
        chatId: 100000,
        userId: 100000,
        action: const MemberActionKick(),
      );
      final b = UpdateMemberPayload(
        chatId: 100000,
        userId: 100000,
        action: const MemberActionKick(),
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('MessageDeletedPayload', () {
    test('equality', () {
      final a = MessageDeletedPayload(chatId: 100000, messageId: 100000);
      final b = MessageDeletedPayload(chatId: 100000, messageId: 100000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('ReceiptUpdatePayload', () {
    test('equality', () {
      final a = ReceiptUpdatePayload(
        chatId: 100000,
        userId: 100000,
        messageId: 100000,
      );
      final b = ReceiptUpdatePayload(
        chatId: 100000,
        userId: 100000,
        messageId: 100000,
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('TypingUpdatePayload', () {
    test('equality', () {
      final a = TypingUpdatePayload(
        chatId: 100000,
        userId: 100000,
        expiresInMs: 1000,
      );
      final b = TypingUpdatePayload(
        chatId: 100000,
        userId: 100000,
        expiresInMs: 1000,
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('MemberJoinedPayload', () {
    test('equality', () {
      final a = MemberJoinedPayload(
        chatId: 100000,
        userId: 100000,
        role: ChatRole.member,
        invitedBy: 100000,
      );
      final b = MemberJoinedPayload(
        chatId: 100000,
        userId: 100000,
        role: ChatRole.member,
        invitedBy: 100000,
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('MemberLeftPayload', () {
    test('equality', () {
      final a = MemberLeftPayload(chatId: 100000, userId: 100000);
      final b = MemberLeftPayload(chatId: 100000, userId: 100000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('ChatDeletedPayload', () {
    test('equality', () {
      final a = ChatDeletedPayload(chatId: 100000);
      final b = ChatDeletedPayload(chatId: 100000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('MemberUpdatedPayload', () {
    test('equality', () {
      final a = MemberUpdatedPayload(
        chatId: 100000,
        userId: 100000,
        role: ChatRole.member,
        permissions: Permission.sendMessages,
      );
      final b = MemberUpdatedPayload(
        chatId: 100000,
        userId: 100000,
        role: ChatRole.member,
        permissions: Permission.sendMessages,
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('AddReactionPayload', () {
    test('equality', () {
      final a = AddReactionPayload(
        chatId: 100000,
        messageId: 100000,
        packId: 100000,
        emojiIndex: 42,
      );
      final b = AddReactionPayload(
        chatId: 100000,
        messageId: 100000,
        packId: 100000,
        emojiIndex: 42,
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('RemoveReactionPayload', () {
    test('equality', () {
      final a = RemoveReactionPayload(
        chatId: 100000,
        messageId: 100000,
        packId: 100000,
        emojiIndex: 42,
      );
      final b = RemoveReactionPayload(
        chatId: 100000,
        messageId: 100000,
        packId: 100000,
        emojiIndex: 42,
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('ReactionUpdatePayload', () {
    test('equality', () {
      final a = ReactionUpdatePayload(
        chatId: 100000,
        messageId: 100000,
        userId: 100000,
        packId: 100000,
        emojiIndex: 42,
        added: true,
      );
      final b = ReactionUpdatePayload(
        chatId: 100000,
        messageId: 100000,
        userId: 100000,
        packId: 100000,
        emojiIndex: 42,
        added: true,
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('PinMessagePayload', () {
    test('equality', () {
      final a = PinMessagePayload(chatId: 100000, messageId: 100000);
      final b = PinMessagePayload(chatId: 100000, messageId: 100000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('UnpinMessagePayload', () {
    test('equality', () {
      final a = UnpinMessagePayload(chatId: 100000, messageId: 100000);
      final b = UnpinMessagePayload(chatId: 100000, messageId: 100000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('RefreshTokenPayload', () {
    test('equality', () {
      final a = RefreshTokenPayload(token: 'hello');
      final b = RefreshTokenPayload(token: 'hello');
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('ForwardMessagePayload', () {
    test('equality', () {
      final a = ForwardMessagePayload(
        fromChatId: 100000,
        messageId: 100000,
        toChatId: 100000,
        idempotencyKey: '550e8400-e29b-41d4-a716-446655440000',
      );
      final b = ForwardMessagePayload(
        fromChatId: 100000,
        messageId: 100000,
        toChatId: 100000,
        idempotencyKey: '550e8400-e29b-41d4-a716-446655440000',
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('GetUserPayload', () {
    test('equality', () {
      final a = GetUserPayload(userId: 100000);
      final b = GetUserPayload(userId: 100000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('GetUsersPayload', () {
    test('equality', () {
      final a = GetUsersPayload(userIds: [1, 2, 3]);
      final b = GetUsersPayload(userIds: [1, 2, 3]);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('UpdateProfilePayload', () {
    test('equality', () {
      final a = UpdateProfilePayload(
        username: 'updated',
        firstName: 'updated',
        lastName: 'updated',
        avatarUrl: 'updated',
      );
      final b = UpdateProfilePayload(
        username: 'updated',
        firstName: 'updated',
        lastName: 'updated',
        avatarUrl: 'updated',
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('BlockUserPayload', () {
    test('equality', () {
      final a = BlockUserPayload(userId: 100000);
      final b = BlockUserPayload(userId: 100000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('UnblockUserPayload', () {
    test('equality', () {
      final a = UnblockUserPayload(userId: 100000);
      final b = UnblockUserPayload(userId: 100000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('GetBlockListPayload', () {
    test('equality', () {
      final a = GetBlockListPayload(cursor: 100000, limit: 1000);
      final b = GetBlockListPayload(cursor: 100000, limit: 1000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('MuteChatPayload', () {
    test('equality', () {
      final a = MuteChatPayload(chatId: 100000, durationSecs: 100000);
      final b = MuteChatPayload(chatId: 100000, durationSecs: 100000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('UnmuteChatPayload', () {
    test('equality', () {
      final a = UnmuteChatPayload(chatId: 100000);
      final b = UnmuteChatPayload(chatId: 100000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('LoadChatsPayload', () {
    test('FirstPage equality', () {
      final a = LoadChatsFirstPage(limit: 1000);
      final b = LoadChatsFirstPage(limit: 1000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
    test('After equality', () {
      final a = LoadChatsAfter(cursorTs: 1234567890, limit: 1000);
      final b = LoadChatsAfter(cursorTs: 1234567890, limit: 1000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('SearchScope', () {
    test('Chat equality', () {
      final a = SearchScopeChat(chatId: 100000);
      final b = SearchScopeChat(chatId: 100000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
    test('Global equality', () {
      final a = const SearchScopeGlobal();
      final b = const SearchScopeGlobal();
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
    test('User equality', () {
      final a = SearchScopeUser(userId: 100000);
      final b = SearchScopeUser(userId: 100000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('LoadMessagesPayload', () {
    test('Paginate equality', () {
      final a = LoadMessagesPaginate(
        chatId: 100000,
        direction: LoadDirection.older,
        anchorId: 100000,
        limit: 1000,
      );
      final b = LoadMessagesPaginate(
        chatId: 100000,
        direction: LoadDirection.older,
        anchorId: 100000,
        limit: 1000,
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
    test('RangeCheck equality', () {
      final a = LoadMessagesRangeCheck(
        chatId: 100000,
        fromId: 100000,
        toId: 100000,
        sinceTs: 1234567890,
      );
      final b = LoadMessagesRangeCheck(
        chatId: 100000,
        fromId: 100000,
        toId: 100000,
        sinceTs: 1234567890,
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
    test('Chunk equality', () {
      final a = LoadMessagesChunk(
        chatId: 100000,
        chunkId: 100000,
        sinceTs: 1234567890,
      );
      final b = LoadMessagesChunk(
        chatId: 100000,
        chunkId: 100000,
        sinceTs: 1234567890,
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });

  group('MemberAction', () {
    test('Kick equality', () {
      final a = const MemberActionKick();
      final b = const MemberActionKick();
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
    test('Ban equality', () {
      final a = const MemberActionBan();
      final b = const MemberActionBan();
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
    test('Mute equality', () {
      final a = MemberActionMute(durationSecs: 100000);
      final b = MemberActionMute(durationSecs: 100000);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
    test('ChangeRole equality', () {
      final a = MemberActionChangeRole(chatRole: ChatRole.member);
      final b = MemberActionChangeRole(chatRole: ChatRole.member);
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
    test('UpdatePermissions equality', () {
      final a = MemberActionUpdatePermissions(
        permission: Permission.sendMessages,
      );
      final b = MemberActionUpdatePermissions(
        permission: Permission.sendMessages,
      );
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
    test('Unban equality', () {
      final a = const MemberActionUnban();
      final b = const MemberActionUnban();
      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
    });
  });
}
