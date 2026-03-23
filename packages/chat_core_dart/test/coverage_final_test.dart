import 'dart:typed_data';

import 'package:chat_core/chat_core.dart';
import 'package:test/test.dart';

/// True when running under dart2js / dart2wasm.
final bool _isWeb = identical(0, 0.0);

void main() {
  // ---------------------------------------------------------------------------
  // Bitflag types — isNotEmpty and operator ^
  // ---------------------------------------------------------------------------

  group('MessageFlags extended', () {
    test('isNotEmpty', () {
      expect(MessageFlags(0).isNotEmpty, isFalse);
      expect(MessageFlags.edited.isNotEmpty, isTrue);
    });

    test('operator ^', () {
      final a = MessageFlags.edited;
      final b = MessageFlags.edited;
      expect((a ^ b).isEmpty, isTrue);
    });
  });

  group('Permission extended', () {
    test('isNotEmpty', () {
      expect(Permission(0).isNotEmpty, isFalse);
      expect(Permission.sendMessages.isNotEmpty, isTrue);
    });

    test('operator ^', () {
      final a = Permission.sendMessages;
      final b = Permission.sendMessages;
      expect((a ^ b).isEmpty, isTrue);
    });
  });

  group('RichStyle extended', () {
    test('isNotEmpty', () {
      expect(RichStyle(0).isNotEmpty, isFalse);
      expect(RichStyle.bold.isNotEmpty, isTrue);
    });

    test('operator ^', () {
      final a = RichStyle.bold;
      final b = RichStyle.bold;
      expect((a ^ b).isEmpty, isTrue);
    });
  });

  group('ServerCapabilities extended', () {
    test('isNotEmpty', () {
      expect(ServerCapabilities(0).isNotEmpty, isFalse);
      expect(ServerCapabilities(1).isNotEmpty, isTrue);
    });

    test('operator ^', () {
      final a = ServerCapabilities(1);
      final b = ServerCapabilities(1);
      expect((a ^ b).isEmpty, isTrue);
    });
  });

  group('UserFlags extended', () {
    test('isNotEmpty', () {
      expect(UserFlags(0).isNotEmpty, isFalse);
      expect(UserFlags(1).isNotEmpty, isTrue);
    });

    test('operator ^', () {
      final a = UserFlags(1);
      final b = UserFlags(1);
      expect((a ^ b).isEmpty, isTrue);
    });
  });

  // ---------------------------------------------------------------------------
  // AckPayload — hashCode (put in Set to exercise)
  // ---------------------------------------------------------------------------

  group('AckPayload hashCode', () {
    test('AckMessageBatch hashCode consistent', () {
      final a = AckMessageBatch(value: Uint8List.fromList([1, 2, 3]));
      final b = AckMessageBatch(value: Uint8List.fromList([1, 2, 3]));
      expect(a.hashCode, b.hashCode);
    });

    test('AckChatList hashCode consistent', () {
      final a = AckChatList(value: Uint8List.fromList([10]));
      final b = AckChatList(value: Uint8List.fromList([10]));
      expect(a.hashCode, b.hashCode);
    });

    test('AckChatInfo hashCode consistent', () {
      final a = AckChatInfo(value: Uint8List.fromList([1]));
      final b = AckChatInfo(value: Uint8List.fromList([1]));
      expect(a.hashCode, b.hashCode);
    });

    test('AckMemberList hashCode consistent', () {
      final a = AckMemberList(value: Uint8List.fromList([1, 2]));
      final b = AckMemberList(value: Uint8List.fromList([1, 2]));
      expect(a.hashCode, b.hashCode);
    });

    test('AckSearchResults hashCode consistent', () {
      final a = AckSearchResults(value: Uint8List.fromList([5]));
      final b = AckSearchResults(value: Uint8List.fromList([5]));
      expect(a.hashCode, b.hashCode);
    });

    test('AckUserInfo hashCode', () {
      final s = {
        AckUserInfo(value: Uint8List.fromList([7])),
      };
      expect(s.contains(AckUserInfo(value: Uint8List.fromList([7]))), isTrue);
    });

    test('AckUserList hashCode', () {
      final s = {
        AckUserList(value: Uint8List.fromList([8])),
      };
      expect(s.contains(AckUserList(value: Uint8List.fromList([8]))), isTrue);
    });

    test('AckBlockList hashCode', () {
      final s = {
        AckBlockList(value: Uint8List.fromList([9])),
      };
      expect(s.contains(AckBlockList(value: Uint8List.fromList([9]))), isTrue);
    });

    test('AckEmpty identical fast path', () {
      const a = AckEmpty();
      expect(identical(a, a), isTrue);
      expect(a, equals(a));
    });

    test('AckChatId hashCode consistent', () {
      final a = AckChatId(value: 42);
      final b = AckChatId(value: 42);
      expect(a.hashCode, b.hashCode);
      expect({a}.contains(b), isTrue);
    });
  });

  // ---------------------------------------------------------------------------
  // Writer — invalid hex char in UUID
  // ---------------------------------------------------------------------------

  group('Writer _hexVal', () {
    test('invalid hex char throws CodecError', () {
      // UUID with invalid 'Z' character
      const badUuid = '550e8400-e29b-41d4-a716-44665544000Z';
      final w = ProtocolWriter();
      expect(() => w.writeUuid(badUuid), throwsA(isA<CodecError>()));
    });
  });

  // ---------------------------------------------------------------------------
  // Codec — invalid discriminant branches
  // ---------------------------------------------------------------------------

  group(
    'Codec invalid discriminants',
    skip: _isWeb ? 'Int64 not supported by dart2js' : null,
    () {
      test('unknown ErrorCode throws', () {
        final w = ProtocolWriter();
        w.writeU16(9999); // invalid error code
        w.writeU8(0); // slug_len = 0
        w.writeString('err');
        w.writeU32(0); // retry_after_ms
        w.writeOptionalString(null); // extra
        expect(
          () => decodeErrorPayload(ProtocolReader(w.toBytes())),
          throwsA(isA<CodecError>()),
        );
      });

      test('unknown LoadChatsPayload discriminant throws', () {
        final w = ProtocolWriter();
        w.writeU8(99); // invalid discriminant
        expect(
          () => decodeLoadChatsPayload(ProtocolReader(w.toBytes())),
          throwsA(isA<CodecError>()),
        );
      });

      test('unknown SearchScope discriminant throws', () {
        final w = ProtocolWriter();
        w.writeU8(99); // invalid discriminant
        expect(
          () => decodeSearchScope(ProtocolReader(w.toBytes())),
          throwsA(isA<CodecError>()),
        );
      });

      test('unknown LoadMessagesPayload mode throws', () {
        final w = ProtocolWriter();
        w.writeU32(1); // chatId
        w.writeU8(99); // invalid mode
        expect(
          () => decodeLoadMessagesPayload(ProtocolReader(w.toBytes())),
          throwsA(isA<CodecError>()),
        );
      });

      test('unknown MemberAction discriminant throws', () {
        final w = ProtocolWriter();
        w.writeU8(99); // invalid action
        expect(
          () => decodeMemberAction(ProtocolReader(w.toBytes())),
          throwsA(isA<CodecError>()),
        );
      });
    },
  );

  // ---------------------------------------------------------------------------
  // Sealed class equality for second variants
  // ---------------------------------------------------------------------------

  group('Sealed class equality', () {
    test('LoadChatsFirstPage equality', () {
      expect(
        const LoadChatsFirstPage(limit: 50),
        equals(const LoadChatsFirstPage(limit: 50)),
      );
    });

    test('MemberActionUpdatePermissions equality', () {
      expect(
        const MemberActionUpdatePermissions(
          permission: Permission.sendMessages,
        ),
        equals(
          const MemberActionUpdatePermissions(
            permission: Permission.sendMessages,
          ),
        ),
      );
    });
  });
}
