//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::RegistryVisitor;
use biome_js_syntax::JsLanguage;
pub fn visit_registry<V: RegistryVisitor<JsLanguage>>(registry: &mut V) {
    registry.record_category::<crate::analyzers::Analyzers>();
    registry.record_category::<crate::semantic_analyzers::SemanticAnalyzers>();
    registry.record_category::<crate::aria_analyzers::AriaAnalyzers>();
    registry.record_category::<crate::assists::Assists>();
    registry.record_category::<crate::syntax::Syntax>();
}
