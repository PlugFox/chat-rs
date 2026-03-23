// ignore_for_file: depend_on_referenced_packages
import 'package:stream_channel/stream_channel.dart';

import 'echo_server.dart';

void hybridMain(StreamChannel<Object?> channel) async {
  final server = await EchoServer.start();
  channel.sink.add({'echo': server.port, 'hang': server.hangPort});
  // Wait for the test runner to signal shutdown.
  await channel.stream.first;
  await server.stop();
}
