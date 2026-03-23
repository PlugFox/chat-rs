import 'package:chat_core/src/util/disposable.dart';
import 'package:test/test.dart';

void main() {
  group('DisposableChain', () {
    test('executes callbacks in LIFO order', () {
      final order = <int>[];
      final chain = DisposableChain();
      chain.add(() => order.add(1));
      chain.add(() => order.add(2));
      chain.add(() => order.add(3));
      chain();
      expect(order, [3, 2, 1]);
    });

    test('empty chain can be disposed', () {
      final chain = DisposableChain();
      chain(); // must not throw
      expect(chain.isDisposed, isTrue);
    });

    test('double call is a no-op', () {
      var count = 0;
      final chain = DisposableChain();
      chain.add(() => count++);
      chain();
      chain();
      expect(count, 1);
    });

    test('isDisposed reflects state', () {
      final chain = DisposableChain();
      expect(chain.isDisposed, isFalse);
      chain();
      expect(chain.isDisposed, isTrue);
    });

    test('exception in callback does not prevent subsequent callbacks', () {
      final order = <int>[];
      final chain = DisposableChain();
      chain.add(() => order.add(1));
      chain.add(() => throw Exception('boom'));
      chain.add(() => order.add(3));
      expect(() => chain(), throwsException);
      // callback 3 ran, then the exception, then callback 1 ran
      expect(order, [3, 1]);
    });

    test('single callback', () {
      var called = false;
      final chain = DisposableChain();
      chain.add(() => called = true);
      chain();
      expect(called, isTrue);
    });
  });
}
