/// Rust source parser — extracts IR from chat_protocol types.
///
/// Uses `syn` to parse Rust source files and convert them into
/// the intermediate representation defined in `ir.rs`.
///
/// Two-pass approach:
/// 1. Scan all files, collect type names → categories (enum/struct/bitflags/tagged).
/// 2. Parse fully, resolving field type cross-references using the registry.
use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result, bail};
use proc_macro2::TokenStream;
use syn::parse::Parse;
use syn::{Attribute, Expr, ExprLit, Fields, Item, Lit, Meta, PathArguments, Type};

use crate::codegen::ir::*;

/// Structs where every `Option<String>` field uses updatable-string (u8 flag) wire semantics.
const UPDATABLE_STRING_STRUCTS: &[&str] = &["UpdateChatPayload", "UpdateProfilePayload"];

/// Types to skip entirely — internal dispatch logic, not generated for clients.
const SKIP_TYPES: &[&str] = &["FramePayload", "Frame", "FrameHeader"];

// ---------------------------------------------------------------------------
// Type registry
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TypeCat {
    Enum,
    Bitflags,
    Struct,
    TaggedEnum,
}

type Registry = HashMap<String, TypeCat>;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Parse all protocol source files and return the combined IR.
pub(crate) fn parse_protocol(protocol_src: &Path) -> Result<ParsedModule> {
    let types_dir = protocol_src.join("types");
    let file_names = ["chat.rs", "message.rs", "user.rs", "error.rs", "frame.rs"];

    // Parse all source files with syn.
    let mut files = Vec::new();
    for name in &file_names {
        let path = types_dir.join(name);
        let src = std::fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        let ast = syn::parse_file(&src).with_context(|| format!("parse {}", path.display()))?;
        files.push((*name, ast));
    }

    let lib_path = protocol_src.join("lib.rs");
    let lib_src = std::fs::read_to_string(&lib_path).with_context(|| format!("read {}", lib_path.display()))?;
    let lib_ast = syn::parse_file(&lib_src).with_context(|| format!("parse {}", lib_path.display()))?;

    // Pass 1 — build type registry for cross-reference resolution.
    let mut reg = Registry::new();
    for (_, ast) in &files {
        scan_types(ast, &mut reg);
    }

    // Pass 2 — extract full definitions.
    let mut module = ParsedModule::default();
    for (fname, ast) in &files {
        extract_items(ast, &reg, &mut module).with_context(|| format!("extracting from {fname}"))?;
    }

    // Constants from lib.rs.
    extract_constants(&lib_ast, &mut module)?;

    Ok(module)
}

// ---------------------------------------------------------------------------
// Pass 1: scan type names
// ---------------------------------------------------------------------------

fn scan_types(ast: &syn::File, reg: &mut Registry) {
    for item in &ast.items {
        match item {
            Item::Enum(e) => {
                let name = e.ident.to_string();
                if SKIP_TYPES.contains(&name.as_str()) {
                    continue;
                }
                let has_data = e.variants.iter().any(|v| !matches!(v.fields, Fields::Unit));
                if has_data {
                    reg.insert(name, TypeCat::TaggedEnum);
                } else if get_repr(&e.attrs).is_some() {
                    reg.insert(name, TypeCat::Enum);
                }
            }
            Item::Struct(s) => {
                let name = s.ident.to_string();
                if !SKIP_TYPES.contains(&name.as_str()) {
                    reg.insert(name, TypeCat::Struct);
                }
            }
            Item::Macro(m) if is_bitflags(m) => {
                if let Some(name) = bitflags_name_quick(&m.mac.tokens) {
                    reg.insert(name, TypeCat::Bitflags);
                }
            }
            _ => {}
        }
    }
}

/// Extract `#[repr(u8)]` / `#[repr(u16)]` / `#[repr(u32)]` from attributes.
fn get_repr(attrs: &[Attribute]) -> Option<ReprType> {
    for attr in attrs {
        if attr.path().is_ident("repr") {
            let ident = attr.parse_args::<syn::Ident>().ok()?;
            return match ident.to_string().as_str() {
                "u8" => Some(ReprType::U8),
                "u16" => Some(ReprType::U16),
                "u32" => Some(ReprType::U32),
                _ => None,
            };
        }
    }
    None
}

