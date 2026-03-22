/// TypeScript code emitter — generates TS package from IR.

use std::collections::BTreeSet;
use std::fmt::Write;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::codegen::ir::*;

const HEADER: &str = "// GENERATED CODE — DO NOT MODIFY BY HAND\n// Source: chat_protocol\n";

/// ErrorCode variants that are permanent (do not retry).
const PERMANENT_ERRORS: &[&str] = &[
    "Forbidden",
    "ChatNotFound",
    "NotChatMember",
    "MessageTooLarge",
    "ExtraTooLarge",
    "ContentFiltered",
    "UnsupportedMediaType",
];

/// ErrorCode variants that are transient (retry with backoff).
const TRANSIENT_ERRORS: &[&str] = &[
    "InternalError",
    "ServiceUnavailable",
    "DatabaseError",
    "RateLimited",
];

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Generate the TypeScript package from parsed IR.
pub(crate) fn generate(ir: &ParsedModule, output_dir: &Path) -> Result<()> {
    let src_dir = output_dir.join("src");
    let types_dir = src_dir.join("types");

    fs::create_dir_all(&types_dir).context("creating types dir")?;
    fs::create_dir_all(src_dir.join("codec")).context("creating codec dir")?;
    fs::create_dir_all(output_dir.join("tests")).context("creating tests dir")?;

    let mut exports: Vec<String> = Vec::new();

    // package.json + tsconfig.json
    write_file(&output_dir.join("package.json"), &emit_package_json())?;
    write_file(&output_dir.join("tsconfig.json"), &emit_tsconfig())?;

    // Enums
    for e in &ir.enums {
        let fname = format!("{}.ts", to_kebab_case(&e.name));
        write_file(&types_dir.join(&fname), &emit_enum(e))?;
        exports.push(format!("./types/{fname}"));
    }

    // Bitflags
    for b in &ir.bitflags {
        let fname = format!("{}.ts", to_kebab_case(&b.name));
        write_file(&types_dir.join(&fname), &emit_bitflags(b))?;
        exports.push(format!("./types/{fname}"));
    }

    // Structs
    for s in &ir.structs {
        let fname = format!("{}.ts", to_kebab_case(&s.name));
        write_file(&types_dir.join(&fname), &emit_struct(s))?;
        exports.push(format!("./types/{fname}"));
    }

    // Tagged enums
    for t in &ir.tagged_enums {
        let fname = format!("{}.ts", to_kebab_case(&t.name));
        write_file(&types_dir.join(&fname), &emit_tagged_enum(t))?;
        exports.push(format!("./types/{fname}"));
    }

    // Constants
    if !ir.constants.is_empty() {
        write_file(&src_dir.join("constants.ts"), &emit_constants(&ir.constants))?;
        exports.push("./constants.ts".into());
    }

    // Codec files (Phase 4)
    let codec_dir = src_dir.join("codec");
    write_file(&codec_dir.join("error.ts"), &emit_codec_error_ts())?;
    write_file(&codec_dir.join("reader.ts"), &emit_reader_ts())?;
    write_file(&codec_dir.join("writer.ts"), &emit_writer_ts())?;
    write_file(&codec_dir.join("codecs.ts"), &emit_codecs_ts(ir))?;
    write_file(&codec_dir.join("frame.ts"), &emit_frame_codec_ts(ir))?;

    exports.push("./codec/error.ts".into());
    exports.push("./codec/reader.ts".into());
    exports.push("./codec/writer.ts".into());
    exports.push("./codec/codecs.ts".into());
    exports.push("./codec/frame.ts".into());

    // Barrel export
    exports.sort();
    write_file(&src_dir.join("index.ts"), &emit_barrel(&exports))?;

    eprintln!(
        "TypeScript: {} enums, {} bitflags, {} structs, {} tagged enums, {} constants + codec",
        ir.enums.len(),
        ir.bitflags.len(),
        ir.structs.len(),
        ir.tagged_enums.len(),
        ir.constants.len(),
    );

    Ok(())
}

fn write_file(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content).with_context(|| format!("writing {}", path.display()))
}

// ---------------------------------------------------------------------------
// Name conversion
// ---------------------------------------------------------------------------

/// `PascalCase` → `kebab-case`.
fn to_kebab_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && s.as_bytes()[i - 1].is_ascii_lowercase() {
                out.push('-');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// `PascalCase` → `snake_case`.
fn to_snake_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && s.as_bytes()[i - 1].is_ascii_lowercase() {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// `snake_case` / `SCREAMING_SNAKE` → `camelCase`.
fn to_camel_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut cap_next = false;
    for (i, c) in s.chars().enumerate() {
        if c == '_' {
            cap_next = true;
        } else if cap_next {
            out.push(c.to_ascii_uppercase());
            cap_next = false;
        } else if i == 0 {
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c.to_ascii_lowercase());
        }
    }
    out
}

/// `PascalCase` → `camelCase` (lowercase first letter only).
fn to_lower_camel(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => {
            let mut out = c.to_lowercase().to_string();
            out.push_str(chars.as_str());
            out
        }
    }
}

/// Strip Rust integer type suffixes for TS compatibility.
fn clean_rust_expr(expr: &str) -> String {
    let mut s = expr.to_string();
    for suffix in &[
        "_i128", "_u128", "_isize", "_usize", "_i64", "_u64", "_i32", "_u32", "_i16", "_u16",
        "_i8", "_u8",
    ] {
        s = s.replace(suffix, "");
    }
    s
}

// ---------------------------------------------------------------------------
// Type mapping
// ---------------------------------------------------------------------------

/// Map `FieldType` to TypeScript type string.
fn ts_type(ty: &FieldType) -> String {
    match ty {
        FieldType::U8 | FieldType::U16 | FieldType::U32 | FieldType::I64 => "number".into(),
        FieldType::Bool => "boolean".into(),
        FieldType::String => "string".into(),
        FieldType::OptionalString | FieldType::UpdatableString => "string | null".into(),
        FieldType::Uuid => "string".into(),
        FieldType::OptionalU32 => "number | null".into(),
        FieldType::VecU32 => "readonly number[]".into(),
        FieldType::VecU8 => "Uint8Array".into(),
        FieldType::OptionalBytes => "Uint8Array | null".into(),
        FieldType::VecString => "readonly string[]".into(),
        FieldType::Enum(n) | FieldType::Bitflags(n) | FieldType::Struct(n) => n.clone(),
        FieldType::TaggedEnum(n) => n.clone(),
        FieldType::OptionalStruct(n) | FieldType::OptionalBitflags(n) => format!("{n} | null"),
        FieldType::OptionalVecStruct(n) => format!("readonly {n}[] | null"),
        FieldType::VecStruct(n) => format!("readonly {n}[]"),
    }
}

/// Collect named type references from a field type.
fn collect_field_refs(ty: &FieldType, refs: &mut BTreeSet<String>) {
    match ty {
        FieldType::Enum(n)
        | FieldType::Bitflags(n)
        | FieldType::Struct(n)
        | FieldType::OptionalStruct(n)
        | FieldType::OptionalBitflags(n)
        | FieldType::VecStruct(n)
        | FieldType::OptionalVecStruct(n)
        | FieldType::TaggedEnum(n) => {
            refs.insert(n.clone());
        }
        _ => {}
    }
}


// ---------------------------------------------------------------------------
// Doc & import helpers
// ---------------------------------------------------------------------------

fn write_doc(out: &mut String, doc: &str, indent: &str) {
    if doc.is_empty() {
        return;
    }
    let lines: Vec<&str> = doc.lines().collect();
    if lines.len() == 1 {
        writeln!(out, "{indent}/** {doc} */").unwrap();
    } else {
        writeln!(out, "{indent}/**").unwrap();
        for line in &lines {
            if line.is_empty() {
                writeln!(out, "{indent} *").unwrap();
            } else {
                writeln!(out, "{indent} * {line}").unwrap();
            }
        }
        writeln!(out, "{indent} */").unwrap();
    }
}


