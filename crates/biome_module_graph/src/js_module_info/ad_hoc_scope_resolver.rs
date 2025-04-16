use biome_js_type_info::{Type, TypeReferenceQualifier, TypeResolver};
use biome_rowan::Text;

use crate::ModuleGraph;

use super::{JsModuleInfo, scope::JsScope};

/// Type resolver that is able to resolve types from an arbitrary scope on an
/// ad-hoc basic.
///
/// TODO: The ad-hoc basis of this resolver means we don't cache the lookups it
///       performs. This probably needs revising still.
pub(super) struct AdHocScopeResolver<'a> {
    scope: JsScope,
    module_info: &'a JsModuleInfo,
    module_graph: &'a ModuleGraph,
}

impl<'a> AdHocScopeResolver<'a> {
    pub(super) fn from_scope_in_module(
        scope: JsScope,
        module_info: &'a JsModuleInfo,
        module_graph: &'a ModuleGraph,
    ) -> Self {
        Self {
            scope,
            module_info,
            module_graph,
        }
    }
}

impl TypeResolver for AdHocScopeResolver<'_> {
    fn resolve_qualifier(&self, qualifier: &TypeReferenceQualifier) -> Option<Type> {
        if qualifier.parts().len() == 1 {
            self.resolve_type_of(&qualifier.parts()[0])
        } else {
            // TODO: Resolve nested qualifiers
            None
        }
    }

    fn resolve_type_of(&self, identifier: &Text) -> Option<Type> {
        let mut scope = self.scope.clone();
        loop {
            if let Some(binding) = scope.get_binding(identifier.text()) {
                if binding.is_imported() {
                    return self
                        .module_info
                        .find_exported_symbol(self.module_graph, &binding.name())
                        .map(|export| export.ty);
                } else {
                    let ty = binding.ty();
                    return match ty.is_inferred() {
                        true => Some(ty.clone()),
                        false => None,
                    };
                }
            }

            match scope.parent() {
                Some(parent) => scope = parent,
                None => break,
            }
        }

        // globals fallback
        match identifier.text() {
            "Array" => Some(Type::array_of(Type::unknown())),
            "Promise" => Some(Type::promise_of(Type::unknown())),
            "globalThis" | "window" => Some(Type::window()),
            _ => None,
        }
    }
}
