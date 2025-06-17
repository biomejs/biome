use std::{
    borrow::Cow,
    collections::{BTreeMap, btree_map::Entry},
    sync::Arc,
};

use biome_js_syntax::AnyJsExpression;
use biome_js_type_info::{
    GLOBAL_RESOLVER, GLOBAL_UNKNOWN_ID, ImportSymbol, ModuleId, Resolvable, ResolvedTypeData,
    ResolvedTypeId, ResolverId, ScopeId, Type, TypeData, TypeId, TypeImportQualifier,
    TypeReference, TypeReferenceQualifier, TypeResolver, TypeResolverLevel, TypeStore,
};
use biome_resolver::ResolvedPath;
use biome_rowan::{AstNode, Text, TextRange};
use rustc_hash::FxHashMap;

use crate::{JsExport, JsOwnExport, ModuleGraph, js_module_info::JsModuleInfoInner};

use super::JsModuleInfo;

const MAX_IMPORT_DEPTH: usize = 10; // Arbitrary depth, may require tweaking.

// Any references that resolve to module 0 will be rewritten to
// reference the scoped resolver. The reason we do this is that any
// references to other modules can be resolved within the scoped
// resolver, whereas the references that still point to the module types
// would remain unresolved.
const MODULE_0_ID: ResolverId = ResolverId::from_level(TypeResolverLevel::Module);

/// Type resolver that is able to resolve types from arbitrary scopes in a
/// module.
///
/// The scoped resolver can register types in order to infer the type of
/// expressions, but it cannot create bindings for such types. For this reason,
/// whenever the type of an expression is inferred, a `ScopeId` needs to be
/// provided explicitly.
///
/// The scoped resolver is also able to resolve imported symbols from other
/// modules, but any expressions it evaluates must be from the module for
/// which the resolver was created.
#[derive(Debug)]
pub struct ScopedResolver {
    module_graph: Arc<ModuleGraph>,

    /// Modules from which this resolver is using types.
    ///
    /// Any scopes referenced by this resolver are always assumed to be part of
    /// the first module in this vector. Other modules are those imported by
    /// the first module, either directly or transitively.
    modules: Vec<JsModuleInfo>,

    /// Map of module IDs keyed by their resolved paths.
    ///
    /// Module IDs are used to index the [`Self::modules`] vector.
    pub modules_by_path: BTreeMap<ResolvedPath, ModuleId>,

    /// Parsed expressions, mapped by their starting index to the ID of their
    /// type.
    expressions: FxHashMap<TextRange, TypeId>,

    /// Types registered within this resolver.
    types: TypeStore,

    /// Maps from `TypeId` indices in `modules[0]` to our own modules.
    type_id_map: Vec<TypeId>,
}

impl ScopedResolver {
    pub fn from_global_scope(module_info: JsModuleInfo, module_graph: Arc<ModuleGraph>) -> Self {
        Self {
            module_graph,
            modules: vec![module_info],
            modules_by_path: Default::default(),
            expressions: Default::default(),
            types: Default::default(),
            type_id_map: Default::default(),
        }
    }

    /// Returns the resolved type of the given expression within this module.
    ///
    /// Assumes that [`Self::register_types_for_expression()`] has already been
    /// called for this expression and the resolver's inference has run.
    pub fn resolved_type_for_expression(self: &Arc<Self>, expr: &AnyJsExpression) -> Type {
        let type_id = self.resolved_id_for_expression(expr);
        Type::from_id(self.clone(), type_id)
    }

    fn find_module(&self, path: &ResolvedPath) -> Option<ModuleId> {
        self.modules_by_path.get(path).copied()
    }

    fn register_module(&mut self, path: ResolvedPath) -> Option<ModuleId> {
        match self.modules_by_path.entry(path) {
            Entry::Occupied(entry) => Some(*entry.get()),
            Entry::Vacant(entry) => {
                let path = entry.key().as_path()?;
                let module_info = self.module_graph.module_info_for_path(path)?;
                let module_id = ModuleId::new(self.modules.len());
                self.modules.push(module_info);
                Some(*entry.insert(module_id))
            }
        }
    }

    pub fn run_inference(&mut self) {
        self.resolve_imports_in_modules();
        self.resolve_all();
        self.flatten_all();
    }