// ---------------------------------------------------------------------------
// Emitters
// ---------------------------------------------------------------------------

fn emit_package_json() -> String {
    r#"{
  "name": "chat-core",
  "version": "0.1.0",
  "description": "Chat protocol types and binary codec.",
  "type": "module",
  "main": "./dist/index.js",
  "types": "./dist/index.d.ts",
  "exports": {
    ".": {
      "types": "./dist/index.d.ts",
      "import": "./dist/index.js"
    }
  },
  "scripts": {
    "build": "tsc",
    "check": "tsc --noEmit"
  },
  "devDependencies": {
    "typescript": "~5.7.0"
  }
}
"#
    .into()
}

fn emit_tsconfig() -> String {
    r#"{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true,
    "outDir": "dist",
    "rootDir": "src",
    "strict": true,
    "noUncheckedIndexedAccess": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "isolatedModules": true,
    "skipLibCheck": true
  },
  "include": ["src"]
}
"#
    .into()
}

fn emit_enum(e: &EnumDef) -> String {
    let mut out = String::from(HEADER);
    out.push('\n');

    // const enum
    write_doc(&mut out, &e.doc, "");
    writeln!(out, "export const enum {} {{", e.name).unwrap();
    for v in &e.variants {
        write_doc(&mut out, &v.doc, "  ");
        writeln!(out, "  {} = {},", v.name, v.discriminant).unwrap();
    }
    out.push_str("}\n");

    // fromValue function
    let fn_name = format!("{}FromValue", to_lower_camel(&e.name));
    let max_disc = e.variants.iter().map(|v| v.discriminant).max().unwrap_or(0);
    let min_disc = e.variants.iter().map(|v| v.discriminant).min().unwrap_or(0);

    // Check if discriminants are contiguous
    let is_contiguous = e.variants.len() as u64 == (max_disc - min_disc + 1)
        && e.variants
            .windows(2)
            .all(|w| w[1].discriminant == w[0].discriminant + 1);

    out.push('\n');
    writeln!(
        out,
        "/** Convert wire value to {}, or undefined if unknown. */",
        e.name
    )
    .unwrap();
    writeln!(
        out,
        "export function {fn_name}(value: number): {} | undefined {{",
        e.name
    )
    .unwrap();

    if is_contiguous {
        writeln!(
            out,
            "  if (value >= {min_disc} && value <= {max_disc}) return value as {};",
            e.name
        )
        .unwrap();
        out.push_str("  return undefined;\n");
    } else {
        out.push_str("  switch (value) {\n");
        for v in &e.variants {
            writeln!(
                out,
                "    case {}: return {}.{};",
                v.discriminant, e.name, v.name
            )
            .unwrap();
        }
        out.push_str("    default: return undefined;\n");
        out.push_str("  }\n");
    }
    out.push_str("}\n");

    // ErrorCode extras
    if e.name == "ErrorCode" {
        emit_error_code_extras(&mut out, e);
    }

    // DisconnectCode extras
    if e.name == "DisconnectCode" {
        emit_disconnect_code_extras(&mut out);
    }

    out
}

fn emit_error_code_extras(out: &mut String, e: &EnumDef) {
    // slug function
    out.push('\n');
    out.push_str("/** Stable snake_case identifier for client matching. */\n");
    out.push_str("export function errorCodeSlug(code: ErrorCode): string {\n");
    out.push_str("  switch (code) {\n");
    for v in &e.variants {
        let slug = to_snake_case(&v.name);
        writeln!(out, "    case ErrorCode.{}: return '{slug}';", v.name).unwrap();
    }
    out.push_str("  }\n");
    out.push_str("}\n");

    // isPermanent
    out.push('\n');
    out.push_str("/** Whether this error is permanent (do not retry). */\n");
    out.push_str("export function isErrorPermanent(code: ErrorCode): boolean {\n");
    out.push_str("  switch (code) {\n");
    for name in PERMANENT_ERRORS {
        writeln!(out, "    case ErrorCode.{name}:").unwrap();
    }
    out.push_str("      return true;\n");
    out.push_str("    default:\n");
    out.push_str("      return false;\n");
    out.push_str("  }\n");
    out.push_str("}\n");

    // isTransient
    out.push('\n');
    out.push_str("/** Whether this error is transient (retry with backoff). */\n");
    out.push_str("export function isErrorTransient(code: ErrorCode): boolean {\n");
    out.push_str("  switch (code) {\n");
    for name in TRANSIENT_ERRORS {
        writeln!(out, "    case ErrorCode.{name}:").unwrap();
    }
    out.push_str("      return true;\n");
    out.push_str("    default:\n");
    out.push_str("      return false;\n");
    out.push_str("  }\n");
    out.push_str("}\n");
}

fn emit_disconnect_code_extras(out: &mut String) {
    out.push('\n');
    out.push_str("/** Whether the client should attempt reconnection. */\n");
    out.push_str("export function shouldReconnect(code: DisconnectCode): boolean {\n");
    out.push_str("  const v = code as number;\n");
    out.push_str(
        "  return (v >= 0 && v < 1000) || (v >= 3000 && v < 3500) || (v >= 4000 && v < 4500);\n",
    );
    out.push_str("}\n");
}

fn emit_bitflags(b: &BitflagsDef) -> String {
    let mut out = String::from(HEADER);
    out.push('\n');

    let n = &b.name;

    write_doc(&mut out, &b.doc, "");
    writeln!(out, "export namespace {n} {{").unwrap();

    // Flag constants
    for f in &b.flags {
        write_doc(&mut out, &f.doc, "  ");
        let expr = clean_rust_expr(&f.value_expr);
        writeln!(out, "  export const {} = {};", f.name, expr).unwrap();
    }

    // Utility functions
    out.push('\n');
    out.push_str("  export function contains(flags: number, flag: number): boolean {\n");
    out.push_str("    return (flags & flag) !== 0;\n");
    out.push_str("  }\n");
    out.push_str("  export function add(flags: number, flag: number): number {\n");
    out.push_str("    return flags | flag;\n");
    out.push_str("  }\n");
    out.push_str("  export function remove(flags: number, flag: number): number {\n");
    out.push_str("    return flags & ~flag;\n");
    out.push_str("  }\n");
    out.push_str("  export function toggle(flags: number, flag: number): number {\n");
    out.push_str("    return flags ^ flag;\n");
    out.push_str("  }\n");

    out.push_str("}\n");

    // Type alias
    writeln!(out, "export type {n} = number;").unwrap();

    out
}

fn emit_struct(s: &StructDef) -> String {
    let mut out = String::from(HEADER);

    // Collect type refs for imports
    let mut type_refs = BTreeSet::new();
    for f in &s.fields {
        collect_field_refs(&f.ty, &mut type_refs);
    }

    // We don't know the full IR here, but for struct imports we can use
    // type-only imports for everything — the struct file itself won't
    // reference enum values at runtime (just types in the interface).
    if !type_refs.is_empty() {
        out.push('\n');
        for name in &type_refs {
            let file = to_kebab_case(name);
            writeln!(out, "import type {{ {name} }} from './{file}.js';").unwrap();
        }
    }

    out.push('\n');
    write_doc(&mut out, &s.doc, "");
    writeln!(out, "export interface {} {{", s.name).unwrap();

    for f in &s.fields {
        write_doc(&mut out, &f.doc, "  ");
        let field_name = to_camel_case(&f.name);
        let field_type = ts_type(&f.ty);
        writeln!(out, "  readonly {field_name}: {field_type};").unwrap();
    }

    out.push_str("}\n");
    out
}

