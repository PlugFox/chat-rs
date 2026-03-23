@TestOn('browser')
library;

import 'package:test/test.dart';

import 'ws_shared_tests.dart';

void main() {
  late int echoPort;
  late int hangPort;

  setUpAll(() async {
    final channel = spawnHybridUri('helpers/echo_server_hybrid.dart');
    final ports = await channel.stream.first as Map;
    echoPort = ports['echo'] as int;
    hangPort = ports['hang'] as int;
  });

  group('ChatWebSocket (Browser)', () {
    wsTests(
      echoPort: () => echoPort,
      hangPort: () => hangPort,
    );
  });
}
