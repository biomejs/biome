//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::RegistryVisitor;
use biome_json_syntax::JsonLanguage;
pub fn visit_registry<V: RegistryVisitor<JsonLanguage>>(registry: &mut V) {
    registry.record_category::<crate::lint::Lint>();
    registry.record_category::<crate::assists::Assists>();
}