fn is_bitflags(m: &syn::ItemMacro) -> bool {
    m.mac.path.is_ident("bitflags")
}

/// Quick name extraction from bitflags! token stream (pass 1 only).
fn bitflags_name_quick(tokens: &TokenStream) -> Option<String> {
    let mut iter = tokens.clone().into_iter();
    while let Some(tt) = iter.next() {
        if let proc_macro2::TokenTree::Ident(id) = &tt {
            if id == "struct" {
                if let Some(proc_macro2::TokenTree::Ident(name)) = iter.next() {
                    return Some(name.to_string());
                }
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Pass 2: extract full definitions
// ---------------------------------------------------------------------------

fn extract_items(ast: &syn::File, reg: &Registry, module: &mut ParsedModule) -> Result<()> {
    for item in &ast.items {
        match item {
            Item::Enum(e) => {
                let name = e.ident.to_string();
                if SKIP_TYPES.contains(&name.as_str()) {
                    continue;
                }
                let has_data = e.variants.iter().any(|v| !matches!(v.fields, Fields::Unit));
                if has_data {
                    module.tagged_enums.push(parse_tagged_enum(e, reg)?);
                } else if let Some(repr) = get_repr(&e.attrs) {
                    module.enums.push(parse_plain_enum(e, repr)?);
                }
            }
            Item::Struct(s) => {
                let name = s.ident.to_string();
                if SKIP_TYPES.contains(&name.as_str()) {
                    continue;
                }
                module.structs.push(parse_struct(s, reg)?);
            }
            Item::Macro(m) if is_bitflags(m) => {
                module.bitflags.push(parse_bitflags(m)?);
            }
            _ => {}
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Plain enum parser (#[repr(uN)] with unit variants)
// ---------------------------------------------------------------------------

fn parse_plain_enum(e: &syn::ItemEnum, repr: ReprType) -> Result<EnumDef> {
    let mut variants = Vec::new();
    for v in &e.variants {
        let disc = v
            .discriminant
            .as_ref()
            .map(|(_, expr)| eval_expr(expr))
            .transpose()?
            .unwrap_or_else(|| {
                // Auto-increment from previous.
                variants.last().map_or(0, |prev: &EnumVariant| prev.discriminant + 1)
            });
        variants.push(EnumVariant {
            name: v.ident.to_string(),
            doc: extract_doc(&v.attrs),
            discriminant: disc,
        });
    }
    Ok(EnumDef {
        name: e.ident.to_string(),
        doc: extract_doc(&e.attrs),
        repr,
        variants,
    })
}

// ---------------------------------------------------------------------------
// Tagged enum parser (enum with data variants)
// ---------------------------------------------------------------------------

fn parse_tagged_enum(e: &syn::ItemEnum, reg: &Registry) -> Result<TaggedEnumDef> {
    let enum_name = e.ident.to_string();
    let mut variants = Vec::new();
    for v in &e.variants {
        let kind = match &v.fields {
            Fields::Unit => VariantKind::Unit,
            Fields::Unnamed(fields) => {
                let types: Vec<FieldType> = fields
                    .unnamed
                    .iter()
                    .map(|f| resolve_type(&f.ty, reg, false))
                    .collect::<Result<_>>()?;
                VariantKind::Tuple(types)
            }
            Fields::Named(fields) => {
                let fs: Vec<Field> = fields
                    .named
                    .iter()
                    .map(|f| {
                        Ok(Field {
                            name: f.ident.as_ref().unwrap().to_string(),
                            doc: extract_doc(&f.attrs),
                            ty: resolve_type(&f.ty, reg, false)?,
                        })
                    })
                    .collect::<Result<_>>()?;
                VariantKind::Struct(fs)
            }
        };
        variants.push(TaggedVariant {
            name: v.ident.to_string(),
            doc: extract_doc(&v.attrs),
            kind,
        });
    }
    Ok(TaggedEnumDef {
        name: enum_name,
        doc: extract_doc(&e.attrs),
        variants,
    })
}

// ---------------------------------------------------------------------------
// Struct parser
// ---------------------------------------------------------------------------

fn parse_struct(s: &syn::ItemStruct, reg: &Registry) -> Result<StructDef> {
    let name = s.ident.to_string();
    let updatable = UPDATABLE_STRING_STRUCTS.contains(&name.as_str());
    let fields = match &s.fields {
        Fields::Named(named) => named
            .named
            .iter()
            .map(|f| {
                Ok(Field {
                    name: f.ident.as_ref().unwrap().to_string(),
                    doc: extract_doc(&f.attrs),
                    ty: resolve_type(&f.ty, reg, updatable)?,
                })
            })
            .collect::<Result<_>>()?,
        _ => Vec::new(),
    };
    Ok(StructDef {
        name,
        doc: extract_doc(&s.attrs),
        fields,
    })
}

// ---------------------------------------------------------------------------
// Bitflags parser (custom syn::Parse for macro content)
// ---------------------------------------------------------------------------

struct BfInput {
    attrs: Vec<Attribute>,
    name: syn::Ident,
    repr_ty: syn::Ident,
    consts: Vec<BfConst>,
}

struct BfConst {
    attrs: Vec<Attribute>,
    name: syn::Ident,
    value_expr: Expr,
}

impl Parse for BfInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let _vis: syn::Visibility = input.parse()?;
        let _: syn::Token![struct] = input.parse()?;
        let name: syn::Ident = input.parse()?;
        let _: syn::Token![:] = input.parse()?;
        let repr_ty: syn::Ident = input.parse()?;

        let content;
        syn::braced!(content in input);

        let mut consts = Vec::new();
        while !content.is_empty() {
            let entry_attrs = content.call(Attribute::parse_outer)?;
            if content.peek(syn::Token![const]) {
                let _: syn::Token![const] = content.parse()?;
                let flag_name: syn::Ident = content.parse()?;
                let _: syn::Token![=] = content.parse()?;
                let value_expr: Expr = content.parse()?;
                let _: syn::Token![;] = content.parse()?;
                consts.push(BfConst {
                    attrs: entry_attrs,
                    name: flag_name,
                    value_expr,
                });
            } else {
                // Unexpected token — skip rest to avoid infinite loop.
                break;
            }
        }

        Ok(BfInput {
            attrs,
            name,
            repr_ty,
            consts,
        })
    }
}

fn parse_bitflags(m: &syn::ItemMacro) -> Result<BitflagsDef> {
    let bf: BfInput = syn::parse2(m.mac.tokens.clone()).context("parsing bitflags! macro content")?;

    let repr = match bf.repr_ty.to_string().as_str() {
        "u8" => ReprType::U8,
        "u16" => ReprType::U16,
        "u32" => ReprType::U32,
        other => bail!("unsupported bitflags repr: {other}"),
    };

    let flags = bf
        .consts
        .iter()
        .map(|c| {
            let value = eval_expr(&c.value_expr)?;
            let value_expr = expr_to_string(&c.value_expr);
            Ok(FlagEntry {
                name: c.name.to_string(),
                doc: extract_doc(&c.attrs),
                value,
                value_expr,
            })
        })
        .collect::<Result<_>>()?;

    Ok(BitflagsDef {
        name: bf.name.to_string(),
        doc: extract_doc(&bf.attrs),
        repr,
        flags,
    })
}

// ---------------------------------------------------------------------------
// Constants parser (lib.rs)
// ---------------------------------------------------------------------------

fn extract_constants(ast: &syn::File, module: &mut ParsedModule) -> Result<()> {
    for item in &ast.items {
        if let Item::Const(c) = item {
            let ty = match &*c.ty {
                Type::Path(p) => {
                    let seg = p.path.segments.last().unwrap().ident.to_string();
                    match seg.as_str() {
                        "u8" => FieldType::U8,
                        "u16" => FieldType::U16,
                        "u32" => FieldType::U32,
                        "i64" => FieldType::I64,
                        "usize" => FieldType::U32, // All usize constants fit in u32.
                        _ => continue,
                    }
                }
                _ => continue,
            };
            module.constants.push(ConstDef {
                name: c.ident.to_string(),
                doc: extract_doc(&c.attrs),
                ty,
                value_expr: expr_to_string(&c.expr),
            });
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Field type resolution
// ---------------------------------------------------------------------------

fn resolve_type(ty: &Type, reg: &Registry, updatable_ctx: bool) -> Result<FieldType> {
    let Type::Path(tp) = ty else {
        bail!("unsupported type syntax: {}", quote::quote!(#ty));
    };

    let last_seg = tp.path.segments.last().unwrap();
    let ident = last_seg.ident.to_string();

    match ident.as_str() {
        "u8" => Ok(FieldType::U8),
        "u16" => Ok(FieldType::U16),
        "u32" => Ok(FieldType::U32),
        "i64" => Ok(FieldType::I64),
        "bool" => Ok(FieldType::Bool),
        "String" => Ok(FieldType::String),
        "Uuid" => Ok(FieldType::Uuid),
        "Option" => {
            let inner = single_generic_arg(last_seg)?;
            resolve_option(inner, reg, updatable_ctx)
        }
        "Vec" => {
            let inner = single_generic_arg(last_seg)?;
            resolve_vec(inner, reg)
        }
        _ => {
            // Named type — resolve via registry (stripping super:: prefix).
            let resolved = type_name_from_path(tp);
            match reg.get(resolved.as_str()) {
                Some(TypeCat::Enum) => Ok(FieldType::Enum(resolved)),
                Some(TypeCat::Bitflags) => Ok(FieldType::Bitflags(resolved)),
                Some(TypeCat::Struct) => Ok(FieldType::Struct(resolved)),
                Some(TypeCat::TaggedEnum) => Ok(FieldType::TaggedEnum(resolved)),
                None => bail!("unknown type reference: {resolved}"),
            }
        }
    }
}

fn resolve_option(inner: &Type, reg: &Registry, updatable: bool) -> Result<FieldType> {
    let Type::Path(tp) = inner else {
        bail!("unsupported Option inner type");
    };
    let seg = tp.path.segments.last().unwrap();
    let name = seg.ident.to_string();

    match name.as_str() {
        "String" => {
            if updatable {
                Ok(FieldType::UpdatableString)
            } else {
                Ok(FieldType::OptionalString)
            }
        }
        "u32" => Ok(FieldType::OptionalU32),
        "Vec" => {
            // Option<Vec<T>> — dispatch on inner T.
            let inner2 = single_generic_arg(seg)?;
            let Type::Path(tp2) = inner2 else {
                bail!("unsupported Option<Vec<_>> inner type");
            };
            let inner_name = tp2.path.segments.last().unwrap().ident.to_string();
            match inner_name.as_str() {
                "u8" => Ok(FieldType::OptionalBytes),
                _ => {
                    let resolved = type_name_from_path(tp2);
                    Ok(FieldType::OptionalVecStruct(resolved))
                }
            }
        }
        _ => {
            // Option<NamedType> — look up in registry.
            let resolved = type_name_from_path(tp);
            match reg.get(resolved.as_str()) {
                Some(TypeCat::Struct) => Ok(FieldType::OptionalStruct(resolved)),
                Some(TypeCat::Bitflags) => Ok(FieldType::OptionalBitflags(resolved)),
                Some(TypeCat::Enum) => {
                    // Not currently used but handle gracefully.
                    Ok(FieldType::OptionalU32)
                }
                _ => bail!("unsupported Option<{resolved}>"),
            }
        }
    }
}

fn resolve_vec(inner: &Type, _reg: &Registry) -> Result<FieldType> {
    let Type::Path(tp) = inner else {
        bail!("unsupported Vec inner type");
    };
    let name = tp.path.segments.last().unwrap().ident.to_string();

    match name.as_str() {
        "u8" => Ok(FieldType::VecU8),
        "u32" => Ok(FieldType::VecU32),
        "String" => Ok(FieldType::VecString),
        _ => {
            let resolved = type_name_from_path(tp);
            Ok(FieldType::VecStruct(resolved))
        }
    }
}

/// Extract the last segment name from a type path, ignoring `super::` prefix.
fn type_name_from_path(tp: &syn::TypePath) -> String {
    tp.path.segments.last().unwrap().ident.to_string()
}

/// Extract the single generic type argument from `Foo<T>`.
fn single_generic_arg(seg: &syn::PathSegment) -> Result<&Type> {
    let PathArguments::AngleBracketed(ab) = &seg.arguments else {
        bail!("expected angle-bracket generic on {}", seg.ident);
    };
    let Some(syn::GenericArgument::Type(ty)) = ab.args.first() else {
        bail!("expected type argument in generic");
    };
    Ok(ty)
}

// ---------------------------------------------------------------------------
// Doc comment extraction
// ---------------------------------------------------------------------------

fn extract_doc(attrs: &[Attribute]) -> String {
    let lines: Vec<String> = attrs
        .iter()
        .filter_map(|attr| {
            if !attr.path().is_ident("doc") {
                return None;
            }
            if let Meta::NameValue(nv) = &attr.meta {
                if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = &nv.value {
                    return Some(s.value());
                }
            }
            None
        })
        .map(|s| {
            // Doc comments have a leading space: `/// text` → `#[doc = " text"]`.
            let trimmed = s.strip_prefix(' ').unwrap_or(&s);
            trimmed.to_string()
        })
        .collect();
    lines.join("\n")
}

// ---------------------------------------------------------------------------
// Expression evaluator (discriminants, bitflag values, constants)
// ---------------------------------------------------------------------------

fn eval_expr(expr: &Expr) -> Result<u64> {
    match expr {
        Expr::Lit(lit) => match &lit.lit {
            Lit::Int(i) => Ok(i.base10_parse::<u64>()?),
            _ => bail!("unsupported literal in expression"),
        },
        Expr::Binary(bin) => {
            let lhs = eval_expr(&bin.left)?;
            let rhs = eval_expr(&bin.right)?;
            match &bin.op {
                syn::BinOp::Shl(_) => Ok(lhs << rhs),
                syn::BinOp::Sub(_) => Ok(lhs.wrapping_sub(rhs)),
                syn::BinOp::BitOr(_) => Ok(lhs | rhs),
                syn::BinOp::BitAnd(_) => Ok(lhs & rhs),
                _ => bail!("unsupported binary operator"),
            }
        }
        Expr::Paren(p) => eval_expr(&p.expr),
        Expr::Group(g) => eval_expr(&g.expr),
        Expr::Unary(u) => {
            if matches!(u.op, syn::UnOp::Not(_)) {
                Ok(!eval_expr(&u.expr)?)
            } else {
                bail!("unsupported unary operator")
            }
        }
        _ => bail!("unsupported expression: {}", quote::quote!(#expr)),
    }
}

fn expr_to_string(expr: &Expr) -> String {
    quote::quote!(#expr).to_string()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn protocol_src() -> std::path::PathBuf {
        let manifest = env!("CARGO_MANIFEST_DIR");
        std::path::Path::new(manifest)
            .parent()
            .unwrap()
            .join("crates/chat_protocol/src")
    }

    #[test]
    fn parse_all_enums() {
        let module = parse_protocol(&protocol_src()).expect("parse_protocol failed");

        let names: Vec<&str> = module.enums.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"ChatKind"));
        assert!(names.contains(&"ChatRole"));
        assert!(names.contains(&"MessageKind"));
        assert!(names.contains(&"PresenceStatus"));
        assert!(names.contains(&"ErrorCode"));
        assert!(names.contains(&"DisconnectCode"));
        assert!(names.contains(&"FrameKind"));
        assert!(names.contains(&"LoadDirection"));
        assert_eq!(module.enums.len(), 8);
    }

    #[test]
    fn enum_discriminants() {
        let module = parse_protocol(&protocol_src()).unwrap();

        let chat_kind = module.enums.iter().find(|e| e.name == "ChatKind").unwrap();
        assert_eq!(chat_kind.repr, ReprType::U8);
        assert_eq!(chat_kind.variants.len(), 3);
        assert_eq!(chat_kind.variants[0].name, "Direct");
        assert_eq!(chat_kind.variants[0].discriminant, 0);
        assert_eq!(chat_kind.variants[2].name, "Channel");
        assert_eq!(chat_kind.variants[2].discriminant, 2);

        let frame_kind = module.enums.iter().find(|e| e.name == "FrameKind").unwrap();
        let send_msg = frame_kind.variants.iter().find(|v| v.name == "SendMessage").unwrap();
        assert_eq!(send_msg.discriminant, 0x10);

        let error_code = module.enums.iter().find(|e| e.name == "ErrorCode").unwrap();
        assert_eq!(error_code.repr, ReprType::U16);
        assert_eq!(error_code.variants.len(), 23);
        let unauthorized = error_code.variants.iter().find(|v| v.name == "Unauthorized").unwrap();
        assert_eq!(unauthorized.discriminant, 1000);
    }

    #[test]
    fn bitflags_parsing() {
        let module = parse_protocol(&protocol_src()).unwrap();

        let names: Vec<&str> = module.bitflags.iter().map(|b| b.name.as_str()).collect();
        assert!(names.contains(&"Permission"));
        assert!(names.contains(&"UserFlags"));
        assert!(names.contains(&"MessageFlags"));
        assert!(names.contains(&"RichStyle"));
        assert!(names.contains(&"ServerCapabilities"));
        assert_eq!(module.bitflags.len(), 5);

        let perm = module.bitflags.iter().find(|b| b.name == "Permission").unwrap();
        assert_eq!(perm.repr, ReprType::U32);
        assert_eq!(perm.flags.len(), 15);

        let send = perm.flags.iter().find(|f| f.name == "SEND_MESSAGES").unwrap();
        assert_eq!(send.value, 1);

        let manage_roles = perm.flags.iter().find(|f| f.name == "MANAGE_ROLES").unwrap();
        assert_eq!(manage_roles.value, 1 << 23);

        let delete_chat = perm.flags.iter().find(|f| f.name == "DELETE_CHAT").unwrap();
        assert_eq!(delete_chat.value, 1u64 << 31);
    }

    #[test]
    fn struct_fields() {
        let module = parse_protocol(&protocol_src()).unwrap();

        let chat_entry = module.structs.iter().find(|s| s.name == "ChatEntry").unwrap();
        assert_eq!(chat_entry.fields.len(), 10);

        assert_eq!(chat_entry.fields[0].name, "id");
        assert_eq!(chat_entry.fields[0].ty, FieldType::U32);

        assert_eq!(chat_entry.fields[1].name, "kind");
        assert_eq!(chat_entry.fields[1].ty, FieldType::Enum("ChatKind".into()));

        let parent = chat_entry.fields.iter().find(|f| f.name == "parent_id").unwrap();
        assert_eq!(parent.ty, FieldType::OptionalU32);

        let title = chat_entry.fields.iter().find(|f| f.name == "title").unwrap();
        assert_eq!(title.ty, FieldType::OptionalString);

        let last_msg = chat_entry.fields.iter().find(|f| f.name == "last_message").unwrap();
        assert_eq!(last_msg.ty, FieldType::OptionalStruct("LastMessagePreview".into()));
    }

    #[test]
    fn updatable_string_fields() {
        let module = parse_protocol(&protocol_src()).unwrap();

        let update_chat = module.structs.iter().find(|s| s.name == "UpdateChatPayload").unwrap();
        let title = update_chat.fields.iter().find(|f| f.name == "title").unwrap();
        assert_eq!(title.ty, FieldType::UpdatableString);

        let update_profile = module
            .structs
            .iter()
            .find(|s| s.name == "UpdateProfilePayload")
            .unwrap();
        let username = update_profile.fields.iter().find(|f| f.name == "username").unwrap();
        assert_eq!(username.ty, FieldType::UpdatableString);
    }

    #[test]
    fn optional_bitflags_field() {
        let module = parse_protocol(&protocol_src()).unwrap();

        let member = module.structs.iter().find(|s| s.name == "ChatMemberEntry").unwrap();
        let perms = member.fields.iter().find(|f| f.name == "permissions").unwrap();
        assert_eq!(perms.ty, FieldType::OptionalBitflags("Permission".into()));
    }

    #[test]
    fn optional_vec_struct_field() {
        let module = parse_protocol(&protocol_src()).unwrap();

        let message = module.structs.iter().find(|s| s.name == "Message").unwrap();
        let rich = message.fields.iter().find(|f| f.name == "rich_content").unwrap();
        assert_eq!(rich.ty, FieldType::OptionalVecStruct("RichSpan".into()));
    }

    #[test]
    fn tagged_enums() {
        let module = parse_protocol(&protocol_src()).unwrap();

        let names: Vec<&str> = module.tagged_enums.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"LoadChatsPayload"));
        assert!(names.contains(&"LoadMessagesPayload"));
        assert!(names.contains(&"SearchScope"));
        assert!(names.contains(&"MemberAction"));
        assert!(names.contains(&"AckPayload"));

        // MemberAction has mixed variant kinds.
        let action = module.tagged_enums.iter().find(|t| t.name == "MemberAction").unwrap();
        assert_eq!(action.variants.len(), 6);

        let kick = action.variants.iter().find(|v| v.name == "Kick").unwrap();
        assert!(matches!(kick.kind, VariantKind::Unit));

        let mute = action.variants.iter().find(|v| v.name == "Mute").unwrap();
        assert!(matches!(mute.kind, VariantKind::Struct(_)));

        let change_role = action.variants.iter().find(|v| v.name == "ChangeRole").unwrap();
        if let VariantKind::Tuple(types) = &change_role.kind {
            assert_eq!(types.len(), 1);
            assert_eq!(types[0], FieldType::Enum("ChatRole".into()));
        } else {
            panic!("ChangeRole should be Tuple variant");
        }
    }

    #[test]
    fn constants() {
        let module = parse_protocol(&protocol_src()).unwrap();

        assert_eq!(module.constants.len(), 7);

        let version = module.constants.iter().find(|c| c.name == "PROTOCOL_VERSION").unwrap();
        assert_eq!(version.ty, FieldType::U8);

        let max_ts = module.constants.iter().find(|c| c.name == "MAX_TIMESTAMP").unwrap();
        assert_eq!(max_ts.ty, FieldType::I64);

        let mask = module
            .constants
            .iter()
            .find(|c| c.name == "EVENT_SEQ_OVERFLOW_MASK")
            .unwrap();
        assert_eq!(mask.ty, FieldType::U32);
    }

    #[test]
    fn doc_comments_preserved() {
        let module = parse_protocol(&protocol_src()).unwrap();

        let chat_kind = module.enums.iter().find(|e| e.name == "ChatKind").unwrap();
        assert!(chat_kind.doc.contains("Chat type"));
        assert!(chat_kind.variants[0].doc.contains("Direct message"));
    }

    #[test]
    fn skip_types_not_present() {
        let module = parse_protocol(&protocol_src()).unwrap();

        assert!(!module.structs.iter().any(|s| s.name == "Frame"));
        assert!(!module.structs.iter().any(|s| s.name == "FrameHeader"));
        assert!(!module.tagged_enums.iter().any(|t| t.name == "FramePayload"));
    }

    #[test]
    fn uuid_field_type() {
        let module = parse_protocol(&protocol_src()).unwrap();

        let hello = module.structs.iter().find(|s| s.name == "HelloPayload").unwrap();
        let device_id = hello.fields.iter().find(|f| f.name == "device_id").unwrap();
        assert_eq!(device_id.ty, FieldType::Uuid);
    }

    #[test]
    fn vec_field_types() {
        let module = parse_protocol(&protocol_src()).unwrap();

        let get_presence = module.structs.iter().find(|s| s.name == "GetPresencePayload").unwrap();
        let user_ids = get_presence.fields.iter().find(|f| f.name == "user_ids").unwrap();
        assert_eq!(user_ids.ty, FieldType::VecU32);

        let subscribe = module.structs.iter().find(|s| s.name == "SubscribePayload").unwrap();
        let channels = subscribe.fields.iter().find(|f| f.name == "channels").unwrap();
        assert_eq!(channels.ty, FieldType::VecString);

        let msg_batch = module.structs.iter().find(|s| s.name == "MessageBatch").unwrap();
        let messages = msg_batch.fields.iter().find(|f| f.name == "messages").unwrap();
        assert_eq!(messages.ty, FieldType::VecStruct("Message".into()));
    }
}
