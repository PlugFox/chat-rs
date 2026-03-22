/// Dart code emitter — generates Dart package from IR.

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

/// Generate the Dart package from parsed IR.
pub(crate) fn generate(ir: &ParsedModule, output_dir: &Path) -> Result<()> {
    let types_dir = output_dir.join("lib/src/types");
    let src_dir = output_dir.join("lib/src");
    let lib_dir = output_dir.join("lib");

    fs::create_dir_all(&types_dir).context("creating types dir")?;
    fs::create_dir_all(output_dir.join("lib/src/codec")).context("creating codec dir")?;
    fs::create_dir_all(output_dir.join("test")).context("creating test dir")?;

    let mut exports: Vec<String> = Vec::new();

    // pubspec.yaml
    write_file(&output_dir.join("pubspec.yaml"), &emit_pubspec())?;

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

    // Emit _util.dart if any struct has list fields
    let needs_util = ir
        .structs
        .iter()
        .any(|s| s.fields.iter().any(|f| is_list_type(&f.ty)));
    if needs_util {
        write_file(&src_dir.join("_util.dart"), &emit_util())?;
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
        write_file(
            &src_dir.join("protocol_constants.dart"),
            &emit_constants(&ir.constants),
        )?;
        exports.push("src/protocol_constants.dart".into());
    }

    // Barrel export
    exports.sort();
    write_file(&lib_dir.join("chat_core.dart"), &emit_barrel(&exports))?;

    eprintln!(
        "Dart: {} enums, {} bitflags, {} structs, {} tagged enums, {} constants",
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
        FieldType::Enum(n)
        | FieldType::Bitflags(n)
        | FieldType::Struct(n)
        | FieldType::TaggedEnum(n) => n.clone(),
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

fn emit_import_block(
    out: &mut String,
    has_typed_data: bool,
    has_util: bool,
    type_refs: &BTreeSet<String>,
) {
    let has_relative = has_util || !type_refs.is_empty();
    if !has_typed_data && !has_relative {
        return;
    }

    out.push('\n');

    if has_typed_data {
        out.push_str("import 'dart:typed_data';\n");
        if has_relative {
            out.push('\n');
        }
    }

    if has_util {
        out.push_str("import '../_util.dart';\n");
    }
    for r in type_refs {
        writeln!(out, "import '{}.dart';", to_snake_case(r)).unwrap();
    }
}

/// Derive a field name from a tuple field type.
fn tuple_field_name(ty: &FieldType) -> String {
    match ty {
        FieldType::Enum(n)
        | FieldType::Bitflags(n)
        | FieldType::Struct(n)
        | FieldType::TaggedEnum(n) => to_lower_camel(n),
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
    writeln!(
        out,
        "  static {}? fromValue(int value) => switch (value) {{",
        e.name
    )
    .unwrap();
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
    let permanent: Vec<String> = PERMANENT_ERRORS
        .iter()
        .map(|n| to_lower_camel(n))
        .collect();
    writeln!(out, "    {} => true,", permanent.join(" || ")).unwrap();
    out.push_str("    _ => false,\n");
    out.push_str("  };\n");

    // isTransient
    out.push('\n');
    out.push_str("  /// Whether this error is transient (retry with backoff).\n");
    out.push_str("  bool get isTransient => switch (this) {\n");
    let transient: Vec<String> = TRANSIENT_ERRORS
        .iter()
        .map(|n| to_lower_camel(n))
        .collect();
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
    writeln!(
        out,
        "  static const List<{n}> values = [{}];",
        names.join(", ")
    )
    .unwrap();

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
            writeln!(
                out,
                "  final {} {};",
                dart_type(&f.ty),
                to_camel_case(&f.name)
            )
            .unwrap();
        }
    }

    // == operator
    out.push('\n');
    out.push_str("  @override\n");
    if s.fields.is_empty() {
        writeln!(
            out,
            "  bool operator ==(Object other) => identical(this, other) || other is {};",
            s.name
        )
        .unwrap();
    } else {
        out.push_str("  bool operator ==(Object other) =>\n");
        write!(out, "      identical(this, other) ||\n      other is {}", s.name).unwrap();
        for f in &s.fields {
            let cmp = eq_comparison(f);
            write!(out, " &&\n          {cmp}").unwrap();
        }
        out.push_str(";\n");
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

    // toString
    out.push('\n');
    out.push_str("  @override\n");
    let field_parts: Vec<String> = s
        .fields
        .iter()
        .map(|f| {
            let n = to_camel_case(&f.name);
            format!("{n}: ${n}")
        })
        .collect();
    writeln!(
        out,
        "  String toString() => '{}({})';",
        s.name,
        field_parts.join(", ")
    )
    .unwrap();

    out.push_str("}\n");
    out
}

fn emit_tagged_enum(t: &TaggedEnumDef) -> String {
    let mut out = String::from(HEADER);

    // Collect imports from all variant fields
    let mut type_refs = BTreeSet::new();
    let mut has_typed_data = false;

    for v in &t.variants {
        match &v.kind {
            VariantKind::Unit => {}
            VariantKind::Tuple(types) => {
                for ty in types {
                    if needs_typed_data(ty) {
                        has_typed_data = true;
                    }
                    collect_field_refs(ty, &mut type_refs);
                }
            }
            VariantKind::Struct(fields) => {
                for f in fields {
                    if needs_typed_data(&f.ty) {
                        has_typed_data = true;
                    }
                    collect_field_refs(&f.ty, &mut type_refs);
                }
            }
        }
    }

    emit_import_block(&mut out, has_typed_data, false, &type_refs);
    out.push('\n');

    // Sealed base class
    write_doc(&mut out, &t.doc, "");
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

                out.push_str("}\n");
            }
            VariantKind::Struct(fields) => {
                writeln!(out, "class {class_name} extends {} {{", t.name).unwrap();

                if fields.is_empty() {
                    writeln!(out, "  const {class_name}();").unwrap();
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
                        writeln!(
                            out,
                            "  final {} {};",
                            dart_type(&f.ty),
                            to_camel_case(&f.name)
                        )
                        .unwrap();
                    }
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
        writeln!(out, "export '{export}';").unwrap();
    }

    out
}

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
