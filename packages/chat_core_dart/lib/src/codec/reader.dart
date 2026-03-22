import 'dart:convert';
import 'dart:typed_data';

import 'error.dart';

class ProtocolReader {
  ProtocolReader(Uint8List data, [this._pos = 0])
    : _data = ByteData.sublistView(data),
      _bytes = data;

  final ByteData _data;
  final Uint8List _bytes;
  int _pos;

  int get remaining => _data.lengthInBytes - _pos;

  @pragma('vm:prefer-inline')
  void ensureRemaining(int n) {
    if (remaining < n) {
      throw CodecError('truncated: need $n bytes but only $remaining remain');
    }
  }

  @pragma('vm:prefer-inline')
  int readU8() {
    ensureRemaining(1);
    return _data.getUint8(_pos++);
  }

  @pragma('vm:prefer-inline')
  int readU16() {
    ensureRemaining(2);
    final v = _data.getUint16(_pos, Endian.little);
    _pos += 2;
    return v;
  }

  @pragma('vm:prefer-inline')
  int readU32() {
    ensureRemaining(4);
    final v = _data.getUint32(_pos, Endian.little);
    _pos += 4;
    return v;
  }

  @pragma('vm:prefer-inline')
  int readI64() {
    ensureRemaining(8);
    final v = _data.getInt64(_pos, Endian.little);
    _pos += 8;
    return v;
  }

  @pragma('vm:prefer-inline')
  int readTimestamp() {
    final v = readI64();
    if (v < 0 || v > 2199023255551) {
      throw CodecError('timestamp out of range: $v');
    }
    return v;
  }

  String readString() {
    final len = readU32();
    if (len == 0) return '';
    ensureRemaining(len);
    final s = _decodeUtf8(len);
    _pos += len;
    return s;
  }

  String? readOptionalString() {
    final len = readU32();
    if (len == 0) return null;
    ensureRemaining(len);
    final s = _decodeUtf8(len);
    _pos += len;
    return s;
  }

  /// Decode [len] bytes at current position as UTF-8.
  /// Fast path: if all bytes are ASCII, build string directly
  /// (skips UTF-8 validation, ~20% faster for short ASCII strings).
  String _decodeUtf8(int len) {
    bool ascii = true;
    for (var i = 0; i < len; i++) {
      if (_bytes[_pos + i] > 0x7F) {
        ascii = false;
        break;
      }
    }
    if (ascii) {
      return String.fromCharCodes(_bytes, _pos, _pos + len);
    }
    return utf8.decode(Uint8List.sublistView(_bytes, _pos, _pos + len));
  }

  Uint8List? readOptionalBytes() {
    final len = readU32();
    if (len == 0) return null;
    ensureRemaining(len);
    final out = Uint8List.sublistView(_bytes, _pos, _pos + len);
    _pos += len;
    return out;
  }

  String readUuid() {
    ensureRemaining(16);
    final hex = StringBuffer();
    for (var i = 0; i < 16; i++) {
      {
        hex.write(_bytes[_pos + i].toRadixString(16).padLeft(2, '0'));
      }
    }
    _pos += 16;
    final h = hex.toString();
    return '${h.substring(0, 8)}-${h.substring(8, 12)}-${h.substring(12, 16)}-${h.substring(16, 20)}-${h.substring(20)}';
  }

  @pragma('vm:prefer-inline')
  int? readOptionU32() {
    final flag = readU8();
    if (flag == 0) return null;
    if (flag == 1) return readU32();
    throw CodecError('invalid Option<u32> flag: $flag');
  }

  @pragma('vm:prefer-inline')
  String? readUpdatableString() {
    final flag = readU8();
    if (flag == 0) return null;
    if (flag == 1) return readString();
    throw CodecError('invalid updatable string flag: $flag');
  }

  Uint8List readBytes(int n) {
    ensureRemaining(n);
    final out = Uint8List.sublistView(_bytes, _pos, _pos + n);
    _pos += n;
    return out;
  }

  List<int> readVecU32() {
    final count = readU16();
    return [for (var i = 0; i < count; i++) readU32()];
  }

  List<String> readVecString() {
    final count = readU16();
    return [for (var i = 0; i < count; i++) readString()];
  }

  List<T> readArray<T>(int count, T Function() readOne) {
    return [for (var i = 0; i < count; i++) readOne()];
  }

  void skip(int n) {
    ensureRemaining(n);
    _pos += n;
  }
}
