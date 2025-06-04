use std::{
    borrow::Cow,
    collections::{BTreeMap, btree_map::Entry},
    sync::Arc,
};

use biome_js_syntax::{AnyJsExpression, JsSyntaxNode};
use biome_js_type_info::{
    GLOBAL_RESOLVER, GLOBAL_UNKNOWN_ID, ImportSymbol, ModuleId, Resolvable, ResolvedTypeData,
    ResolvedTypeId, ScopeId, Type, TypeData, TypeId, TypeImportQualifier, TypeReference,
    TypeReferenceQualifier, TypeResolver, TypeResolverLevel, TypeStore,
};
use biome_resolver::ResolvedPath;
use biome_rowan::{AstNode, Text};
use rustc_hash::FxHashMap;

use crate::{JsExport, JsOwnExport, ModuleGraph, js_module_info::JsModuleInfoInner};

use super::JsModuleInfo;

const MAX_IMPORT_DEPTH: usize = 10; // Arbitrary depth, may require tweaking.

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

    /// Types registered within this resolver.
    types: TypeStore,

    /// Map for looking up types of expressions.
    expressions: FxHashMap<JsSyntaxNode, ResolvedTypeId>,

    /// Type index up to which inference has run thus far.
    inference_index: usize,
}

impl ScopedResolver {
    pub fn from_global_scope(module_info: JsModuleInfo, module_graph: Arc<ModuleGraph>) -> Self {
        Self {
            module_graph,
            modules: vec![module_info],
            modules_by_path: Default::default(),
            types: Default::default(),
            expressions: Default::default(),
            inference_index: 0,
        }
    }

    /// Registers all the necessary types so the given expression becomes
    /// resolvable.
    ///
    /// The given expression must be from the module for which the resolver was
    /// created.
    pub fn register_types_for_expression(&mut self, expr: &AnyJsExpression) {
        let scope_id = self.modules[0].scope_id_for_range(expr.range());

        let mut resolver = ScopeRestrictedRegistrationResolver::new(self, scope_id);
        let ty = TypeData::from_any_js_expression(&mut resolver, scope_id, expr);
        resolver.run_inference();

        let ty = ty.inferred(&mut resolver);
        let id = resolver.register_and_resolve(ty);
        self.expressions.insert(expr.syntax().clone(), id);
    }

    /// Returns the resolved type of the given expression within this module.
    ///
    /// Assumes that [`Self::register_types_for_expression()`] has already been
    /// called for this expression and the resolver's inference has run.
    pub fn resolved_type_for_expression(self: &Arc<Self>, expr: &AnyJsExpression) -> Type {
        let type_id = self
            .expressions
            .get(expr.syntax())
            .copied()
            .unwrap_or(GLOBAL_UNKNOWN_ID);

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
        let from_index = self.inference_index;

        self.resolve_all_own_types(from_index);

        // We only perform import resolution for the first inference run in the
        // global scope.
        if from_index == 0 {
            self.resolve_imports_in_modules();
        }

        self.flatten_all(from_index);

        self.inference_index = self.types.len();
    }

