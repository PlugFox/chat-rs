import 'package:chat_core/src/util/list_equals.dart';
import 'package:test/test.dart';

void main() {
  group('listEquals', () {
    test('identical references', () {
      final list = [1, 2, 3];
      expect(listEquals(list, list), isTrue);
    });

    test('equal lists', () {
      expect(listEquals([1, 2, 3], [1, 2, 3]), isTrue);
    });

    test('different elements', () {
      expect(listEquals([1, 2, 3], [1, 2, 4]), isFalse);
    });

    test('different lengths', () {
      expect(listEquals([1, 2], [1, 2, 3]), isFalse);
    });

    test('both null', () {
      expect(listEquals<int>(null, null), isTrue);
    });

    test('first null', () {
      expect(listEquals(null, [1]), isFalse);
    });

    test('second null', () {
      expect(listEquals([1], null), isFalse);
    });

    test('both empty', () {
      expect(listEquals(<int>[], <int>[]), isTrue);
    });

    test('empty vs non-empty', () {
      expect(listEquals(<int>[], [1]), isFalse);
    });

    test('string lists', () {
      expect(listEquals(['a', 'b'], ['a', 'b']), isTrue);
      expect(listEquals(['a', 'b'], ['a', 'c']), isFalse);
    });
  });
}
