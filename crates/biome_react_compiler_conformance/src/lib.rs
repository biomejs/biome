//! Differential conformance harness for `biome_react_compiler`.
//!
//! It runs the React Compiler over the same source through two frontends — the
//! Biome CST converter (`biome_react_compiler`) and the OXC reference frontend
//! (`react_compiler_oxc`) — and compares the resulting compiler diagnostics.
//! The OXC frontend is the oracle: any difference is a gap or bug in Biome's
//! AST/scope conversion.
//!
//! Both frontends feed the *same* `compile_program` core, so the only variable
//! is the produced `File` + `ScopeInfo`. We compare at the diagnostics level
//! (`(category, reason)` pairs extracted from the compiler's `LoggerEvent`s),
//! which is robust to incidental AST-shape and node-id differences.

use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::{AnyJsRoot, JsFileSource};
use biome_react_compiler::{CompileInput, ReactCompilerError, compile_program, default_lint_options};
use react_compiler::entrypoint::compile_result::LoggerEvent;

/// A normalized compiler diagnostic key: `(category, reason)`.
pub type DiagKey = (String, String);

/// Extract sorted `(category, reason)` pairs from compiler logger events. Only
/// `CompileError`/`CompileErrorWithLoc` events carry a reportable detail.
pub fn diag_keys(events: &[LoggerEvent]) -> Vec<DiagKey> {
    let mut keys: Vec<DiagKey> = events
        .iter()
        .filter_map(|event| match event {
            LoggerEvent::CompileError { detail, .. }
            | LoggerEvent::CompileErrorWithLoc { detail, .. } => {
                Some((detail.category.clone(), detail.reason.clone()))
            }
            _ => None,
        })
        .collect();
    keys.sort();
    keys
}

/// Run the Biome frontend over `source` and return the compiler diagnostic keys.
///
/// An `Err` means Biome's converter could not even produce a `File`/`ScopeInfo`
/// (e.g. `UnsupportedSyntax`/`MissingSyntax`) — a hard Tier-1 coverage gap, as
/// distinct from "compiled, produced no diagnostics".
pub fn biome_diag_keys(
    source: &str,
    source_type: JsFileSource,
) -> Result<Vec<DiagKey>, ReactCompilerError> {
    let parsed = parse(source, source_type, JsParserOptions::default());
    biome_diag_keys_from_root(&parsed.tree(), source, source_type)
}

/// Like [`biome_diag_keys`], but operates on an already-parsed root so callers
/// (e.g. the corpus sweep) can parse once and inspect parser errors first.
pub fn biome_diag_keys_from_root(
    root: &AnyJsRoot,
    source: &str,
    source_type: JsFileSource,
) -> Result<Vec<DiagKey>, ReactCompilerError> {
    let model = semantic_model(root, SemanticModelOptions::from(&source_type));
    let output = compile_program(CompileInput {
        root,
        model: &model,
        source,
        source_type,
        options: default_lint_options(source),
    })?;
    Ok(diag_keys(&output.events))
}

/// Run the OXC reference frontend (the oracle) over `source`.
pub fn oxc_diag_keys(source: &str, source_type: oxc_span::SourceType) -> Vec<DiagKey> {
    let result =
        react_compiler_oxc::transform_source(source, source_type, default_lint_options(source));
    diag_keys(&result.events)
}
