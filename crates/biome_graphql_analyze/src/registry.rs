//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::RegistryVisitor;
use biome_graphql_syntax::GraphqlLanguage;
pub fn visit_registry<V: RegistryVisitor<GraphqlLanguage>>(registry: &mut V) {
    registry.record_category::<crate::lint::Lint>();
}
