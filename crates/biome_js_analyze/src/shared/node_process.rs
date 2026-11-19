//! Helpers for rules that check uses of the Node.js `process` object.

use biome_js_semantic::Binding;
use biome_js_syntax::JsImport;
use biome_rowan::AstNode;

/// Module specifiers that resolve to the Node.js `process` module.
pub const PROCESS_MODULE_NAMES: [&str; 2] = ["process", "node:process"];

/// Whether the `process` `binding` is imported from the `process`/`node:process`
/// module through an `import` statement.
pub fn is_process_module_import(binding: &Binding) -> bool {
    binding
        .syntax()
        .ancestors()
        .find_map(|ancestor| JsImport::cast(ancestor)?.source_text().ok())
        .is_some_and(|source| PROCESS_MODULE_NAMES.contains(&source.text()))
}