fn emit_tagged_enum(t: &TaggedEnumDef) -> String {
    let mut out = String::from(HEADER);

    // Collect type refs from all variant fields
    let mut type_refs = BTreeSet::new();
    for v in &t.variants {
        match &v.kind {
            VariantKind::Unit => {}
            VariantKind::Tuple(types) => {
                for ty in types {
                    collect_field_refs(ty, &mut type_refs);
                }
            }
            VariantKind::Struct(fields) => {
                for f in fields {
                    collect_field_refs(&f.ty, &mut type_refs);
                }
            }
        }
    }

    if !type_refs.is_empty() {
        out.push('\n');
        for name in &type_refs {
            let file = to_kebab_case(name);
            writeln!(out, "import type {{ {name} }} from './{file}.js';").unwrap();
        }
    }

    out.push('\n');
    write_doc(&mut out, &t.doc, "");
    writeln!(out, "export type {} =", t.name).unwrap();

    for (i, v) in t.variants.iter().enumerate() {
        let tag = to_lower_camel(&v.name);

        match &v.kind {
            VariantKind::Unit => {
                write!(out, "  | {{ readonly type: '{tag}' }}").unwrap();
            }
            VariantKind::Tuple(types) => {
                write!(out, "  | {{ readonly type: '{tag}'").unwrap();
                for (j, ty) in types.iter().enumerate() {
                    let field_name = if types.len() == 1 {
                        tuple_field_name(ty)
                    } else {
                        format!("value{}", j + 1)
                    };
                    write!(out, "; readonly {field_name}: {}", ts_type(ty)).unwrap();
                }
                write!(out, " }}").unwrap();
            }
            VariantKind::Struct(fields) => {
                write!(out, "  | {{ readonly type: '{tag}'").unwrap();
                for f in fields {
                    let field_name = to_camel_case(&f.name);
                    write!(out, "; readonly {field_name}: {}", ts_type(&f.ty)).unwrap();
                }
                write!(out, " }}").unwrap();
            }
        }

        if i < t.variants.len() - 1 {
            out.push('\n');
        } else {
            out.push_str(";\n");
        }
    }

    out
}

/// Derive a field name from a tuple field type.
fn tuple_field_name(ty: &FieldType) -> String {
    match ty {
        FieldType::Enum(n) | FieldType::Bitflags(n) | FieldType::Struct(n) | FieldType::TaggedEnum(n) => {
            to_lower_camel(n)
        }
        _ => "value".to_string(),
    }
}

fn emit_constants(constants: &[ConstDef]) -> String {
    let mut out = String::from(HEADER);
    out.push('\n');

    for (i, c) in constants.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        write_doc(&mut out, &c.doc, "");
        let ts_name = to_camel_case(&c.name);
        let value = clean_rust_expr(&c.value_expr);
        writeln!(out, "export const {ts_name} = {value};").unwrap();
    }

    out
}

fn emit_barrel(exports: &[String]) -> String {
    let mut out = String::from(HEADER);
    out.push('\n');

    for export in exports {
        // Strip the leading "./" and ".ts" to get the path relative to src/
        let path = export.strip_prefix("./").unwrap_or(export);
        let path = path.strip_suffix(".ts").unwrap_or(path);
        writeln!(out, "export * from './{path}.js';").unwrap();
    }

    out
}

// ===========================================================================
// Phase 4: Binary Codec Generation
// ===========================================================================

// ---------------------------------------------------------------------------
// Frame dispatch table
// ---------------------------------------------------------------------------

/// How a FrameKind maps to its payload codec.
///
/// 0 = no payload, 1 = struct, 2 = tagged enum, 3 = vec struct (u16 count),
/// 4 = ack (raw bytes).
const FRAME_DISPATCH: &[(&str, u8, &str)] = &[
    ("Hello", 1, "HelloPayload"),
    ("Welcome", 1, "WelcomePayload"),
    ("Ping", 0, ""),
    ("Pong", 0, ""),
    ("RefreshToken", 1, "RefreshTokenPayload"),
    ("SendMessage", 1, "SendMessagePayload"),
    ("EditMessage", 1, "EditMessagePayload"),
    ("DeleteMessage", 1, "DeleteMessagePayload"),
    ("ReadReceipt", 1, "ReadReceiptPayload"),
    ("Typing", 1, "TypingPayload"),
    ("GetPresence", 1, "GetPresencePayload"),
    ("LoadChats", 2, "LoadChatsPayload"),
    ("Search", 1, "SearchPayload"),
    ("Subscribe", 1, "SubscribePayload"),
    ("Unsubscribe", 1, "UnsubscribePayload"),
    ("LoadMessages", 2, "LoadMessagesPayload"),
    ("AddReaction", 1, "AddReactionPayload"),
    ("RemoveReaction", 1, "RemoveReactionPayload"),
    ("PinMessage", 1, "PinMessagePayload"),
    ("UnpinMessage", 1, "UnpinMessagePayload"),
    ("ForwardMessage", 1, "ForwardMessagePayload"),
    ("MessageNew", 1, "Message"),
    ("MessageEdited", 1, "Message"),
    ("MessageDeleted", 1, "MessageDeletedPayload"),
    ("ReceiptUpdate", 1, "ReceiptUpdatePayload"),
    ("TypingUpdate", 1, "TypingUpdatePayload"),
    ("MemberJoined", 1, "MemberJoinedPayload"),
    ("MemberLeft", 1, "MemberLeftPayload"),
    ("PresenceResult", 3, "PresenceEntry"),
    ("ChatUpdated", 1, "ChatEntry"),
    ("ChatCreated", 1, "ChatEntry"),
    ("ReactionUpdate", 1, "ReactionUpdatePayload"),
    ("UserUpdated", 1, "UserEntry"),
    ("ChatDeleted", 1, "ChatDeletedPayload"),
    ("MemberUpdated", 1, "MemberUpdatedPayload"),
    ("Ack", 4, ""),
    ("Error", 1, "ErrorPayload"),
    ("CreateChat", 1, "CreateChatPayload"),
    ("UpdateChat", 1, "UpdateChatPayload"),
    ("DeleteChat", 1, "DeleteChatPayload"),
    ("GetChatInfo", 1, "GetChatInfoPayload"),
    ("GetChatMembers", 1, "GetChatMembersPayload"),
    ("InviteMembers", 1, "InviteMembersPayload"),
    ("UpdateMember", 1, "UpdateMemberPayload"),
    ("LeaveChat", 1, "LeaveChatPayload"),
    ("MuteChat", 1, "MuteChatPayload"),
    ("UnmuteChat", 1, "UnmuteChatPayload"),
    ("GetUser", 1, "GetUserPayload"),
    ("GetUsers", 1, "GetUsersPayload"),
    ("UpdateProfile", 1, "UpdateProfilePayload"),
    ("BlockUser", 1, "BlockUserPayload"),
    ("UnblockUser", 1, "UnblockUserPayload"),
    ("GetBlockList", 1, "GetBlockListPayload"),
];

/// Structs with hand-coded codec (not auto-generated from fields).
const SPECIAL_STRUCTS: &[&str] = &["ErrorPayload", "Message", "MessageBatch"];

/// Tagged enums to skip codec generation for.
const SKIP_TAGGED_ENUMS: &[&str] = &["AckPayload", "FramePayload"];

// ---------------------------------------------------------------------------
// Repr helpers
// ---------------------------------------------------------------------------

fn repr_write(repr: ReprType) -> &'static str {
    match repr {
        ReprType::U8 => "writeU8",
        ReprType::U16 => "writeU16",
        ReprType::U32 => "writeU32",
    }
}

fn repr_read(repr: ReprType) -> &'static str {
    match repr {
        ReprType::U8 => "readU8",
        ReprType::U16 => "readU16",
        ReprType::U32 => "readU32",
    }
}