    /// Actively resolves imports in the modules we are of.
    fn resolve_imports_in_modules(&mut self) {
        let mut i = 0;
        while i < self.modules.len() {
            let module = self.modules[i].clone();
            for resolved_path in module.static_import_paths.values() {
                self.register_module(resolved_path.clone());
            }

            i += 1;
        }
    }

    fn resolve_all(&mut self) {
        self.type_id_map = self.modules[0]
            .types
            .iter()
            .map(|ty| self.types.register_type(Cow::Borrowed(ty)))
            .collect();

        for (range, type_id) in &self.modules[0].expressions {
            self.expressions
                .insert(*range, self.type_id_map[type_id.index()]);
        }

        let mut i = 0;
        while i < self.types.len() {
            // SAFETY: We reinsert before anyone got a chance to do lookups.
            unsafe {
                let ty = self.types.take_from_index_temporarily(i);
                let ty = ty.resolved(self);
                self.types.reinsert_temporarily_taken_data(i, ty);
            }
            i += 1;
        }
    }

    fn flatten_all(&mut self) {
        let mut i = 0;
        while i < self.types.len() {
            // SAFETY: We reinsert before anyone got a chance to do lookups.
            unsafe {
                let ty = self.types.take_from_index_temporarily(i);
                let ty = ty.flattened(self);
                self.types.reinsert_temporarily_taken_data(i, ty);
            }
            i += 1;
        }
    }

    pub fn resolved_id_for_expression(&self, expr: &AnyJsExpression) -> ResolvedTypeId {
        match self.expressions.get(&expr.range()) {
            Some(id) => ResolvedTypeId::new(TypeResolverLevel::Scope, *id),
            None => GLOBAL_UNKNOWN_ID,
        }
    }
}

impl TypeResolver for ScopedResolver {
    fn level(&self) -> TypeResolverLevel {
        TypeResolverLevel::Scope
    }

    fn find_type(&self, type_data: &TypeData) -> Option<TypeId> {
        self.types.find_type(type_data)
    }

    fn get_by_id(&self, id: TypeId) -> &TypeData {
        self.types.get_by_id(id)
    }

    fn get_by_resolved_id(&self, id: ResolvedTypeId) -> Option<ResolvedTypeData> {
        match id.level() {
            TypeResolverLevel::Scope => Some((id, self.get_by_id(id.id())).into()),
            TypeResolverLevel::Module => {
                let module_id = id.module_id();
                let module = &self.modules[module_id.index()];
                if let Some(ty) = module.types.get(id.index()) {
                    Some((id, ty).into())
                } else {
                    debug_assert!(false, "Invalid type reference: {id:?}");
                    None
                }
            }
            TypeResolverLevel::Import => {
                panic!("import IDs should not be exposed outside the module info collector")
            }
            TypeResolverLevel::Global => Some((id, GLOBAL_RESOLVER.get_by_id(id.id())).into()),
        }
    }

    fn register_type(&mut self, type_data: Cow<TypeData>) -> TypeId {
        self.types.register_type(type_data)
    }

    fn resolve_reference(&self, ty: &TypeReference) -> Option<ResolvedTypeId> {
        match ty {
            TypeReference::Qualifier(_qualifier) => None,
            TypeReference::Resolved(resolved_id) => {
                Some(if resolved_id.resolver_id() == MODULE_0_ID {
                    ResolvedTypeId::new(
                        TypeResolverLevel::Scope,
                        self.type_id_map[resolved_id.id().index()],
                    )
                } else {
                    *resolved_id
                })
            }
            TypeReference::Import(import) => self.resolve_import(import),
            TypeReference::Unknown => Some(GLOBAL_UNKNOWN_ID),
        }
    }

