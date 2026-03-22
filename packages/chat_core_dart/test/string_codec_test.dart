import 'dart:convert';

import 'package:test/test.dart';
import 'package:chat_core/chat_core.dart';

void main() {
  group('writeString/readString UTF-8 roundtrip', () {
    void roundtrip(String label, String input) {
      test(label, () {
        final w = ProtocolWriter();
        w.writeString(input);
        final r = ProtocolReader(w.toBytes());
        expect(r.readString(), equals(input));
      });
    }

    // Verify byte-level encoding matches dart:convert.
    void verifyBytes(String label, String input) {
      test('$label bytes match dart:convert', () {
        final w = ProtocolWriter();
        w.writeString(input);
        final bytes = w.toBytes();
        // Skip 4-byte length prefix.
        final payload = bytes.sublist(4);
        expect(payload, equals(utf8.encode(input)));
      });
    }

    // ASCII
    roundtrip('empty string', '');
    roundtrip('ASCII short', 'hello');
    roundtrip('ASCII 49 chars', 'Hello, this is a test message with some content.');

    // 2-byte UTF-8 (U+0080..U+07FF)
    roundtrip('Cyrillic', 'Привет, мир!');
    roundtrip('Latin extended', 'café résumé naïve');
    roundtrip('Greek', 'Ωμέγα αλφα βήτα');
    verifyBytes('Cyrillic bytes', 'Привет, мир!');

    // 3-byte UTF-8 (U+0800..U+FFFF)
    roundtrip('CJK characters', '你好世界');
    roundtrip('Japanese', 'こんにちは世界');
    roundtrip('Korean', '안녕하세요');
    roundtrip('Thai', 'สวัสดีครับ');
    verifyBytes('CJK bytes', '你好世界');

    // 4-byte UTF-8 (U+10000+) — surrogate pairs in UTF-16
    roundtrip('emoji simple', '😀🎉🚀');
    roundtrip('emoji flags', '🇺🇸🇯🇵🇩🇪');
    roundtrip('emoji skin tone', '👋🏽');
    roundtrip('emoji ZWJ sequence', '👨‍👩‍👧‍👦');
    roundtrip('musical symbols', '𝄞𝄡𝄢');
    verifyBytes('emoji bytes', '😀🎉🚀');
    verifyBytes('musical symbols bytes', '𝄞𝄡𝄢');

    // Mixed
    roundtrip('mixed ASCII + Cyrillic', 'Hello Привет');
    roundtrip('mixed ASCII + emoji', 'Hey 👋 how are you?');
    roundtrip('mixed CJK + emoji', '你好 😀 世界');
    roundtrip('mixed everything', 'Hello Привет 你好 😀🎉');
    verifyBytes('mixed everything bytes', 'Hello Привет 你好 😀🎉');

    // Edge cases
    roundtrip('single 2-byte char', 'ñ');
    roundtrip('single 3-byte char', '中');
    roundtrip('single 4-byte char', '😀');
    roundtrip('null-like char U+0000', '\u0000');
    roundtrip('max BMP U+FFFD', '\uFFFD');
    roundtrip('boundary U+007F', '\u007F');
    roundtrip('boundary U+0080', '\u0080');
    roundtrip('boundary U+07FF', '\u07FF');
    roundtrip('boundary U+0800', '\u0800');
    verifyBytes('boundary U+0080 bytes', '\u0080');
    verifyBytes('boundary U+07FF bytes', '\u07FF');
    verifyBytes('boundary U+0800 bytes', '\u0800');

    // Long non-ASCII strings
    roundtrip('long Cyrillic', 'Б' * 1000);
    roundtrip('long CJK', '中' * 1000);
    roundtrip('long emoji', '😀' * 500);
  });

  group('writeOptionalString UTF-8', () {
    test('null', () {
      final w = ProtocolWriter();
      w.writeOptionalString(null);
      final r = ProtocolReader(w.toBytes());
      expect(r.readOptionalString(), isNull);
    });

    test('non-ASCII', () {
      final w = ProtocolWriter();
      w.writeOptionalString('Привет 😀');
      final r = ProtocolReader(w.toBytes());
      expect(r.readOptionalString(), equals('Привет 😀'));
    });
  });
}
