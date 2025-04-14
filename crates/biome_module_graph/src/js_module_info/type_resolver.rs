use biome_js_type_info::{Type, TypeReferenceQualifier, TypeResolver};
use biome_rowan::Text;

use super::collector::JsModuleInfoCollector;

/// Type resolver that is able to resolve types within a given scope of a
/// module.
///
/// Responsible for what we call "thin type resolution". See [TypeResolver] for
/// more information.
pub(super) struct JsModuleTypeResolver<'a> {
    collector: &'a JsModuleInfoCollector,
}

impl<'a> JsModuleTypeResolver<'a> {
    pub(super) fn from_collector(collector: &'a JsModuleInfoCollector) -> Self {
        Self { collector }
    }
}

impl TypeResolver for JsModuleTypeResolver<'_> {
    fn resolve_qualifier(&self, qualifier: &TypeReferenceQualifier) -> Option<Type> {
        if qualifier.parts().len() == 1 {
            self.resolve_type_of(&qualifier.parts()[0])
        } else {
            // TODO: Resolve nested qualifiers
            None
        }
    }

    fn resolve_type_of(&self, identifier: &Text) -> Option<Type> {
        // TODO: We probably need to traverse parent scopes at some point.
        //       But since we mainly care about exported symbols, which only
        //       exist at the module's global scope, we can get away with this
        //       for now.
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
