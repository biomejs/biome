use crate::analyzers::indent_size::IndentSize;
use biome_analyze::{GroupCategory, RegistryVisitor, RuleCategory, RuleGroup};
use biome_json_syntax::JsonLanguage;

mod indent_size;

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
    }
}

impl GroupCategory for MigrationCategory {
    type Language = JsonLanguage;
    const CATEGORY: RuleCategory = RuleCategory::Action;

    fn record_groups<V: RegistryVisitor<Self::Language> + ?Sized>(registry: &mut V) {
        registry.record_group::<MigrationGroup>();
    }
}
