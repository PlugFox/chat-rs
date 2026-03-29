import 'dart:typed_data';

import 'package:chat_core/chat_core.dart';
import 'package:test/test.dart';

void main() {
  group('AckEmpty', () {
    test('equality', () {
      expect(const AckEmpty(), equals(const AckEmpty()));
    });

    test('hashCode is stable', () {
      expect(const AckEmpty().hashCode, const AckEmpty().hashCode);
    });
  });

  group('AckMessageId', () {
    test('equality', () {
      expect(
        const AckMessageId(value: 42),
        equals(const AckMessageId(value: 42)),
      );
      expect(
        const AckMessageId(value: 42),
        isNot(equals(const AckMessageId(value: 43))),
      );
    });

    test('hashCode', () {
      expect(
        const AckMessageId(value: 42).hashCode,
        const AckMessageId(value: 42).hashCode,
      );
    });
  });

  group('AckChatId', () {
    test('equality', () {
      expect(const AckChatId(value: 1), equals(const AckChatId(value: 1)));
      expect(
        const AckChatId(value: 1),
        isNot(equals(const AckChatId(value: 2))),
      );
    });
  });

  group('AckMessageBatch', () {
    test('equality', () {
      final a = AckMessageBatch(value: Uint8List.fromList([1, 2, 3]));
      final b = AckMessageBatch(value: Uint8List.fromList([1, 2, 3]));
      final c = AckMessageBatch(value: Uint8List.fromList([4, 5]));
      expect(a, equals(b));
      expect(a, isNot(equals(c)));
    });
  });

  group('AckChatList', () {
    test('equality', () {
      final a = AckChatList(value: Uint8List.fromList([10]));
      final b = AckChatList(value: Uint8List.fromList([10]));
      expect(a, equals(b));
    });
  });

  group('AckChatInfo', () {
    test('equality', () {
      final a = AckChatInfo(value: Uint8List.fromList([1]));
      final b = AckChatInfo(value: Uint8List.fromList([1]));
      expect(a, equals(b));
    });
  });

  group('AckMemberList', () {
    test('equality', () {
      final a = AckMemberList(value: Uint8List.fromList([1, 2]));
      final b = AckMemberList(value: Uint8List.fromList([1, 2]));
      expect(a, equals(b));
    });
  });

  group('AckSearchResults', () {
    test('equality', () {
      final a = AckSearchResults(value: Uint8List.fromList([5]));
      final b = AckSearchResults(value: Uint8List.fromList([5]));
      expect(a, equals(b));
    });
  });

  group('AckUserInfo', () {
    test('equality', () {
      final a = AckUserInfo(value: Uint8List.fromList([7]));
      final b = AckUserInfo(value: Uint8List.fromList([7]));
      expect(a, equals(b));
    });
  });

  group('AckUserList', () {
    test('equality', () {
      final a = AckUserList(value: Uint8List.fromList([8, 9]));
      final b = AckUserList(value: Uint8List.fromList([8, 9]));
      expect(a, equals(b));
    });
  });

  group('AckBlockList', () {
    test('equality', () {
      final a = AckBlockList(value: Uint8List.fromList([1]));
      final b = AckBlockList(value: Uint8List.fromList([1]));
      expect(a, equals(b));
    });
  });
}
