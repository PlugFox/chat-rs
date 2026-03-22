/// Rust source parser — extracts IR from chat_protocol types.
///
/// Uses `syn` to parse Rust source files and convert them into
/// the intermediate representation defined in `ir.rs`.

use crate::codegen::ir::ParsedModule;

/// Parse all protocol source files and return the combined IR.
pub(crate) fn parse_protocol(_protocol_src: &std::path::Path) -> anyhow::Result<ParsedModule> {
    // Phase 1 will implement this.
    Ok(ParsedModule::default())
}
