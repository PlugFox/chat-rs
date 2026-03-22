# Code Generation: Dart & TypeScript Protocol Packages

> Roadmap for generating `packages/chat_core_dart` and `packages/chat_core_ts` from `chat_protocol` Rust crate.

## Decisions Log

| Decision            | Choice                                                       | Rationale                                                      |
| ------------------- | ------------------------------------------------------------ | -------------------------------------------------------------- |
| UUID representation | `String` (hex with dashes)                                   | Easier debugging, JSON compat; codec converts to/from 16 bytes |
| Dart bitflags       | `extension type` on `int`                                    | Zero-cost, operator support, per user's existing pattern       |
| TS bitflags         | `const enum` + helper object                                 | Tree-shakeable, numeric at runtime                             |
| Dart naming         | Standard (`ChatKind`, `MessageFlags`)                        | No `$` prefix                                                  |
| TS target           | ES2022, with `tsconfig.json` and build step                  | Must produce `.js` + `.d.ts` for Dart `js_interop`             |
| Timestamps          | `number` in TS, `int` in Dart                                | Unix seconds < 2^41, safe in JS Number                         |
| Codegen tool        | `cargo xtask codegen` using `syn`                            | All automation via xtask per project rules                     |
| Tests               | Standalone per-language first, cross-language fixtures later |
| AckPayload codec    | Not generated — hand-written client logic                    | Context-dependent decoding                                     |

---

## Phase 0: xtask Codegen Infrastructure

### 0.1 — xtask restructure

Refactor `xtask/src/main.rs` into modules:
```
xtask/
  src/
    main.rs          — CLI dispatch (add "codegen" command)
    check.rs         — existing check/fmt/test
    codegen/
      mod.rs         — orchestrator: parse → IR → generate
      parser.rs      — syn-based Rust source parser
      ir.rs          — intermediate representation types
      dart.rs        — Dart code emitter
      typescript.rs  — TypeScript code emitter
```

**Acceptance criteria:**
- `cargo xtask codegen` runs without error
- `cargo xtask help` lists the new command
- Existing `check`/`fmt`/`test` commands unchanged

### 0.2 — Add `syn` and `proc-macro2` to xtask dependencies

Add to `xtask/Cargo.toml`:
```toml
[dependencies]
syn = { version = "2", features = ["full", "parsing", "visit"] }
proc-macro2 = "1"
quote = "1"          # for tests only, if needed
```

**Acceptance criteria:**
- `cargo build -p xtask` compiles with new dependencies

---

## Phase 1: Rust Source Parser

### 1.1 — Intermediate Representation (IR)

Define IR types that capture everything needed for codegen:

```rust
/// A parsed Rust source file.
struct ParsedModule {
    enums: Vec<EnumDef>,
    structs: Vec<StructDef>,
    bitflags: Vec<BitflagsDef>,
    constants: Vec<ConstDef>,
}

/// #[repr(u8/u16)] enum with explicit discriminants.
struct EnumDef {
    name: String,
    doc: String,
    repr: ReprType,         // U8, U16
    variants: Vec<EnumVariant>,
}

struct EnumVariant {
    name: String,
    doc: String,
    discriminant: u64,      // e.g. 0x10 for SendMessage
}

/// Tagged enum (Rust enum with data fields per variant).
/// Examples: LoadChatsPayload, MemberAction, SearchScope, FramePayload.
struct TaggedEnumDef {
    name: String,
    doc: String,
    variants: Vec<TaggedVariant>,
}

struct TaggedVariant {
    name: String,
    doc: String,
    kind: VariantKind,      // Unit, Tuple(Vec<FieldType>), Struct(Vec<Field>)
}

/// Regular struct.
struct StructDef {
    name: String,
    doc: String,
    fields: Vec<Field>,
}

struct Field {
    name: String,
    doc: String,
    ty: FieldType,
}

/// bitflags! block.
struct BitflagsDef {
    name: String,
    doc: String,
    repr: ReprType,         // U16, U32
    flags: Vec<FlagEntry>,
}

struct FlagEntry {
    name: String,
    doc: String,
    value: u64,             // literal or expression like 1 << 10
    value_expr: String,     // preserved expression string for codegen
}

enum FieldType {
    U8, U16, U32, I64,
    Bool,
    String,
    OptionalString,         // Option<String> — wire: len=0 for None
    UpdatableString,        // Option<String> with u8-flag semantics (UpdateChat, UpdateProfile)
    Uuid,
    OptionalU32,            // Option<u32> — wire: u8 flag + u32
    VecU32,                 // Vec<u32> — wire: u16 count + u32[]
    VecU8,                  // Vec<u8> — raw bytes
    OptionalBytes,          // Option<Vec<u8>> — wire: u32 len, None when 0
    VecString,              // Vec<String> — wire: u16 count + strings
    Enum(String),           // named enum (e.g. ChatKind, MessageKind)
    Bitflags(String),       // named bitflags (e.g. Permission, MessageFlags)
    Struct(String),         // nested struct (e.g. ServerLimits)
    OptionalStruct(String), // Option<struct> — wire: u8 flag + struct
    VecStruct(String),      // Vec<struct> — wire: count + structs
    TaggedEnum(String),     // inline tagged enum (e.g. MemberAction)
}
```

