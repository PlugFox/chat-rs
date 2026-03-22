/// TypeScript code emitter — generates TS package from IR.

use crate::codegen::ir::ParsedModule;

/// Generate the TypeScript package from parsed IR.
pub(crate) fn generate(_ir: &ParsedModule, _output_dir: &std::path::Path) -> anyhow::Result<()> {
    // Phase 3 will implement this.
    eprintln!("TypeScript codegen: not yet implemented");
    Ok(())
}
