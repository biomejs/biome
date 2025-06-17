//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::RegistryVisitor;
use biome_js_syntax::JsLanguage;
pub fn visit_registry<V: RegistryVisitor<JsLanguage>>(registry: &mut V) {
    registry.record_category::<crate::assist::Assist>();
    registry.record_category::<crate::lint::Lint>();
    registry.record_category::<crate::syntax::Syntax>();
}