fn find_enum_repr(ir: &ParsedModule, name: &str) -> ReprType {
    ir.enums
        .iter()
        .find(|e| e.name == name)
        .map(|e| e.repr)
        .unwrap_or(ReprType::U8)
}

fn find_bitflags_repr(ir: &ParsedModule, name: &str) -> ReprType {
    ir.bitflags
        .iter()
        .find(|b| b.name == name)
        .map(|b| b.repr)
        .unwrap_or(ReprType::U32)
}

// ---------------------------------------------------------------------------
// Field-level encode/decode
// ---------------------------------------------------------------------------

fn emit_encode_field(
    out: &mut String,
    accessor: &str,
    ty: &FieldType,
    ir: &ParsedModule,
    indent: &str,
) {
    match ty {
        FieldType::U8 => writeln!(out, "{indent}w.writeU8({accessor});").unwrap(),
        FieldType::U16 => writeln!(out, "{indent}w.writeU16({accessor});").unwrap(),
        FieldType::U32 => writeln!(out, "{indent}w.writeU32({accessor});").unwrap(),
        FieldType::I64 => writeln!(out, "{indent}w.writeTimestamp({accessor});").unwrap(),
        FieldType::Bool => {
            writeln!(out, "{indent}w.writeU8({accessor} ? 1 : 0);").unwrap();
        }
        FieldType::String => writeln!(out, "{indent}w.writeString({accessor});").unwrap(),
        FieldType::OptionalString => {
            writeln!(out, "{indent}w.writeOptionalString({accessor});").unwrap();
        }
        FieldType::UpdatableString => {
            writeln!(out, "{indent}w.writeUpdatableString({accessor});").unwrap();
        }
        FieldType::Uuid => writeln!(out, "{indent}w.writeUuid({accessor});").unwrap(),
        FieldType::OptionalU32 => {
            writeln!(out, "{indent}w.writeOptionU32({accessor});").unwrap();
        }
        FieldType::VecU32 => {
            writeln!(out, "{indent}w.writeU16({accessor}.length);").unwrap();
            writeln!(out, "{indent}for (const _v of {accessor}) w.writeU32(_v);").unwrap();
        }
        FieldType::VecU8 => {
            writeln!(out, "{indent}w.writeU32({accessor}.length);").unwrap();
            writeln!(out, "{indent}w.writeRawBytes({accessor});").unwrap();
        }
        FieldType::OptionalBytes => {
            writeln!(out, "{indent}w.writeOptionalBytes({accessor});").unwrap();
        }
        FieldType::VecString => {
            writeln!(out, "{indent}w.writeU16({accessor}.length);").unwrap();
            writeln!(out, "{indent}for (const _v of {accessor}) w.writeString(_v);").unwrap();
        }
        FieldType::Enum(name) => {
            let wfn = repr_write(find_enum_repr(ir, name));
            writeln!(out, "{indent}w.{wfn}({accessor});").unwrap();
        }
        FieldType::Bitflags(name) => {
            let wfn = repr_write(find_bitflags_repr(ir, name));
            writeln!(out, "{indent}w.{wfn}({accessor});").unwrap();
        }
        FieldType::Struct(name) => {
            writeln!(out, "{indent}encode{name}(w, {accessor});").unwrap();
        }
        FieldType::OptionalStruct(name) => {
            writeln!(out, "{indent}if ({accessor} !== null) {{ w.writeU8(1); encode{name}(w, {accessor}); }} else {{ w.writeU8(0); }}").unwrap();
        }
        FieldType::OptionalBitflags(name) => {
            let wfn = repr_write(find_bitflags_repr(ir, name));
            writeln!(out, "{indent}if ({accessor} !== null) {{ w.writeU8(1); w.{wfn}({accessor}); }} else {{ w.writeU8(0); }}").unwrap();
        }
        FieldType::OptionalVecStruct(name) => {
            writeln!(out, "{indent}if ({accessor} !== null) {{ w.writeU8(1); w.writeU32({accessor}.length); for (const _v of {accessor}) encode{name}(w, _v); }} else {{ w.writeU8(0); }}").unwrap();
        }
        FieldType::VecStruct(name) => {
            writeln!(out, "{indent}w.writeU32({accessor}.length);").unwrap();
            writeln!(out, "{indent}for (const _v of {accessor}) encode{name}(w, _v);").unwrap();
        }
        FieldType::TaggedEnum(name) => {
            writeln!(out, "{indent}encode{name}(w, {accessor});").unwrap();
        }
    }
}

fn field_decode_expr(ty: &FieldType, ir: &ParsedModule) -> String {
    match ty {
        FieldType::U8 => "r.readU8()".into(),
        FieldType::U16 => "r.readU16()".into(),
        FieldType::U32 => "r.readU32()".into(),
        FieldType::I64 => "r.readTimestamp()".into(),
        FieldType::Bool => "r.readU8() !== 0".into(),
        FieldType::String => "r.readString()".into(),
        FieldType::OptionalString => "r.readOptionalString()".into(),
        FieldType::UpdatableString => "r.readUpdatableString()".into(),
        FieldType::Uuid => "r.readUuid()".into(),
        FieldType::OptionalU32 => "r.readOptionU32()".into(),
        FieldType::VecU32 => "r.readVecU32()".into(),
        FieldType::VecU8 => "r.readBytes(r.readU32())".into(),
        FieldType::OptionalBytes => "r.readOptionalBytes()".into(),
        FieldType::VecString => "r.readVecString()".into(),
        FieldType::Enum(name) => {
            let rfn = repr_read(find_enum_repr(ir, name));
            let from_fn = format!("{}FromValue", to_lower_camel(name));
            format!("r.readEnum(r.{rfn}(), {from_fn}, '{name}')")
        }
        FieldType::Bitflags(name) => {
            let rfn = repr_read(find_bitflags_repr(ir, name));
            format!("r.{rfn}()")
        }
        FieldType::Struct(name) => format!("decode{name}(r)"),
        FieldType::OptionalStruct(name) => {
            format!("r.readU8() === 1 ? decode{name}(r) : null")
        }
        FieldType::OptionalBitflags(name) => {
            let rfn = repr_read(find_bitflags_repr(ir, name));
            format!("r.readU8() === 1 ? r.{rfn}() : null")
        }
        FieldType::OptionalVecStruct(name) => {
            format!(
                "r.readU8() === 1 ? r.readArray(r.readU32(), () => decode{name}(r)) : null"
            )
        }
        FieldType::VecStruct(name) => {
            format!("r.readArray(r.readU32(), () => decode{name}(r))")
        }
        FieldType::TaggedEnum(name) => format!("decode{name}(r)"),
    }
}

// ---------------------------------------------------------------------------
// Struct codec generation
// ---------------------------------------------------------------------------

fn emit_struct_codec(out: &mut String, s: &StructDef, ir: &ParsedModule) {
    let name = &s.name;
    writeln!(out, "export function encode{name}(w: ProtocolWriter, v: {name}): void {{").unwrap();
    for f in &s.fields {
        let accessor = format!("v.{}", to_camel_case(&f.name));
        emit_encode_field(out, &accessor, &f.ty, ir, "  ");
    }
    out.push_str("}\n\n");

    writeln!(out, "export function decode{name}(r: ProtocolReader): {name} {{").unwrap();
    out.push_str("  return {\n");
    for f in &s.fields {
        let ts_name = to_camel_case(&f.name);
        let expr = field_decode_expr(&f.ty, ir);
        writeln!(out, "    {ts_name}: {expr},").unwrap();
    }
    out.push_str("  };\n}\n");
}

