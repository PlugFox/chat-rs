/// Intermediate representation for code generation.
///
/// These types capture everything needed to emit Dart and TypeScript
/// from parsed Rust protocol types.
/// A parsed Rust source module.
#[derive(Debug, Default)]
pub(crate) struct ParsedModule {
    pub enums: Vec<EnumDef>,
    pub structs: Vec<StructDef>,
    pub bitflags: Vec<BitflagsDef>,
    pub tagged_enums: Vec<TaggedEnumDef>,
    pub constants: Vec<ConstDef>,
}

/// Representation width for `#[repr(u8)]`, `#[repr(u16)]`, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReprType {
    U8,
    U16,
    U32,
}

/// A `#[repr(uN)]` enum with explicit discriminants.
#[derive(Debug)]
pub(crate) struct EnumDef {
    pub name: String,
    pub doc: String,
    pub repr: ReprType,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug)]
pub(crate) struct EnumVariant {
    pub name: String,
    pub doc: String,
    pub discriminant: u64,
}

/// A `bitflags!` block.
#[derive(Debug)]
pub(crate) struct BitflagsDef {
    pub name: String,
    pub doc: String,
    pub repr: ReprType,
    pub flags: Vec<FlagEntry>,
}

#[derive(Debug)]
pub(crate) struct FlagEntry {
    pub name: String,
    pub doc: String,
    #[allow(dead_code)]
    pub value: u64,
    /// Preserved expression string for codegen (e.g. `1 << 10`).
    pub value_expr: String,
}

/// A plain struct with named fields.
#[derive(Debug)]
pub(crate) struct StructDef {
    pub name: String,
    pub doc: String,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub(crate) struct Field {
    pub name: String,
    pub doc: String,
    pub ty: FieldType,
}

/// A tagged enum (Rust enum with data in variants).
#[derive(Debug)]
pub(crate) struct TaggedEnumDef {
    pub name: String,
    pub doc: String,
    pub variants: Vec<TaggedVariant>,
}

#[derive(Debug)]
pub(crate) struct TaggedVariant {
    pub name: String,
    pub doc: String,
    pub kind: VariantKind,
}

#[derive(Debug)]
pub(crate) enum VariantKind {
    Unit,
    Tuple(Vec<FieldType>),
    Struct(Vec<Field>),
}

/// Protocol constant.
#[derive(Debug)]
pub(crate) struct ConstDef {
    pub name: String,
    pub doc: String,
    pub ty: FieldType,
    pub value_expr: String,
}

/// Field types — covers every pattern used in the wire protocol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FieldType {
    U8,
    U16,
    U32,
    I64,
    Bool,
    String,
    /// `Option<String>` — wire: len=0 for None.
    OptionalString,
    /// `Option<String>` with u8-flag semantics (UpdateChat, UpdateProfile).
    UpdatableString,
    Uuid,
    /// `Option<u32>` — wire: u8 flag + u32.
    OptionalU32,
    /// `Vec<u32>` — wire: u16 count + u32[].
    VecU32,
    /// `Vec<u8>` — raw bytes.
    VecU8,
    /// `Option<Vec<u8>>` — wire: u32 len, None when 0.
    OptionalBytes,
    /// `Vec<String>` — wire: u16 count + strings.
    VecString,
    /// Named enum (e.g. ChatKind, MessageKind).
    Enum(std::string::String),
    /// Named bitflags (e.g. Permission, MessageFlags).
    Bitflags(std::string::String),
    /// Nested struct (e.g. ServerLimits).
    Struct(std::string::String),
    /// `Option<struct>` — wire: u8 flag + struct.
    OptionalStruct(std::string::String),
    /// `Option<bitflags>` — wire: u8 flag + repr value.
    OptionalBitflags(std::string::String),
    /// `Option<Vec<struct>>` — wire: u8 flag + count + structs (or count=0 for None).
    OptionalVecStruct(std::string::String),
    /// `Vec<struct>` — wire: count + structs.
    VecStruct(std::string::String),
    /// Inline tagged enum (e.g. MemberAction).
    TaggedEnum(std::string::String),
}
