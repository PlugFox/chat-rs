/// Dart code emitter — generates Dart package from IR.

use crate::codegen::ir::ParsedModule;

/// Generate the Dart package from parsed IR.
pub(crate) fn generate(_ir: &ParsedModule, _output_dir: &std::path::Path) -> anyhow::Result<()> {
    // Phase 2 will implement this.
    eprintln!("Dart codegen: not yet implemented");
    Ok(())
}
