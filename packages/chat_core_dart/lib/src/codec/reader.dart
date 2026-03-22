// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

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

void ensureRemaining(int n) {
if (remaining < n) throw CodecError('truncated: need $n bytes but only $remaining remain');
}

int readU8() { ensureRemaining(1); return _data.getUint8(_pos++); }

int readU16() {
ensureRemaining(2);
final v = _data.getUint16(_pos, Endian.little);
_pos += 2;
return v;
}

int readU32() {
ensureRemaining(4);
final v = _data.getUint32(_pos, Endian.little);
_pos += 4;
return v;
}

int readI64() {
ensureRemaining(8);
final v = _data.getInt64(_pos, Endian.little);
_pos += 8;
return v;
}

int readTimestamp() {
final v = readI64();
if (v < 0 || v > 2199023255551) throw CodecError('timestamp out of range: $v');
return v;
}

String readString() {
final len = readU32();
if (len == 0) return '';
ensureRemaining(len);
final s = utf8.decode(_bytes.sublist(_pos, _pos + len));
_pos += len;
return s;
}

String? readOptionalString() {
final len = readU32();
if (len == 0) return null;
ensureRemaining(len);
final s = utf8.decode(_bytes.sublist(_pos, _pos + len));
_pos += len;
return s;
}

Uint8List? readOptionalBytes() {
final len = readU32();
if (len == 0) return null;
ensureRemaining(len);
final out = Uint8List.fromList(_bytes.sublist(_pos, _pos + len));
_pos += len;
return out;
}

String readUuid() {
ensureRemaining(16);
final hex = StringBuffer();
for (var i = 0; i < 16; i++) {{ hex.write(_bytes[_pos + i].toRadixString(16).padLeft(2, '0')); }}
_pos += 16;
final h = hex.toString();
return '${h.substring(0, 8)}-${h.substring(8, 12)}-${h.substring(12, 16)}-${h.substring(16, 20)}-${h.substring(20)}';
}

int? readOptionU32() {
final flag = readU8();
if (flag == 0) return null;
if (flag == 1) return readU32();
throw CodecError('invalid Option<u32> flag: $flag');
}

String? readUpdatableString() {
final flag = readU8();
if (flag == 0) return null;
if (flag == 1) return readString();
throw CodecError('invalid updatable string flag: $flag');
}

Uint8List readBytes(int n) {
ensureRemaining(n);
final out = Uint8List.fromList(_bytes.sublist(_pos, _pos + n));
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

void skip(int n) { ensureRemaining(n); _pos += n; }
}
