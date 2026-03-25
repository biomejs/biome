//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::RegistryVisitor;
use biome_tailwind_syntax::TailwindLanguage;

pub fn visit_registry<V: RegistryVisitor<TailwindLanguage>>(registry: &mut V) {
    registry.record_category::<crate::lint::Lint>();
}
