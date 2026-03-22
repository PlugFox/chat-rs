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

    // Barrel export
    exports.sort();
    write_file(&src_dir.join("index.ts"), &emit_barrel(&exports))?;

    eprintln!(
        "TypeScript: {} enums, {} bitflags, {} structs, {} tagged enums, {} constants",
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