fn emit_error_payload_codec(out: &mut String) {
    out.push_str(
        "export function encodeErrorPayload(w: ProtocolWriter, v: ErrorPayload): void {\n\
         \x20 w.writeU16(v.code);\n\
         \x20 const slug = errorCodeSlug(v.code);\n\
         \x20 w.writeU8(slug.length);\n\
         \x20 for (let i = 0; i < slug.length; i++) w.writeU8(slug.charCodeAt(i));\n\
         \x20 w.writeString(v.message);\n\
         \x20 w.writeU32(v.retryAfterMs);\n\
         \x20 w.writeOptionalString(v.extra);\n\
         }\n\n\
         export function decodeErrorPayload(r: ProtocolReader): ErrorPayload {\n\
         \x20 const codeRaw = r.readU16();\n\
         \x20 const code = r.readEnum(codeRaw, errorCodeFromValue, 'ErrorCode');\n\
         \x20 r.skip(r.readU8());\n\
         \x20 return {\n\
         \x20   code,\n\
         \x20   message: r.readString(),\n\
         \x20   retryAfterMs: r.readU32(),\n\
         \x20   extra: r.readOptionalString(),\n\
         \x20 };\n\
         }\n",
    );
}

fn emit_message_codec(out: &mut String) {
    out.push_str(
        "export function encodeMessage(w: ProtocolWriter, v: Message): void {\n\
         \x20 w.writeU32(v.id);\n\
         \x20 w.writeU32(v.chatId);\n\
         \x20 w.writeU32(v.senderId);\n\
         \x20 w.writeTimestamp(v.createdAt);\n\
         \x20 w.writeTimestamp(v.updatedAt);\n\
         \x20 w.writeU8(v.kind);\n\
         \x20 w.writeU16(v.flags);\n\
         \x20 w.writeOptionU32(v.replyToId);\n\
         \x20 w.writeString(v.content);\n\
         \x20 if (v.richContent !== null) {\n\
         \x20   const tmp = new ProtocolWriter();\n\
         \x20   tmp.writeU16(v.richContent.length);\n\
         \x20   for (const span of v.richContent) encodeRichSpan(tmp, span);\n\
         \x20   const blob = tmp.toBytes();\n\
         \x20   w.writeU32(blob.length);\n\
         \x20   w.writeRawBytes(blob);\n\
         \x20 } else {\n\
         \x20   w.writeU32(0);\n\
         \x20 }\n\
         \x20 w.writeOptionalString(v.extra);\n\
         }\n\n\
         export function decodeMessage(r: ProtocolReader): Message {\n\
         \x20 const id = r.readU32();\n\
         \x20 const chatId = r.readU32();\n\
         \x20 const senderId = r.readU32();\n\
         \x20 const createdAt = r.readTimestamp();\n\
         \x20 const updatedAt = r.readTimestamp();\n\
         \x20 const kind = r.readEnum(r.readU8(), messageKindFromValue, 'MessageKind');\n\
         \x20 const flags = r.readU16();\n\
         \x20 const replyToId = r.readOptionU32();\n\
         \x20 const content = r.readString();\n\
         \x20 const richLen = r.readU32();\n\
         \x20 let richContent: RichSpan[] | null = null;\n\
         \x20 if (richLen > 0) {\n\
         \x20   const richData = r.readBytes(richLen);\n\
         \x20   const rr = new ProtocolReader(richData);\n\
         \x20   richContent = rr.readArray(rr.readU16(), () => decodeRichSpan(rr));\n\
         \x20 }\n\
         \x20 const extra = r.readOptionalString();\n\
         \x20 return { id, chatId, senderId, createdAt, updatedAt, kind, flags, replyToId, content, richContent, extra };\n\
         }\n",
    );
}

fn emit_message_batch_codec(out: &mut String) {
    out.push_str(
        "export function encodeMessageBatch(w: ProtocolWriter, v: MessageBatch): void {\n\
         \x20 w.writeU8(v.hasMore ? 1 : 0);\n\
         \x20 w.writeU32(v.messages.length);\n\
         \x20 for (const msg of v.messages) encodeMessage(w, msg);\n\
         }\n\n\
         export function decodeMessageBatch(r: ProtocolReader): MessageBatch {\n\
         \x20 const hasMore = r.readU8() !== 0;\n\
         \x20 const messages = r.readArray(r.readU32(), () => decodeMessage(r));\n\
         \x20 return { messages, hasMore };\n\
         }\n",
    );
}

// ---------------------------------------------------------------------------
// Tagged enum codec generation
// ---------------------------------------------------------------------------

fn emit_tagged_enum_codec(out: &mut String, t: &TaggedEnumDef, ir: &ParsedModule) {
    let name = &t.name;

    if name == "LoadMessagesPayload" {
        emit_load_messages_codec(out, t, ir);
        return;
    }

    // --- encode ---
    writeln!(out, "export function encode{name}(w: ProtocolWriter, v: {name}): void {{").unwrap();
    out.push_str("  switch (v.type) {\n");
    for (i, v) in t.variants.iter().enumerate() {
        let tag = to_lower_camel(&v.name);
        writeln!(out, "    case '{tag}':").unwrap();
        writeln!(out, "      w.writeU8({i});").unwrap();
        match &v.kind {
            VariantKind::Unit => {}
            VariantKind::Tuple(types) => {
                for (j, ty) in types.iter().enumerate() {
                    let field = if types.len() == 1 {
                        format!("v.{}", tuple_field_name(ty))
                    } else {
                        format!("v.value{}", j + 1)
                    };
                    emit_encode_field(out, &field, ty, ir, "      ");
                }
            }
            VariantKind::Struct(fields) => {
                for f in fields {
                    let accessor = format!("v.{}", to_camel_case(&f.name));
                    emit_encode_field(out, &accessor, &f.ty, ir, "      ");
                }
            }
        }
        out.push_str("      break;\n");
    }
    out.push_str("  }\n}\n\n");

    // --- decode ---
    writeln!(out, "export function decode{name}(r: ProtocolReader): {name} {{").unwrap();
    out.push_str("  const _d = r.readU8();\n  switch (_d) {\n");
    for (i, v) in t.variants.iter().enumerate() {
        let tag = to_lower_camel(&v.name);
        writeln!(out, "    case {i}:").unwrap();
        match &v.kind {
            VariantKind::Unit => {
                writeln!(out, "      return {{ type: '{tag}' }};").unwrap();
            }
            VariantKind::Tuple(types) => {
                let fields: Vec<String> = types
                    .iter()
                    .enumerate()
                    .map(|(j, ty)| {
                        let fname = if types.len() == 1 {
                            tuple_field_name(ty)
                        } else {
                            format!("value{}", j + 1)
                        };
                        format!("{}: {}", fname, field_decode_expr(ty, ir))
                    })
                    .collect();
                writeln!(out, "      return {{ type: '{tag}', {} }};", fields.join(", ")).unwrap();
            }
            VariantKind::Struct(fields) => {
                let parts: Vec<String> = fields
                    .iter()
                    .map(|f| {
                        format!(
                            "{}: {}",
                            to_camel_case(&f.name),
                            field_decode_expr(&f.ty, ir)
                        )
                    })
                    .collect();
                writeln!(out, "      return {{ type: '{tag}', {} }};", parts.join(", ")).unwrap();
            }
        }
    }
    writeln!(out, "    default: throw new CodecError(`unknown {name} discriminant: ${{_d}}`);")
        .unwrap();
    out.push_str("  }\n}\n");
}

