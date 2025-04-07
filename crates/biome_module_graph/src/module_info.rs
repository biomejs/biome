use std::collections::BTreeMap;

use biome_js_syntax::AnyJsImportLike;
use biome_js_type_info::Type;
use biome_rowan::Text;
use camino::Utf8PathBuf;

use crate::{ModuleGraph, jsdoc_comment::JsdocComment};

/// Information restricted to a single module in the [ModuleGraph].
#[derive(Clone, Debug, Default)]
pub struct ModuleInfo {
    /// Map of all static imports found in the module.
    ///
    /// Maps from the identifier found in the import statement to the absolute
    /// path it resolves to. The resolved path may be looked up as key in the
    /// [DependencyGraphModel::modules] map, although it is not required to
    /// exist (for instance, if the path is outside the project's scope).
    ///
    /// Note that re-exports may introduce additional dependencies, because they
    /// import another module and immediately re-export from that module.
    /// Re-exports are tracked as part of [Self::exports] and
    /// [Self::blanket_reexports].
    pub static_imports: BTreeMap<String, Import>,

    /// Map of all dynamic imports found in the module for which the import
    /// identifier could be statically determined.
    ///
    /// Dynamic imports for which the identifier cannot be statically determined
    /// (for instance, because a template string with variables is used) will be
    /// omitted from this map.
    ///
    /// Maps from the identifier found in the import expression to the absolute
    /// path it resolves to. The resolved path may be looked up as key in the
    /// [DependencyGraphModel::modules] map, although it is not required to
    /// exist (for instance, if the path is outside the project's scope).
    ///
    /// `require()` expressions in CommonJS sources are also included with the
    /// dynamic imports.
    pub dynamic_imports: BTreeMap<String, Import>,

    /// Map of exports from the module.
    ///
    /// The keys are the names of the exports, where "default" is used for the
    /// default export. See [Export] for information tracked per export.
    ///
    /// Re-exports are tracked in this map as well. The exception are "blanket"
    /// re-exports, such as `export * from "other-module"`. Those are tracked in
    /// [Self::forwarding_exports] instead.
    pub exports: BTreeMap<Text, Export>,

    /// Re-exports that apply to all symbols from another module, without
    /// assigning a name to them.
    pub blanket_reexports: Vec<ReexportAll>,
}

static_assertions::assert_impl_all!(ModuleInfo: Send, Sync);

impl ModuleInfo {
    /// Allows draining a single entry from the imports.
    ///
    /// Returns a `(specifier, import)` pair from either the static or dynamic
    /// imports, whichever is non-empty. Returns `None` if both are empty.
    ///
    /// Using this method allows for consuming the struct while iterating over
    /// it, without necessarily turning the entire struct into an iterator at
    /// once.
    pub fn drain_import(&mut self) -> Option<(String, Import)> {
        if self.static_imports.is_empty() {
            self.dynamic_imports.pop_first()
        } else {
            self.static_imports.pop_first()
        }
    }

    /// Finds an exported symbol by `name`, using the `module_graph` to
    /// lookup re-exports if necessary.
    #[inline]
    pub fn find_exported_symbol(
        &self,
        module_graph: &ModuleGraph,
        name: &str,
    ) -> Option<OwnExport> {
        module_graph.find_exported_symbol(self, name)
    }

    /// Returns the information about a given import by its syntax node.
    pub fn get_import_by_js_node(&self, node: &AnyJsImportLike) -> Option<&Import> {
        let specifier_text = node.inner_string_text()?;
        let specifier = specifier_text.text();
        if node.is_static_import() {
            self.static_imports.get(specifier)
        } else {
            self.dynamic_imports.get(specifier)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Import {
    /// Absolute path of the resource being imported, if it can be resolved.
    ///
    /// If the import statement referred to a package dependency, the path will
    /// point towards the resolved entry point of the package.
    ///
    /// If `None`, import resolution failed.
    pub resolved_path: Result<Utf8PathBuf, String>,
}

/// Information tracked for every export.
///
/// Exports come in three varieties: "own" exports that are defined in the
/// module itself, re-exports for named exports, and re-exports that apply to
/// all symbols from another module.
#[derive(Clone, Debug, PartialEq)]
pub enum Export {
    /// An export that is defined in this module.
    Own(OwnExport),

    /// An exported type that is defined in this module.
    OwnType(OwnExport),

    /// An export that is re-exported by this module, but which is defined
    /// within another module.
    ///
    /// E.g. `export { someSymbol } from "other-module"`.
    Reexport(Import),

    /// A type that is re-exported by this module, but which is defined
    /// within another module.
    ///
    /// E.g. `export type { someSymbol } from "other-module"`.
    ReexportType(Import),

    /// An export that creates an alias for all symbols from another module.
    ///
    /// E.g. `export { * as alias } from "other-module"`.
    ReexportAll(ReexportAll),
}

/// Information tracked for every "own" export.
#[derive(Clone, Debug, PartialEq)]
pub struct OwnExport {
    /// Optional JSDoc comment associated with the symbol being exported.
    pub jsdoc_comment: Option<JsdocComment>,

    /// Type of the exported symbol.
    pub ty: Type,
}

/// Information about an export statement that re-exports all symbols from
/// another module.
#[derive(Clone, Debug, PartialEq)]
pub struct ReexportAll {
    /// Optional JSDoc comment associated with the re-export statement.
    pub jsdoc_comment: Option<JsdocComment>,

    /// Import from which the symbols are being re-exported.
    pub import: Import,
}
