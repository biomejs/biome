//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::RegistryVisitor;
use biome_css_syntax::CssLanguage;
pub fn visit_registry<V: RegistryVisitor<CssLanguage>>(registry: &mut V) {
    registry.record_category::<crate::assist::Assist>();
    registry.record_category::<crate::lint::Lint>();
}
