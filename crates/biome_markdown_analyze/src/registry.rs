//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::RegistryVisitor;
use biome_markdown_syntax::MarkdownLanguage;
pub fn visit_registry<V: RegistryVisitor<MarkdownLanguage>>(registry: &mut V) {
    registry.record_category::<crate::lint::Lint>();
}
