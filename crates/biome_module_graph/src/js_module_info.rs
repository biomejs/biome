mod ad_hoc_scope_resolver;
mod binding;
mod collector;
mod scope;
mod visitor;

use std::{collections::BTreeMap, ops::Deref, sync::Arc};

use biome_js_semantic::ScopeId;
use biome_js_syntax::{AnyJsExpression, AnyJsImportLike};
use biome_js_type_info::{
    GLOBAL_RESOLVER, GLOBAL_UNKNOWN_ID, ImportSymbol, ResolvedPath, ResolvedTypeId, Type, TypeData,
    TypeId, TypeReference, TypeReferenceQualifier, TypeResolver, TypeResolverLevel,
};
use biome_rowan::{AstNode, Text, TextRange, TokenText};

use crate::{ModuleGraph, jsdoc_comment::JsdocComment};

use binding::{BindingId, JsBindingData};
use scope::{JsScope, JsScopeData};

pub use ad_hoc_scope_resolver::AdHocScopeResolver;
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
        module_graph: Arc<ModuleGraph>,
    ) -> Type {
        let scope = self.scope_for_range(expr.range());
        let mut resolver =
            AdHocScopeResolver::from_scope_in_module(scope, self.clone(), module_graph);
        let ty = TypeData::from_any_js_expression(&mut resolver, expr);
        resolver.run_inference();

        let ty = ty.inferred(&mut resolver);
        Type::from_data(Box::new(resolver), ty)
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

    /// All scopes in this module.
    ///
    /// The first entry is expected to be the global scope.
    pub(crate) scopes: Box<[JsScopeData]>,

    /// Lookup tree to find scopes by text range.
    pub(crate) scope_by_range: rust_lapper::Lapper<u32, ScopeId>,

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
    pub(crate) fn binding(&self, binding_id: BindingId) -> &JsBindingData {
        &self.bindings[binding_id.index()]
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

    fn get_by_resolved_id(&self, id: ResolvedTypeId) -> Option<&TypeData> {
        match id.level() {
            TypeResolverLevel::Module => Some(self.get_by_id(id.id())),
            TypeResolverLevel::Global => Some(GLOBAL_RESOLVER.get_by_id(id.id())),
            TypeResolverLevel::AdHoc | TypeResolverLevel::Import => None,
        }
    }

    fn register_type(&mut self, _type_data: TypeData) -> TypeId {
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
        if qualifier.path.len() == 1 {
            self.resolve_type_of(&qualifier.path[0])
                .or_else(|| GLOBAL_RESOLVER.resolve_qualifier(qualifier))
        } else {
            // TODO: Resolve nested qualifiers
            None
        }
    }

    fn resolve_type_of(&self, identifier: &Text) -> Option<ResolvedTypeId> {
        if let Some(export) = self.exports.get(identifier) {
            export
                .as_own_export()
                .and_then(|own_export| self.resolve_reference(&own_export.ty))
        } else {
            GLOBAL_RESOLVER.resolve_type_of(identifier)
        }
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
#[derive(Clone, Debug, PartialEq)]
pub struct JsOwnExport {
    /// Optional JSDoc comment associated with the symbol being exported.
    pub jsdoc_comment: Option<JsdocComment>,

    /// Name of the binding in the module's global scope.
    pub local_name: Option<TokenText>,

    /// Type of the exported symbol.
    pub ty: TypeReference,
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