**Acceptance criteria:**
- IR types compile
- `FieldType` covers every field pattern in `types/*.rs` (verified by manual audit)

### 1.2 — syn Parser: Enums and Structs

Parse `crates/chat_protocol/src/types/*.rs` files using `syn`:

**Input files (in order):**
1. `types/chat.rs` → ChatKind, ChatRole, Permission, LastMessagePreview, ChatEntry, ChatMemberEntry
2. `types/message.rs` → MessageKind, MessageFlags, RichStyle, RichSpan, Message, MessageBatch
3. `types/user.rs` → UserFlags, UserEntry, PresenceStatus, PresenceEntry
4. `types/error.rs` → ErrorCode, DisconnectCode, ErrorPayload
5. `types/frame.rs` → FrameKind, all payload structs, FramePayload, Frame, AckPayload, etc.

**Parser must handle:**
- `#[repr(u8)]` / `#[repr(u16)]` enums with literal discriminants (decimal and hex)
- Plain structs with `pub` fields
- Doc comments (`///`) preserved as strings
- Field type mapping: `u32` → `U32`, `Option<u32>` → `OptionalU32`, `Vec<u32>` → `VecU32`, etc.
- Cross-file type references: `super::MessageKind` → `Enum("MessageKind")`

**Acceptance criteria:**
- Parser extracts all enums from all 5 files with correct names, discriminants, docs
- Parser extracts all structs with correct field names, types, docs
- Unit tests: parse each file, assert expected IR output

### 1.3 — syn Parser: bitflags! Macro

`bitflags!` invocations are not standard Rust items — `syn` won't parse them as enums.

**Approach:** Custom parser that:
1. Finds `bitflags! { ... }` blocks via text search or `syn::Macro` items
2. Extracts the inner struct name, repr type from `#[derive(...)]` and struct definition
3. Parses `const NAME = expr;` entries

**Target bitflags blocks (6 total):**
- `Permission: u32` (chat.rs)
- `UserFlags: u16` (user.rs)
- `MessageFlags: u16` (message.rs)
- `RichStyle: u16` (message.rs)
- `ServerCapabilities: u32` (frame.rs)

**Acceptance criteria:**
- All 5 bitflags parsed with correct names, repr types, flag entries
- Flag expressions preserved (e.g. `1 << 10`, `0x0001`)
- Unit tests for each bitflags block

### 1.4 — syn Parser: Tagged Enums

Tagged enums have data in variants. These need special handling for both types AND codec.

**Target tagged enums:**
- `LoadChatsPayload` — 2 variants with named fields
- `LoadMessagesPayload` — 2 variants with named fields
- `SearchScope` — 3 variants (2 with fields, 1 unit)
- `MemberAction` — 6 variants (mix of unit, newtype, struct)
- `LoadDirection` — simple repr(u8) but without data, so it's a plain enum
- `AckPayload` — 10 variants (skip codec generation, types only)
- `FramePayload` — 40+ variants (NOT generated as a type — this is dispatch logic)

**Acceptance criteria:**
- All tagged enums parsed into `TaggedEnumDef` IR
- Variant fields correctly typed
- Unit tests