    fn resolve_import(&self, qualifier: &TypeImportQualifier) -> Option<ResolvedTypeId> {
        let mut qualifier = Cow::Borrowed(qualifier);

        for _ in 0..MAX_IMPORT_DEPTH {
            let module_id = self.find_module(&qualifier.resolved_path)?;
            let module = &self.modules[module_id.index()];

            let name = match &qualifier.symbol {
                ImportSymbol::Default => "default",
                ImportSymbol::Named(name) => name.text(),
                ImportSymbol::All => {
                    // TODO: Register type for imported namespace.
                    return None;
                }
            };

            let export = match module.exports.get(name) {
                Some(JsExport::Own(export) | JsExport::OwnType(export)) => export,
                Some(JsExport::Reexport(reexport)) => {
                    qualifier = Cow::Owned(TypeImportQualifier {
                        symbol: reexport.import.symbol.clone(),
                        resolved_path: reexport.import.resolved_path.clone(),
                        type_only: false,
                    });
                    continue;
                }
                Some(JsExport::ReexportType(reexport)) => {
                    qualifier = Cow::Owned(TypeImportQualifier {
                        symbol: reexport.import.symbol.clone(),
                        resolved_path: reexport.import.resolved_path.clone(),
                        type_only: true,
                    });
                    continue;
                }
                None => {
                    // TODO: Follow blanket reexports.
                    return None;
                }
            };

            match resolve_from_export(module_id, module, export) {
                ResolveFromExportResult::Resolved(resolved) => return resolved,
                ResolveFromExportResult::FollowImport(import) => {
                    qualifier = Cow::Borrowed(import);
                }
            }
        }

        None
    }

    fn resolve_qualifier(&self, _qualifier: &TypeReferenceQualifier) -> Option<ResolvedTypeId> {
        // We rely on qualifiers to have been resolved during the construction
        // of the module graph.
        None
    }

    fn resolve_type_of(&self, identifier: &Text, scope_id: ScopeId) -> Option<ResolvedTypeId> {
        let module = &self.modules[0];
        let Some(binding_ref) = module.find_binding_in_scope(identifier, scope_id) else {
            return GLOBAL_RESOLVER.resolve_type_of(identifier, scope_id);
        };

        let binding = module.binding(binding_ref.value_ty_or_ty());
        if binding.declaration_kind.is_import_declaration() {
            module.static_imports.get(&binding.name).and_then(|import| {
                self.resolve_import(&TypeImportQualifier {
                    symbol: import.symbol.clone(),
                    resolved_path: import.resolved_path.clone(),
                    type_only: binding.declaration_kind.is_import_type_declaration(),
                })
            })
        } else {
            self.resolve_reference(&binding.ty)
        }
    }

    fn resolve_expression(&mut self, _scope_id: ScopeId, expr: &AnyJsExpression) -> Cow<TypeData> {
        let id = self.resolved_id_for_expression(expr);
        match self.get_by_resolved_id(id) {
            Some(resolved) => Cow::Owned(resolved.to_data()),
            None => Cow::Owned(TypeData::unknown()),
        }
    }

    fn fallback_resolver(&self) -> Option<&dyn TypeResolver> {
        Some(&*GLOBAL_RESOLVER)
    }

    fn registered_types(&self) -> &[TypeData] {
        self.types.as_slice()
    }
}

enum ResolveFromExportResult<'a> {
    Resolved(Option<ResolvedTypeId>),
    FollowImport(&'a TypeImportQualifier),
}

/// Resolves an export from the given `module` with the given `module_id`.
///
/// If the export can be resolved within the given module, an optional
/// [`ResolvedTypeId`] is returned. If the export references another module, it
/// returns the [`TypeImportQualifier`] to follow.
#[inline]
fn resolve_from_export<'a>(
    module_id: ModuleId,
    module: &'a JsModuleInfoInner,
    export: &JsOwnExport,
) -> ResolveFromExportResult<'a> {
    let resolved = match export {
        JsOwnExport::Binding(binding_id) => match &module.bindings[binding_id.index()].ty {
            TypeReference::Qualifier(_qualifier) => {
                // If it wasn't resolved before exporting, we can't help it
                // anymore.
                None
            }
            TypeReference::Resolved(resolved_id) => Some(resolved_id.with_module_id(module_id)),
            TypeReference::Import(import) => {
                return ResolveFromExportResult::FollowImport(import);
            }
            TypeReference::Unknown => None,
        },
        JsOwnExport::Type(resolved_id) => Some(resolved_id.with_module_id(module_id)),
    };

    ResolveFromExportResult::Resolved(resolved)
}
