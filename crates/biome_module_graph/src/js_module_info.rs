mod binding;
mod collector;
mod diagnostics;
mod module_resolver;
mod scope;
mod utils;
mod visitor;

use biome_js_syntax::AnyJsImportLike;
use biome_js_type_info::{BindingId, ImportSymbol, ResolvedTypeId, ScopeId, TypeData};
use biome_jsdoc_comment::JsdocComment;
use biome_resolver::ResolvedPath;
use biome_rowan::{Text, TextRange};
use camino::Utf8Path;
use indexmap::IndexMap;
use rust_lapper::Lapper;
use rustc_hash::FxHashMap;
use std::collections::BTreeSet;
use std::{collections::BTreeMap, ops::Deref, sync::Arc};

use crate::ModuleGraph;

use scope::{JsScope, JsScopeData, TsBindingReference};

use crate::diagnostics::ModuleDiagnostic;
pub(super) use binding::JsBindingData;
pub use diagnostics::JsModuleInfoDiagnostic;
pub use module_resolver::ModuleResolver;
pub(crate) use visitor::JsModuleVisitor;

/// Information restricted to a single module in the [ModuleGraph].
#[derive(Clone, Debug)]
pub struct JsModuleInfo(pub(super) Arc<JsModuleInfoInner>);

impl Deref for JsModuleInfo {
    type Target = JsModuleInfoInner;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl JsModuleInfo {
    /// Returns an iterator over all the static and dynamic imports in this
    /// module.
    pub fn all_import_paths(&self) -> impl Iterator<Item = JsImportPath> + use<> {
        ImportPathIterator {
            module_info: self.clone(),
            index: 0,
        }
    }

    pub fn diagnostics(&self) -> &[ModuleDiagnostic] {
        self.diagnostics.as_slice()
    }

    /// Finds an exported symbol by `name`, using the `module_graph` to
    /// lookup re-exports if necessary.
    #[inline]
    pub fn find_exported_symbol(
        &self,
        module_graph: &ModuleGraph,
        name: &str,
    ) -> Option<JsOwnExport> {
        module_graph.find_exported_symbol(self, name)
    }

    /// Finds an exported symbol by `name`, using the `module_graph` to
    /// lookup re-exports if necessary.
    #[inline]
    pub fn find_jsdoc_for_exported_symbol(
        &self,
        module_graph: &ModuleGraph,
        name: &str,
    ) -> Option<JsdocComment> {
        module_graph.find_jsdoc_for_exported_symbol(self, name)
    }

    /// Returns the module's global scope.
    pub fn global_scope(&self) -> JsScope {
        JsScope {
            info: self.0.clone(),
            id: ScopeId::new(0),
        }
    }

    /// Returns the scope to be used for the given `range`.
    pub fn scope_for_range(&self, range: TextRange) -> JsScope {
        JsScope {
            info: self.0.clone(),
            id: scope_id_for_range(&self.0.scope_by_range, range),
        }
    }

    /// Returns a serializable representation of this module.
    pub fn dump(&self) -> SerializedJsModuleInfo {
        SerializedJsModuleInfo {
            static_imports: self
                .static_imports
                .iter()
                .map(|(text, static_import)| {
                    (text.to_string(), static_import.specifier.to_string())
                })
                .collect(),

            static_import_paths: self
                .static_import_paths
                .iter()
                .map(|(specifier, JsImportPath { resolved_path, .. })| {
                    (
                        specifier.to_string(),
                        resolved_path
                            .as_ref()
                            .map_or_else(|_| specifier.to_string(), ToString::to_string),
                    )
                })
                .collect(),

            exports: self
                .exports
                .iter()
                .map(|(text, _)| text.to_string())
                .collect::<BTreeSet<_>>(),

            dynamic_imports: self
                .dynamic_import_paths
                .iter()
                .map(|(text, _)| text.to_string())
                .collect::<BTreeSet<_>>(),
        }
    }
}

#[derive(Debug)]
pub struct JsModuleInfoInner {
    /// Map of all static imports found in the module.
    ///
    /// Maps from the local imported name to a [JsImport] with the absolute path
    /// it resolves to. The resolved path may be looked up as key in the
    /// [ModuleGraph::data] map, although it is not required to exist
    /// (for instance, if the path is outside the project's scope).
    ///
    /// Note that re-exports may introduce additional dependencies, because they
    /// import another module and immediately re-export from that module.
    /// Re-exports are tracked as part of [Self::exports] and
    /// [Self::blanket_reexports].
    pub static_imports: Imports,