fn emit_load_messages_codec(out: &mut String, t: &TaggedEnumDef, ir: &ParsedModule) {
    let name = &t.name;

    writeln!(out, "export function encode{name}(w: ProtocolWriter, v: {name}): void {{").unwrap();
    out.push_str("  switch (v.type) {\n");
    for (i, v) in t.variants.iter().enumerate() {
        let tag = to_lower_camel(&v.name);
        writeln!(out, "    case '{tag}':").unwrap();
        out.push_str("      w.writeU32(v.chatId);\n");
        writeln!(out, "      w.writeU8({i});").unwrap();
        if let VariantKind::Struct(fields) = &v.kind {
            for f in fields {
                if f.name == "chat_id" {
                    continue;
                }
                let accessor = format!("v.{}", to_camel_case(&f.name));
                emit_encode_field(out, &accessor, &f.ty, ir, "      ");
            }
        }
        out.push_str("      break;\n");
    }
    out.push_str("  }\n}\n\n");

    writeln!(out, "export function decode{name}(r: ProtocolReader): {name} {{").unwrap();
    out.push_str("  const chatId = r.readU32();\n  const _d = r.readU8();\n  switch (_d) {\n");
    for (i, v) in t.variants.iter().enumerate() {
        let tag = to_lower_camel(&v.name);
        writeln!(out, "    case {i}:").unwrap();
        if let VariantKind::Struct(fields) = &v.kind {
            let parts: Vec<String> = fields
                .iter()
                .map(|f| {
                    if f.name == "chat_id" {
                        format!("{}: chatId", to_camel_case(&f.name))
                    } else {
                        format!(
                            "{}: {}",
                            to_camel_case(&f.name),
                            field_decode_expr(&f.ty, ir)
                        )
                    }
                })
                .collect();
            writeln!(out, "      return {{ type: '{tag}', {} }};", parts.join(", ")).unwrap();
        }
    }
    writeln!(out, "    default: throw new CodecError(`unknown {name} mode: ${{_d}}`);").unwrap();
    out.push_str("  }\n}\n");
}

// ---------------------------------------------------------------------------
// Codecs file orchestrator
// ---------------------------------------------------------------------------

fn emit_codecs_ts(ir: &ParsedModule) -> String {
    let mut out = String::from(HEADER);
    out.push('\n');

    let mut enum_refs = BTreeSet::new();
    let mut other_refs = BTreeSet::new();
    let mut all_struct_names = BTreeSet::new();
    let mut all_tagged_names = BTreeSet::new();
    let has_error_payload = ir.structs.iter().any(|s| s.name == "ErrorPayload");
    let has_message = ir.structs.iter().any(|s| s.name == "Message");

    for s in &ir.structs {
        all_struct_names.insert(s.name.clone());
        for f in &s.fields {
            classify_field_ref(&f.ty, ir, &mut enum_refs, &mut other_refs);
        }
    }
    for t in &ir.tagged_enums {
        if SKIP_TAGGED_ENUMS.contains(&t.name.as_str()) {
            continue;
        }
        all_tagged_names.insert(t.name.clone());
        for v in &t.variants {
            match &v.kind {
                VariantKind::Unit => {}
                VariantKind::Tuple(types) => {
                    for ty in types {
                        classify_field_ref(ty, ir, &mut enum_refs, &mut other_refs);
                    }
                }
                VariantKind::Struct(fields) => {
                    for f in fields {
                        classify_field_ref(&f.ty, ir, &mut enum_refs, &mut other_refs);
                    }
                }
            }
        }
    }

    // Imports
    out.push_str("import { CodecError } from './error.js';\n");
    out.push_str("import { ProtocolReader } from './reader.js';\n");
    out.push_str("import { ProtocolWriter } from './writer.js';\n\n");

    // Enum imports (fromValue functions only — types are not needed directly)
    for name in &enum_refs {
        let file = to_kebab_case(name);
        let from_fn = format!("{}FromValue", to_lower_camel(name));
        writeln!(out, "import {{ {from_fn} }} from '../types/{file}.js';").unwrap();
    }
    if has_error_payload && enum_refs.contains("ErrorCode") {
        out.push_str("import { errorCodeSlug } from '../types/error-code.js';\n");
    } else if has_error_payload {
        out.push_str(
            "import { errorCodeFromValue, errorCodeSlug } from '../types/error-code.js';\n",
        );
    }
    if has_message && !enum_refs.contains("MessageKind") {
        out.push_str(
            "import { messageKindFromValue } from '../types/message-kind.js';\n",
        );
    }
    // Bitflags: no imports needed (they're just numbers)
    // Struct/tagged enum type imports (for function signatures)
    let mut imported = BTreeSet::new();
    imported.extend(enum_refs.iter().cloned());
    for name in &other_refs {
        if imported.contains(name) {
            continue;
        }
        // Skip bitflags — they're just `number` at runtime
        if ir.bitflags.iter().any(|b| b.name == *name) {
            continue;
        }
        let file = to_kebab_case(name);
        writeln!(out, "import type {{ {name} }} from '../types/{file}.js';").unwrap();
        imported.insert(name.clone());
    }
    for name in &all_struct_names {
        if imported.contains(name) {
            continue;
        }
        let file = to_kebab_case(name);
        writeln!(out, "import type {{ {name} }} from '../types/{file}.js';").unwrap();
        imported.insert(name.clone());
    }
    for name in &all_tagged_names {
        if imported.contains(name) {
            continue;
        }
        let file = to_kebab_case(name);
        writeln!(out, "import type {{ {name} }} from '../types/{file}.js';").unwrap();
    }

    out.push('\n');

    // Struct codecs
    for s in &ir.structs {
        if SPECIAL_STRUCTS.contains(&s.name.as_str()) {
            continue;
        }
        emit_struct_codec(&mut out, s, ir);
        out.push('\n');
    }

    // Special struct codecs
    if has_error_payload {
        emit_error_payload_codec(&mut out);
        out.push('\n');
    }
    if has_message {
        emit_message_codec(&mut out);
        out.push('\n');
    }
    if ir.structs.iter().any(|s| s.name == "MessageBatch") {
        emit_message_batch_codec(&mut out);
        out.push('\n');
    }

    // Tagged enum codecs
    for t in &ir.tagged_enums {
        if SKIP_TAGGED_ENUMS.contains(&t.name.as_str()) {
            continue;
        }
        emit_tagged_enum_codec(&mut out, t, ir);
        out.push('\n');
    }

    out
}

