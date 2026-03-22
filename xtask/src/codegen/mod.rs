/// Code generation orchestrator: parse → IR → generate.

pub(crate) mod dart;
pub(crate) mod ir;
pub(crate) mod parser;
pub(crate) mod typescript;

use std::path::Path;
use std::process::ExitCode;

/// Run code generation from the workspace root.
pub(crate) fn run(workspace_root: &Path, check: bool) -> ExitCode {
    let protocol_src = workspace_root.join("crates/chat_protocol/src");
    let dart_out = workspace_root.join("packages/chat_core_dart");
    let ts_out = workspace_root.join("packages/chat_core_ts");

    if check {
        eprintln!("codegen --check: not yet implemented (Phase 5)");
        return ExitCode::FAILURE;
    }

    eprintln!("Parsing protocol sources...");
    let ir = match parser::parse_protocol(&protocol_src) {
        Ok(ir) => ir,
        Err(e) => {
            eprintln!("Parse error: {e}");
            return ExitCode::FAILURE;
        }
    };

    eprintln!(
        "Parsed: {} enums, {} structs, {} bitflags, {} tagged enums, {} constants",
        ir.enums.len(),
        ir.structs.len(),
        ir.bitflags.len(),
        ir.tagged_enums.len(),
        ir.constants.len(),
    );

    eprintln!("Generating Dart package...");
    if let Err(e) = dart::generate(&ir, &dart_out) {
        eprintln!("Dart generation error: {e}");
        return ExitCode::FAILURE;
    }

    eprintln!("Generating TypeScript package...");
    if let Err(e) = typescript::generate(&ir, &ts_out) {
        eprintln!("TypeScript generation error: {e}");
        return ExitCode::FAILURE;
    }

    eprintln!("Code generation complete.");
    ExitCode::SUCCESS
}
