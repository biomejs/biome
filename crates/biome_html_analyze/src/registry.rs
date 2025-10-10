//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::RegistryVisitor;
use biome_html_syntax::HtmlLanguage;
pub fn visit_registry<V: RegistryVisitor<HtmlLanguage>>(registry: &mut V) {
    registry.record_category::<crate::lint::Lint>();
}
