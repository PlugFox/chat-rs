// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'dart:convert';
import 'dart:typed_data';

import 'error.dart';

class ProtocolWriter {
ProtocolWriter([int initialCapacity = 256])
: _buf = Uint8List(initialCapacity),
_data = ByteData(initialCapacity);

Uint8List _buf;
ByteData _data;
int _pos = 0;

void _grow(int needed) {
final required = _pos + needed;
if (required <= _buf.length) return;
var newLen = _buf.length * 2;
while (newLen < required) {{ newLen *= 2; }}
final next = Uint8List(newLen);
next.setAll(0, _buf);
_buf = next;
_data = ByteData.sublistView(next);
}

void writeU8(int v) { _grow(1); _data.setUint8(_pos++, v); }

void writeU16(int v) {
_grow(2); _data.setUint16(_pos, v, Endian.little); _pos += 2;
}

void writeU32(int v) {
_grow(4); _data.setUint32(_pos, v, Endian.little); _pos += 4;
}

void writeI64(int v) {
_grow(8); _data.setInt64(_pos, v, Endian.little); _pos += 8;
}

void writeTimestamp(int v) {
if (v < 0 || v > 2199023255551) throw CodecError('timestamp out of range: $v');
writeI64(v);
}

void writeString(String v) {
if (v.isEmpty) { writeU32(0); return; }
final encoded = utf8.encode(v);
writeU32(encoded.length);
_grow(encoded.length);
_buf.setAll(_pos, encoded);
_pos += encoded.length;
}

void writeOptionalString(String? v) {
if (v == null) { writeU32(0); } else { writeString(v); }
}

void writeOptionalBytes(Uint8List? v) {
if (v == null) { writeU32(0); return; }
writeU32(v.length);
_grow(v.length);
_buf.setAll(_pos, v);
_pos += v.length;
}

void writeUuid(String uuid) {
_grow(16);
final hex = uuid.replaceAll('-', '');
for (var i = 0; i < 16; i++) {{ _buf[_pos++] = int.parse(hex.substring(i * 2, i * 2 + 2), radix: 16); }}
}

void writeOptionU32(int? v) {
if (v == null) { writeU8(0); } else { writeU8(1); writeU32(v); }
}

void writeUpdatableString(String? v) {
if (v == null) { writeU8(0); } else { writeU8(1); writeString(v); }
}

void writeRawBytes(Uint8List data) {
_grow(data.length);
_buf.setAll(_pos, data);
_pos += data.length;
}

Uint8List toBytes() => Uint8List.fromList(_buf.sublist(0, _pos));
}
