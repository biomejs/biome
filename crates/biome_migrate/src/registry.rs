use crate::analyzers::MigrationCategory;
use biome_analyze::RegistryVisitor;
use rome_json_syntax::JsonLanguage;

pub fn visit_migration_registry<V: RegistryVisitor<JsonLanguage>>(registry: &mut V) {
    registry.record_category::<MigrationCategory>();
}
