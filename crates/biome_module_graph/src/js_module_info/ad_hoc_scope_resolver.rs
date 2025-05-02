use std::{
    borrow::Cow,
    collections::{BTreeMap, btree_map::Entry},
    sync::Arc,
};

use biome_js_type_info::{
    GLOBAL_RESOLVER, GLOBAL_UNKNOWN_ID, ImportSymbol, ModuleId, Resolvable, ResolvedPath,
    ResolvedTypeId, TypeData, TypeId, TypeImportQualifier, TypeReference, TypeReferenceQualifier,
    TypeResolver, TypeResolverLevel,
};
use biome_rowan::Text;

use crate::ModuleGraph;

use super::{JsModuleInfo, scope::JsScope};

/// Type resolver that is able to resolve types from an arbitrary scope on an
/// ad-hoc basic.
///
/// The ad-hoc scope resolver can register types in order to infer the type of
/// expressions, but it cannot create bindings for such types. Any references to
/// named bindings are assumed to come from the scope in the module the resolver
/// applies to.
pub struct AdHocScopeResolver {
    scope: JsScope,
    module_graph: Arc<ModuleGraph>,
    modules: Vec<JsModuleInfo>,
    pub modules_by_path: BTreeMap<ResolvedPath, ModuleId>,
    types: Vec<TypeData>,
}

impl AdHocScopeResolver {
    pub fn from_scope_in_module(
        scope: JsScope,
        module_info: JsModuleInfo,
        module_graph: Arc<ModuleGraph>,
    ) -> Self {
        Self {
            scope,
            module_graph,
            modules: vec![module_info],
            modules_by_path: Default::default(),
            types: Default::default(),
        }
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
        self.resolve_all_own_types();
        self.resolve_imports_in_modules();
        self.flatten_all();
    }

    fn resolve_all_own_types(&mut self) {
        let mut i = 0;
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

    fn flatten_all(&mut self) {
        let mut i = 0;
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

            let export = module.find_exported_symbol(self.module_graph.as_ref(), name)?;
            return match export.ty {
                TypeReference::Qualifier(_qualifier) => {
                    // If it wasn't resolved before exporting, we can't
                    // help it anymore.
                    None
                }
                TypeReference::Resolved(resolved_id) => {
                    let data = module.get_by_resolved_id(resolved_id)?;
                    let data = data.clone().with_module_id(module_id);
                    self.find_type(&data)
                        .map(|type_id| ResolvedTypeId::new(TypeResolverLevel::AdHoc, type_id))
                }
                TypeReference::Import(import) => {
                    qualifier = Cow::Owned(import);
                    continue;
                }
                TypeReference::Unknown => None,
            };
        }

        None
    }
}

impl TypeResolver for AdHocScopeResolver {
    fn level(&self) -> TypeResolverLevel {
        TypeResolverLevel::AdHoc
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
            TypeResolverLevel::AdHoc => Some(self.get_by_id(resolved_id.id())),
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

        let export = module.find_exported_symbol(self.module_graph.as_ref(), name)?;
        match export.ty {
            TypeReference::Qualifier(_qualifier) => {
                // If it wasn't resolved before exporting, we can't
                // help it anymore.
                None
            }
            TypeReference::Resolved(resolved) => {
                let data = module.resolve_and_get_with_module_id(&resolved.into(), module_id)?;
                Some(self.register_and_resolve(data.clone().with_module_id(module_id)))
            }
            TypeReference::Import(import) => {
                Some(self.register_and_resolve(TypeData::reference(import)))
            }
            TypeReference::Unknown => None,
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
        let mut scope = self.scope.clone();
        loop {
            if let Some(binding) = scope.get_binding(identifier.text()) {
                return if binding.is_imported() {
                    self.modules[0]
                        .find_exported_symbol(self.module_graph.as_ref(), &binding.name())
                        .and_then(|export| self.resolve_reference(&export.ty))
                } else {
                    self.resolve_reference(binding.ty())
                };
            }

            match scope.parent() {
                Some(parent) => scope = parent,
                None => break,
            }
        }

        GLOBAL_RESOLVER.resolve_type_of(identifier)
    }

    fn fallback_resolver(&self) -> Option<&dyn TypeResolver> {
        Some(&*GLOBAL_RESOLVER)
    }

    fn registered_types(&self) -> &[TypeData] {
        &self.types
    }
}