    /// Map of all the paths from static imports in the module.
    ///
    /// Maps from the source specifier name to a [JsImportPath] with the
    /// absolute path it resolves to. The resolved path may be looked up as key
    /// in the [ModuleGraph::data] map, although it is not required to exist
    /// (for instance, if the path is outside the project's scope).
    pub static_import_paths: IndexMap<Text, JsImportPath>,

    /// Map of all dynamic import paths found in the module for which the import
    /// specifier could be statically determined.
    ///
    /// Dynamic imports for which the specifier cannot be statically determined
    /// (for instance, because a template string with variables is used) will be
    /// omitted from this map.
    ///
    /// Maps from the source specifier name to a [JsImportPath] with the
    /// absolute path it resolves to. The resolved path may be looked up as key
    /// in the [ModuleGraph::data] map, although it is not required to exist
    /// (for instance, if the path is outside the project's scope).
    ///
    /// Paths found in `require()` expressions in CommonJS sources are also
    /// included with the dynamic import paths.
    pub dynamic_import_paths: IndexMap<Text, JsImportPath>,

    /// Map of exports from the module.
    ///
    /// The keys are the names of the exports, where "default" is used for the
    /// default export. See [JsExport] for information tracked per export.
    ///
    /// Re-exports are tracked in this map as well. The exception is "blanket"
    /// re-exports, such as `export * from "other-module"`. Those are tracked in
    /// [Self::forwarding_exports] instead.
    pub exports: Exports,

    /// Re-exports that apply to all symbols from another module, without
    /// assigning a name to them.
    pub blanket_reexports: Vec<JsReexport>,

    /// Collection of all the declarations in the module.
    pub(crate) bindings: Vec<JsBindingData>,

    /// Parsed expressions, mapped from their range to their type ID.
    pub(crate) expressions: FxHashMap<TextRange, ResolvedTypeId>,

    /// All scopes in this module.
    ///
    /// The first entry is expected to be the global scope.
    pub(crate) scopes: Vec<JsScopeData>,

    /// Lookup tree to find scopes by text range.
    pub(crate) scope_by_range: Lapper<u32, ScopeId>,

    /// Collection of all types in the module.
    ///
    /// We do not store these using our `TypeStore`, because once the module
    /// info is constructed, no new types can be registered in it, and we have
    /// no use for a hash table anymore.
    pub(crate) types: Vec<Arc<TypeData>>,

    /// Diagnostics emitted during the resolution of the module
    pub(crate) diagnostics: Vec<ModuleDiagnostic>,
}

#[derive(Debug, Default)]
pub struct Exports(pub(crate) IndexMap<Text, JsExport>);

impl Deref for Exports {
    type Target = IndexMap<Text, JsExport>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default)]
pub struct Imports(pub(crate) IndexMap<Text, JsImport>);