### 1.5 — Constants Parser

Parse `lib.rs` for protocol constants:
- `PROTOCOL_VERSION: u8 = 1`
- `FRAME_HEADER_SIZE: usize = 9`
- `MIN_TIMESTAMP: i64 = 0`
- `MAX_TIMESTAMP: i64 = (1 << 41) - 1`
- `EVENT_SEQ_OVERFLOW_MASK: u32 = 0xC000_0000`

Also `ServerLimits::default()` values (can be hardcoded in IR since they're constants).

**Acceptance criteria:**
- All 5 constants extracted with names, types, values
- Unit test

---

## Phase 2: Dart Package Generation

### 2.0 — Package Scaffold

Create `packages/chat_core_dart/` with:
```
packages/chat_core_dart/
  pubspec.yaml
  lib/
    chat_core.dart              ← barrel export (generated)
    src/
      types/                    ← generated
        chat_kind.dart
        chat_role.dart
        permission.dart
        ...
      codec/                    ← generated
        reader.dart             ← ByteData reader utilities
        writer.dart             ← ByteData writer utilities
        codec.dart              ← encode/decode per payload
      protocol_constants.dart   ← generated
  test/
    types_test.dart             ← generated scaffolds, hand-fill
    codec_test.dart
```

**`pubspec.yaml`:**
```yaml
name: chat_core
description: Chat protocol types and binary codec.
version: 0.1.0
environment:
  sdk: ^3.7.0
dev_dependencies:
  test: ^1.25.0
```

No external dependencies — `dart:typed_data` for codec.

**Acceptance criteria:**
- `dart pub get` succeeds
- `dart analyze` clean on empty scaffold

### 2.1 — Dart Emitter: Enums

Generate Dart enums from `EnumDef` IR.

**Pattern for `#[repr(u8)]` enums (e.g. ChatKind):**
```dart
/// Chat type.
enum ChatKind {
  /// Direct message (exactly two participants, no title).
  direct(0),
  /// Group conversation with multiple members.
  group(1),
  /// Read-mostly broadcast room nested inside a Group.
  channel(2);

  const ChatKind(this.value);
  final int value;

  static ChatKind? fromValue(int value) => switch (value) {
    0 => direct,
    1 => group,
    2 => channel,
    _ => null,
  };
}
```

**Target enums (6):**
- `ChatKind` (u8, 3 variants)
- `ChatRole` (u8, 4 variants)
- `MessageKind` (u8, 4 variants)
- `PresenceStatus` (u8, 2 variants)
- `LoadDirection` (u8, 2 variants)
- `FrameKind` (u8, 56 variants — hex discriminants)
- `ErrorCode` (u16, 22 variants)
- `DisconnectCode` (u16, 11 variants)

**Extra methods to generate:**
- `ErrorCode`: `slug` getter, `isPermanent`, `isTransient` (from Rust impl)
- `DisconnectCode`: `shouldReconnect` (from Rust impl)

**Acceptance criteria:**
- All 8 enums generated with correct values and docs
- `ErrorCode.slug`, `isPermanent`, `isTransient` present
- `DisconnectCode.shouldReconnect` present
- `dart analyze` clean

### 2.2 — Dart Emitter: Bitflags as Extension Types

Generate bitflags from `BitflagsDef` IR using the `extension type` pattern.

**Pattern (e.g. Permission):**
```dart
/// Per-member permission flags.
extension type const Permission(int value) implements int {
  static const Permission sendMessages = Permission(1 << 0);
  static const Permission sendMedia = Permission(1 << 1);
  // ...
  static const Permission deleteChatPerm = Permission(1 << 31);

  static const List<Permission> values = [sendMessages, sendMedia, ...];

  bool contains(Permission flag) => (value & flag.value) != 0;
  Permission add(Permission flag) => Permission(value | flag.value);
  Permission remove(Permission flag) => Permission(value & ~flag.value);
  Permission toggle(Permission flag) => Permission(value ^ flag.value);
  bool get isEmpty => value == 0;
  bool get isNotEmpty => value != 0;
  Permission operator ^(Permission other) => Permission(value ^ other.value);
}
```

**Target bitflags (5):**
- `Permission: u32` (15 flags)
- `UserFlags: u16` (3 flags)
- `MessageFlags: u16` (8 flags)
- `RichStyle: u16` (11 flags)
- `ServerCapabilities: u32` (5 flags)

**Naming:** Rust `SCREAMING_SNAKE` → Dart `camelCase` (e.g. `SEND_MESSAGES` → `sendMessages`).

**Acceptance criteria:**
- All 5 bitflags generated with correct values
- `contains`, `add`, `remove`, `toggle`, `isEmpty`, `isNotEmpty` methods
- `dart analyze` clean

### 2.3 — Dart Emitter: Structs (Data Classes)

Generate immutable Dart classes from `StructDef` IR.

**Pattern (e.g. ChatEntry):**
```dart
/// A chat entry as transmitted on the wire.
class ChatEntry {
  const ChatEntry({
    required this.id,
    required this.kind,
    this.parentId,
    required this.createdAt,
    required this.updatedAt,
    this.title,
    this.avatarUrl,
    this.lastMessage,
    required this.unreadCount,
    required this.memberCount,
  });

  final int id;
  final ChatKind kind;
  final int? parentId;
  final int createdAt;
  final int updatedAt;
  final String? title;
  final String? avatarUrl;
  final LastMessagePreview? lastMessage;
  final int unreadCount;
  final int memberCount;

  @override
  bool operator ==(Object other) => ...;
  @override
  int get hashCode => ...;
  @override
  String toString() => ...;
}
```

**Field naming:** Rust `snake_case` → Dart `camelCase`.

**Type mapping:**
| Rust               | Dart           |
| ------------------ | -------------- |
| `u8`, `u16`, `u32` | `int`          |
| `i64`              | `int`          |
| `bool`             | `bool`         |
| `String`           | `String`       |
| `Option<String>`   | `String?`      |
| `Option<u32>`      | `int?`         |
| `Uuid`             | `String`       |
| `Vec<u32>`         | `List<int>`    |
| `Vec<u8>`          | `Uint8List`    |
| `Option<Vec<u8>>`  | `Uint8List?`   |
| `Vec<String>`      | `List<String>` |
| Enum reference     | Enum type      |
| Bitflags reference | Extension type |
| Struct reference   | Class type     |
| `Option<Struct>`   | `Class?`       |
| `Vec<Struct>`      | `List<Class>`  |

**Target structs (~30):**
All payload structs from `frame.rs` + entity structs from `chat.rs`, `message.rs`, `user.rs`, `error.rs`.

**Acceptance criteria:**
- All structs generated with correct fields, types, and docs
- Named constructors with `required` for non-nullable fields
- `==`, `hashCode`, `toString` overrides
- `dart analyze` clean

### 2.4 — Dart Emitter: Tagged Enums

Generate sealed class hierarchies from `TaggedEnumDef` IR.

**Pattern (e.g. LoadChatsPayload):**
```dart
sealed class LoadChatsPayload {
  const LoadChatsPayload();
}

class LoadChatsFirstPage extends LoadChatsPayload {
  const LoadChatsFirstPage({required this.limit});
  final int limit;
}

class LoadChatsAfter extends LoadChatsPayload {
  const LoadChatsAfter({required this.cursorTs, required this.limit});
  final int cursorTs;
  final int limit;
}
```

**Target tagged enums (5, excluding FramePayload and AckPayload):**
- `LoadChatsPayload` (2 variants)
- `LoadMessagesPayload` (2 variants)
- `SearchScope` (3 variants)
- `MemberAction` (6 variants)
- `AckPayload` (10 variants — types only, no codec)

**Acceptance criteria:**
- All tagged enums generated as sealed class hierarchies
- `dart analyze` clean

### 2.5 — Dart Emitter: Barrel Export

Generate `lib/chat_core.dart`:
```dart
/// Chat protocol types and binary codec.
library;

export 'src/types/chat_kind.dart';
export 'src/types/chat_role.dart';
// ... all type files
export 'src/protocol_constants.dart';
export 'src/codec/reader.dart';
export 'src/codec/writer.dart';
export 'src/codec/codec.dart';
```

**Acceptance criteria:**
- Single import `package:chat_core/chat_core.dart` gives access to all public types
- No naming conflicts in barrel

---

## Phase 3: TypeScript Package Generation

### 3.0 — Package Scaffold

Create `packages/chat_core_ts/` with:
```
packages/chat_core_ts/
  package.json
  tsconfig.json
  src/
    types/                      ← generated
      chat-kind.ts
      chat-role.ts
      permission.ts
      ...
    codec/                      ← generated
      reader.ts                 ← DataView reader utilities
      writer.ts                 ← DataView writer utilities
      codec.ts                  ← encode/decode per payload
    constants.ts                ← generated
    index.ts                    ← barrel export (generated)
  tests/
    types.test.ts
    codec.test.ts
```

**`tsconfig.json`:** target ES2022, module ESNext, declaration: true, outDir: dist/.

**`package.json`:** exports both ESM and types.

**Acceptance criteria:**
- `npm install` (or `pnpm install`) succeeds
- `tsc --noEmit` clean on empty scaffold

### 3.1 — TS Emitter: Enums

**Pattern (e.g. ChatKind):**
```typescript
/** Chat type. */
export const enum ChatKind {
  /** Direct message (exactly two participants, no title). */
  Direct = 0,
  /** Group conversation with multiple members. */
  Group = 1,
  /** Read-mostly broadcast room nested inside a Group. */
  Channel = 2,
}

/** Convert wire value to ChatKind, or undefined if unknown. */
export function chatKindFromValue(value: number): ChatKind | undefined {
  if (value >= 0 && value <= 2) return value as ChatKind;
  return undefined;
}
```

**Acceptance criteria:**
- All 8 enums generated
- `ErrorCode` has `errorCodeSlug()`, `isErrorPermanent()`, `isErrorTransient()`
- `DisconnectCode` has `shouldReconnect()`
- `tsc --noEmit` clean

### 3.2 — TS Emitter: Bitflags

**Pattern (e.g. Permission):**
```typescript
/** Per-member permission flags. */
export namespace Permission {
  export const SendMessages         = 1 << 0;
  export const SendMedia            = 1 << 1;
  // ...

  export function contains(flags: number, flag: number): boolean {
    return (flags & flag) !== 0;
  }
  export function add(flags: number, flag: number): number {
    return flags | flag;
  }
  export function remove(flags: number, flag: number): number {
    return flags & ~flag;
  }
}
export type Permission = number;
```

**Acceptance criteria:**
- All 5 bitflags generated
- `tsc --noEmit` clean

### 3.3 — TS Emitter: Interfaces

**Pattern (e.g. ChatEntry):**
```typescript
/** A chat entry as transmitted on the wire. */
export interface ChatEntry {
  readonly id: number;
  readonly kind: ChatKind;
  readonly parentId: number | null;
  readonly createdAt: number;
  readonly updatedAt: number;
  readonly title: string | null;
  readonly avatarUrl: string | null;
  readonly lastMessage: LastMessagePreview | null;
  readonly unreadCount: number;
  readonly memberCount: number;
}
```

**Type mapping:**
| Rust                      | TypeScript           |
| ------------------------- | -------------------- |
| `u8`, `u16`, `u32`, `i64` | `number`             |
| `bool`                    | `boolean`            |
| `String`                  | `string`             |
| `Option<String>`          | `string \| null`     |
| `Option<u32>`             | `number \| null`     |
| `Uuid`                    | `string`             |
| `Vec<u32>`                | `readonly number[]`  |
| `Vec<u8>`                 | `Uint8Array`         |
| `Option<Vec<u8>>`         | `Uint8Array \| null` |
| `Vec<String>`             | `readonly string[]`  |
| Enum/Bitflags/Struct      | named type           |

**Acceptance criteria:**
- All interfaces generated
- `tsc --noEmit` clean

### 3.4 — TS Emitter: Tagged Enums

**Pattern (discriminated union):**
```typescript
export type LoadChatsPayload =
  | { readonly type: 'firstPage'; readonly limit: number }
  | { readonly type: 'after'; readonly cursorTs: number; readonly limit: number };
```

**Acceptance criteria:**
- All tagged enums generated as discriminated unions
- `tsc --noEmit` clean

### 3.5 — TS Emitter: Barrel Export

Generate `src/index.ts` that re-exports everything.

**Acceptance criteria:**
- Single import from package gives all public types
- `tsc` builds to `dist/` with `.js` + `.d.ts`

---

## Phase 4: Binary Codec Generation

### 4.0 — Codec Utilities (Hand-Written)

**Dart — `lib/src/codec/reader.dart`:**
```dart
class ProtocolReader {
  ProtocolReader(this.data, [this.offset = 0]);
  final ByteData data;
  int offset;

  int readU8() => ...;
  int readU16() => ...;   // LE
  int readU32() => ...;   // LE
  int readI64() => ...;   // LE
  int readTimestamp() => ...; // validates 0 ≤ v < 2^41
  String readString() => ...; // u32 len + UTF-8
  String? readOptionalString() => ...; // u32 len, null when 0
  Uint8List? readOptionalBytes() => ...;
  String readUuid() => ...; // 16 bytes → hex string
  int? readOptionU32() => ...; // u8 flag + u32
  void ensureRemaining(int n) => ...;
}
```

**Dart — `lib/src/codec/writer.dart`:**
```dart
class ProtocolWriter {
  ProtocolWriter([int initialCapacity = 256]);
  // grows automatically

  void writeU8(int v) => ...;
  void writeU16(int v) => ...;
  void writeU32(int v) => ...;
  void writeI64(int v) => ...;
  void writeTimestamp(int v) => ...;
  void writeString(String v) => ...;
  void writeOptionalString(String? v) => ...;
  void writeOptionalBytes(Uint8List? v) => ...;
  void writeUuid(String uuid) => ...;
  void writeOptionU32(int? v) => ...;

  Uint8List toBytes() => ...;
}
```

**TypeScript — `src/codec/reader.ts` and `writer.ts`:** Same API using `DataView`.

**Acceptance criteria:**
- Reader/Writer handle all wire primitives from codec.md
- Unit tests for each primitive: roundtrip, edge cases (empty string, null, max u32, timestamp validation)

### 4.1 — Codec Generation: Structs

For each `StructDef`, generate `encode` and `decode` functions.

**Dart pattern:**
```dart
void encodeChatEntry(ProtocolWriter w, ChatEntry v) {
  w.writeU32(v.id);
  w.writeU8(v.kind.value);
  w.writeOptionU32(v.parentId);
  w.writeTimestamp(v.createdAt);
  w.writeTimestamp(v.updatedAt);
  w.writeOptionalString(v.title);
  w.writeOptionalString(v.avatarUrl);
  if (v.lastMessage != null) {
    w.writeU8(1);
    encodeLastMessagePreview(w, v.lastMessage!);
  } else {
    w.writeU8(0);
  }
  w.writeU32(v.unreadCount);
  w.writeU32(v.memberCount);
}

ChatEntry decodeChatEntry(ProtocolReader r) {
  return ChatEntry(
    id: r.readU32(),
    kind: ChatKind.fromValue(r.readU8())!,
    parentId: r.readOptionU32(),
    createdAt: r.readTimestamp(),
    updatedAt: r.readTimestamp(),
    title: r.readOptionalString(),
    avatarUrl: r.readOptionalString(),
    lastMessage: r.readU8() == 1 ? decodeLastMessagePreview(r) : null,
    unreadCount: r.readU32(),
    memberCount: r.readU32(),
  );
}
```

**Wire format encoding rules (derived from Rust codec):**

| FieldType              | Encode                          | Decode                     |
| ---------------------- | ------------------------------- | -------------------------- |
| `U8`                   | `writeU8`                       | `readU8`                   |
| `U16`                  | `writeU16`                      | `readU16`                  |
| `U32`                  | `writeU32`                      | `readU32`                  |
| `I64`                  | `writeI64`                      | `readI64`                  |
| `Bool`                 | `writeU8(v ? 1 : 0)`            | `readU8() != 0`            |
| `String`               | `writeString`                   | `readString`               |
| `OptionalString`       | `writeOptionalString`           | `readOptionalString`       |
| `UpdatableString`      | `writeU8(flag) + writeString`   | `readU8 → readString`      |
| `Uuid`                 | `writeUuid`                     | `readUuid`                 |
| `OptionalU32`          | `writeOptionU32`                | `readOptionU32`            |
| `VecU32`               | `writeU16(len) + writeU32[]`    | `readU16 → readU32[]`      |
| `OptionalBytes`        | `writeOptionalBytes`            | `readOptionalBytes`        |
| `VecString`            | `writeU16(len) + writeString[]` | `readU16 → readString[]`   |
| `Enum(name)`           | `writeU8(v.value)`              | `Enum.fromValue(readU8())` |
| `Bitflags(name)`       | `writeU16/U32(v.value)`         | `Type(readU16/U32())`      |
| `Struct(name)`         | `encodeX(w, v)`                 | `decodeX(r)`               |
| `OptionalStruct(name)` | `writeU8(flag) + encodeX`       | `readU8 → decodeX`         |
| `VecStruct(name)`      | `writeU32(len) + encodeX[]`     | `readU32 → decodeX[]`      |

**Special cases:**
- `ErrorPayload.code` — encoded as `u16`, plus slug as `u8 len + UTF-8` (not standard string)
- `Message.reply_to_id` — `u8 flag + u32`, not standard `Option<u32>` (same wire, just documenting)
- `UpdateChatPayload` / `UpdateProfilePayload` — `UpdatableString` fields use `u8` flag before each string

**Acceptance criteria:**
- Encode/decode generated for every struct (except Frame/FramePayload which are dispatch)
- Field order matches Rust codec exactly
- `dart analyze` / `tsc --noEmit` clean

### 4.2 — Codec Generation: Tagged Enums

For each tagged enum, generate encode/decode with discriminant byte.

**Pattern (LoadMessagesPayload):**
```dart
void encodeLoadMessages(ProtocolWriter w, LoadMessagesPayload v) {
  switch (v) {
    case LoadMessagesPaginate p:
      w.writeU32(p.chatId);
      w.writeU8(0); // mode
      w.writeU8(p.direction.value);
      w.writeU32(p.anchorId);
      w.writeU16(p.limit);
    case LoadMessagesRangeCheck p:
      w.writeU32(p.chatId);
      w.writeU8(1); // mode
      w.writeU32(p.fromId);
      w.writeU32(p.toId);
      w.writeTimestamp(p.sinceTs);
  }
}
```

**Acceptance criteria:**
- All tagged enums (except AckPayload, FramePayload) have encode/decode
- Wire format matches Rust codec

### 4.3 — Frame Header Codec

Generate frame header encode/decode (9 bytes: kind + seq + event_seq).

**Acceptance criteria:**
- `encodeFrameHeader` / `decodeFrameHeader` in both Dart and TS
- Roundtrip tests

### 4.4 — Frame Dispatch (Encode/Decode Full Frame)

Generate the top-level `encodeFrame` / `decodeFrame` that dispatches on `FrameKind`.

This is the glue that connects header + payload codec. Maps each `FrameKind` to its
encode/decode function.

**Not generated for AckPayload** — the caller provides decoding context.

**Acceptance criteria:**
- `encodeFrame(Frame)` → `Uint8Array` / `Uint8List`
- `decodeFrame(bytes)` → `Frame` (with typed payload)
- All frame kinds except Ack are handled
- Ack returns raw bytes that caller decodes

---

## Phase 5: Tests

### 5.1 — Dart Unit Tests: Types

Test enum roundtrip (`fromValue`), bitflags operations, struct equality.

**Acceptance criteria:**
- Every enum: `fromValue(x.value) == x` for all variants + `null` for invalid
- Every bitflags: `contains`, `add`, `remove`, `toggle`, `isEmpty`
- Every struct: equality, toString not null

### 5.2 — Dart Unit Tests: Codec

Test encode → decode roundtrip for every payload type.

**Acceptance criteria:**
- Every struct: `decode(encode(x)) == x`
- Edge cases: empty strings, null optionals, zero-length vecs, max u32, timestamp boundaries
- Frame roundtrip: full frame encode → decode for representative frame kinds

### 5.3 — TS Unit Tests: Types + Codec

Mirror the Dart tests in TypeScript.

**Acceptance criteria:**
- Same coverage as Dart tests
- `npm test` passes

### 5.4 — xtask Verification

Add `cargo xtask codegen --check` mode that:
1. Runs codegen to a temp directory
2. Diffs against existing generated files
3. Fails if any differences (like `cargo fmt --check`)

**Acceptance criteria:**
- CI can run `cargo xtask codegen --check` to verify generated code is up to date
- Non-zero exit on drift

---

## Phase 6: WebSocket Client (Post-Codegen, Manual)

> Not part of codegen. Separate task after Phases 0–5.

### 6.1 — Dart WebSocket Client
### 6.2 — TS WebSocket Client
### 6.3 — Reconnect Logic
### 6.4 — Seq Manager + Pending Request Map

---

## Execution Order

```
Phase 0 (xtask infra)     ███░░░░░░░░░░░░░░░░░
Phase 1 (parser)           ░░███████░░░░░░░░░░░
Phase 2 (Dart types)       ░░░░░░░░████░░░░░░░░
Phase 3 (TS types)         ░░░░░░░░████░░░░░░░░  ← parallel with Phase 2
Phase 4 (codec)            ░░░░░░░░░░░░█████░░░
Phase 5 (tests)            ░░░░░░░░░░░░░░░░████
```

Phases 2 and 3 can be done in parallel since they read the same IR and output to different directories.

---

## File Inventory: What Gets Generated

### Dart (`packages/chat_core_dart/lib/src/`)

| File                               | Source                                |
| ---------------------------------- | ------------------------------------- |
| `types/chat_kind.dart`             | `ChatKind` enum                       |
| `types/chat_role.dart`             | `ChatRole` enum                       |
| `types/message_kind.dart`          | `MessageKind` enum                    |
| `types/presence_status.dart`       | `PresenceStatus` enum                 |
| `types/load_direction.dart`        | `LoadDirection` enum                  |
| `types/frame_kind.dart`            | `FrameKind` enum                      |
| `types/error_code.dart`            | `ErrorCode` enum                      |
| `types/disconnect_code.dart`       | `DisconnectCode` enum                 |
| `types/permission.dart`            | `Permission` bitflags                 |
| `types/user_flags.dart`            | `UserFlags` bitflags                  |
| `types/message_flags.dart`         | `MessageFlags` bitflags               |
| `types/rich_style.dart`            | `RichStyle` bitflags                  |
| `types/server_capabilities.dart`   | `ServerCapabilities` bitflags         |
| `types/chat_entry.dart`            | `ChatEntry` + `LastMessagePreview`    |
| `types/chat_member_entry.dart`     | `ChatMemberEntry`                     |
| `types/user_entry.dart`            | `UserEntry`                           |
| `types/presence_entry.dart`        | `PresenceEntry`                       |
| `types/message.dart`               | `Message`, `MessageBatch`, `RichSpan` |
| `types/error_payload.dart`         | `ErrorPayload`                        |
| `types/frame_header.dart`          | `FrameHeader`                         |
| `types/server_limits.dart`         | `ServerLimits`                        |
| `types/*_payload.dart`             | ~25 payload struct files              |
| `types/load_chats_payload.dart`    | sealed class                          |
| `types/load_messages_payload.dart` | sealed class                          |
| `types/search_scope.dart`          | sealed class                          |
| `types/member_action.dart`         | sealed class                          |
| `types/ack_payload.dart`           | sealed class (types only)             |
| `codec/reader.dart`                | Hand-written reader                   |
| `codec/writer.dart`                | Hand-written writer                   |
| `codec/codec.dart`                 | Generated encode/decode functions     |
| `protocol_constants.dart`          | Constants                             |
| **Barrel:** `lib/chat_core.dart`   | All exports                           |

### TypeScript (`packages/chat_core_ts/src/`)

Same structure with `.ts` extension, kebab-case filenames.

---

## Markers & Conventions

Every generated file starts with:
```
// GENERATED CODE — DO NOT EDIT
// Source: chat_protocol (crates/chat_protocol/src/types/)
// Generator: cargo xtask codegen
```

Dart files additionally have `// ignore_for_file: ...` for relevant lints on generated code.
