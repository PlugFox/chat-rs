// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import { CodecError } from "./error.js";

const textDecoder = new TextDecoder();

export class ProtocolReader {
  private readonly view: DataView;
  private readonly bytes: Uint8Array;
  private pos: number;

  constructor(data: Uint8Array) {
    this.bytes = data;
    this.view = new DataView(data.buffer, data.byteOffset, data.byteLength);
    this.pos = 0;
  }

  get remaining(): number {
    return this.bytes.byteLength - this.pos;
  }

  ensureRemaining(n: number): void {
    if (this.remaining < n)
      throw new CodecError(
        `truncated: need ${n} bytes but only ${this.remaining} remain`,
      );
  }

  readU8(): number {
    this.ensureRemaining(1);
    return this.view.getUint8(this.pos++);
  }

  readU16(): number {
    this.ensureRemaining(2);
    const v = this.view.getUint16(this.pos, true);
    this.pos += 2;
    return v;
  }

  readU32(): number {
    this.ensureRemaining(4);
    const v = this.view.getUint32(this.pos, true);
    this.pos += 4;
    return v;
  }

  readI64(): number {
    this.ensureRemaining(8);
    const lo = this.view.getUint32(this.pos, true);
    const hi = this.view.getInt32(this.pos + 4, true);
    this.pos += 8;
    return hi * 4294967296 + lo;
  }

  readTimestamp(): number {
    const v = this.readI64();
    if (v < 0 || v > 2199023255551)
      throw new CodecError(`timestamp out of range: ${v}`);
    return v;
  }

  readString(): string {
    const len = this.readU32();
    if (len === 0) return "";
    this.ensureRemaining(len);
    const s = textDecoder.decode(this.bytes.subarray(this.pos, this.pos + len));
    this.pos += len;
    return s;
  }

  readOptionalString(): string | null {
    const len = this.readU32();
    if (len === 0) return null;
    this.ensureRemaining(len);
    const s = textDecoder.decode(this.bytes.subarray(this.pos, this.pos + len));
    this.pos += len;
    return s;
  }

  readOptionalBytes(): Uint8Array | null {
    const len = this.readU32();
    if (len === 0) return null;
    this.ensureRemaining(len);
    const out = this.bytes.slice(this.pos, this.pos + len);
    this.pos += len;
    return out;
  }

  readUuid(): string {
    this.ensureRemaining(16);
    const hex: string[] = [];
    for (let i = 0; i < 16; i++)
      hex.push(this.bytes[this.pos + i]!.toString(16).padStart(2, "0"));
    this.pos += 16;
    const h = hex.join("");
    return `${h.slice(0, 8)}-${h.slice(8, 12)}-${h.slice(12, 16)}-${h.slice(16, 20)}-${h.slice(20)}`;
  }

  readOptionU32(): number | null {
    const flag = this.readU8();
    if (flag === 0) return null;
    if (flag === 1) return this.readU32();
    throw new CodecError(`invalid Option<u32> flag: ${flag}`);
  }

  readUpdatableString(): string | null {
    const flag = this.readU8();
    if (flag === 0) return null;
    if (flag === 1) return this.readString();
    throw new CodecError(`invalid updatable string flag: ${flag}`);
  }

  readBytes(n: number): Uint8Array {
    this.ensureRemaining(n);
    const out = this.bytes.slice(this.pos, this.pos + n);
    this.pos += n;
    return out;
  }

  readVecU32(): number[] {
    const count = this.readU16();
    const out: number[] = [];
    for (let i = 0; i < count; i++) out.push(this.readU32());
    return out;
  }

  readVecString(): string[] {
    const count = this.readU16();
    const out: string[] = [];
    for (let i = 0; i < count; i++) out.push(this.readString());
    return out;
  }

  readArray<T>(count: number, readOne: () => T): T[] {
    const out: T[] = [];
    for (let i = 0; i < count; i++) out.push(readOne());
    return out;
  }

  readEnum<T>(
    raw: number,
    fromValue: (v: number) => T | undefined,
    typeName: string,
  ): T {
    const v = fromValue(raw);
    if (v === undefined)
      throw new CodecError(`unknown ${typeName} discriminant: ${raw}`);
    return v;
  }

  skip(n: number): void {
    this.ensureRemaining(n);
    this.pos += n;
  }
}
