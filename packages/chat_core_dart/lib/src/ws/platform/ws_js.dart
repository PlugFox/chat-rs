import 'dart:async' show Completer;
import 'dart:js_interop';
import 'dart:typed_data';

import 'package:chat_core/src/ws/_disposable.dart';
import 'package:chat_core/src/ws/ws.dart' show ChatWebSocket;
import 'package:meta/meta.dart';

// ---------------------------------------------------------------------------
// JS interop bindings (inline — no package:web dependency)
// ---------------------------------------------------------------------------

@JS('WebSocket')
extension type _JsWebSocket._(JSObject _) implements JSObject {
  external factory _JsWebSocket(String url);
  external void close([int code, String reason]);
  external void send(JSAny data);
  external int get readyState;
  external set binaryType(String value);
  external set onopen(JSFunction? value);
  external set onclose(JSFunction? value);
  external set onerror(JSFunction? value);
  external set onmessage(JSFunction? value);
}

@JS('CloseEvent')
extension type _JsCloseEvent._(JSObject _) implements JSObject {
  external int get code;
  external String get reason;
}

@JS('MessageEvent')
extension type _JsMessageEvent._(JSObject _) implements JSObject {
  external JSAny? get data;
}

/// WebSocket readyState constant for OPEN.
const _kOpen = 1;

// ---------------------------------------------------------------------------
// Factory
// ---------------------------------------------------------------------------

@internal
Future<ChatWebSocket> $connectChatWebSocket({
  required String url,
  required void Function(Uint8List message) onMessage,
  required void Function(Object error, StackTrace stackTrace) onError,
  required void Function(int code, String reason) onClose,
}) {
  final completer = Completer<ChatWebSocket>();

  late final _JsWebSocket ws;
  final connectChain = DisposableChain();

  try {
    ws = _JsWebSocket(url);
    ws.binaryType = 'arraybuffer';
  } on Object catch (e, st) {
    completer.completeError(e, st);
    return completer.future;
  }

  ws.onopen = ((JSAny _) {
    connectChain(); // tear down connect-phase handlers
    final socket = _JsChatWebSocket(ws, onMessage, onError, onClose);
    completer.complete(socket);
  }).toJS;
  connectChain.add(() => ws.onopen = null);

  ws.onerror = ((JSAny _) {
    if (completer.isCompleted) return;
    connectChain();
    try {
      ws.close();
    } on Object catch (_) {}
    completer.completeError(
      Exception('WebSocket connection failed'),
      StackTrace.current,
    );
  }).toJS;
  connectChain.add(() => ws.onerror = null);

  ws.onclose = ((JSAny event) {
    if (completer.isCompleted) return;
    connectChain();
    final ce = event as _JsCloseEvent;
    completer.completeError(
      Exception(
        'WebSocket closed during connect: ${ce.code} ${ce.reason}',
      ),
      StackTrace.current,
    );
  }).toJS;
  connectChain.add(() => ws.onclose = null);

  return completer.future;
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

final class _JsChatWebSocket implements ChatWebSocket {
  _JsChatWebSocket(
    this._ws,
    this._onMessage,
    this._onError,
    this._onClose,
  ) {
    _ws.onmessage = ((JSAny event) {
      final data = (event as _JsMessageEvent).data;
      if (data != null && data.isA<JSArrayBuffer>()) {
        _onMessage((data as JSArrayBuffer).toDart.asUint8List());
      }
    }).toJS;
    _chain.add(() => _ws.onmessage = null);

    _ws.onerror = ((JSAny _) {
      _onError(Exception('WebSocket error'), StackTrace.current);
    }).toJS;
    _chain.add(() => _ws.onerror = null);

    _ws.onclose = ((JSAny event) {
      if (_closed) return;
      _closed = true;
      final ce = event as _JsCloseEvent;
      final code = ce.code;
      final reason = ce.reason;
      _chain();
      _onClose(code, reason.isEmpty ? 'closed' : reason);
    }).toJS;
    _chain.add(() => _ws.onclose = null);
  }

  final _JsWebSocket _ws;
  final void Function(Uint8List message) _onMessage;
  final void Function(Object error, StackTrace stackTrace) _onError;
  final void Function(int code, String reason) _onClose;

  final _chain = DisposableChain();
  bool _closed = false;

  @override
  void send(Uint8List message) {
    if (_closed) throw StateError('WebSocket is closed');
    if (_ws.readyState != _kOpen) throw StateError('WebSocket is not open');
    _ws.send(message.toJS);
  }

  @override
  void close([int? code, String? reason]) {
    if (_closed) return;
    _closed = true;
    _chain();
    final c = code ?? 1000;
    final r = reason ?? 'closed';
    try {
      _ws.close(c, r);
    } on Object catch (_) {
      // Browser may reject invalid close codes.
    }
    _onClose(c, r);
  }
}