fn classify_field_ref(
    ty: &FieldType,
    ir: &ParsedModule,
    enum_refs: &mut BTreeSet<String>,
    other_refs: &mut BTreeSet<String>,
) {
    match ty {
        FieldType::Enum(n) => {
            enum_refs.insert(n.clone());
        }
        FieldType::Bitflags(n) | FieldType::OptionalBitflags(n) => {
            other_refs.insert(n.clone());
        }
        FieldType::Struct(n)
        | FieldType::OptionalStruct(n)
        | FieldType::VecStruct(n)
        | FieldType::OptionalVecStruct(n) => {
            if ir.enums.iter().any(|e| e.name == *n) {
                enum_refs.insert(n.clone());
            } else {
                other_refs.insert(n.clone());
            }
        }
        FieldType::TaggedEnum(n) => {
            other_refs.insert(n.clone());
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Frame codec
// ---------------------------------------------------------------------------

fn emit_frame_codec_ts(ir: &ParsedModule) -> String {
    let mut out = String::from(HEADER);
    out.push('\n');

    out.push_str("import { CodecError } from './error.js';\n");
    out.push_str("import { ProtocolReader } from './reader.js';\n");
    out.push_str("import { ProtocolWriter } from './writer.js';\n");
    out.push_str(
        "import { type FrameKind, frameKindFromValue } from '../types/frame-kind.js';\n",
    );

    let mut codec_fns = BTreeSet::new();
    let mut type_imports = BTreeSet::new();
    for &(_, flag, payload_type) in FRAME_DISPATCH {
        if flag == 0 || flag == 4 || payload_type.is_empty() {
            continue;
        }
        type_imports.insert(payload_type);
        codec_fns.insert(format!("encode{payload_type}"));
        codec_fns.insert(format!("decode{payload_type}"));
    }

    if !codec_fns.is_empty() {
        out.push_str("import {\n");
        for f in &codec_fns {
            writeln!(out, "  {f},").unwrap();
        }
        out.push_str("} from './codecs.js';\n");
    }
    for name in &type_imports {
        let file = to_kebab_case(name);
        writeln!(out, "import type {{ {name} }} from '../types/{file}.js';").unwrap();
    }
    out.push('\n');

    // FrameHeader
    out.push_str(
        "export interface FrameHeader {\n\
         \x20 readonly kind: FrameKind;\n\
         \x20 readonly seq: number;\n\
         \x20 readonly eventSeq: number;\n\
         }\n\n",
    );

    // FramePayload discriminated union
    out.push_str("export type FramePayload =\n");
    for &(kind_name, flag, payload_type) in FRAME_DISPATCH {
        let tag = to_lower_camel(kind_name);
        match flag {
            0 => writeln!(out, "  | {{ readonly type: '{tag}' }}").unwrap(),
            4 => writeln!(
                out,
                "  | {{ readonly type: '{tag}'; readonly data: Uint8Array }}"
            )
            .unwrap(),
            3 => writeln!(
                out,
                "  | {{ readonly type: '{tag}'; readonly data: readonly {payload_type}[] }}"
            )
            .unwrap(),
            _ => writeln!(
                out,
                "  | {{ readonly type: '{tag}'; readonly data: {payload_type} }}"
            )
            .unwrap(),
        }
    }
    out.push_str(";\n\n");

    // Frame
    out.push_str(
        "export interface Frame {\n\
         \x20 readonly seq: number;\n\
         \x20 readonly eventSeq: number;\n\
         \x20 readonly payload: FramePayload;\n\
         }\n\n",
    );

    // encodeFrameHeader / decodeFrameHeader
    out.push_str(
        "export function encodeFrameHeader(w: ProtocolWriter, h: FrameHeader): void {\n\
         \x20 w.writeU8(h.kind);\n\
         \x20 w.writeU32(h.seq);\n\
         \x20 w.writeU32(h.eventSeq);\n\
         }\n\n\
         export function decodeFrameHeader(r: ProtocolReader): FrameHeader {\n\
         \x20 const kindByte = r.readU8();\n\
         \x20 const kind = frameKindFromValue(kindByte);\n\
         \x20 if (kind === undefined) throw new CodecError(`unknown FrameKind: ${kindByte}`);\n\
         \x20 return { kind, seq: r.readU32(), eventSeq: r.readU32() };\n\
         }\n\n",
    );

    // framePayloadKind helper
    let kind_values: std::collections::HashMap<&str, u64> = ir
        .enums
        .iter()
        .find(|e| e.name == "FrameKind")
        .map(|e| {
            e.variants
                .iter()
                .map(|v| (v.name.as_str(), v.discriminant))
                .collect()
        })
        .unwrap_or_default();

    out.push_str("function framePayloadKind(p: FramePayload): FrameKind {\n  switch (p.type) {\n");
    for &(kind_name, _, _) in FRAME_DISPATCH {
        let tag = to_lower_camel(kind_name);
        if let Some(&val) = kind_values.get(kind_name) {
            writeln!(out, "    case '{tag}': return {val} as FrameKind;").unwrap();
        }
    }
    out.push_str("  }\n}\n\n");

    // encodeFrame
    out.push_str(
        "export function encodeFrame(w: ProtocolWriter, frame: Frame): void {\n\
         \x20 const kind = framePayloadKind(frame.payload);\n\
         \x20 w.writeU8(kind);\n\
         \x20 w.writeU32(frame.seq);\n\
         \x20 w.writeU32(frame.eventSeq);\n\
         \x20 switch (frame.payload.type) {\n",
    );
    for &(kind_name, flag, payload_type) in FRAME_DISPATCH {
        let tag = to_lower_camel(kind_name);
        match flag {
            0 => writeln!(out, "    case '{tag}': break;").unwrap(),
            4 => writeln!(out, "    case '{tag}': w.writeRawBytes(frame.payload.data); break;")
                .unwrap(),
            3 => {
                let enc = format!("encode{payload_type}");
                writeln!(out, "    case '{tag}': w.writeU16(frame.payload.data.length); for (const _e of frame.payload.data) {enc}(w, _e); break;").unwrap();
            }
            _ => {
                let enc = format!("encode{payload_type}");
                writeln!(out, "    case '{tag}': {enc}(w, frame.payload.data); break;").unwrap();
            }
        }
    }
    out.push_str("  }\n}\n\n");

    // decodeFrame
    out.push_str(
        "export function decodeFrame(r: ProtocolReader): Frame {\n\
         \x20 const header = decodeFrameHeader(r);\n\
         \x20 let payload: FramePayload;\n\
         \x20 switch (header.kind) {\n",
    );
    for &(kind_name, flag, payload_type) in FRAME_DISPATCH {
        let tag = to_lower_camel(kind_name);
        if let Some(&val) = kind_values.get(kind_name) {
            match flag {
                0 => writeln!(out, "    case {val}: payload = {{ type: '{tag}' }}; break;")
                    .unwrap(),
                4 => writeln!(out, "    case {val}: payload = {{ type: '{tag}', data: r.remaining > 0 ? r.readBytes(r.remaining) : new Uint8Array(0) }}; break;").unwrap(),
                3 => {
                    let dec = format!("decode{payload_type}");
                    writeln!(out, "    case {val}: payload = {{ type: '{tag}', data: r.readArray(r.readU16(), () => {dec}(r)) }}; break;").unwrap();
                }
                _ => {
                    let dec = format!("decode{payload_type}");
                    writeln!(out, "    case {val}: payload = {{ type: '{tag}', data: {dec}(r) }}; break;").unwrap();
                }
            }
        }
    }
    out.push_str("    default: throw new CodecError(`unhandled FrameKind: ${header.kind}`);\n");
    out.push_str("  }\n  return { seq: header.seq, eventSeq: header.eventSeq, payload };\n}\n");

    out
}

// ---------------------------------------------------------------------------
// Static codec utility templates
// ---------------------------------------------------------------------------

fn emit_codec_error_ts() -> String {
    format!(
        "{HEADER}\n\
         export class CodecError extends Error {{\n\
         \x20 constructor(message: string) {{\n\
         \x20   super(message);\n\
         \x20   this.name = 'CodecError';\n\
         \x20 }}\n\
         }}\n"
    )
}

fn emit_reader_ts() -> String {
    format!(
        "{HEADER}\n\
import {{ CodecError }} from './error.js';\n\
\n\
const textDecoder = new TextDecoder();\n\
\n\
export class ProtocolReader {{\n\
  private readonly view: DataView;\n\
  private readonly bytes: Uint8Array;\n\
  private pos: number;\n\
\n\
  constructor(data: Uint8Array) {{\n\
    this.bytes = data;\n\
    this.view = new DataView(data.buffer, data.byteOffset, data.byteLength);\n\
    this.pos = 0;\n\
  }}\n\
\n\
  get remaining(): number {{ return this.bytes.byteLength - this.pos; }}\n\
\n\
  ensureRemaining(n: number): void {{\n\
    if (this.remaining < n) throw new CodecError(`truncated: need ${{n}} bytes but only ${{this.remaining}} remain`);\n\
  }}\n\
\n\
  readU8(): number {{ this.ensureRemaining(1); return this.view.getUint8(this.pos++); }}\n\
\n\
  readU16(): number {{\n\
    this.ensureRemaining(2);\n\
    const v = this.view.getUint16(this.pos, true);\n\
    this.pos += 2;\n\
    return v;\n\
  }}\n\
\n\
  readU32(): number {{\n\
    this.ensureRemaining(4);\n\
    const v = this.view.getUint32(this.pos, true);\n\
    this.pos += 4;\n\
    return v;\n\
  }}\n\
\n\
  readI64(): number {{\n\
    this.ensureRemaining(8);\n\
    const lo = this.view.getUint32(this.pos, true);\n\
    const hi = this.view.getInt32(this.pos + 4, true);\n\
    this.pos += 8;\n\
    return hi * 4294967296 + lo;\n\
  }}\n\
\n\
  readTimestamp(): number {{\n\
    const v = this.readI64();\n\
    if (v < 0 || v > 2199023255551) throw new CodecError(`timestamp out of range: ${{v}}`);\n\
    return v;\n\
  }}\n\
\n\
  readString(): string {{\n\
    const len = this.readU32();\n\
    if (len === 0) return '';\n\
    this.ensureRemaining(len);\n\
    const s = textDecoder.decode(this.bytes.subarray(this.pos, this.pos + len));\n\
    this.pos += len;\n\
    return s;\n\
  }}\n\
\n\
  readOptionalString(): string | null {{\n\
    const len = this.readU32();\n\
    if (len === 0) return null;\n\
    this.ensureRemaining(len);\n\
    const s = textDecoder.decode(this.bytes.subarray(this.pos, this.pos + len));\n\
    this.pos += len;\n\
    return s;\n\
  }}\n\
\n\
  readOptionalBytes(): Uint8Array | null {{\n\
    const len = this.readU32();\n\
    if (len === 0) return null;\n\
    this.ensureRemaining(len);\n\
    const out = this.bytes.slice(this.pos, this.pos + len);\n\
    this.pos += len;\n\
    return out;\n\
  }}\n\
\n\
  readUuid(): string {{\n\
    this.ensureRemaining(16);\n\
    const hex: string[] = [];\n\
    for (let i = 0; i < 16; i++) hex.push(this.bytes[this.pos + i]!.toString(16).padStart(2, '0'));\n\
    this.pos += 16;\n\
    const h = hex.join('');\n\
    return `${{h.slice(0, 8)}}-${{h.slice(8, 12)}}-${{h.slice(12, 16)}}-${{h.slice(16, 20)}}-${{h.slice(20)}}`;\n\
  }}\n\
\n\
  readOptionU32(): number | null {{\n\
    const flag = this.readU8();\n\
    if (flag === 0) return null;\n\
    if (flag === 1) return this.readU32();\n\
    throw new CodecError(`invalid Option<u32> flag: ${{flag}}`);\n\
  }}\n\
\n\
  readUpdatableString(): string | null {{\n\
    const flag = this.readU8();\n\
    if (flag === 0) return null;\n\
    if (flag === 1) return this.readString();\n\
    throw new CodecError(`invalid updatable string flag: ${{flag}}`);\n\
  }}\n\
\n\
  readBytes(n: number): Uint8Array {{\n\
    this.ensureRemaining(n);\n\
    const out = this.bytes.slice(this.pos, this.pos + n);\n\
    this.pos += n;\n\
    return out;\n\
  }}\n\
\n\
  readVecU32(): number[] {{\n\
    const count = this.readU16();\n\
    const out: number[] = [];\n\
    for (let i = 0; i < count; i++) out.push(this.readU32());\n\
    return out;\n\
  }}\n\
\n\
  readVecString(): string[] {{\n\
    const count = this.readU16();\n\
    const out: string[] = [];\n\
    for (let i = 0; i < count; i++) out.push(this.readString());\n\
    return out;\n\
  }}\n\
\n\
  readArray<T>(count: number, readOne: () => T): T[] {{\n\
    const out: T[] = [];\n\
    for (let i = 0; i < count; i++) out.push(readOne());\n\
    return out;\n\
  }}\n\
\n\
  readEnum<T>(raw: number, fromValue: (v: number) => T | undefined, typeName: string): T {{\n\
    const v = fromValue(raw);\n\
    if (v === undefined) throw new CodecError(`unknown ${{typeName}} discriminant: ${{raw}}`);\n\
    return v;\n\
  }}\n\
\n\
  skip(n: number): void {{ this.ensureRemaining(n); this.pos += n; }}\n\
}}\n"
    )
}

fn emit_writer_ts() -> String {
    format!(
        "{HEADER}\n\
import {{ CodecError }} from './error.js';\n\
\n\
const textEncoder = new TextEncoder();\n\
\n\
export class ProtocolWriter {{\n\
  private buf: Uint8Array;\n\
  private view: DataView;\n\
  private pos: number;\n\
\n\
  constructor(initialCapacity = 256) {{\n\
    this.buf = new Uint8Array(initialCapacity);\n\
    this.view = new DataView(this.buf.buffer);\n\
    this.pos = 0;\n\
  }}\n\
\n\
  private grow(needed: number): void {{\n\
    const required = this.pos + needed;\n\
    if (required <= this.buf.length) return;\n\
    let newLen = this.buf.length * 2;\n\
    while (newLen < required) newLen *= 2;\n\
    const next = new Uint8Array(newLen);\n\
    next.set(this.buf);\n\
    this.buf = next;\n\
    this.view = new DataView(this.buf.buffer);\n\
  }}\n\
\n\
  writeU8(v: number): void {{ this.grow(1); this.view.setUint8(this.pos++, v); }}\n\
\n\
  writeU16(v: number): void {{\n\
    this.grow(2); this.view.setUint16(this.pos, v, true); this.pos += 2;\n\
  }}\n\
\n\
  writeU32(v: number): void {{\n\
    this.grow(4); this.view.setUint32(this.pos, v, true); this.pos += 4;\n\
  }}\n\
\n\
  writeI64(v: number): void {{\n\
    this.grow(8);\n\
    this.view.setUint32(this.pos, v % 4294967296, true);\n\
    this.view.setInt32(this.pos + 4, Math.floor(v / 4294967296), true);\n\
    this.pos += 8;\n\
  }}\n\
\n\
  writeTimestamp(v: number): void {{\n\
    if (v < 0 || v > 2199023255551) throw new CodecError(`timestamp out of range: ${{v}}`);\n\
    this.writeI64(v);\n\
  }}\n\
\n\
  writeString(v: string): void {{\n\
    if (v.length === 0) {{ this.writeU32(0); return; }}\n\
    const encoded = textEncoder.encode(v);\n\
    this.writeU32(encoded.length);\n\
    this.grow(encoded.length);\n\
    this.buf.set(encoded, this.pos);\n\
    this.pos += encoded.length;\n\
  }}\n\
\n\
  writeOptionalString(v: string | null): void {{\n\
    if (v === null) {{ this.writeU32(0); }} else {{ this.writeString(v); }}\n\
  }}\n\
\n\
  writeOptionalBytes(v: Uint8Array | null): void {{\n\
    if (v === null) {{ this.writeU32(0); return; }}\n\
    this.writeU32(v.length);\n\
    this.grow(v.length);\n\
    this.buf.set(v, this.pos);\n\
    this.pos += v.length;\n\
  }}\n\
\n\
  writeUuid(uuid: string): void {{\n\
    this.grow(16);\n\
    const hex = uuid.replace(/-/g, '');\n\
    for (let i = 0; i < 16; i++) this.buf[this.pos++] = parseInt(hex.slice(i * 2, i * 2 + 2), 16);\n\
  }}\n\
\n\
  writeOptionU32(v: number | null): void {{\n\
    if (v === null) {{ this.writeU8(0); }} else {{ this.writeU8(1); this.writeU32(v); }}\n\
  }}\n\
\n\
  writeUpdatableString(v: string | null): void {{\n\
    if (v === null) {{ this.writeU8(0); }} else {{ this.writeU8(1); this.writeString(v); }}\n\
  }}\n\
\n\
  writeRawBytes(data: Uint8Array): void {{\n\
    this.grow(data.length);\n\
    this.buf.set(data, this.pos);\n\
    this.pos += data.length;\n\
  }}\n\
\n\
  toBytes(): Uint8Array {{ return this.buf.slice(0, this.pos); }}\n\
}}\n"
    )
}
