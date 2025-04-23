use biome_js_type_info::{Type, TypeReferenceQualifier, TypeResolver};
use biome_rowan::Text;

use super::collector::JsModuleInfoCollector;

/// Type resolver that is able to resolve types within the global scope of a
/// module.
///
/// Responsible for what we call "thin type resolution". See [TypeResolver] for
/// more information.
pub(super) struct GlobalScopeResolver<'a> {
    collector: &'a JsModuleInfoCollector,
}

impl<'a> GlobalScopeResolver<'a> {
    pub(super) fn from_collector(collector: &'a JsModuleInfoCollector) -> Self {
        Self { collector }
    }
}

impl TypeResolver for GlobalScopeResolver<'_> {
    fn resolve_qualifier(&self, qualifier: &TypeReferenceQualifier) -> Option<Type> {
        if qualifier.parts().len() == 1 {
            self.resolve_type_of(&qualifier.parts()[0]).or_else(|| {
                qualifier
                    .is_array()
                    .then(|| Type::promise_of(Type::unknown()))
            })
        } else {
            // TODO: Resolve nested qualifiers
            None
        }
    }

    fn resolve_type_of(&self, identifier: &Text) -> Option<Type> {
        self.collector.scopes[0]
            .bindings_by_name
            .get(identifier.text())
            .and_then(|binding_id| {
                let ty = &self.collector.bindings[binding_id.index()].ty;
                match ty.is_inferred() {
                    true => Some(ty.clone()),
                    false => None,
                }
            })
    }
}
