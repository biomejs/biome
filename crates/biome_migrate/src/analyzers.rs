use crate::analyzers::indent_size::IndentSize;
use crate::analyzers::schema::Schema;
use biome_analyze::{GroupCategory, RegistryVisitor, RuleCategory, RuleGroup};
use biome_json_syntax::JsonLanguage;

mod indent_size;
mod schema;

pub(crate) struct MigrationGroup;
pub(crate) struct MigrationCategory;

impl RuleGroup for MigrationGroup {
    type Language = JsonLanguage;
    type Category = MigrationCategory;
    const NAME: &'static str = "migrations";

    fn record_rules<V: RegistryVisitor<Self::Language> + ?Sized>(registry: &mut V) {
        // Order here is important, rules should be added from the most old, to the most recent
        // v1.3.0
        registry.record_rule::<IndentSize>();
        // v1.5.0
        registry.record_rule::<Schema>()
    }
}

impl GroupCategory for MigrationCategory {
    type Language = JsonLanguage;
    const CATEGORY: RuleCategory = RuleCategory::Action;

    fn record_groups<V: RegistryVisitor<Self::Language> + ?Sized>(registry: &mut V) {
        registry.record_group::<MigrationGroup>();
    }
}
