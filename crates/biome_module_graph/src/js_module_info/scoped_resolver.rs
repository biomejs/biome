use std::{
    borrow::Cow,
    collections::{BTreeMap, btree_map::Entry},
    sync::Arc,
};

use biome_js_syntax::{AnyJsExpression, JsSyntaxNode};
use biome_js_type_info::{
    GLOBAL_RESOLVER, GLOBAL_UNKNOWN_ID, ImportSymbol, ModuleId, Resolvable, ResolvedPath,
    ResolvedTypeId, ScopeId, Type, TypeData, TypeId, TypeImportQualifier, TypeReference,
    TypeReferenceQualifier, TypeResolver, TypeResolverLevel,
};
use biome_rowan::{AstNode, Text};
use rustc_hash::FxHashMap;

use crate::{JsExport, ModuleGraph};

use super::JsModuleInfo;

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
#[derive(Clone, Debug)]
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
    types: Vec<TypeData>,

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
        let ty = TypeData::from_any_js_expression(&mut resolver, expr);
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
            // First take the type to satisfy the borrow checker:
            let ty = std::mem::take(&mut self.types[i]);
            self.types[i] = ty.resolved_with_mapped_references(
                |reference, resolver| match reference {
                    TypeReference::Import(import) => resolver
                        .resolve_import(&import)
                        .map_or(TypeReference::Unknown, Into::into),
                    other => other,
                },
                self,
            );
            i += 1;
        }
    }

    /// Actively resolves imports in the modules we are of.
    fn resolve_imports_in_modules(&mut self) {
        let mut i = 0;
        while i < self.modules.len() {
            let module = self.modules[i].clone();
            let module_id = ModuleId::new(i);
            for ty in &module.types {
                // We don't care about the returned type here. Instead, we rely
                // on the side-effect of registering the resolved imports.
                let _ = ty.resolved_with_module_id(module_id, self);
            }

            i += 1;
        }
    }

    fn flatten_all(&mut self, from_index: usize) {
        let mut i = from_index;
        while i < self.types.len() {
            // First take the type to satisfy the borrow checker:
            let ty = std::mem::take(&mut self.types[i]);
            self.types[i] = ty.flattened(self);
            i += 1;
        }
    }

    /// Attempts to resolve an import reference by its qualifier.
    ///
    /// Assumes full inference has already been performed.
    fn resolve_import_reference(&self, qualifier: &TypeImportQualifier) -> Option<ResolvedTypeId> {
        const MAX_DEPTH: usize = 10; // Arbitrary depth, may require tweaking.

        let mut qualifier = Cow::Borrowed(qualifier);

        for _ in 0..MAX_DEPTH {
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
                Some(JsExport::Reexport(reexport) | JsExport::ReexportType(reexport)) => {
                    qualifier = Cow::Owned(TypeImportQualifier {
                        symbol: reexport.import.symbol.clone(),
                        resolved_path: reexport.import.resolved_path.clone(),
                    });
                    continue;
                }
                None => {
                    // TODO: Follow blanket reexports.
                    return None;
                }
            };

            return match &export.ty {
                TypeReference::Qualifier(_qualifier) => {
                    // If it wasn't resolved before exporting, we can't
                    // help it anymore.
                    None
                }
                TypeReference::Resolved(resolved_id) => {
                    let data = module.get_by_resolved_id(*resolved_id)?;
                    let data = data.clone().with_module_id(module_id);
                    self.find_type(&data)
                        .map(|type_id| ResolvedTypeId::new(TypeResolverLevel::Scope, type_id))
                }
                TypeReference::Import(import) => {
                    qualifier = Cow::Borrowed(import);
                    continue;
                }
                TypeReference::Unknown => None,
            };
        }

        None
    }
}

impl TypeResolver for ScopedResolver {
    fn level(&self) -> TypeResolverLevel {
        TypeResolverLevel::Scope
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

    fn get_by_resolved_id(&self, resolved_id: ResolvedTypeId) -> Option<&TypeData> {
        match resolved_id.level() {
            TypeResolverLevel::Scope => Some(self.get_by_id(resolved_id.id())),
            TypeResolverLevel::Module => {
                let module_id = resolved_id.module_id();
                Some(&self.modules[module_id.index()].types[resolved_id.index()])
            }
            TypeResolverLevel::Import => {
                panic!("import IDs should not be exposed outside the module info collector")
            }
            TypeResolverLevel::Global => Some(GLOBAL_RESOLVER.get_by_id(resolved_id.id())),
        }
    }

    fn register_type(&mut self, type_data: TypeData) -> TypeId {
        // Searching linearly may potentially become quite expensive, but it
        // should be outweighed by index lookups quite heavily.
        match self.types.iter().position(|data| data == &type_data) {
            Some(index) => TypeId::new(index),
            None => {
                let id = TypeId::new(self.types.len());
                self.types.push(type_data);
                id
            }
        }
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
            Some(JsExport::Reexport(reexport) | JsExport::ReexportType(reexport)) => {
                return Some(
                    self.register_and_resolve(TypeData::reference(TypeImportQualifier {
                        symbol: reexport.import.symbol.clone(),
                        resolved_path: reexport.import.resolved_path.clone(),
                    })),
                );
            }
            None => {
                // TODO: Follow blanket reexports.
                return None;
            }
        };

        match &export.ty {
            TypeReference::Qualifier(_qualifier) => {
                // If it wasn't resolved before exporting, we can't
                // help it anymore.
                None
            }
            TypeReference::Resolved(resolved) => {
                let data = module.resolve_and_get_with_module_id(&(*resolved).into(), module_id)?;
                Some(self.register_and_resolve(data.clone().with_module_id(module_id)))
            }
            TypeReference::Import(import) => {
                Some(self.register_and_resolve(TypeData::reference(import.clone())))
            }
            TypeReference::Unknown => None,
        }
    }

    fn resolve_qualifier(&self, qualifier: &TypeReferenceQualifier) -> Option<ResolvedTypeId> {
        if qualifier.path.len() == 1 {
            self.resolve_type_of(
                &qualifier.path[0],
                qualifier.scope_id.unwrap_or(ScopeId::GLOBAL),
            )
            .or_else(|| GLOBAL_RESOLVER.resolve_qualifier(qualifier))
        } else {
            // TODO: Resolve nested qualifiers
            None
        }
    }

    fn resolve_type_of(&self, identifier: &Text, scope_id: ScopeId) -> Option<ResolvedTypeId> {
        let module = &self.modules[0];
        let mut scope = &module.scopes[scope_id.index()];
        loop {
            if let Some(binding) = scope
                .bindings_by_name
                .get(identifier.text())
                .map(|id| &module.bindings[id.index()])
            {
                return if binding.declaration_kind.is_import_declaration() {
                    module.static_imports.get(&binding.name).and_then(|import| {
                        self.resolve_import_reference(&TypeImportQualifier {
                            symbol: import.symbol.clone(),
                            resolved_path: import.resolved_path.clone(),
                        })
                    })
                } else {
                    self.resolve_reference(&binding.ty)
                };
            }

            match &scope.parent {
                Some(parent_id) => scope = &module.scopes[parent_id.index()],
                None => break,
            }
        }

        GLOBAL_RESOLVER.resolve_type_of(identifier, scope_id)
    }

    fn fallback_resolver(&self) -> Option<&dyn TypeResolver> {
        Some(&*GLOBAL_RESOLVER)
    }

    fn registered_types(&self) -> &[TypeData] {
        &self.types
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

    fn get_by_resolved_id(&self, id: ResolvedTypeId) -> Option<&TypeData> {
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
