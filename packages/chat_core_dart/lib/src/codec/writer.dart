import 'dart:convert';
import 'dart:typed_data';

import 'error.dart';

class ProtocolWriter {
  ProtocolWriter([int initialCapacity = 256])
    : _buf = Uint8List(initialCapacity) {
    _data = ByteData.sublistView(_buf);
  }

  Uint8List _buf;
  late ByteData _data;
  int _pos = 0;

  /// Reset position to zero, reusing the existing buffer.
  void reset() {
    _pos = 0;
  }

  /// Current number of bytes written.
  int get length => _pos;

  void _grow(int needed) {
    final required = _pos + needed;
    if (required <= _buf.length) return;
    var newLen = _buf.length * 2;
    while (newLen < required) {
      {
        newLen *= 2;
      }
    }
    final next = Uint8List(newLen);
    next.setAll(0, Uint8List.sublistView(_buf, 0, _pos));
    _buf = next;
    _data = ByteData.sublistView(next);
  }

  void writeU8(int v) {
    _grow(1);
    _data.setUint8(_pos++, v);
  }

  void writeU16(int v) {
    _grow(2);
    _data.setUint16(_pos, v, Endian.little);
    _pos += 2;
  }

  void writeU32(int v) {
    _grow(4);
    _data.setUint32(_pos, v, Endian.little);
    _pos += 4;
  }

  void writeI64(int v) {
    _grow(8);
    _data.setInt64(_pos, v, Endian.little);
    _pos += 8;
  }

  void writeTimestamp(int v) {
    if (v < 0 || v > 2199023255551) {
      throw CodecError('timestamp out of range: $v');
    }
    writeI64(v);
  }

  void writeString(String v) {
    if (v.isEmpty) {
      writeU32(0);
      return;
    }
    // Fast path: pure ASCII — avoid utf8.encode() allocation.
    if (_isAscii(v)) {
      writeU32(v.length);
      _grow(v.length);
      for (var i = 0; i < v.length; i++) {
        _buf[_pos++] = v.codeUnitAt(i);
      }
    } else {
      final encoded = utf8.encode(v);
      writeU32(encoded.length);
      _grow(encoded.length);
      _buf.setAll(_pos, encoded);
      _pos += encoded.length;
    }
  }

  static bool _isAscii(String v) {
    for (var i = 0; i < v.length; i++) {
      if (v.codeUnitAt(i) > 0x7F) return false;
    }
    return true;
  }

  void writeOptionalString(String? v) {
    if (v == null) {
      writeU32(0);
    } else {
      writeString(v);
    }
  }

  void writeOptionalBytes(Uint8List? v) {
    if (v == null) {
      writeU32(0);
      return;
    }
    writeU32(v.length);
    _grow(v.length);
    _buf.setAll(_pos, v);
    _pos += v.length;
  }

  void writeUuid(String uuid) {
    _grow(16);
    for (var i = 0, j = 0; i < uuid.length && j < 16; i += 2) {
      if (i < uuid.length && uuid.codeUnitAt(i) == 0x2D) {
        i++;
      } // skip '-'
      _buf[_pos++] =
          (_hexVal(uuid.codeUnitAt(i)) << 4) | _hexVal(uuid.codeUnitAt(i + 1));
      j++;
    }
  }

  void writeOptionU32(int? v) {
    if (v == null) {
      writeU8(0);
    } else {
      writeU8(1);
      writeU32(v);
    }
  }

  void writeUpdatableString(String? v) {
    if (v == null) {
      writeU8(0);
    } else {
      writeU8(1);
      writeString(v);
    }
  }

  void writeRawBytes(Uint8List data) {
    _grow(data.length);
    _buf.setAll(_pos, data);
    _pos += data.length;
  }

  /// Patch a u32 at a previously written position.
  void patchU32(int offset, int v) {
    _data.setUint32(offset, v, Endian.little);
  }

  /// Reserve [n] bytes and return the offset. Caller fills them later.
  int reserve(int n) {
    _grow(n);
    final o = _pos;
    _pos += n;
    return o;
  }

  /// Return a copy of the written bytes.
  Uint8List toBytes() => _buf.sublist(0, _pos);

  /// Return a view of the written bytes. Valid only until the next write/reset.
  Uint8List toBytesView() => Uint8List.sublistView(_buf, 0, _pos);
}

int _hexVal(int c) {
  if (c >= 0x30 && c <= 0x39) return c - 0x30; // '0'-'9'
  if (c >= 0x61 && c <= 0x66) return c - 0x61 + 10; // 'a'-'f'
  if (c >= 0x41 && c <= 0x46) return c - 0x41 + 10; // 'A'-'F'
  throw CodecError('invalid hex char: ${String.fromCharCode(c)}');
}
