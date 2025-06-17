mod binding;
mod collector;
mod scope;
mod scoped_resolver;
mod visitor;

use std::{borrow::Cow, collections::BTreeMap, ops::Deref, sync::Arc};

use biome_js_syntax::{AnyJsExpression, AnyJsImportLike};
use biome_js_type_info::{
    BindingId, GLOBAL_RESOLVER, GLOBAL_UNKNOWN_ID, ImportSymbol, ResolvedTypeData, ResolvedTypeId,
    ScopeId, TypeData, TypeId, TypeReference, TypeReferenceQualifier, TypeResolver,
    TypeResolverLevel,
};
use biome_jsdoc_comment::JsdocComment;
use biome_resolver::ResolvedPath;
use biome_rowan::{AstNode, Text, TextRange};
use rust_lapper::Lapper;
use rustc_hash::FxHashMap;

use crate::ModuleGraph;

use scope::{JsScope, JsScopeData, TsBindingReference};

pub(super) use binding::JsBindingData;
pub use scoped_resolver::ScopedResolver;
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
    pub fn all_import_paths(&self) -> impl Iterator<Item = ResolvedPath> + use<> {
        let module_info = self.0.as_ref();
        ImportPathIterator {
            static_import_paths: module_info.static_import_paths.clone(),
            dynamic_import_paths: module_info.dynamic_import_paths.clone(),
        }
    }

    pub fn as_resolver(&self) -> &impl TypeResolver {
        self.0.as_ref()
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
    pub static_import_paths: BTreeMap<Text, ResolvedPath>,

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
    pub dynamic_import_paths: BTreeMap<Text, ResolvedPath>,

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

    /// Parsed expressions, mapped from their range to their type ID.
    pub(crate) expressions: FxHashMap<TextRange, TypeId>,

    /// All scopes in this module.
    ///
    /// The first entry is expected to be the global scope.
    pub(crate) scopes: Box<[JsScopeData]>,

    /// Lookup tree to find scopes by text range.
    pub(crate) scope_by_range: Lapper<u32, ScopeId>,

    /// Collection of all types in the module.
    pub(crate) types: Box<[TypeData]>,
}

#[derive(Debug, Default)]
pub struct Exports(pub(crate) BTreeMap<Text, JsExport>);

impl Deref for Exports {
    type Target = BTreeMap<Text, JsExport>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default)]
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
    pub fn get_import_path_by_js_node(&self, node: &AnyJsImportLike) -> Option<&ResolvedPath> {
        let specifier_text = node.inner_string_text()?;
        let specifier = specifier_text.text();
        if node.is_static_import() {
            self.static_import_paths.get(specifier)
        } else {
            self.dynamic_import_paths.get(specifier)
        }
    }
}

impl TypeResolver for JsModuleInfoInner {
    fn level(&self) -> TypeResolverLevel {
        TypeResolverLevel::Module
    }

    fn find_type(&self, type_data: &TypeData) -> Option<TypeId> {
        self.types
            .iter()
            .position(|data| data == type_data)
            .map(TypeId::new)
    }

    fn get_by_id(&self, id: TypeId) -> &TypeData {
        &self.types[id.index()]
    }

    fn get_by_resolved_id(&self, id: ResolvedTypeId) -> Option<ResolvedTypeData> {
        match id.level() {
            TypeResolverLevel::Module => Some((id, self.get_by_id(id.id())).into()),
            TypeResolverLevel::Global => Some((id, GLOBAL_RESOLVER.get_by_id(id.id())).into()),
            TypeResolverLevel::Scope | TypeResolverLevel::Import => None,
        }
    }

    fn register_type(&mut self, _type_data: Cow<TypeData>) -> TypeId {
        panic!("Cannot register new types after the module has been constructed");
    }

    fn resolve_reference(&self, ty: &TypeReference) -> Option<ResolvedTypeId> {
        match ty {
            TypeReference::Qualifier(qualifier) => self.resolve_qualifier(qualifier),
            TypeReference::Resolved(resolved_id) => Some(*resolved_id),
            TypeReference::Import(_) => None,
            TypeReference::Unknown => Some(GLOBAL_UNKNOWN_ID),
        }
    }

    fn resolve_qualifier(&self, qualifier: &TypeReferenceQualifier) -> Option<ResolvedTypeId> {
        if qualifier.path.len() != 1 {
            return None;
        }

        if let Some(export) = self.exports.get(&qualifier.path[0]) {
            export
                .as_own_export()
                .and_then(|own_export| match own_export {
                    JsOwnExport::Binding(binding_id) => {
                        self.resolve_reference(&self.bindings[binding_id.index()].ty)
                    }
                    JsOwnExport::Type(type_id) => Some(*type_id),
                })
        } else {
            GLOBAL_RESOLVER.resolve_qualifier(qualifier)
        }
    }

    fn resolve_type_of(&self, identifier: &Text, scope_id: ScopeId) -> Option<ResolvedTypeId> {
        if let Some(export) = self.exports.get(identifier) {
            export
                .as_own_export()
                .and_then(|own_export| match own_export {
                    JsOwnExport::Binding(binding_id) => {
                        self.resolve_reference(&self.bindings[binding_id.index()].ty)
                    }
                    JsOwnExport::Type(type_id) => Some(*type_id),
                })
        } else {
            GLOBAL_RESOLVER.resolve_type_of(identifier, scope_id)
        }
    }

    fn resolve_expression(&mut self, _scope_id: ScopeId, expr: &AnyJsExpression) -> Cow<TypeData> {
        self.expressions.get(&expr.range()).map_or_else(
            || Cow::Owned(TypeData::unknown()),
            |id| Cow::Borrowed(self.get_by_id(*id)),
        )
    }

    fn registered_types(&self) -> &[TypeData] {
        &self.types
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
    static_import_paths: BTreeMap<Text, ResolvedPath>,
    dynamic_import_paths: BTreeMap<Text, ResolvedPath>,
}

impl Iterator for ImportPathIterator {
    type Item = ResolvedPath;

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
