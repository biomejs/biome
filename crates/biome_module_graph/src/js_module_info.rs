mod ad_hoc_scope_resolver;
mod binding;
mod collector;
mod global_scope_resolver;
mod scope;
mod visitor;

use crate::{ModuleGraph, jsdoc_comment::JsdocComment};
use ad_hoc_scope_resolver::AdHocScopeResolver;
use binding::JsBindingData;
use biome_js_semantic::{BindingId, ScopeId};
use biome_js_syntax::{AnyJsExpression, AnyJsImportLike};
use biome_js_type_info::Type;
use biome_rowan::{AstNode, Text, TextRange, TokenText};
use camino::{Utf8Path, Utf8PathBuf};
use scope::{JsScope, JsScopeData};
use std::{collections::BTreeMap, ops::Deref, sync::Arc};

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
    pub fn all_import_paths(&self) -> impl Iterator<Item = JsResolvedPath> + use<> {
        let module_info = self.0.as_ref();
        ImportPathIterator {
            static_import_paths: module_info.static_import_paths.clone(),
            dynamic_import_paths: module_info.dynamic_import_paths.clone(),
        }
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

    /// Returns the module's global scope.
    pub fn global_scope(&self) -> JsScope {
        JsScope {
            info: self.0.clone(),
            id: ScopeId::new(0),
        }
    }

    /// Returns the resolved type of the given expression within this module.
    pub fn resolved_type_for_expression(
        &self,
        expr: &AnyJsExpression,
        module_graph: &ModuleGraph,
    ) -> Type {
        let mut ty = Type::from_any_js_expression(expr);
        let scope = self.scope_for_range(expr.range());
        ty.resolve(&AdHocScopeResolver::from_scope_in_module(
            scope,
            self,
            module_graph,
        ));
        ty
    }

    /// Returns the scope to be used for the given `range`.
    pub fn scope_for_range(&self, range: TextRange) -> JsScope {
        let start = range.start().into();
        let end = range.end().into();
        self.0
            .scope_by_range
            .find(start, end)
            .filter(|interval| !(start < interval.start || end > interval.stop))
            .max_by_key(|interval| interval.val)
            .map_or_else(
                || self.global_scope(),
                |interval| JsScope {
                    info: self.0.clone(),
                    id: ScopeId::new(interval.val.index()),
                },
            )
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
    /// Maps from the source specifier name to a [JsResolvedPath] with the
    /// absolute path it resolves to. The resolved path may be looked up as key
    /// in the [ModuleGraph::data] map, although it is not required to exist
    /// (for instance, if the path is outside the project's scope).
    pub static_import_paths: BTreeMap<Text, JsResolvedPath>,

    /// Map of all dynamic import paths found in the module for which the import
    /// specifier could be statically determined.
    ///
    /// Dynamic imports for which the specifier cannot be statically determined
    /// (for instance, because a template string with variables is used) will be
    /// omitted from this map.
    ///
    /// Maps from the source specifier name to a [JsResolvedPath] with the
    /// absolute path it resolves to. The resolved path may be looked up as key
    /// in the [ModuleGraph::data] map, although it is not required to exist
    /// (for instance, if the path is outside the project's scope).
    ///
    /// Paths found in `require()` expressions in CommonJS sources are also
    /// included with the dynamic import paths.
    pub dynamic_import_paths: BTreeMap<Text, JsResolvedPath>,

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
    pub blanket_reexports: Box<[JsReexport]>,

    /// Collection of all the declarations in the module.
    pub(crate) bindings: Box<[JsBindingData]>,

    /// All scopes in this module.
    ///
    /// The first entry is expected to be the global scope.
    pub(crate) scopes: Box<[JsScopeData]>,

    /// Lookup tree to find scopes by text range.
    pub(crate) scope_by_range: rust_lapper::Lapper<u32, ScopeId>,
}

#[derive(Debug)]
pub struct Exports(pub(crate) BTreeMap<Text, JsExport>);

impl Deref for Exports {
    type Target = BTreeMap<Text, JsExport>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct Imports(pub(crate) BTreeMap<Text, JsImport>);

impl Deref for Imports {
    type Target = BTreeMap<Text, JsImport>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

static_assertions::assert_impl_all!(JsModuleInfo: Send, Sync);

impl JsModuleInfoInner {
    /// Returns one of the bindings by ID.
    pub(crate) fn binding(&self, binding_id: BindingId) -> &JsBindingData {
        &self.bindings[binding_id.index()]
    }

    /// Returns the information about a given import by its syntax node.
    pub fn get_import_path_by_js_node(&self, node: &AnyJsImportLike) -> Option<&JsResolvedPath> {
        let specifier_text = node.inner_string_text()?;
        let specifier = specifier_text.text();
        if node.is_static_import() {
            self.static_import_paths.get(specifier)
        } else {
            self.dynamic_import_paths.get(specifier)
        }
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
    pub resolved_path: JsResolvedPath,

    /// The symbol(s) being imported.
    pub symbol: JsImportSymbol,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum JsImportSymbol {
    /// Imports the `default` export.
    #[default]
    Default,

    /// Imports a named symbol.
    Named(Text),

    /// Imports all symbols, including the `default` export.
    All,
}

impl From<Text> for JsImportSymbol {
    fn from(name: Text) -> Self {
        Self::Named(name)
    }
}

impl From<&'static str> for JsImportSymbol {
    fn from(name: &'static str) -> Self {
        Self::Named(name.into())
    }
}

/// Information tracked for every "own" export.
#[derive(Clone, Debug, PartialEq)]
pub struct JsOwnExport {
    /// Optional JSDoc comment associated with the symbol being exported.
    pub jsdoc_comment: Option<JsdocComment>,

    /// Name of the binding in the module's global scope.
    pub local_name: Option<TokenText>,

    /// Type of the exported symbol.
    pub ty: Type,
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

/// Reference-counted resolved path wrapped in a [Result] that contains a string
/// message if resolution failed.
#[derive(Clone, Debug, PartialEq)]
pub struct JsResolvedPath(Arc<Result<Utf8PathBuf, String>>);

impl Deref for JsResolvedPath {
    type Target = Result<Utf8PathBuf, String>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl JsResolvedPath {
    pub(super) fn new(resolved_path: Result<Utf8PathBuf, String>) -> Self {
        Self(Arc::new(resolved_path))
    }

    pub fn as_path(&self) -> Option<&Utf8Path> {
        self.as_deref().ok()
    }

    pub fn from_path(path: impl Into<Utf8PathBuf>) -> Self {
        Self::new(Ok(path.into()))
    }
}

struct ImportPathIterator {
    static_import_paths: BTreeMap<Text, JsResolvedPath>,
    dynamic_import_paths: BTreeMap<Text, JsResolvedPath>,
}

impl Iterator for ImportPathIterator {
    type Item = JsResolvedPath;

    fn next(&mut self) -> Option<Self::Item> {
        if self.static_import_paths.is_empty() {
            self.dynamic_import_paths
                .pop_first()
                .map(|(_source, path)| path)
        } else {
            self.static_import_paths
                .pop_first()
                .map(|(_identifier, path)| path)
        }
    }
}
