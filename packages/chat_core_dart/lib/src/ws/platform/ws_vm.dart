import 'dart:io' as io;
import 'dart:typed_data';

import 'package:chat_core/src/util/disposable.dart';
import 'package:chat_core/src/ws/ws.dart' show ChatWebSocket;
import 'package:meta/meta.dart';

@internal
Future<ChatWebSocket> $connectChatWebSocket({
  required String url,
  required void Function(Uint8List message) onMessage,
  required void Function(Object error, StackTrace stackTrace) onError,
  required void Function(int code, String reason) onClose,
  Iterable<String>? protocols,
  Duration? timeout,
}) async {
  var future = io.WebSocket.connect(url, protocols: protocols);
  if (timeout != null) future = future.timeout(timeout);
  final socket =
      await future
        ..pingInterval = null;
  return _VmChatWebSocket(socket, onMessage, onError, onClose);
}

final class _VmChatWebSocket implements ChatWebSocket {
  _VmChatWebSocket(
    this._socket,
    this._onMessage,
    this._onError,
    this._onClose,
  ) {
    final sub = _socket.listen(
      (Object? data) {
        if (data is Uint8List) {
          _onMessage(data);
        } else if (data is List<int>) {
          _onMessage(Uint8List.fromList(data));
        }
      },
      onError: (Object error, StackTrace stackTrace) {
        _onError(error, stackTrace);
      },
      onDone: () {
        if (_closed) return;
        _closed = true;
        final code = _socket.closeCode ?? 1005;
        final reason = _socket.closeReason;
        _chain();
        _onClose(code, (reason == null || reason.isEmpty) ? 'closed' : reason);
      },
    );
    _chain.add(() => sub.cancel());
  }

  final io.WebSocket _socket;
  final void Function(Uint8List message) _onMessage;
  final void Function(Object error, StackTrace stackTrace) _onError;
  final void Function(int code, String reason) _onClose;

  final _chain = DisposableChain();
  bool _closed = false;

  @override
  void send(Uint8List message) {
    if (_closed) throw StateError('WebSocket is closed');
    _socket.add(message);
  }

  @override
  void close([int? code, String? reason]) {
    if (_closed) return;
    _closed = true;
    _chain();
    final c = code ?? 1000;
    final r = reason ?? 'closed';
    try {
      _socket.close(c, r);
    } on Object catch (_) {
      // Socket may already be in a bad state.
    }
    _onClose(c, r);
  }
}
