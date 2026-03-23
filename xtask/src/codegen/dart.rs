/// Dart code emitter — generates Dart package from IR.
use std::collections::BTreeSet;
use std::fmt::Write;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::codegen::ir::*;

const HEADER: &str = "// GENERATED CODE — DO NOT MODIFY BY HAND\n// Source: chat_protocol\n";

/// Dart package name — used for `package:` imports in generated code.
const PKG: &str = "package:chat_core";

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
const TRANSIENT_ERRORS: &[&str] = &["InternalError", "ServiceUnavailable", "DatabaseError", "RateLimited"];

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Generate the Dart package from parsed IR.
pub(crate) fn generate(ir: &ParsedModule, output_dir: &Path) -> Result<()> {
    let types_dir = output_dir.join("lib/src/types");
    let src_dir = output_dir.join("lib/src");
    let lib_dir = output_dir.join("lib");
    let codec_dir = output_dir.join("lib/src/codec");
    let test_dir = output_dir.join("test");

    fs::create_dir_all(&types_dir).context("creating types dir")?;
    fs::create_dir_all(&codec_dir).context("creating codec dir")?;
    fs::create_dir_all(&test_dir).context("creating test dir")?;

    // -- Static scaffolding (written only if missing) -------------------------
    write_if_missing(&output_dir.join("pubspec.yaml"), &emit_pubspec())?;
    write_if_missing(&codec_dir.join("error.dart"), &emit_codec_error_dart())?;
    write_if_missing(&codec_dir.join("reader.dart"), &emit_reader_dart())?;
    write_if_missing(&codec_dir.join("writer.dart"), &emit_writer_dart())?;

    let util_dir = output_dir.join("lib/src/util");
    fs::create_dir_all(&util_dir).context("creating util dir")?;

    let needs_util = ir.structs.iter().any(|s| s.fields.iter().any(|f| is_list_type(&f.ty)));
    if needs_util {
        write_if_missing(&util_dir.join("list_equals.dart"), &emit_util())?;
    }

    // -- Generated from IR (always overwritten) -------------------------------
    let mut exports: Vec<String> = Vec::new();

    // Enums
    for e in &ir.enums {
        let fname = format!("{}.dart", to_snake_case(&e.name));
        write_file(&types_dir.join(&fname), &emit_enum(e))?;
        exports.push(format!("src/types/{fname}"));
    }

    // Bitflags
    for b in &ir.bitflags {
        let fname = format!("{}.dart", to_snake_case(&b.name));
        write_file(&types_dir.join(&fname), &emit_bitflags(b))?;
        exports.push(format!("src/types/{fname}"));
    }

    // Structs
    for s in &ir.structs {
        let fname = format!("{}.dart", to_snake_case(&s.name));
        write_file(&types_dir.join(&fname), &emit_struct(s))?;
        exports.push(format!("src/types/{fname}"));
    }

    // Tagged enums
    for t in &ir.tagged_enums {
        let fname = format!("{}.dart", to_snake_case(&t.name));
        write_file(&types_dir.join(&fname), &emit_tagged_enum(t))?;
        exports.push(format!("src/types/{fname}"));
    }

    // Constants
    if !ir.constants.is_empty() {
        write_file(&src_dir.join("protocol_constants.dart"), &emit_constants(&ir.constants))?;
        exports.push("src/protocol_constants.dart".into());
    }

    // Codec (IR-dependent)
    write_file(&codec_dir.join("codecs.dart"), &emit_codecs_dart(ir))?;
    write_file(&codec_dir.join("frame.dart"), &emit_frame_codec_dart(ir))?;

    exports.push("src/codec/error.dart".into());
    exports.push("src/codec/reader.dart".into());
    exports.push("src/codec/writer.dart".into());
    exports.push("src/codec/codecs.dart".into());
    exports.push("src/codec/frame.dart".into());

    // Barrel export
    exports.sort();
    write_file(&lib_dir.join("chat_core.dart"), &emit_barrel(&exports))?;

    // Tests
    write_file(&test_dir.join("types_test.dart"), &emit_types_test_dart(ir))?;
    write_file(&test_dir.join("codec_test.dart"), &emit_codec_test_dart(ir))?;

    eprintln!(
        "Dart: {} enums, {} bitflags, {} structs, {} tagged enums, {} constants + codec + tests",
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

/// Write file only if it does not already exist. Scaffolding files (pubspec,
/// reader, writer, error) are stable — they don't depend on IR and may be
/// hand-edited (e.g. adding dev_dependencies or optimising hot paths).
fn write_if_missing(path: &Path, content: &str) -> Result<()> {
    if !path.exists() {
        fs::write(path, content).with_context(|| format!("writing {}", path.display()))?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Name conversion
// ---------------------------------------------------------------------------

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

/// Strip Rust integer type suffixes for Dart compatibility.
fn clean_rust_expr(expr: &str) -> String {
    let mut s = expr.to_string();
    for suffix in &[
        "_i128", "_u128", "_isize", "_usize", "_i64", "_u64", "_i32", "_u32", "_i16", "_u16", "_i8", "_u8",
    ] {
        s = s.replace(suffix, "");
    }
    s
}

// ---------------------------------------------------------------------------
// Type mapping
// ---------------------------------------------------------------------------

/// Map `FieldType` to Dart type string.
fn dart_type(ty: &FieldType) -> String {
    match ty {
        FieldType::U8 | FieldType::U16 | FieldType::U32 | FieldType::I64 => "int".into(),
        FieldType::Bool => "bool".into(),
        FieldType::String => "String".into(),
        FieldType::OptionalString | FieldType::UpdatableString => "String?".into(),
        FieldType::Uuid => "String".into(),
        FieldType::OptionalU32 => "int?".into(),
        FieldType::VecU32 => "List<int>".into(),
        FieldType::VecU8 => "Uint8List".into(),
        FieldType::OptionalBytes => "Uint8List?".into(),
        FieldType::VecString => "List<String>".into(),
        FieldType::Enum(n) | FieldType::Bitflags(n) | FieldType::Struct(n) | FieldType::TaggedEnum(n) => n.clone(),
        FieldType::OptionalStruct(n) | FieldType::OptionalBitflags(n) => format!("{n}?"),
        FieldType::OptionalVecStruct(n) => format!("List<{n}>?"),
        FieldType::VecStruct(n) => format!("List<{n}>"),
    }
}

fn is_nullable(ty: &FieldType) -> bool {
    matches!(
        ty,
        FieldType::OptionalString
            | FieldType::UpdatableString
            | FieldType::OptionalU32
            | FieldType::OptionalBytes
            | FieldType::OptionalStruct(_)
            | FieldType::OptionalBitflags(_)
            | FieldType::OptionalVecStruct(_)
    )
}

fn is_list_type(ty: &FieldType) -> bool {
    matches!(
        ty,
        FieldType::VecU32
            | FieldType::VecU8
            | FieldType::OptionalBytes
            | FieldType::VecString
            | FieldType::VecStruct(_)
            | FieldType::OptionalVecStruct(_)
    )
}

fn needs_typed_data(ty: &FieldType) -> bool {
    matches!(ty, FieldType::VecU8 | FieldType::OptionalBytes)
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
    for line in doc.lines() {
        if line.is_empty() {
            writeln!(out, "{indent}///").unwrap();
        } else {
            writeln!(out, "{indent}/// {line}").unwrap();
        }
    }
}

fn emit_import_block(out: &mut String, has_typed_data: bool, has_util: bool, type_refs: &BTreeSet<String>) {
    out.push('\n');

    if has_typed_data {
        out.push_str("import 'dart:typed_data';\n\n");
    }

    out.push_str("import 'package:meta/meta.dart';\n");

    let has_pkg = has_util || !type_refs.is_empty();
    if has_pkg {
        out.push('\n');
    }

    if has_util {
        writeln!(out, "import '{PKG}/src/util/list_equals.dart';").unwrap();
    }
    for r in type_refs {
        writeln!(out, "import '{PKG}/src/types/{}.dart';", to_snake_case(r)).unwrap();
    }
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

/// Generate the hashCode contribution for a single field.
fn hash_element(f: &Field) -> String {
    let name = to_camel_case(&f.name);
    if is_list_type(&f.ty) {
        if is_nullable(&f.ty) {
            format!("Object.hashAll({name} ?? const [])")
        } else {
            format!("Object.hashAll({name})")
        }
    } else {
        name
    }
}

/// Generate the equality comparison for a single field.
fn eq_comparison(f: &Field) -> String {
    let name = to_camel_case(&f.name);
    if is_list_type(&f.ty) {
        format!("listEquals({name}, other.{name})")
    } else {
        format!("{name} == other.{name}")
    }
}

// ---------------------------------------------------------------------------
// Emitters
// ---------------------------------------------------------------------------

fn emit_pubspec() -> String {
    [
        "# GENERATED — DO NOT MODIFY BY HAND",
        "name: chat_core",
        "description: Chat protocol types and binary codec.",
        "version: 0.1.0",
        "environment:",
        "  sdk: ^3.7.0",
        "dev_dependencies:",
        "  benchmark_harness: ^2.3.0",
        "  lints: ^6.0.0",
        "  test: ^1.25.0",
        "",
    ]
    .join("\n")
}

fn emit_enum(e: &EnumDef) -> String {
    let mut out = String::from(HEADER);
    out.push('\n');
    write_doc(&mut out, &e.doc, "");

    writeln!(out, "enum {} {{", e.name).unwrap();

    for (i, v) in e.variants.iter().enumerate() {
        write_doc(&mut out, &v.doc, "  ");
        let dart_name = to_lower_camel(&v.name);
        let sep = if i < e.variants.len() - 1 { ',' } else { ';' };
        writeln!(out, "  {dart_name}({}){sep}", v.discriminant).unwrap();
    }

    out.push('\n');
    writeln!(out, "  const {}(this.value);", e.name).unwrap();
    out.push_str("  final int value;\n");

    // fromValue
    out.push('\n');
    writeln!(out, "  static {}? fromValue(int value) => switch (value) {{", e.name).unwrap();
    for v in &e.variants {
        let dart_name = to_lower_camel(&v.name);
        writeln!(out, "    {} => {dart_name},", v.discriminant).unwrap();
    }
    out.push_str("    _ => null,\n");
    out.push_str("  };\n");

    // ErrorCode extras
    if e.name == "ErrorCode" {
        emit_error_code_extras(&mut out, e);
    }

    // DisconnectCode extras
    if e.name == "DisconnectCode" {
        emit_disconnect_code_extras(&mut out);
    }

    out.push_str("}\n");
    out
}

fn emit_error_code_extras(out: &mut String, e: &EnumDef) {
    // slug getter
    out.push('\n');
    out.push_str("  /// Stable snake_case identifier for client matching.\n");
    out.push_str("  String get slug => switch (this) {\n");
    for v in &e.variants {
        let dart_name = to_lower_camel(&v.name);
        let slug = to_snake_case(&v.name);
        writeln!(out, "    {dart_name} => '{slug}',").unwrap();
    }
    out.push_str("  };\n");

    // isPermanent
    out.push('\n');
    out.push_str("  /// Whether this error is permanent (do not retry).\n");
    out.push_str("  bool get isPermanent => switch (this) {\n");
    let permanent: Vec<String> = PERMANENT_ERRORS.iter().map(|n| to_lower_camel(n)).collect();
    writeln!(out, "    {} => true,", permanent.join(" || ")).unwrap();
    out.push_str("    _ => false,\n");
    out.push_str("  };\n");

    // isTransient
    out.push('\n');
    out.push_str("  /// Whether this error is transient (retry with backoff).\n");
    out.push_str("  bool get isTransient => switch (this) {\n");
    let transient: Vec<String> = TRANSIENT_ERRORS.iter().map(|n| to_lower_camel(n)).collect();
    writeln!(out, "    {} => true,", transient.join(" || ")).unwrap();
    out.push_str("    _ => false,\n");
    out.push_str("  };\n");
}

fn emit_disconnect_code_extras(out: &mut String) {
    out.push('\n');
    out.push_str("  /// Whether the client should attempt reconnection.\n");
    out.push_str("  bool get shouldReconnect {\n");
    out.push_str("    return (value >= 0 && value < 1000) ||\n");
    out.push_str("        (value >= 3000 && value < 3500) ||\n");
    out.push_str("        (value >= 4000 && value < 4500);\n");
    out.push_str("  }\n");
}

fn emit_bitflags(b: &BitflagsDef) -> String {
    let mut out = String::from(HEADER);
    out.push('\n');
    write_doc(&mut out, &b.doc, "");

    let n = &b.name;
    writeln!(out, "extension type const {n}(int value) implements int {{").unwrap();

    // Flag constants
    for f in &b.flags {
        write_doc(&mut out, &f.doc, "  ");
        let dart_name = to_camel_case(&f.name);
        let expr = clean_rust_expr(&f.value_expr);
        writeln!(out, "  static const {n} {dart_name} = {n}({expr});").unwrap();
    }

    // values list
    out.push('\n');
    let names: Vec<String> = b.flags.iter().map(|f| to_camel_case(&f.name)).collect();
    writeln!(out, "  static const List<{n}> values = [{}];", names.join(", ")).unwrap();

    // Methods
    out.push('\n');
    writeln!(out, "  bool contains({n} flag) => (value & flag.value) != 0;").unwrap();
    writeln!(out, "  {n} add({n} flag) => {n}(value | flag.value);").unwrap();
    writeln!(out, "  {n} remove({n} flag) => {n}(value & ~flag.value);").unwrap();
    writeln!(out, "  {n} toggle({n} flag) => {n}(value ^ flag.value);").unwrap();
    out.push_str("  bool get isEmpty => value == 0;\n");
    out.push_str("  bool get isNotEmpty => value != 0;\n");
    writeln!(out, "  {n} operator ^({n} other) => {n}(value ^ other.value);").unwrap();

    out.push_str("}\n");
    out
}

fn emit_struct(s: &StructDef) -> String {
    let mut out = String::from(HEADER);

    // Analyze fields for imports
    let mut has_typed_data = false;
    let mut has_list = false;
    let mut type_refs = BTreeSet::new();

    for f in &s.fields {
        if needs_typed_data(&f.ty) {
            has_typed_data = true;
        }
        if is_list_type(&f.ty) {
            has_list = true;
        }
        collect_field_refs(&f.ty, &mut type_refs);
    }

    emit_import_block(&mut out, has_typed_data, has_list, &type_refs);
    out.push('\n');

    // Class doc + declaration
    write_doc(&mut out, &s.doc, "");
    out.push_str("@immutable\n");
    writeln!(out, "class {} {{", s.name).unwrap();

    // Constructor
    if s.fields.is_empty() {
        writeln!(out, "  const {}();", s.name).unwrap();
    } else {
        writeln!(out, "  const {}({{", s.name).unwrap();
        for f in &s.fields {
            let dart_name = to_camel_case(&f.name);
            if is_nullable(&f.ty) {
                writeln!(out, "    this.{dart_name},").unwrap();
            } else {
                writeln!(out, "    required this.{dart_name},").unwrap();
            }
        }
        out.push_str("  });\n");
    }

    // Fields
    if !s.fields.is_empty() {
        out.push('\n');
        for f in &s.fields {
            write_doc(&mut out, &f.doc, "  ");
            writeln!(out, "  final {} {};", dart_type(&f.ty), to_camel_case(&f.name)).unwrap();
        }
    }

    // == operator
    out.push('\n');
    if s.fields.is_empty() {
        out.push_str("  @override\n");
        writeln!(
            out,
            "  bool operator ==(Object other) => identical(this, other) || other is {}; // coverage:ignore-line",
            s.name
        )
        .unwrap();
    } else {
        out.push_str("  // coverage:ignore-start\n");
        out.push_str("  @override\n");
        out.push_str("  bool operator ==(Object other) =>\n");
        write!(out, "      identical(this, other) ||\n      other is {}", s.name).unwrap();
        for f in &s.fields {
            let cmp = eq_comparison(f);
            write!(out, " &&\n          {cmp}").unwrap();
        }
        out.push_str(";\n");
        out.push_str("  // coverage:ignore-end\n");
    }

    // hashCode
    out.push('\n');
    out.push_str("  @override\n");
    if s.fields.is_empty() {
        out.push_str("  int get hashCode => 0;\n");
    } else if s.fields.len() == 1 {
        let elem = hash_element(&s.fields[0]);
        if is_list_type(&s.fields[0].ty) {
            writeln!(out, "  int get hashCode => {elem};").unwrap();
        } else {
            writeln!(out, "  int get hashCode => {elem}.hashCode;").unwrap();
        }
    } else {
        out.push_str("  int get hashCode => Object.hash(\n");
        for f in &s.fields {
            let elem = hash_element(f);
            writeln!(out, "        {elem},").unwrap();
        }
        out.push_str("      );\n");
    }

    out.push_str("}\n");
    out
}

/// Emit == and hashCode for a tagged enum variant subclass with fields.
fn emit_variant_equality(out: &mut String, class_name: &str, field_names: &[&str], field_types: &[FieldType]) {
    // ==
    out.push('\n');
    out.push_str("  // coverage:ignore-start\n");
    out.push_str("  @override\n");
    out.push_str("  bool operator ==(Object other) =>\n");
    write!(out, "      identical(this, other) ||\n      other is {class_name}").unwrap();
    for (name, ty) in field_names.iter().zip(field_types.iter()) {
        if is_list_type(ty) {
            write!(out, " &&\n          listEquals({name}, other.{name})").unwrap();
        } else {
            write!(out, " &&\n          {name} == other.{name}").unwrap();
        }
    }
    out.push_str(";\n");
    out.push_str("  // coverage:ignore-end\n");

    // hashCode
    out.push('\n');
    out.push_str("  @override\n");
    if field_names.len() == 1 {
        let name = field_names[0];
        let ty = &field_types[0];
        if is_list_type(ty) {
            if is_nullable(ty) {
                writeln!(out, "  int get hashCode => Object.hashAll({name} ?? const []);").unwrap();
            } else {
                writeln!(out, "  int get hashCode => Object.hashAll({name});").unwrap();
            }
        } else {
            writeln!(out, "  int get hashCode => {name}.hashCode;").unwrap();
        }
    } else {
        out.push_str("  int get hashCode => Object.hash(\n");
        for (name, ty) in field_names.iter().zip(field_types.iter()) {
            if is_list_type(ty) {
                if is_nullable(ty) {
                    writeln!(out, "        Object.hashAll({name} ?? const []),").unwrap();
                } else {
                    writeln!(out, "        Object.hashAll({name}),").unwrap();
                }
            } else {
                writeln!(out, "        {name},").unwrap();
            }
        }
        out.push_str("      );\n");
    }
}

fn emit_tagged_enum(t: &TaggedEnumDef) -> String {
    let mut out = String::from(HEADER);

    // Collect imports from all variant fields
    let mut type_refs = BTreeSet::new();
    let mut has_typed_data = false;
    let mut has_list = false;

    for v in &t.variants {
        match &v.kind {
            VariantKind::Unit => {}
            VariantKind::Tuple(types) => {
                for ty in types {
                    if needs_typed_data(ty) {
                        has_typed_data = true;
                    }
                    if is_list_type(ty) {
                        has_list = true;
                    }
                    collect_field_refs(ty, &mut type_refs);
                }
            }
            VariantKind::Struct(fields) => {
                for f in fields {
                    if needs_typed_data(&f.ty) {
                        has_typed_data = true;
                    }
                    if is_list_type(&f.ty) {
                        has_list = true;
                    }
                    collect_field_refs(&f.ty, &mut type_refs);
                }
            }
        }
    }

    emit_import_block(&mut out, has_typed_data, has_list, &type_refs);
    out.push('\n');

    // Sealed base class
    write_doc(&mut out, &t.doc, "");
    out.push_str("@immutable\n");
    writeln!(out, "sealed class {} {{", t.name).unwrap();
    writeln!(out, "  const {}();", t.name).unwrap();
    out.push_str("}\n");

    // Variant subclasses
    let base = t.name.strip_suffix("Payload").unwrap_or(&t.name);
    for v in &t.variants {
        let class_name = format!("{base}{}", v.name);

        out.push('\n');
        write_doc(&mut out, &v.doc, "");

        match &v.kind {
            VariantKind::Unit => {
                writeln!(out, "class {class_name} extends {} {{", t.name).unwrap();
                writeln!(out, "  const {class_name}();").unwrap();
                // == / hashCode for unit variant
                out.push('\n');
                out.push_str("  @override\n");
                writeln!(out, "  bool operator ==(Object other) => identical(this, other) || other is {class_name}; // coverage:ignore-line").unwrap();
                out.push('\n');
                out.push_str("  @override\n");
                out.push_str("  int get hashCode => 0;\n");
                out.push_str("}\n");
            }
            VariantKind::Tuple(types) => {
                writeln!(out, "class {class_name} extends {} {{", t.name).unwrap();

                let fields: Vec<(String, String)> = types
                    .iter()
                    .enumerate()
                    .map(|(i, ty)| {
                        let name = if types.len() == 1 {
                            tuple_field_name(ty)
                        } else {
                            format!("value{}", i + 1)
                        };
                        (name, dart_type(ty))
                    })
                    .collect();

                // Constructor
                writeln!(out, "  const {class_name}({{").unwrap();
                for (name, _) in &fields {
                    writeln!(out, "    required this.{name},").unwrap();
                }
                out.push_str("  });\n");

                // Fields
                out.push('\n');
                for (name, ty) in &fields {
                    writeln!(out, "  final {ty} {name};").unwrap();
                }

                // == / hashCode
                emit_variant_equality(
                    &mut out,
                    &class_name,
                    &fields.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>(),
                    types,
                );
                out.push_str("}\n");
            }
            VariantKind::Struct(fields) => {
                writeln!(out, "class {class_name} extends {} {{", t.name).unwrap();

                if fields.is_empty() {
                    writeln!(out, "  const {class_name}();").unwrap();
                    // == / hashCode for empty struct variant
                    out.push('\n');
                    out.push_str("  @override\n");
                    writeln!(out, "  bool operator ==(Object other) => identical(this, other) || other is {class_name}; // coverage:ignore-line").unwrap();
                    out.push('\n');
                    out.push_str("  @override\n");
                    out.push_str("  int get hashCode => 0;\n");
                } else {
                    // Constructor
                    writeln!(out, "  const {class_name}({{").unwrap();
                    for f in fields {
                        let dart_name = to_camel_case(&f.name);
                        if is_nullable(&f.ty) {
                            writeln!(out, "    this.{dart_name},").unwrap();
                        } else {
                            writeln!(out, "    required this.{dart_name},").unwrap();
                        }
                    }
                    out.push_str("  });\n");

                    // Fields
                    out.push('\n');
                    for f in fields {
                        write_doc(&mut out, &f.doc, "  ");
                        writeln!(out, "  final {} {};", dart_type(&f.ty), to_camel_case(&f.name)).unwrap();
                    }

                    // == / hashCode
                    let names: Vec<&str> = fields.iter().map(|f| f.name.as_str()).collect();
                    let types: Vec<FieldType> = fields.iter().map(|f| f.ty.clone()).collect();
                    let dart_names: Vec<String> = names.iter().map(|n| to_camel_case(n)).collect();
                    emit_variant_equality(
                        &mut out,
                        &class_name,
                        &dart_names.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                        &types,
                    );
                }

                out.push_str("}\n");
            }
        }
    }

    out
}

fn emit_constants(constants: &[ConstDef]) -> String {
    let mut out = String::from(HEADER);
    out.push('\n');

    for (i, c) in constants.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        write_doc(&mut out, &c.doc, "");
        let dart_name = to_camel_case(&c.name);
        let dart_ty = dart_type(&c.ty);
        let value = clean_rust_expr(&c.value_expr);
        writeln!(out, "const {dart_ty} {dart_name} = {value};").unwrap();
    }

    out
}

fn emit_barrel(exports: &[String]) -> String {
    let mut out = String::from(HEADER);
    out.push_str("\n/// Chat protocol types and binary codec.\nlibrary;\n\n");

    for export in exports {
        writeln!(out, "export '{PKG}/{export}';").unwrap();
    }

    out
}

// ===========================================================================
// Phase 4: Binary Codec Generation (Dart)
// ===========================================================================

/// Same dispatch table as TS (kind_name, flag, payload_type).
/// 0=none, 1=struct, 2=tagged_enum, 3=vec_struct(u16), 4=ack.
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

const SPECIAL_STRUCTS: &[&str] = &["ErrorPayload", "Message", "MessageBatch"];
const SKIP_TAGGED_ENUMS: &[&str] = &["AckPayload", "FramePayload"];

// --- Repr helpers ---

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

// --- Dart field encode/decode ---

fn dart_emit_encode_field(out: &mut String, accessor: &str, ty: &FieldType, ir: &ParsedModule, indent: &str) {
    match ty {
        FieldType::U8 => writeln!(out, "{indent}w.writeU8({accessor});").unwrap(),
        FieldType::U16 => writeln!(out, "{indent}w.writeU16({accessor});").unwrap(),
        FieldType::U32 => writeln!(out, "{indent}w.writeU32({accessor});").unwrap(),
        FieldType::I64 => writeln!(out, "{indent}w.writeTimestamp({accessor});").unwrap(),
        FieldType::Bool => writeln!(out, "{indent}w.writeU8({accessor} ? 1 : 0);").unwrap(),
        FieldType::String => writeln!(out, "{indent}w.writeString({accessor});").unwrap(),
        FieldType::OptionalString => writeln!(out, "{indent}w.writeOptionalString({accessor});").unwrap(),
        FieldType::UpdatableString => writeln!(out, "{indent}w.writeUpdatableString({accessor});").unwrap(),
        FieldType::Uuid => writeln!(out, "{indent}w.writeUuid({accessor});").unwrap(),
        FieldType::OptionalU32 => writeln!(out, "{indent}w.writeOptionU32({accessor});").unwrap(),
        FieldType::VecU32 => {
            writeln!(out, "{indent}w.writeU16({accessor}.length);").unwrap();
            writeln!(out, "{indent}for (final v in {accessor}) {{ w.writeU32(v); }}").unwrap();
        }
        FieldType::VecU8 => {
            writeln!(out, "{indent}w.writeU32({accessor}.length);").unwrap();
            writeln!(out, "{indent}w.writeRawBytes({accessor});").unwrap();
        }
        FieldType::OptionalBytes => writeln!(out, "{indent}w.writeOptionalBytes({accessor});").unwrap(),
        FieldType::VecString => {
            writeln!(out, "{indent}w.writeU16({accessor}.length);").unwrap();
            writeln!(out, "{indent}for (final v in {accessor}) {{ w.writeString(v); }}").unwrap();
        }
        FieldType::Enum(name) => {
            let wfn = repr_write(find_enum_repr(ir, name));
            writeln!(out, "{indent}w.{wfn}({accessor}.value);").unwrap();
        }
        FieldType::Bitflags(name) => {
            let wfn = repr_write(find_bitflags_repr(ir, name));
            writeln!(out, "{indent}w.{wfn}({accessor}.value);").unwrap();
        }
        FieldType::Struct(name) => writeln!(out, "{indent}encode{name}(w, {accessor});").unwrap(),
        FieldType::OptionalStruct(name) => {
            writeln!(out, "{indent}if ({accessor} != null) {{ w.writeU8(1); encode{name}(w, {accessor}!); }} else {{ w.writeU8(0); }}").unwrap();
        }
        FieldType::OptionalBitflags(name) => {
            let wfn = repr_write(find_bitflags_repr(ir, name));
            writeln!(out, "{indent}if ({accessor} != null) {{ w.writeU8(1); w.{wfn}({accessor}!.value); }} else {{ w.writeU8(0); }}").unwrap();
        }
        FieldType::OptionalVecStruct(name) => {
            writeln!(out, "{indent}if ({accessor} != null) {{ w.writeU8(1); w.writeU32({accessor}!.length); for (final v in {accessor}!) {{ encode{name}(w, v); }} }} else {{ w.writeU8(0); }}").unwrap();
        }
        FieldType::VecStruct(name) => {
            writeln!(out, "{indent}w.writeU32({accessor}.length);").unwrap();
            writeln!(out, "{indent}for (final v in {accessor}) {{ encode{name}(w, v); }}").unwrap();
        }
        FieldType::TaggedEnum(name) => writeln!(out, "{indent}encode{name}(w, {accessor});").unwrap(),
    }
}

fn dart_field_decode_expr(ty: &FieldType, ir: &ParsedModule) -> String {
    match ty {
        FieldType::U8 => "r.readU8()".into(),
        FieldType::U16 => "r.readU16()".into(),
        FieldType::U32 => "r.readU32()".into(),
        FieldType::I64 => "r.readTimestamp()".into(),
        FieldType::Bool => "r.readU8() != 0".into(),
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
            format!("{name}.fromValue(r.{rfn}())!")
        }
        FieldType::Bitflags(name) => {
            let rfn = repr_read(find_bitflags_repr(ir, name));
            format!("{name}(r.{rfn}())")
        }
        FieldType::Struct(name) => format!("decode{name}(r)"),
        FieldType::OptionalStruct(name) => format!("r.readU8() == 1 ? decode{name}(r) : null"),
        FieldType::OptionalBitflags(name) => {
            let rfn = repr_read(find_bitflags_repr(ir, name));
            format!("r.readU8() == 1 ? {name}(r.{rfn}()) : null")
        }
        FieldType::OptionalVecStruct(name) => {
            format!("r.readU8() == 1 ? r.readArray(r.readU32(), () => decode{name}(r)) : null")
        }
        FieldType::VecStruct(name) => format!("r.readArray(r.readU32(), () => decode{name}(r))"),
        FieldType::TaggedEnum(name) => format!("decode{name}(r)"),
    }
}

// --- Struct codec ---

fn dart_emit_struct_codec(out: &mut String, s: &StructDef, ir: &ParsedModule) {
    let name = &s.name;
    writeln!(out, "void encode{name}(ProtocolWriter w, {name} v) {{").unwrap();
    for f in &s.fields {
        let accessor = format!("v.{}", to_camel_case(&f.name));
        dart_emit_encode_field(out, &accessor, &f.ty, ir, "  ");
    }
    out.push_str("}\n\n");

    writeln!(out, "{name} decode{name}(ProtocolReader r) {{").unwrap();
    writeln!(out, "  return {name}(").unwrap();
    for f in &s.fields {
        let dart_name = to_camel_case(&f.name);
        let expr = dart_field_decode_expr(&f.ty, ir);
        writeln!(out, "    {dart_name}: {expr},").unwrap();
    }
    out.push_str("  );\n}\n");
}

fn dart_emit_error_payload_codec(out: &mut String) {
    out.push_str(
        "void encodeErrorPayload(ProtocolWriter w, ErrorPayload v) {\n\
         \x20 w.writeU16(v.code.value);\n\
         \x20 final slug = v.code.slug;\n\
         \x20 w.writeU8(slug.length);\n\
         \x20 for (var i = 0; i < slug.length; i++) { w.writeU8(slug.codeUnitAt(i)); }\n\
         \x20 w.writeString(v.message);\n\
         \x20 w.writeU32(v.retryAfterMs);\n\
         \x20 w.writeOptionalString(v.extra);\n\
         }\n\n\
         ErrorPayload decodeErrorPayload(ProtocolReader r) {\n\
         \x20 final codeRaw = r.readU16();\n\
         \x20 final code = ErrorCode.fromValue(codeRaw);\n\
         \x20 if (code == null) throw CodecError('unknown ErrorCode: $codeRaw');\n\
         \x20 r.skip(r.readU8());\n\
         \x20 return ErrorPayload(\n\
         \x20   code: code,\n\
         \x20   message: r.readString(),\n\
         \x20   retryAfterMs: r.readU32(),\n\
         \x20   extra: r.readOptionalString(),\n\
         \x20 );\n\
         }\n",
    );
}

fn dart_emit_message_codec(out: &mut String) {
    out.push_str(
        "void encodeMessage(ProtocolWriter w, Message v) {\n\
         \x20 w.writeU32(v.id);\n\
         \x20 w.writeU32(v.chatId);\n\
         \x20 w.writeU32(v.senderId);\n\
         \x20 w.writeTimestamp(v.createdAt);\n\
         \x20 w.writeTimestamp(v.updatedAt);\n\
         \x20 w.writeU8(v.kind.value);\n\
         \x20 w.writeU16(v.flags.value);\n\
         \x20 w.writeOptionU32(v.replyToId);\n\
         \x20 w.writeString(v.content);\n\
         \x20 if (v.richContent != null) {\n\
         \x20   final lenOffset = w.reserve(4);\n\
         \x20   final blobStart = w.length;\n\
         \x20   w.writeU16(v.richContent!.length);\n\
         \x20   for (final span in v.richContent!) { encodeRichSpan(w, span); }\n\
         \x20   w.patchU32(lenOffset, w.length - blobStart);\n\
         \x20 } else {\n\
         \x20   w.writeU32(0);\n\
         \x20 }\n\
         \x20 w.writeOptionalString(v.extra);\n\
         }\n\n\
         Message decodeMessage(ProtocolReader r) {\n\
         \x20 final id = r.readU32();\n\
         \x20 final chatId = r.readU32();\n\
         \x20 final senderId = r.readU32();\n\
         \x20 final createdAt = r.readTimestamp();\n\
         \x20 final updatedAt = r.readTimestamp();\n\
         \x20 final kind = MessageKind.fromValue(r.readU8())!;\n\
         \x20 final flags = MessageFlags(r.readU16());\n\
         \x20 final replyToId = r.readOptionU32();\n\
         \x20 final content = r.readString();\n\
         \x20 final richLen = r.readU32();\n\
         \x20 List<RichSpan>? richContent;\n\
         \x20 if (richLen > 0) {\n\
         \x20   final richData = r.readBytes(richLen);\n\
         \x20   final rr = ProtocolReader(richData);\n\
         \x20   richContent = rr.readArray(rr.readU16(), () => decodeRichSpan(rr));\n\
         \x20 }\n\
         \x20 final extra = r.readOptionalString();\n\
         \x20 return Message(id: id, chatId: chatId, senderId: senderId, createdAt: createdAt, updatedAt: updatedAt, kind: kind, flags: flags, replyToId: replyToId, content: content, richContent: richContent, extra: extra);\n\
         }\n",
    );
}

fn dart_emit_message_batch_codec(out: &mut String) {
    out.push_str(
        "void encodeMessageBatch(ProtocolWriter w, MessageBatch v) {\n\
         \x20 w.writeU8(v.hasMore ? 1 : 0);\n\
         \x20 w.writeU32(v.messages.length);\n\
         \x20 for (final msg in v.messages) { encodeMessage(w, msg); }\n\
         }\n\n\
         MessageBatch decodeMessageBatch(ProtocolReader r) {\n\
         \x20 final hasMore = r.readU8() != 0;\n\
         \x20 final messages = r.readArray(r.readU32(), () => decodeMessage(r));\n\
         \x20 return MessageBatch(messages: messages, hasMore: hasMore);\n\
         }\n",
    );
}

// --- Tagged enum codec ---

fn dart_emit_tagged_enum_codec(out: &mut String, t: &TaggedEnumDef, ir: &ParsedModule) {
    let name = &t.name;
    let base = name.strip_suffix("Payload").unwrap_or(name);

    if name == "LoadMessagesPayload" {
        dart_emit_load_messages_codec(out, t, ir);
        return;
    }

    // encode
    writeln!(out, "void encode{name}(ProtocolWriter w, {name} v) {{").unwrap();
    out.push_str("  switch (v) {\n");
    for (i, v) in t.variants.iter().enumerate() {
        let class_name = format!("{base}{}", v.name);
        let has_fields = !matches!(&v.kind, VariantKind::Unit);
        if has_fields {
            writeln!(out, "    case {class_name} p:").unwrap();
        } else {
            writeln!(out, "    case {class_name}():").unwrap();
        }
        writeln!(out, "      w.writeU8({i});").unwrap();
        match &v.kind {
            VariantKind::Unit => {}
            VariantKind::Tuple(types) => {
                for (j, ty) in types.iter().enumerate() {
                    let field = if types.len() == 1 {
                        format!("p.{}", tuple_field_name(ty))
                    } else {
                        format!("p.value{}", j + 1)
                    };
                    dart_emit_encode_field(out, &field, ty, ir, "      ");
                }
            }
            VariantKind::Struct(fields) => {
                for f in fields {
                    let accessor = format!("p.{}", to_camel_case(&f.name));
                    dart_emit_encode_field(out, &accessor, &f.ty, ir, "      ");
                }
            }
        }
    }
    out.push_str("  }\n}\n\n");

    // decode
    writeln!(out, "{name} decode{name}(ProtocolReader r) {{").unwrap();
    out.push_str("  final d = r.readU8();\n  return switch (d) {\n");
    for (i, v) in t.variants.iter().enumerate() {
        let class_name = format!("{base}{}", v.name);
        match &v.kind {
            VariantKind::Unit => {
                writeln!(out, "    {i} => {class_name}(),").unwrap();
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
                        format!("{}: {}", fname, dart_field_decode_expr(ty, ir))
                    })
                    .collect();
                writeln!(out, "    {i} => {class_name}({}),", fields.join(", ")).unwrap();
            }
            VariantKind::Struct(fields) => {
                let parts: Vec<String> = fields
                    .iter()
                    .map(|f| format!("{}: {}", to_camel_case(&f.name), dart_field_decode_expr(&f.ty, ir)))
                    .collect();
                writeln!(out, "    {i} => {class_name}({}),", parts.join(", ")).unwrap();
            }
        }
    }
    writeln!(out, "    _ => throw CodecError('unknown {name} discriminant: $d'),").unwrap();
    out.push_str("  };\n}\n");
}

fn dart_emit_load_messages_codec(out: &mut String, t: &TaggedEnumDef, ir: &ParsedModule) {
    let name = &t.name;
    let base = name.strip_suffix("Payload").unwrap_or(name);

    // encode
    writeln!(out, "void encode{name}(ProtocolWriter w, {name} v) {{").unwrap();
    out.push_str("  switch (v) {\n");
    for (i, v) in t.variants.iter().enumerate() {
        let class_name = format!("{base}{}", v.name);
        writeln!(out, "    case {class_name} p:").unwrap();
        out.push_str("      w.writeU32(p.chatId);\n");
        writeln!(out, "      w.writeU8({i});").unwrap();
        if let VariantKind::Struct(fields) = &v.kind {
            for f in fields {
                if f.name == "chat_id" {
                    continue;
                }
                let accessor = format!("p.{}", to_camel_case(&f.name));
                dart_emit_encode_field(out, &accessor, &f.ty, ir, "      ");
            }
        }
    }
    out.push_str("  }\n}\n\n");

    // decode
    writeln!(out, "{name} decode{name}(ProtocolReader r) {{").unwrap();
    out.push_str("  final chatId = r.readU32();\n  final d = r.readU8();\n  return switch (d) {\n");
    for (i, v) in t.variants.iter().enumerate() {
        let class_name = format!("{base}{}", v.name);
        if let VariantKind::Struct(fields) = &v.kind {
            let parts: Vec<String> = fields
                .iter()
                .map(|f| {
                    if f.name == "chat_id" {
                        format!("{}: chatId", to_camel_case(&f.name))
                    } else {
                        format!("{}: {}", to_camel_case(&f.name), dart_field_decode_expr(&f.ty, ir))
                    }
                })
                .collect();
            writeln!(out, "    {i} => {class_name}({}),", parts.join(", ")).unwrap();
        }
    }
    writeln!(out, "    _ => throw CodecError('unknown {name} mode: $d'),").unwrap();
    out.push_str("  };\n}\n");
}

// --- Codecs file ---

fn emit_codecs_dart(ir: &ParsedModule) -> String {
    let mut out = String::from(HEADER);
    // Import barrel (provides all types + codec utilities)
    out.push_str("\nimport 'package:chat_core/chat_core.dart';\n\n");

    // Struct codecs
    for s in &ir.structs {
        if SPECIAL_STRUCTS.contains(&s.name.as_str()) {
            continue;
        }
        dart_emit_struct_codec(&mut out, s, ir);
        out.push('\n');
    }
    if ir.structs.iter().any(|s| s.name == "ErrorPayload") {
        dart_emit_error_payload_codec(&mut out);
        out.push('\n');
    }
    if ir.structs.iter().any(|s| s.name == "Message") {
        dart_emit_message_codec(&mut out);
        out.push('\n');
    }
    if ir.structs.iter().any(|s| s.name == "MessageBatch") {
        dart_emit_message_batch_codec(&mut out);
        out.push('\n');
    }

    // Tagged enum codecs
    for t in &ir.tagged_enums {
        if SKIP_TAGGED_ENUMS.contains(&t.name.as_str()) {
            continue;
        }
        dart_emit_tagged_enum_codec(&mut out, t, ir);
        out.push('\n');
    }

    out
}

// --- Frame codec ---

fn emit_frame_codec_dart(ir: &ParsedModule) -> String {
    let mut out = String::from(HEADER);
    out.push_str("\nimport 'dart:typed_data';\n\n");
    out.push_str("import 'package:meta/meta.dart';\n\n");
    out.push_str("import 'package:chat_core/chat_core.dart';\n\n");

    // FrameHeader class
    out.push_str(
        "@immutable\nclass FrameHeader {\n\
         \x20 const FrameHeader({required this.kind, required this.seq, required this.eventSeq});\n\
         \x20 final FrameKind kind;\n\
         \x20 final int seq;\n\
         \x20 final int eventSeq;\n\
         }\n\n",
    );

    // FramePayload sealed class + variants
    out.push_str("@immutable\nsealed class FramePayload {\n  const FramePayload();\n}\n\n");
    for &(kind_name, flag, payload_type) in FRAME_DISPATCH {
        let class_name = format!("FramePayload{kind_name}");
        match flag {
            0 => writeln!(out, "class {class_name} extends FramePayload {{\n  const {class_name}();\n}}").unwrap(),
            4 => writeln!(out, "class {class_name} extends FramePayload {{\n  const {class_name}(this.data);\n  final Uint8List data;\n}}").unwrap(),
            3 => writeln!(out, "class {class_name} extends FramePayload {{\n  const {class_name}(this.data);\n  final List<{payload_type}> data;\n}}").unwrap(),
            _ => writeln!(out, "class {class_name} extends FramePayload {{\n  const {class_name}(this.data);\n  final {payload_type} data;\n}}").unwrap(),
        }
        out.push('\n');
    }

    // Frame class
    out.push_str(
        "@immutable\nclass Frame {\n\
         \x20 const Frame({required this.seq, required this.eventSeq, required this.payload});\n\
         \x20 final int seq;\n\
         \x20 final int eventSeq;\n\
         \x20 final FramePayload payload;\n\
         }\n\n",
    );

    // encodeFrameHeader / decodeFrameHeader
    out.push_str(
        "void encodeFrameHeader(ProtocolWriter w, FrameHeader h) {\n\
         \x20 w.writeU8(h.kind.value);\n\
         \x20 w.writeU32(h.seq);\n\
         \x20 w.writeU32(h.eventSeq);\n\
         }\n\n\
         FrameHeader decodeFrameHeader(ProtocolReader r) {\n\
         \x20 final kindByte = r.readU8();\n\
         \x20 final kind = FrameKind.fromValue(kindByte);\n\
         \x20 if (kind == null) throw CodecError('unknown FrameKind: $kindByte');\n\
         \x20 return FrameHeader(kind: kind, seq: r.readU32(), eventSeq: r.readU32());\n\
         }\n\n",
    );

    // FrameKind values from IR
    let kind_values: std::collections::HashMap<&str, u64> = ir
        .enums
        .iter()
        .find(|e| e.name == "FrameKind")
        .map(|e| e.variants.iter().map(|v| (v.name.as_str(), v.discriminant)).collect())
        .unwrap_or_default();

    // _framePayloadKind
    out.push_str("FrameKind _framePayloadKind(FramePayload p) {\n  return switch (p) {\n");
    for &(kind_name, _, _) in FRAME_DISPATCH {
        let class_name = format!("FramePayload{kind_name}");
        let dart_variant = to_lower_camel(kind_name);
        writeln!(out, "    {class_name}() => FrameKind.{dart_variant},").unwrap();
    }
    out.push_str("  };\n}\n\n");

    // encodeFrame
    out.push_str("void encodeFrame(ProtocolWriter w, Frame frame) {\n");
    out.push_str("  final kind = _framePayloadKind(frame.payload);\n");
    out.push_str("  w.writeU8(kind.value);\n  w.writeU32(frame.seq);\n  w.writeU32(frame.eventSeq);\n");
    out.push_str("  switch (frame.payload) {\n");
    for &(kind_name, flag, payload_type) in FRAME_DISPATCH {
        let class_name = format!("FramePayload{kind_name}");
        match flag {
            0 => writeln!(out, "    case {class_name}(): break;").unwrap(),
            4 => writeln!(out, "    case {class_name} p: w.writeRawBytes(p.data);").unwrap(),
            3 => {
                let enc = format!("encode{payload_type}");
                writeln!(
                    out,
                    "    case {class_name} p: w.writeU16(p.data.length); for (final e in p.data) {{ {enc}(w, e); }}"
                )
                .unwrap();
            }
            _ => {
                let enc = format!("encode{payload_type}");
                writeln!(out, "    case {class_name} p: {enc}(w, p.data);").unwrap();
            }
        }
    }
    out.push_str("  }\n}\n\n");

    // decodeFrame
    out.push_str("Frame decodeFrame(ProtocolReader r) {\n");
    out.push_str("  final header = decodeFrameHeader(r);\n  late FramePayload payload;\n");
    out.push_str("  switch (header.kind.value) {\n");
    for &(kind_name, flag, payload_type) in FRAME_DISPATCH {
        let class_name = format!("FramePayload{kind_name}");
        if let Some(&val) = kind_values.get(kind_name) {
            match flag {
                0 => writeln!(out, "    case {val}: payload = {class_name}();").unwrap(),
                4 => writeln!(
                    out,
                    "    case {val}: payload = {class_name}(r.remaining > 0 ? r.readBytes(r.remaining) : Uint8List(0));"
                )
                .unwrap(),
                3 => {
                    let dec = format!("decode{payload_type}");
                    writeln!(
                        out,
                        "    case {val}: payload = {class_name}(r.readArray(r.readU16(), () => {dec}(r)));"
                    )
                    .unwrap();
                }
                _ => {
                    let dec = format!("decode{payload_type}");
                    writeln!(out, "    case {val}: payload = {class_name}({dec}(r));").unwrap();
                }
            }
        }
    }
    out.push_str("    default: throw CodecError('unhandled FrameKind: ${header.kind.value}');\n");
    out.push_str("  }\n  return Frame(seq: header.seq, eventSeq: header.eventSeq, payload: payload);\n}\n");

    out
}

// --- Static templates ---

fn emit_codec_error_dart() -> String {
    format!(
        "{HEADER}\n\
         class CodecError implements Exception {{\n\
         \x20 const CodecError(this.message);\n\
         \x20 final String message;\n\
         \x20 @override\n\
         \x20 String toString() => 'CodecError: $message';\n\
         }}\n"
    )
}

fn emit_reader_dart() -> String {
    format!(
        "{HEADER}\n\
import 'dart:convert';\n\
import 'dart:typed_data';\n\
\n\
import 'error.dart';\n\
\n\
class ProtocolReader {{\n\
  ProtocolReader(Uint8List data, [this._pos = 0])\n\
      : _data = ByteData.sublistView(data),\n\
        _bytes = data;\n\
\n\
  final ByteData _data;\n\
  final Uint8List _bytes;\n\
  int _pos;\n\
\n\
  int get remaining => _data.lengthInBytes - _pos;\n\
\n\
  void ensureRemaining(int n) {{\n\
    if (remaining < n) throw CodecError('truncated: need $n bytes but only $remaining remain');\n\
  }}\n\
\n\
  int readU8() {{ ensureRemaining(1); return _data.getUint8(_pos++); }}\n\
\n\
  int readU16() {{\n\
    ensureRemaining(2);\n\
    final v = _data.getUint16(_pos, Endian.little);\n\
    _pos += 2;\n\
    return v;\n\
  }}\n\
\n\
  int readU32() {{\n\
    ensureRemaining(4);\n\
    final v = _data.getUint32(_pos, Endian.little);\n\
    _pos += 4;\n\
    return v;\n\
  }}\n\
\n\
  int readI64() {{\n\
    ensureRemaining(8);\n\
    final v = _data.getInt64(_pos, Endian.little);\n\
    _pos += 8;\n\
    return v;\n\
  }}\n\
\n\
  int readTimestamp() {{\n\
    final v = readI64();\n\
    if (v < 0 || v > 2199023255551) throw CodecError('timestamp out of range: $v');\n\
    return v;\n\
  }}\n\
\n\
  String readString() {{\n\
    final len = readU32();\n\
    if (len == 0) return '';\n\
    ensureRemaining(len);\n\
    final s = _decodeUtf8(len);\n\
    _pos += len;\n\
    return s;\n\
  }}\n\
\n\
  String? readOptionalString() {{\n\
    final len = readU32();\n\
    if (len == 0) return null;\n\
    ensureRemaining(len);\n\
    final s = _decodeUtf8(len);\n\
    _pos += len;\n\
    return s;\n\
  }}\n\
\n\
  /// Decode [len] bytes at current position as UTF-8.\n\
  /// Fast path: if all bytes are ASCII, build string directly.\n\
  String _decodeUtf8(int len) {{\n\
    bool ascii = true;\n\
    for (var i = 0; i < len; i++) {{\n\
      if (_bytes[_pos + i] > 0x7F) {{ ascii = false; break; }}\n\
    }}\n\
    if (ascii) {{\n\
      return String.fromCharCodes(_bytes, _pos, _pos + len);\n\
    }}\n\
    return utf8.decode(Uint8List.sublistView(_bytes, _pos, _pos + len));\n\
  }}\n\
\n\
  Uint8List? readOptionalBytes() {{\n\
    final len = readU32();\n\
    if (len == 0) return null;\n\
    ensureRemaining(len);\n\
    final out = Uint8List.sublistView(_bytes, _pos, _pos + len);\n\
    _pos += len;\n\
    return out;\n\
  }}\n\
\n\
  String readUuid() {{\n\
    ensureRemaining(16);\n\
    final hex = StringBuffer();\n\
    for (var i = 0; i < 16; i++) {{{{ hex.write(_bytes[_pos + i].toRadixString(16).padLeft(2, '0')); }}}}\n\
    _pos += 16;\n\
    final h = hex.toString();\n\
    return '${{h.substring(0, 8)}}-${{h.substring(8, 12)}}-${{h.substring(12, 16)}}-${{h.substring(16, 20)}}-${{h.substring(20)}}';\n\
  }}\n\
\n\
  int? readOptionU32() {{\n\
    final flag = readU8();\n\
    if (flag == 0) return null;\n\
    if (flag == 1) return readU32();\n\
    throw CodecError('invalid Option<u32> flag: $flag');\n\
  }}\n\
\n\
  String? readUpdatableString() {{\n\
    final flag = readU8();\n\
    if (flag == 0) return null;\n\
    if (flag == 1) return readString();\n\
    throw CodecError('invalid updatable string flag: $flag');\n\
  }}\n\
\n\
  Uint8List readBytes(int n) {{\n\
    ensureRemaining(n);\n\
    final out = Uint8List.sublistView(_bytes, _pos, _pos + n);\n\
    _pos += n;\n\
    return out;\n\
  }}\n\
\n\
  List<int> readVecU32() {{\n\
    final count = readU16();\n\
    return [for (var i = 0; i < count; i++) readU32()];\n\
  }}\n\
\n\
  List<String> readVecString() {{\n\
    final count = readU16();\n\
    return [for (var i = 0; i < count; i++) readString()];\n\
  }}\n\
\n\
  List<T> readArray<T>(int count, T Function() readOne) {{\n\
    return [for (var i = 0; i < count; i++) readOne()];\n\
  }}\n\
\n\
  void skip(int n) {{ ensureRemaining(n); _pos += n; }}\n\
}}\n"
    )
}

fn emit_writer_dart() -> String {
    format!(
        "{HEADER}\n\
import 'dart:convert';\n\
import 'dart:typed_data';\n\
\n\
import 'error.dart';\n\
\n\
class ProtocolWriter {{\n\
  ProtocolWriter([int initialCapacity = 256])\n\
      : _buf = Uint8List(initialCapacity) {{\n\
    _data = ByteData.sublistView(_buf);\n\
  }}\n\
\n\
  Uint8List _buf;\n\
  late ByteData _data;\n\
  int _pos = 0;\n\
\n\
  /// Reset position to zero, reusing the existing buffer.\n\
  void reset() {{ _pos = 0; }}\n\
\n\
  /// Current number of bytes written.\n\
  int get length => _pos;\n\
\n\
  void _grow(int needed) {{\n\
    final required = _pos + needed;\n\
    if (required <= _buf.length) return;\n\
    var newLen = _buf.length * 2;\n\
    while (newLen < required) {{{{ newLen *= 2; }}}}\n\
    final next = Uint8List(newLen);\n\
    next.setAll(0, Uint8List.sublistView(_buf, 0, _pos));\n\
    _buf = next;\n\
    _data = ByteData.sublistView(next);\n\
  }}\n\
\n\
  void writeU8(int v) {{ _grow(1); _data.setUint8(_pos++, v); }}\n\
\n\
  void writeU16(int v) {{\n\
    _grow(2); _data.setUint16(_pos, v, Endian.little); _pos += 2;\n\
  }}\n\
\n\
  void writeU32(int v) {{\n\
    _grow(4); _data.setUint32(_pos, v, Endian.little); _pos += 4;\n\
  }}\n\
\n\
  void writeI64(int v) {{\n\
    _grow(8); _data.setInt64(_pos, v, Endian.little); _pos += 8;\n\
  }}\n\
\n\
  void writeTimestamp(int v) {{\n\
    if (v < 0 || v > 2199023255551) throw CodecError('timestamp out of range: $v');\n\
    writeI64(v);\n\
  }}\n\
\n\
  void writeString(String v) {{\n\
    if (v.isEmpty) {{ writeU32(0); return; }}\n\
    // Fast path: pure ASCII — avoid utf8.encode() allocation.\n\
    if (_isAscii(v)) {{\n\
      writeU32(v.length);\n\
      _grow(v.length);\n\
      for (var i = 0; i < v.length; i++) {{ _buf[_pos++] = v.codeUnitAt(i); }}\n\
    }} else {{\n\
      final encoded = utf8.encode(v);\n\
      writeU32(encoded.length);\n\
      _grow(encoded.length);\n\
      _buf.setAll(_pos, encoded);\n\
      _pos += encoded.length;\n\
    }}\n\
  }}\n\
\n\
  static bool _isAscii(String v) {{\n\
    for (var i = 0; i < v.length; i++) {{ if (v.codeUnitAt(i) > 0x7F) return false; }}\n\
    return true;\n\
  }}\n\
\n\
  void writeOptionalString(String? v) {{\n\
    if (v == null) {{ writeU32(0); }} else {{ writeString(v); }}\n\
  }}\n\
\n\
  void writeOptionalBytes(Uint8List? v) {{\n\
    if (v == null) {{ writeU32(0); return; }}\n\
    writeU32(v.length);\n\
    _grow(v.length);\n\
    _buf.setAll(_pos, v);\n\
    _pos += v.length;\n\
  }}\n\
\n\
  void writeUuid(String uuid) {{\n\
    _grow(16);\n\
    for (var i = 0, j = 0; i < uuid.length && j < 16; i += 2) {{\n\
      if (i < uuid.length && uuid.codeUnitAt(i) == 0x2D) {{ i++; }} // skip '-'\n\
      _buf[_pos++] = (_hexVal(uuid.codeUnitAt(i)) << 4) | _hexVal(uuid.codeUnitAt(i + 1));\n\
      j++;\n\
    }}\n\
  }}\n\
\n\
  void writeOptionU32(int? v) {{\n\
    if (v == null) {{ writeU8(0); }} else {{ writeU8(1); writeU32(v); }}\n\
  }}\n\
\n\
  void writeUpdatableString(String? v) {{\n\
    if (v == null) {{ writeU8(0); }} else {{ writeU8(1); writeString(v); }}\n\
  }}\n\
\n\
  void writeRawBytes(Uint8List data) {{\n\
    _grow(data.length);\n\
    _buf.setAll(_pos, data);\n\
    _pos += data.length;\n\
  }}\n\
\n\
  /// Patch a u32 at a previously written position.\n\
  void patchU32(int offset, int v) {{\n\
    _data.setUint32(offset, v, Endian.little);\n\
  }}\n\
\n\
  /// Reserve [n] bytes and return the offset. Caller fills them later.\n\
  int reserve(int n) {{ _grow(n); final o = _pos; _pos += n; return o; }}\n\
\n\
  /// Return a copy of the written bytes.\n\
  Uint8List toBytes() => _buf.sublist(0, _pos);\n\
\n\
  /// Return a view of the written bytes. Valid only until the next write/reset.\n\
  Uint8List toBytesView() => Uint8List.sublistView(_buf, 0, _pos);\n\
}}\n\
\n\
int _hexVal(int c) {{\n\
  if (c >= 0x30 && c <= 0x39) return c - 0x30;       // '0'-'9'\n\
  if (c >= 0x61 && c <= 0x66) return c - 0x61 + 10;   // 'a'-'f'\n\
  if (c >= 0x41 && c <= 0x46) return c - 0x41 + 10;   // 'A'-'F'\n\
  throw CodecError('invalid hex char: ${{String.fromCharCode(c)}}');\n\
}}\n"
    )
}

// ===========================================================================

fn emit_util() -> String {
    let mut out = String::from(HEADER);
    out.push_str(
        "\n/// Deep equality check for lists.\nbool listEquals<T>(List<T>? a, List<T>? b) {\n  \
         if (identical(a, b)) return true;\n  \
         if (a == null || b == null) return false;\n  \
         if (a.length != b.length) return false;\n  \
         for (var i = 0; i < a.length; i++) {\n    \
         if (a[i] != b[i]) return false;\n  \
         }\n  \
         return true;\n\
         }\n",
    );
    out
}

// ---------------------------------------------------------------------------
// Phase 5 — Dart test generation
// ---------------------------------------------------------------------------

/// Generate a Dart expression for a test fixture value of the given field type.
fn dart_test_value(ty: &FieldType, ir: &ParsedModule) -> String {
    match ty {
        FieldType::U8 => "42".into(),
        FieldType::U16 => "1000".into(),
        FieldType::U32 => "100000".into(),
        FieldType::I64 => "1234567890".into(),
        FieldType::Bool => "true".into(),
        FieldType::String => "'hello'".into(),
        FieldType::OptionalString => "'test'".into(),
        FieldType::UpdatableString => "'updated'".into(),
        FieldType::Uuid => "'550e8400-e29b-41d4-a716-446655440000'".into(),
        FieldType::OptionalU32 => "7".into(),
        FieldType::VecU32 => "[1, 2, 3]".into(),
        FieldType::VecU8 => "Uint8List.fromList([1, 2, 3])".into(),
        FieldType::OptionalBytes => "Uint8List.fromList([1, 2])".into(),
        FieldType::VecString => "['a', 'b']".into(),
        FieldType::Enum(name) => {
            if let Some(e) = ir.enums.iter().find(|e| e.name == *name) {
                format!("{}.{}", name, to_lower_camel(&e.variants[0].name))
            } else {
                format!("{}.values.first", name)
            }
        }
        FieldType::Bitflags(name) => {
            if let Some(b) = ir.bitflags.iter().find(|b| b.name == *name) {
                format!("{}.{}", name, to_camel_case(&b.flags[0].name))
            } else {
                format!("{}(1)", name)
            }
        }
        FieldType::OptionalBitflags(name) => {
            if let Some(b) = ir.bitflags.iter().find(|b| b.name == *name) {
                format!("{}.{}", name, to_camel_case(&b.flags[0].name))
            } else {
                format!("{}(1)", name)
            }
        }
        FieldType::Struct(name) => dart_struct_fixture(name, ir),
        FieldType::OptionalStruct(name) => dart_struct_fixture(name, ir),
        FieldType::VecStruct(name) => format!("[{}]", dart_struct_fixture(name, ir)),
        FieldType::OptionalVecStruct(name) => format!("[{}]", dart_struct_fixture(name, ir)),
        FieldType::TaggedEnum(name) => dart_tagged_enum_fixture(name, ir),
    }
}

/// Generate an inline Dart struct fixture expression.
fn dart_struct_fixture(name: &str, ir: &ParsedModule) -> String {
    let s = ir.structs.iter().find(|s| s.name == name);
    let s = match s {
        Some(s) => s,
        None => return format!("const {}()", name),
    };

    let mut parts = Vec::new();
    for f in &s.fields {
        let dart_name = to_camel_case(&f.name);
        let value = dart_test_value(&f.ty, ir);
        parts.push(format!("{dart_name}: {value}"));
    }
    if parts.is_empty() {
        format!("const {}()", name)
    } else {
        format!("{}({})", name, parts.join(", "))
    }
}

/// Generate the first variant of a tagged enum as a Dart fixture expression.
fn dart_tagged_enum_fixture(name: &str, ir: &ParsedModule) -> String {
    let t = ir.tagged_enums.iter().find(|t| t.name == name);
    let t = match t {
        Some(t) => t,
        None => return format!("const {}()", name),
    };

    // Find the first variant with data (or fallback to unit)
    let base = name.strip_suffix("Payload").unwrap_or(name);
    let v = &t.variants[0];
    let class_name = format!("{base}{}", v.name);

    match &v.kind {
        VariantKind::Unit => format!("const {class_name}()"),
        VariantKind::Tuple(types) => {
            let fields: Vec<String> = types
                .iter()
                .enumerate()
                .map(|(i, ty)| {
                    let name = if types.len() == 1 {
                        tuple_field_name(ty)
                    } else {
                        format!("value{}", i + 1)
                    };
                    let val = dart_test_value(ty, ir);
                    format!("{name}: {val}")
                })
                .collect();
            format!("{class_name}({})", fields.join(", "))
        }
        VariantKind::Struct(fields) => {
            let parts: Vec<String> = fields
                .iter()
                .map(|f| {
                    let dart_name = to_camel_case(&f.name);
                    let val = dart_test_value(&f.ty, ir);
                    format!("{dart_name}: {val}")
                })
                .collect();
            if parts.is_empty() {
                format!("const {class_name}()")
            } else {
                format!("{class_name}({})", parts.join(", "))
            }
        }
    }
}

/// Generate all variant fixtures for a tagged enum (for codec tests).
fn dart_tagged_enum_all_variants(name: &str, ir: &ParsedModule) -> Vec<(String, String)> {
    let t = match ir.tagged_enums.iter().find(|t| t.name == name) {
        Some(t) => t,
        None => return vec![],
    };

    let base = name.strip_suffix("Payload").unwrap_or(name);
    t.variants
        .iter()
        .map(|v| {
            let class_name = format!("{base}{}", v.name);
            let expr = match &v.kind {
                VariantKind::Unit => format!("const {class_name}()"),
                VariantKind::Tuple(types) => {
                    let fields: Vec<String> = types
                        .iter()
                        .enumerate()
                        .map(|(i, ty)| {
                            let name = if types.len() == 1 {
                                tuple_field_name(ty)
                            } else {
                                format!("value{}", i + 1)
                            };
                            let val = dart_test_value(ty, ir);
                            format!("{name}: {val}")
                        })
                        .collect();
                    format!("{class_name}({})", fields.join(", "))
                }
                VariantKind::Struct(fields) => {
                    let parts: Vec<String> = fields
                        .iter()
                        .map(|f| {
                            let dart_name = to_camel_case(&f.name);
                            let val = dart_test_value(&f.ty, ir);
                            format!("{dart_name}: {val}")
                        })
                        .collect();
                    if parts.is_empty() {
                        format!("const {class_name}()")
                    } else {
                        format!("{class_name}({})", parts.join(", "))
                    }
                }
            };
            (v.name.clone(), expr)
        })
        .collect()
}

/// Generate a Dart struct fixture with all nullable fields set to null.
fn dart_struct_fixture_nulls(name: &str, ir: &ParsedModule) -> Option<String> {
    let s = ir.structs.iter().find(|s| s.name == name)?;
    if !s.fields.iter().any(|f| is_nullable(&f.ty)) {
        return None;
    }

    let mut parts = Vec::new();
    for f in &s.fields {
        let dart_name = to_camel_case(&f.name);
        if is_nullable(&f.ty) {
            // Leave out nullable fields — they default to null
            continue;
        }
        let value = match &f.ty {
            FieldType::String => "''".into(),
            FieldType::VecU32 => "[]".into(),
            FieldType::VecU8 => "Uint8List(0)".into(),
            FieldType::VecString => "[]".into(),
            FieldType::VecStruct(_) => "[]".into(),
            _ => dart_test_value(&f.ty, ir),
        };
        parts.push(format!("{dart_name}: {value}"));
    }
    Some(format!("{}({})", name, parts.join(", ")))
}

// ---------------------------------------------------------------------------
// 5.1 — Dart types test
// ---------------------------------------------------------------------------

fn emit_types_test_dart(ir: &ParsedModule) -> String {
    let mut out = String::from(HEADER);
    out.push_str("\nimport 'dart:typed_data';\n\nimport 'package:test/test.dart';\nimport 'package:chat_core/chat_core.dart';\n\n");
    out.push_str("void main() {\n");

    // Enum tests
    for e in &ir.enums {
        writeln!(out, "  group('{}', () {{", e.name).unwrap();
        // fromValue roundtrip for all variants
        writeln!(out, "    test('fromValue roundtrip', () {{").unwrap();
        for v in &e.variants {
            let variant = to_lower_camel(&v.name);
            writeln!(
                out,
                "      expect({}.fromValue({}.{variant}.value), {}.{variant});",
                e.name, e.name, e.name
            )
            .unwrap();
        }
        out.push_str("    });\n");
        // fromValue returns null for invalid
        writeln!(out, "    test('fromValue returns null for invalid', () {{").unwrap();
        writeln!(out, "      expect({}.fromValue(255), isNull);", e.name).unwrap();
        out.push_str("    });\n");
        out.push_str("  });\n\n");
    }

    // Bitflags tests
    for b in &ir.bitflags {
        writeln!(out, "  group('{}', () {{", b.name).unwrap();
        let first = to_camel_case(&b.flags[0].name);
        let second = if b.flags.len() > 1 {
            to_camel_case(&b.flags[1].name)
        } else {
            first.clone()
        };

        writeln!(out, "    test('contains', () {{").unwrap();
        writeln!(out, "      final flags = {}.{first};", b.name).unwrap();
        writeln!(out, "      expect(flags.contains({}.{first}), isTrue);", b.name).unwrap();
        if b.flags.len() > 1 {
            writeln!(out, "      expect(flags.contains({}.{second}), isFalse);", b.name).unwrap();
        }
        out.push_str("    });\n");

        writeln!(out, "    test('add and remove', () {{").unwrap();
        writeln!(out, "      var flags = {}.{first};", b.name).unwrap();
        writeln!(out, "      flags = flags.add({}.{second});", b.name).unwrap();
        writeln!(out, "      expect(flags.contains({}.{first}), isTrue);", b.name).unwrap();
        writeln!(out, "      expect(flags.contains({}.{second}), isTrue);", b.name).unwrap();
        writeln!(out, "      flags = flags.remove({}.{first});", b.name).unwrap();
        writeln!(out, "      expect(flags.contains({}.{first}), isFalse);", b.name).unwrap();
        writeln!(out, "      expect(flags.contains({}.{second}), isTrue);", b.name).unwrap();
        out.push_str("    });\n");

        writeln!(out, "    test('toggle', () {{").unwrap();
        writeln!(out, "      var flags = {}.{first};", b.name).unwrap();
        writeln!(out, "      flags = flags.toggle({}.{first});", b.name).unwrap();
        writeln!(out, "      expect(flags.isEmpty, isTrue);",).unwrap();
        writeln!(out, "      flags = flags.toggle({}.{first});", b.name).unwrap();
        writeln!(out, "      expect(flags.contains({}.{first}), isTrue);", b.name).unwrap();
        out.push_str("    });\n");

        writeln!(out, "    test('isEmpty', () {{").unwrap();
        writeln!(out, "      expect(const {}(0).isEmpty, isTrue);", b.name).unwrap();
        writeln!(out, "      expect({}.{first}.isEmpty, isFalse);", b.name).unwrap();
        writeln!(out, "      expect({}.{first}.isNotEmpty, isTrue);", b.name).unwrap();
        out.push_str("    });\n");

        out.push_str("  });\n\n");
    }

    // Struct equality tests
    for s in &ir.structs {
        writeln!(out, "  group('{}', () {{", s.name).unwrap();
        let fixture = dart_struct_fixture(&s.name, ir);
        writeln!(out, "    test('equality', () {{").unwrap();
        writeln!(out, "      final a = {};", fixture).unwrap();
        writeln!(out, "      final b = {};", fixture).unwrap();
        out.push_str("      expect(a, equals(b));\n");
        out.push_str("      expect(a.hashCode, equals(b.hashCode));\n");
        out.push_str("    });\n");

        out.push_str("  });\n\n");
    }

    // Tagged enum equality tests
    for t in &ir.tagged_enums {
        if SKIP_TAGGED_ENUMS.contains(&t.name.as_str()) {
            continue;
        }
        writeln!(out, "  group('{}', () {{", t.name).unwrap();
        let variants = dart_tagged_enum_all_variants(&t.name, ir);
        for (vname, expr) in &variants {
            writeln!(out, "    test('{vname} equality', () {{").unwrap();
            writeln!(out, "      final a = {};", expr).unwrap();
            writeln!(out, "      final b = {};", expr).unwrap();
            out.push_str("      expect(a, equals(b));\n");
            out.push_str("      expect(a.hashCode, equals(b.hashCode));\n");
            out.push_str("    });\n");
        }
        out.push_str("  });\n\n");
    }

    out.push_str("}\n");
    out
}

// ---------------------------------------------------------------------------
// 5.2 — Dart codec test
// ---------------------------------------------------------------------------

fn emit_codec_test_dart(ir: &ParsedModule) -> String {
    let mut out = String::from(HEADER);
    out.push_str("\nimport 'dart:typed_data';\n\nimport 'package:test/test.dart';\nimport 'package:chat_core/chat_core.dart';\n\n");
    out.push_str("void main() {\n");

    // Struct codec roundtrip tests
    for s in &ir.structs {
        writeln!(out, "  group('{} codec', () {{", s.name).unwrap();
        let fixture = dart_struct_fixture(&s.name, ir);
        writeln!(out, "    test('roundtrip', () {{").unwrap();
        writeln!(out, "      final original = {};", fixture).unwrap();
        out.push_str("      final w = ProtocolWriter();\n");
        writeln!(out, "      encode{}(w, original);", s.name).unwrap();
        writeln!(
            out,
            "      final decoded = decode{}(ProtocolReader(w.toBytes()));",
            s.name
        )
        .unwrap();
        out.push_str("      expect(decoded, equals(original));\n");
        out.push_str("    });\n");

        // Null edge case if struct has nullable fields
        if let Some(null_fixture) = dart_struct_fixture_nulls(&s.name, ir) {
            writeln!(out, "    test('roundtrip with nulls', () {{").unwrap();
            writeln!(out, "      final original = {};", null_fixture).unwrap();
            out.push_str("      final w = ProtocolWriter();\n");
            writeln!(out, "      encode{}(w, original);", s.name).unwrap();
            writeln!(
                out,
                "      final decoded = decode{}(ProtocolReader(w.toBytes()));",
                s.name
            )
            .unwrap();
            out.push_str("      expect(decoded, equals(original));\n");
            out.push_str("    });\n");
        }

        out.push_str("  });\n\n");
    }

    // Tagged enum codec roundtrip tests
    for t in &ir.tagged_enums {
        if SKIP_TAGGED_ENUMS.contains(&t.name.as_str()) {
            continue;
        }
        writeln!(out, "  group('{} codec', () {{", t.name).unwrap();
        let variants = dart_tagged_enum_all_variants(&t.name, ir);
        for (vname, expr) in &variants {
            writeln!(out, "    test('{vname} roundtrip', () {{").unwrap();
            writeln!(out, "      final original = {};", expr).unwrap();
            out.push_str("      final w = ProtocolWriter();\n");
            writeln!(out, "      encode{}(w, original);", t.name).unwrap();
            writeln!(
                out,
                "      final decoded = decode{}(ProtocolReader(w.toBytes()));",
                t.name
            )
            .unwrap();
            out.push_str("      expect(decoded, equals(original));\n");
            out.push_str("    });\n");
        }
        out.push_str("  });\n\n");
    }

    // Frame header roundtrip
    out.push_str("  group('FrameHeader codec', () {\n");
    out.push_str("    test('roundtrip', () {\n");
    out.push_str("      final header = FrameHeader(kind: FrameKind.hello, seq: 42, eventSeq: 7);\n");
    out.push_str("      final w = ProtocolWriter();\n");
    out.push_str("      encodeFrameHeader(w, header);\n");
    out.push_str("      final decoded = decodeFrameHeader(ProtocolReader(w.toBytes()));\n");
    out.push_str("      expect(decoded.kind, equals(header.kind));\n");
    out.push_str("      expect(decoded.seq, equals(header.seq));\n");
    out.push_str("      expect(decoded.eventSeq, equals(header.eventSeq));\n");
    out.push_str("    });\n");
    out.push_str("  });\n\n");

    // Frame roundtrip tests for representative kinds
    out.push_str("  group('Frame codec', () {\n");

    // Pick representative frame kinds: one with no payload, one struct, one tagged enum, one vec struct
    out.push_str("    test('Ping frame roundtrip (no payload)', () {\n");
    out.push_str("      final frame = Frame(seq: 1, eventSeq: 0, payload: const FramePayloadPing());\n");
    out.push_str("      final w = ProtocolWriter();\n");
    out.push_str("      encodeFrame(w, frame);\n");
    out.push_str("      final decoded = decodeFrame(ProtocolReader(w.toBytes()));\n");
    out.push_str("      expect(decoded.seq, equals(1));\n");
    out.push_str("      expect(decoded.eventSeq, equals(0));\n");
    out.push_str("      expect(decoded.payload, isA<FramePayloadPing>());\n");
    out.push_str("    });\n\n");

    // Struct payload frame
    out.push_str("    test('DeleteMessage frame roundtrip (struct payload)', () {\n");
    out.push_str("      final payload = FramePayloadDeleteMessage(DeleteMessagePayload(chatId: 1, messageId: 2));\n");
    out.push_str("      final frame = Frame(seq: 5, eventSeq: 3, payload: payload);\n");
    out.push_str("      final w = ProtocolWriter();\n");
    out.push_str("      encodeFrame(w, frame);\n");
    out.push_str("      final decoded = decodeFrame(ProtocolReader(w.toBytes()));\n");
    out.push_str("      expect(decoded.seq, equals(5));\n");
    out.push_str("      expect(decoded.payload, isA<FramePayloadDeleteMessage>());\n");
    out.push_str("      final p = decoded.payload as FramePayloadDeleteMessage;\n");
    out.push_str("      expect(p.data.chatId, equals(1));\n");
    out.push_str("      expect(p.data.messageId, equals(2));\n");
    out.push_str("    });\n\n");

    // Tagged enum payload frame
    out.push_str("    test('LoadChats frame roundtrip (tagged enum payload)', () {\n");
    out.push_str("      final payload = FramePayloadLoadChats(LoadChatsFirstPage(limit: 50));\n");
    out.push_str("      final frame = Frame(seq: 10, eventSeq: 0, payload: payload);\n");
    out.push_str("      final w = ProtocolWriter();\n");
    out.push_str("      encodeFrame(w, frame);\n");
    out.push_str("      final decoded = decodeFrame(ProtocolReader(w.toBytes()));\n");
    out.push_str("      expect(decoded.seq, equals(10));\n");
    out.push_str("      expect(decoded.payload, isA<FramePayloadLoadChats>());\n");
    out.push_str("      final p = (decoded.payload as FramePayloadLoadChats).data;\n");
    out.push_str("      expect(p, isA<LoadChatsFirstPage>());\n");
    out.push_str("      expect((p as LoadChatsFirstPage).limit, equals(50));\n");
    out.push_str("    });\n\n");

    // Ack payload frame
    out.push_str("    test('Ack frame roundtrip (raw bytes)', () {\n");
    out.push_str("      final payload = FramePayloadAck(Uint8List.fromList([1, 2, 3, 4]));\n");
    out.push_str("      final frame = Frame(seq: 20, eventSeq: 0, payload: payload);\n");
    out.push_str("      final w = ProtocolWriter();\n");
    out.push_str("      encodeFrame(w, frame);\n");
    out.push_str("      final decoded = decodeFrame(ProtocolReader(w.toBytes()));\n");
    out.push_str("      expect(decoded.seq, equals(20));\n");
    out.push_str("      expect(decoded.payload, isA<FramePayloadAck>());\n");
    out.push_str(
        "      expect((decoded.payload as FramePayloadAck).data, equals(Uint8List.fromList([1, 2, 3, 4])));\n",
    );
    out.push_str("    });\n");

    out.push_str("  });\n");
    out.push_str("}\n");
    out
}
