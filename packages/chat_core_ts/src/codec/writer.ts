// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import { CodecError } from './error.js';

const textEncoder = new TextEncoder();

export class ProtocolWriter {
private buf: Uint8Array;
private view: DataView;
private pos: number;

constructor(initialCapacity = 256) {
this.buf = new Uint8Array(initialCapacity);
this.view = new DataView(this.buf.buffer);
this.pos = 0;
}

private grow(needed: number): void {
const required = this.pos + needed;
if (required <= this.buf.length) return;
let newLen = this.buf.length * 2;
while (newLen < required) newLen *= 2;
const next = new Uint8Array(newLen);
next.set(this.buf);
this.buf = next;
this.view = new DataView(this.buf.buffer);
}

writeU8(v: number): void { this.grow(1); this.view.setUint8(this.pos++, v); }

writeU16(v: number): void {
this.grow(2); this.view.setUint16(this.pos, v, true); this.pos += 2;
}

writeU32(v: number): void {
this.grow(4); this.view.setUint32(this.pos, v, true); this.pos += 4;
}

writeI64(v: number): void {
this.grow(8);
this.view.setUint32(this.pos, v % 4294967296, true);
this.view.setInt32(this.pos + 4, Math.floor(v / 4294967296), true);
this.pos += 8;
}

writeTimestamp(v: number): void {
if (v < 0 || v > 2199023255551) throw new CodecError(`timestamp out of range: ${v}`);
this.writeI64(v);
}

writeString(v: string): void {
if (v.length === 0) { this.writeU32(0); return; }
const encoded = textEncoder.encode(v);
this.writeU32(encoded.length);
this.grow(encoded.length);
this.buf.set(encoded, this.pos);
this.pos += encoded.length;
}

writeOptionalString(v: string | null): void {
if (v === null) { this.writeU32(0); } else { this.writeString(v); }
}

writeOptionalBytes(v: Uint8Array | null): void {
if (v === null) { this.writeU32(0); return; }
this.writeU32(v.length);
this.grow(v.length);
this.buf.set(v, this.pos);
this.pos += v.length;
}

writeUuid(uuid: string): void {
this.grow(16);
const hex = uuid.replace(/-/g, '');
for (let i = 0; i < 16; i++) this.buf[this.pos++] = parseInt(hex.slice(i * 2, i * 2 + 2), 16);
}

writeOptionU32(v: number | null): void {
if (v === null) { this.writeU8(0); } else { this.writeU8(1); this.writeU32(v); }
}

writeUpdatableString(v: string | null): void {
if (v === null) { this.writeU8(0); } else { this.writeU8(1); this.writeString(v); }
}

writeRawBytes(data: Uint8Array): void {
this.grow(data.length);
this.buf.set(data, this.pos);
this.pos += data.length;
}

toBytes(): Uint8Array { return this.buf.slice(0, this.pos); }
}
