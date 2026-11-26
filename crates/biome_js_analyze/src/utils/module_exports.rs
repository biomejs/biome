//! Helpers for rules that read the exports of other modules from the module
//! graph.

use biome_languages::JsFileSource;
use biome_module_graph::JsModuleInfo;
use camino::Utf8Path;

/// Returns whether the exports of the file at `path` describe all of its
/// exports.
///
/// Declaration files often describe CommonJS modules, whose default export
/// isn't declared. Vue, Svelte and Astro components get their default export
/// from the framework compiler.
pub fn has_checkable_exports(path: &Utf8Path) -> bool {
    JsFileSource::try_from(path)
        .is_ok_and(|source| !source.is_embedded() && !source.language().is_definition_file())
}

/// Returns whether the module uses `import` or `export` syntax.
///
/// Modules without it, such as CommonJS modules, have no exports that can be
/// looked up.
pub fn is_es_module(module_info: &JsModuleInfo) -> bool {
    !module_info.exports.is_empty()
        || !module_info.blanket_reexports.is_empty()
        || module_info
            .import_paths
            .iter()
            .any(|import_path| import_path.kind.is_static())
}
