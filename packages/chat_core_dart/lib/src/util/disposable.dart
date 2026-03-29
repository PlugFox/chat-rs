/// A LIFO chain of cleanup callbacks.
///
/// Register cleanup actions with [add]. When [call] is invoked,
/// all actions execute in reverse order of registration (last added — first
/// executed). After [call], subsequent calls are no-ops.
///
/// Based on the "Safe Resource Cleanup with Closure Chains" pattern:
/// https://plugfox.dev/safe-resource-cleanup-with-closure-chains/
///
/// ```dart
/// final chain = DisposableChain();
///
/// final sub = stream.listen(handler);
/// chain.add(() => sub.cancel());
///
/// final timer = Timer.periodic(duration, tick);
/// chain.add(() => timer.cancel());
///
/// // Later — tears down timer first, then subscription:
/// chain();
/// ```
final class DisposableChain {
  void Function() _fn = () {};
  bool _disposed = false;

  /// Whether this chain has already been disposed.
  bool get isDisposed => _disposed;

  /// Registers a cleanup action.
  ///
  /// Actions execute in LIFO order when [call] is invoked.
  /// Must not be called after the chain has been disposed.
  void add(void Function() fn) {
    assert(!_disposed, 'Cannot add to a disposed chain');
    if (_disposed) return;
    final prev = _fn;
    _fn = () {
      try {
        fn();
      } finally {
        prev();
      }
    };
  }

  /// Executes all registered cleanup actions in LIFO order.
  ///
  /// Safe to call multiple times — only the first call has effect.
  /// If any action throws, subsequent actions still execute.
  void call() {
    if (_disposed) return;
    _disposed = true;
    _fn();
    _fn = () {};
  }
}
