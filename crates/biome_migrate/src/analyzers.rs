use crate::analyzers::rule_set::RuleSet;
use biome_analyze::{GroupCategory, RegistryVisitor, RuleCategory, RuleGroup};
use rome_json_syntax::JsonLanguage;

mod rule_set;

pub(crate) struct MigrationGroup;
pub(crate) struct MigrationCategory;

impl RuleGroup for MigrationGroup {
    type Language = JsonLanguage;
    type Category = MigrationCategory;
    const NAME: &'static str = "migrations";

    fn record_rules<V: RegistryVisitor<Self::Language> + ?Sized>(registry: &mut V) {
        // Order here is important, rules should be added from the most old, to the most recent
        // v13.0.0
        registry.record_rule::<RuleSet>();
    }
}

impl GroupCategory for MigrationCategory {
    type Language = JsonLanguage;
    const CATEGORY: RuleCategory = RuleCategory::Action;

    fn record_groups<V: RegistryVisitor<Self::Language> + ?Sized>(registry: &mut V) {
        registry.record_group::<MigrationGroup>();
    }
}
