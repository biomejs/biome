use std::sync::Arc;

use biome_js_type_info::{
    GLOBAL_RESOLVER, GLOBAL_UNKNOWN_ID, Resolvable, ResolvedTypeId, TypeData, TypeId,
    TypeReference, TypeReferenceQualifier, TypeResolver, TypeResolverLevel,
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
pub(super) struct AdHocScopeResolver {
    scope: JsScope,
    module_info: JsModuleInfo,
    module_graph: Arc<ModuleGraph>,
    types: Vec<TypeData>,
}

impl AdHocScopeResolver {
    pub(super) fn from_scope_in_module(
        scope: JsScope,
        module_info: JsModuleInfo,
        module_graph: Arc<ModuleGraph>,
    ) -> Self {
        Self {
            scope,
            module_info,
            module_graph,
            types: Vec::new(),
        }
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

    fn get_by_resolved_id(&self, id: ResolvedTypeId) -> Option<&TypeData> {
        match id.level() {
            TypeResolverLevel::AdHoc => Some(self.get_by_id(id.id())),
            TypeResolverLevel::Module => Some(&self.module_info.types[id.index()]),
            TypeResolverLevel::Project => todo!("Project-level inference not yet implemented"),
            TypeResolverLevel::Global => Some(GLOBAL_RESOLVER.get_by_id(id.id())),
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
            TypeReference::Imported(_) => None,
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
        let mut scope = self.scope.clone();
        loop {
            if let Some(binding) = scope.get_binding(identifier.text()) {
                return if binding.is_imported() {
                    self.module_info
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

    fn resolve_all(&mut self) {
        let mut i = 0;
        while i < self.types.len() {
            // We need to swap to satisfy the borrow checker:
            let mut ty = TypeData::Unknown;
            std::mem::swap(&mut ty, &mut self.types[i]);
            let mut ty = ty.resolved(self);
            std::mem::swap(&mut ty, &mut self.types[i]);
            i += 1;
        }
    }

    fn flatten_all(&mut self) {
        let mut i = 0;
        while i < self.types.len() {
            // We need to swap to satisfy the borrow checker:
            let mut ty = TypeData::Unknown;
            std::mem::swap(&mut ty, &mut self.types[i]);
            let mut ty = ty.flattened(self);
            std::mem::swap(&mut ty, &mut self.types[i]);
            i += 1;
        }
    }
}
