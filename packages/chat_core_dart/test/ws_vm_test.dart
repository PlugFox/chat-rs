@TestOn('vm')
library;

import 'package:test/test.dart';

import 'helpers/echo_server.dart';
import 'ws_shared_tests.dart';

void main() {
  late EchoServer server;

  setUpAll(() async {
    server = await EchoServer.start();
  });

  tearDownAll(() => server.stop());

  group('ChatWebSocket (VM)', () {
    wsTests(echoPort: () => server.port, hangPort: () => server.hangPort);
  });
}
