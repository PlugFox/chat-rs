import 'dart:typed_data';

import 'package:chat_core/chat_core.dart';
import 'package:test/test.dart';

/// True when running under dart2js / dart2wasm (no Int64 typed-data support).
final bool _isWeb = identical(0, 0.0);

void main() {
  // ---------------------------------------------------------------------------
  // ProtocolWriter basics
  // ---------------------------------------------------------------------------

  group('ProtocolWriter', () {
    test('initial length is zero', () {
      final w = ProtocolWriter();
      expect(w.length, 0);
    });

    test('reset clears position', () {
      final w = ProtocolWriter();
      w.writeU32(42);
      expect(w.length, 4);
      w.reset();
      expect(w.length, 0);
    });

    test('toBytes returns copy', () {
      final w = ProtocolWriter();
      w.writeU8(0xAB);
      final a = w.toBytes();
      final b = w.toBytes();
      expect(a, equals(b));
      a[0] = 0; // mutating copy must not affect writer
      expect(w.toBytes()[0], 0xAB);
    });

    test('toBytesView returns view', () {
      final w = ProtocolWriter();
      w.writeU8(0xCD);
      final v = w.toBytesView();
      expect(v.length, 1);
      expect(v[0], 0xCD);
    });

    test('grow handles large writes', () {
      final w = ProtocolWriter(4); // tiny initial capacity
      // Write more than initial capacity to force growth
      for (var i = 0; i < 100; i++) {
        w.writeU32(i);
      }
      expect(w.length, 400);
      final r = ProtocolReader(w.toBytes());
      for (var i = 0; i < 100; i++) {
        expect(r.readU32(), i);
      }
    });

    test('reserve and patchU32', () {
      final w = ProtocolWriter();
      final offset = w.reserve(4);
      w.writeU8(0xFF);
      w.patchU32(offset, 12345);
      final r = ProtocolReader(w.toBytes());
      expect(r.readU32(), 12345);
      expect(r.readU8(), 0xFF);
    });

    test('writeRawBytes', () {
      final w = ProtocolWriter();
      final data = Uint8List.fromList([1, 2, 3, 4, 5]);
      w.writeRawBytes(data);
      expect(w.toBytes(), equals(data));
    });
  });

  // ---------------------------------------------------------------------------
  // Primitive roundtrips
  // ---------------------------------------------------------------------------

  group('primitive roundtrip', () {
    test('u8', () {
      for (final v in [0, 1, 127, 128, 255]) {
        final w = ProtocolWriter();
        w.writeU8(v);
        expect(ProtocolReader(w.toBytes()).readU8(), v);
      }
    });

    test('u16', () {
      for (final v in [0, 1, 255, 256, 0xFFFF]) {
        final w = ProtocolWriter();
        w.writeU16(v);
        expect(ProtocolReader(w.toBytes()).readU16(), v);
      }
    });

    test('u32', () {
      for (final v in [0, 1, 0xFFFF, 0x10000, 0xFFFFFFFF]) {
        final w = ProtocolWriter();
        w.writeU32(v);
        expect(ProtocolReader(w.toBytes()).readU32(), v);
      }
    });

    test('i64', () {
      for (final v in [0, 1, -1, 1234567890, -1234567890]) {
        final w = ProtocolWriter();
        w.writeI64(v);
        expect(ProtocolReader(w.toBytes()).readI64(), v);
      }
    }, skip: _isWeb ? 'Int64 not supported by dart2js' : null);

    test('u16 little-endian byte order', () {
      final w = ProtocolWriter();
      w.writeU16(0x0102);
      final bytes = w.toBytes();
      expect(bytes[0], 0x02); // low byte first
      expect(bytes[1], 0x01);
    });

    test('u32 little-endian byte order', () {
      final w = ProtocolWriter();
      w.writeU32(0x04030201);
      final bytes = w.toBytes();
      expect(bytes, [0x01, 0x02, 0x03, 0x04]);
    });
  });

  // ---------------------------------------------------------------------------
  // Timestamp
  // ---------------------------------------------------------------------------

  group(
    'timestamp',
    skip: _isWeb ? 'Int64 not supported by dart2js' : null,
    () {
      test('valid values roundtrip', () {
        for (final v in [0, 1, 1234567890, maxTimestamp]) {
          final w = ProtocolWriter();
          w.writeTimestamp(v);
          expect(ProtocolReader(w.toBytes()).readTimestamp(), v);
        }
      });

      test('write negative timestamp throws', () {
        final w = ProtocolWriter();
        expect(() => w.writeTimestamp(-1), throwsA(isA<CodecError>()));
      });

      test('write over-max timestamp throws', () {
        final w = ProtocolWriter();
        expect(
          () => w.writeTimestamp(maxTimestamp + 1),
          throwsA(isA<CodecError>()),
        );
      });

      test('read negative timestamp throws', () {
        final w = ProtocolWriter();
        w.writeI64(-1); // bypass validation
        expect(
          () => ProtocolReader(w.toBytes()).readTimestamp(),
          throwsA(isA<CodecError>()),
        );
      });

      test('read over-max timestamp throws', () {
        final w = ProtocolWriter();
        w.writeI64(maxTimestamp + 1); // bypass validation
        expect(
          () => ProtocolReader(w.toBytes()).readTimestamp(),
          throwsA(isA<CodecError>()),
        );
      });
    },
  );

  // ---------------------------------------------------------------------------
  // UUID
  // ---------------------------------------------------------------------------

  group('uuid', () {
    test('roundtrip', () {
      const uuid = '550e8400-e29b-41d4-a716-446655440000';
      final w = ProtocolWriter();
      w.writeUuid(uuid);
      expect(w.length, 16);
      final r = ProtocolReader(w.toBytes());
      expect(r.readUuid(), uuid);
    });

    test('uppercase hex accepted', () {
      const uuid = '550E8400-E29B-41D4-A716-446655440000';
      final w = ProtocolWriter();
      w.writeUuid(uuid);
      final r = ProtocolReader(w.toBytes());
      // readUuid always returns lowercase
      expect(r.readUuid(), uuid.toLowerCase());
    });
  });

  // ---------------------------------------------------------------------------
  // Option<u32>
  // ---------------------------------------------------------------------------

  group('optionU32', () {
    test('null roundtrip', () {
      final w = ProtocolWriter();
      w.writeOptionU32(null);
      expect(ProtocolReader(w.toBytes()).readOptionU32(), isNull);
    });

    test('value roundtrip', () {
      for (final v in [0, 1, 0xFFFFFFFF]) {
        final w = ProtocolWriter();
        w.writeOptionU32(v);
        expect(ProtocolReader(w.toBytes()).readOptionU32(), v);
      }
    });

    test('invalid flag throws', () {
      final w = ProtocolWriter();
      w.writeU8(2); // invalid flag
      expect(
        () => ProtocolReader(w.toBytes()).readOptionU32(),
        throwsA(isA<CodecError>()),
      );
    });
  });

  // ---------------------------------------------------------------------------
  // Updatable string
  // ---------------------------------------------------------------------------

  group('updatableString', () {
    test('null roundtrip', () {
      final w = ProtocolWriter();
      w.writeUpdatableString(null);
      expect(ProtocolReader(w.toBytes()).readUpdatableString(), isNull);
    });

    test('value roundtrip', () {
      final w = ProtocolWriter();
      w.writeUpdatableString('hello');
      expect(ProtocolReader(w.toBytes()).readUpdatableString(), 'hello');
    });

    test('invalid flag throws', () {
      final w = ProtocolWriter();
      w.writeU8(2); // invalid flag
      expect(
        () => ProtocolReader(w.toBytes()).readUpdatableString(),
        throwsA(isA<CodecError>()),
      );
    });
  });

  // ---------------------------------------------------------------------------
  // Optional bytes
  // ---------------------------------------------------------------------------

  group('optionalBytes', () {
    test('null roundtrip', () {
      final w = ProtocolWriter();
      w.writeOptionalBytes(null);
      expect(ProtocolReader(w.toBytes()).readOptionalBytes(), isNull);
    });

    test('value roundtrip', () {
      final data = Uint8List.fromList([1, 2, 3, 4, 5]);
      final w = ProtocolWriter();
      w.writeOptionalBytes(data);
      expect(ProtocolReader(w.toBytes()).readOptionalBytes(), equals(data));
    });
  });

  // ---------------------------------------------------------------------------
  // Vec<u32> and Vec<String>
  // ---------------------------------------------------------------------------

  group('vecU32', () {
    test('roundtrip', () {
      final w = ProtocolWriter();
      final values = [1, 2, 3, 0xFFFFFFFF];
      w.writeU16(values.length);
      for (final v in values) {
        w.writeU32(v);
      }
      expect(ProtocolReader(w.toBytes()).readVecU32(), equals(values));
    });

    test('empty', () {
      final w = ProtocolWriter();
      w.writeU16(0);
      expect(ProtocolReader(w.toBytes()).readVecU32(), isEmpty);
    });
  });

  group('vecString', () {
    test('roundtrip', () {
      final w = ProtocolWriter();
      final values = ['hello', 'мир', '😀'];
      w.writeU16(values.length);
      for (final v in values) {
        w.writeString(v);
      }
      expect(ProtocolReader(w.toBytes()).readVecString(), equals(values));
    });
  });

  // ---------------------------------------------------------------------------
  // Reader: remaining, skip, truncation
  // ---------------------------------------------------------------------------

  group('ProtocolReader', () {
    test('remaining tracks bytes', () {
      final w = ProtocolWriter();
      w.writeU32(1);
      w.writeU8(2);
      final r = ProtocolReader(w.toBytes());
      expect(r.remaining, 5);
      r.readU32();
      expect(r.remaining, 1);
      r.readU8();
      expect(r.remaining, 0);
    });

    test('skip advances position', () {
      final w = ProtocolWriter();
      w.writeU8(0xAA);
      w.writeU8(0xBB);
      w.writeU8(0xCC);
      final r = ProtocolReader(w.toBytes());
      r.skip(2);
      expect(r.readU8(), 0xCC);
    });

    test('readU8 on empty throws', () {
      expect(
        () => ProtocolReader(Uint8List(0)).readU8(),
        throwsA(isA<CodecError>()),
      );
    });

    test('readU16 truncated throws', () {
      expect(
        () => ProtocolReader(Uint8List(1)).readU16(),
        throwsA(isA<CodecError>()),
      );
    });

    test('readU32 truncated throws', () {
      expect(
        () => ProtocolReader(Uint8List(3)).readU32(),
        throwsA(isA<CodecError>()),
      );
    });

    test('readI64 truncated throws', () {
      expect(
        () => ProtocolReader(Uint8List(7)).readI64(),
        throwsA(isA<CodecError>()),
      );
    });

    test('skip past end throws', () {
      expect(
        () => ProtocolReader(Uint8List(2)).skip(3),
        throwsA(isA<CodecError>()),
      );
    });

    test('readString truncated throws', () {
      final w = ProtocolWriter();
      w.writeU32(100); // claims 100 bytes but only header present
      expect(
        () => ProtocolReader(w.toBytes()).readString(),
        throwsA(isA<CodecError>()),
      );
    });

    test('readBytes', () {
      final w = ProtocolWriter();
      w.writeU8(0x01);
      w.writeU8(0x02);
      w.writeU8(0x03);
      final r = ProtocolReader(w.toBytes());
      expect(r.readBytes(3), equals(Uint8List.fromList([1, 2, 3])));
    });

    test('readArray', () {
      final w = ProtocolWriter();
      w.writeU8(10);
      w.writeU8(20);
      w.writeU8(30);
      final r = ProtocolReader(w.toBytes());
      final items = r.readArray(3, () => r.readU8());
      expect(items, [10, 20, 30]);
    });
  });

  // ---------------------------------------------------------------------------
  // CodecError
  // ---------------------------------------------------------------------------

  group('CodecError', () {
    test('toString includes message', () {
      const e = CodecError('test message');
      expect(e.toString(), 'CodecError: test message');
      expect(e.message, 'test message');
    });
  });

  // ---------------------------------------------------------------------------
  // Protocol constants
  // ---------------------------------------------------------------------------

  group('protocol constants', () {
    test('frameHeaderSize matches kind + seq + eventSeq', () {
      // kind(1) + seq(4) + eventSeq(4) = 9
      expect(frameHeaderSize, 9);
    });

    test('maxTimestamp is (1 << 41) - 1', () {
      expect(maxTimestamp, (1 << 41) - 1);
    });

    test('minTimestamp is 0', () {
      expect(minTimestamp, 0);
    });

    test('eventSeqOverflowMask top 2 bits', () {
      expect(eventSeqOverflowMask, 0xC0000000);
    });

    test('protocolVersion is positive', () {
      expect(protocolVersion, greaterThan(0));
    });
  });

  // ---------------------------------------------------------------------------
  // Multiple fields sequentially
  // ---------------------------------------------------------------------------

  group(
    'sequential reads',
    skip: _isWeb ? 'Int64 not supported by dart2js' : null,
    () {
      test('mixed types in sequence', () {
        final w = ProtocolWriter();
        w.writeU8(0x01);
        w.writeU16(0x0203);
        w.writeU32(0x04050607);
        w.writeI64(123456789);
        w.writeString('hello');
        w.writeOptionU32(42);
        w.writeOptionU32(null);

        final r = ProtocolReader(w.toBytes());
        expect(r.readU8(), 0x01);
        expect(r.readU16(), 0x0203);
        expect(r.readU32(), 0x04050607);
        expect(r.readI64(), 123456789);
        expect(r.readString(), 'hello');
        expect(r.readOptionU32(), 42);
        expect(r.readOptionU32(), isNull);
        expect(r.remaining, 0);
      });
    },
  );
}