    fn resolve_all_own_types(&mut self, from_index: usize) {
        let mut i = from_index;
        while i < self.types.len() {
            // SAFETY: We immediately reinsert after taking.
            let ty = unsafe { self.types.take_from_index_temporarily(i) };
            let ty = ty.resolved_with_mapped_references(
                |reference, resolver| match reference {
                    TypeReference::Qualifier(_qualifier) => TypeReference::Unknown,
                    TypeReference::Import(import) => resolver
                        .resolve_import(&import)
                        .map_or(TypeReference::Unknown, Into::into),
                    other => other,
                },
                self,
            );
            // SAFETY: We reinsert before anyone got a chance to do lookups.
            unsafe { self.types.reinsert_temporarily_taken_data(i, ty) };
            i += 1;
        }
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

    fn flatten_all(&mut self, from_index: usize) {
        let mut i = from_index;
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

    /// Attempts to resolve an import reference by its qualifier.
    ///
    /// Assumes full inference has already been performed.
    fn resolve_import_reference(&self, qualifier: &TypeImportQualifier) -> Option<ResolvedTypeId> {
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

    fn register_type(&mut self, type_data: TypeData) -> TypeId {
        self.types.register_type(type_data)
    }

    fn resolve_reference(&self, ty: &TypeReference) -> Option<ResolvedTypeId> {
        match ty {
            TypeReference::Qualifier(qualifier) => self.resolve_qualifier(qualifier),
            TypeReference::Resolved(resolved_id) => Some(*resolved_id),
            TypeReference::Import(import) => self.resolve_import_reference(import),
            TypeReference::Unknown => Some(GLOBAL_UNKNOWN_ID),
        }
    }

    fn resolve_import(&mut self, qualifier: &TypeImportQualifier) -> Option<ResolvedTypeId> {
        let mut qualifier = Cow::Borrowed(qualifier);

        for _ in 0..MAX_IMPORT_DEPTH {
            let module_id = self.register_module(qualifier.resolved_path.clone())?;
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
                    qualifier = Cow::Owned(import.clone());
                }
            }
        }

        None
    }

    fn resolve_qualifier(&self, qualifier: &TypeReferenceQualifier) -> Option<ResolvedTypeId> {
        let module = &self.modules[0];

        let identifier = qualifier.path.first()?;
        let Some(binding_ref) = module.find_binding_in_scope(identifier, qualifier.scope_id) else {
            return GLOBAL_RESOLVER.resolve_qualifier(qualifier);
        };

        let binding_id = binding_ref.get_binding_id_for_qualifier(qualifier)?;

        let binding = module.binding(binding_id);
        let mut ty = if binding.declaration_kind.is_import_declaration() {
            let resolved_id = module
                .static_imports
                .get(&binding.name)
                .and_then(|import| {
                    self.resolve_import_reference(&TypeImportQualifier {
                        symbol: import.symbol.clone(),
                        resolved_path: import.resolved_path.clone(),
                        type_only: binding.declaration_kind.is_import_type_declaration(),
                    })
                })?;
            if qualifier.path.len() == 1 {
                return Some(resolved_id);
            }

            Cow::Owned(resolved_id.into())
        } else {
            Cow::Borrowed(&binding.ty)
        };

        for identifier in &qualifier.path[1..] {
            let resolved = self.resolve_and_get(&ty)?;
            let member = resolved
                .all_members(self)
                .with_excluded_binding_id(binding_id)
                .find(|member| member.is_static() && member.has_name(identifier))?;
            ty = Cow::Owned(member.ty().into_owned());
        }

        self.resolve_reference(&ty)
    }

    fn resolve_type_of(&self, identifier: &Text, scope_id: ScopeId) -> Option<ResolvedTypeId> {
        let module = &self.modules[0];
        let Some(binding_ref) = module.find_binding_in_scope(identifier, scope_id) else {
            return GLOBAL_RESOLVER.resolve_type_of(identifier, scope_id);
        };

        let binding = module.binding(binding_ref.value_ty_or_ty());
        if binding.declaration_kind.is_import_declaration() {
            module.static_imports.get(&binding.name).and_then(|import| {
                self.resolve_import_reference(&TypeImportQualifier {
                    symbol: import.symbol.clone(),
                    resolved_path: import.resolved_path.clone(),
                    type_only: binding.declaration_kind.is_import_type_declaration(),
                })
            })
        } else {
            self.resolve_reference(&binding.ty)
        }
    }

    fn fallback_resolver(&self) -> Option<&dyn TypeResolver> {
        Some(&*GLOBAL_RESOLVER)
    }

    fn registered_types(&self) -> &[TypeData] {
        self.types.as_slice()
    }
}

struct ScopeRestrictedRegistrationResolver<'a> {
    resolver: &'a mut ScopedResolver,
    scope_id: ScopeId,
}

impl<'a> ScopeRestrictedRegistrationResolver<'a> {
    fn new(resolver: &'a mut ScopedResolver, scope_id: ScopeId) -> Self {
        Self { resolver, scope_id }
    }

    fn run_inference(&mut self) {
        self.resolver.run_inference();
    }
}

impl TypeResolver for ScopeRestrictedRegistrationResolver<'_> {
    fn level(&self) -> TypeResolverLevel {
        TypeResolverLevel::Scope
    }

    fn find_type(&self, type_data: &TypeData) -> Option<TypeId> {
        self.resolver
            .find_type(&type_data.clone().with_scope_id(self.scope_id))
    }

    fn get_by_id(&self, id: TypeId) -> &TypeData {
        self.resolver.get_by_id(id)
    }

    fn get_by_resolved_id(&self, id: ResolvedTypeId) -> Option<ResolvedTypeData> {
        self.resolver.get_by_resolved_id(id)
    }

    fn register_type(&mut self, type_data: TypeData) -> TypeId {
        self.resolver
            .register_type(type_data.with_scope_id(self.scope_id))
    }

    fn resolve_reference(&self, ty: &TypeReference) -> Option<ResolvedTypeId> {
        match ty {
            TypeReference::Qualifier(qualifier) => self.resolve_qualifier(qualifier),
            TypeReference::Resolved(id) => Some(*id),
            TypeReference::Import(import) => self.resolver.resolve_import_reference(import),
            TypeReference::Unknown => Some(GLOBAL_UNKNOWN_ID),
        }
    }

    fn resolve_qualifier(&self, qualifier: &TypeReferenceQualifier) -> Option<ResolvedTypeId> {
        self.resolver
            .resolve_qualifier(&qualifier.clone().with_scope_id(self.scope_id))
    }

    fn resolve_type_of(&self, _identifier: &Text, _scope_id: ScopeId) -> Option<ResolvedTypeId> {
        panic!("ScopeRestrictedRegistrationResolver is only used for registering types")
    }

    fn registered_types(&self) -> &[TypeData] {
        panic!("ScopeRestrictedRegistrationResolver is only used for registering types")
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