impl Deref for Imports {
    type Target = IndexMap<Text, JsImport>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum JsImportPhase {
    #[default]
    Default,
    /// https://tc39.es/proposal-defer-import-eval/
    Defer,
    /// https://tc39.es/proposal-source-phase-imports/
    Source,
    /// Technically this is not an import phase defined in ECMAScript, but type-only imports in
    /// TypeScript cannot be imported in other phases.
    Type,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JsImportPath {
    pub resolved_path: ResolvedPath,
    pub phase: JsImportPhase,
}

impl JsImportPath {
    pub fn as_path(&self) -> Option<&Utf8Path> {
        self.resolved_path.as_path()
    }
}

static_assertions::assert_impl_all!(JsModuleInfo: Send, Sync);

impl JsModuleInfoInner {
    /// Returns one of the bindings by ID.
    #[inline]
    pub fn binding(&self, binding_id: BindingId) -> &JsBindingData {
        &self.bindings[binding_id.index()]
    }

    /// Attempts to find a binding by `name` in the scope with the given
    /// `scope_id`.
    ///
    /// Traverses upwards in scope if the binding is not found in the given
    /// scope.
    fn find_binding_in_scope(&self, name: &str, scope_id: ScopeId) -> Option<TsBindingReference> {
        let mut scope = &self.scopes[scope_id.index()];
        loop {
            if let Some(binding_ref) = scope.bindings_by_name.get(name) {
                return Some(*binding_ref);
            }

            match &scope.parent {
                Some(parent_id) => scope = &self.scopes[parent_id.index()],
                None => break,
            }
        }

        None
    }

    /// Returns the information about a given import by its syntax node.
    pub fn get_import_path_by_js_node(&self, node: &AnyJsImportLike) -> Option<&JsImportPath> {
        let specifier_text = node.inner_string_text()?;
        let specifier = specifier_text.text();
        if node.is_static_import() {
            self.static_import_paths.get(specifier)
        } else {
            self.dynamic_import_paths.get(specifier)
        }
    }

    pub fn types(&self) -> Vec<&TypeData> {
        self.types.iter().map(Arc::as_ref).collect()
    }
}

/// Information tracked for every export.
///
/// Exports come in three varieties: "own" exports that are defined in the
/// module itself, re-exports for named exports, and re-exports that apply to
/// all symbols from another module.
#[derive(Clone, Debug, PartialEq)]
pub enum JsExport {
    /// An export that is defined in this module.
    Own(JsOwnExport),

    /// An exported type that is defined in this module.
    OwnType(JsOwnExport),

    /// An export that is re-exported by this module, but which is defined
    /// within another module.
    ///
    /// E.g. `export { someSymbol } from "other-module"`.
    Reexport(JsReexport),

    /// A type that is re-exported by this module, but which is defined
    /// within another module.
    ///
    /// E.g. `export type { someSymbol } from "other-module"`.
    ReexportType(JsReexport),
}

impl JsExport {
    pub fn as_own_export(&self) -> Option<&JsOwnExport> {
        match self {
            Self::Own(own_export) | Self::OwnType(own_export) => Some(own_export),
            Self::Reexport(_) | Self::ReexportType(_) => None,
        }
    }

    pub fn as_own_export_mut(&mut self) -> Option<&mut JsOwnExport> {
        match self {
            Self::Own(own_export) | Self::OwnType(own_export) => Some(own_export),
            Self::Reexport(_) | Self::ReexportType(_) => None,
        }
    }
}

/// Represents an import to one or more symbols from an external path.
///
/// It could point to any kind of resource, such as JavaScript files, CSS files,
/// images, and so on.
#[derive(Clone, Debug, PartialEq)]
pub struct JsImport {
    /// The specifier for the imported as it appeared in the source text.
    pub specifier: Text,

    /// Absolute path of the resource being imported, if it can be resolved.
    ///
    /// If the import statement referred to a package dependency, the path will
    /// point towards the resolved entry point of the package.
    ///
    /// If `None`, import resolution failed.
    pub resolved_path: ResolvedPath,

    /// The symbol(s) being imported.
    pub symbol: ImportSymbol,
}

/// Information tracked for every "own" export.
///
/// Exports can reference bindings, types of expressions or other references for
/// which no binding exists, or namespaces defined by exports of another module.
#[derive(Clone, Debug, PartialEq)]
pub enum JsOwnExport {
    Binding(BindingId),
    Type(ResolvedTypeId),
}

/// Information about an export statement that re-exports all symbols from
/// another module.
#[derive(Clone, Debug, PartialEq)]
pub struct JsReexport {
    /// Optional JSDoc comment associated with the re-export statement.
    pub jsdoc_comment: Option<JsdocComment>,

    /// Import from which the symbols are being re-exported.
    pub import: JsImport,
}

struct ImportPathIterator {
    module_info: JsModuleInfo,
    index: usize,
}

impl Iterator for ImportPathIterator {
    type Item = JsImportPath;

    fn next(&mut self) -> Option<Self::Item> {
        let num_static_imports = self.module_info.static_import_paths.len();
        let resolved_path = if self.index < num_static_imports {
            let resolved_path = &self.module_info.static_import_paths[self.index];
            self.index += 1;
            resolved_path
        } else if self.index < self.module_info.dynamic_import_paths.len() + num_static_imports {
            let resolved_path =
                &self.module_info.dynamic_import_paths[self.index - num_static_imports];
            self.index += 1;
            resolved_path
        } else {
            return None;
        };

        Some(resolved_path.clone())
    }
}

fn scope_id_for_range(scope_by_range: &Lapper<u32, ScopeId>, range: TextRange) -> ScopeId {
    let start = range.start().into();
    let end = range.end().into();
    scope_by_range
        .find(start, end)
        .filter(|interval| !(start < interval.start || end > interval.stop))
        .max_by_key(|interval| interval.val)
        .map_or(ScopeId::GLOBAL, |interval| {
            ScopeId::new(interval.val.index())
        })
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SerializedJsModuleInfo {
    /// Map of all static imports found in the module.
    ///
    /// Maps from the local imported name to the absolute path it resolves to.
    pub static_imports: BTreeMap<String, String>,

    /// Map of all the paths from static imports in the module.
    ///
    /// Maps from the source specifier name to the absolute path it resolves to.
    /// Specifiers that could not be resolved to an absolute will map to the
    /// specifier itself.
    ///
    /// ## Example
    ///
    /// ```json
    /// {
    ///   "./foo": "/absolute/path/to/foo.js",
    ///   "react": "react"
    /// }
    /// ```
    pub static_import_paths: BTreeMap<String, String>,

    /// Dynamic imports.
    pub dynamic_imports: BTreeSet<String>,

    /// Exported symbols.
    pub exports: BTreeSet<String>,
}
