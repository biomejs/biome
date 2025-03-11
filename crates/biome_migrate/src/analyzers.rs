use crate::analyzers::all::RulesAll;
use crate::analyzers::deleted_rules::DeletedRules;
use crate::analyzers::includes::Includes;
use crate::analyzers::no_var::NoVar;
use crate::analyzers::nursery_rules::NurseryRules;
use crate::analyzers::organize_imports::OrganizeImports;
use crate::analyzers::schema::Schema;
use crate::analyzers::style_rules::StyleRules;
use crate::analyzers::trailing_comma::TrailingComma;
use crate::analyzers::use_while::UseWhile;
use biome_analyze::{GroupCategory, RegistryVisitor, RuleCategory, RuleGroup};
use biome_json_syntax::JsonLanguage;
use use_naming_convention_enum_member_case::UseNamingConventionEnumMemberCase;

mod all;
mod deleted_rules;
mod includes;
mod no_var;
mod nursery_rules;
mod organize_imports;
mod schema;
mod style_rules;
mod trailing_comma;
mod use_naming_convention_enum_member_case;
mod use_while;

pub(crate) struct MigrationGroup;
pub(crate) struct MigrationCategory;

impl RuleGroup for MigrationGroup {
    type Language = JsonLanguage;
    type Category = MigrationCategory;
    const NAME: &'static str = "migrations";

    fn record_rules<V: RegistryVisitor<Self::Language> + ?Sized>(registry: &mut V) {
        // Order here is important, rules should be added from the most old, to the most recent
        // v1.5.0
        registry.record_rule::<Schema>();
        // v1.8.0
        registry.record_rule::<NurseryRules>();
        // v2.0.0
        registry.record_rule::<RulesAll>();
        registry.record_rule::<StyleRules>();
        registry.record_rule::<NoVar>();
        registry.record_rule::<DeletedRules>();
        registry.record_rule::<UseWhile>();
        registry.record_rule::<OrganizeImports>();
        registry.record_rule::<Includes>();
        registry.record_rule::<TrailingComma>();
        registry.record_rule::<UseNamingConventionEnumMemberCase>();
    }
}

impl GroupCategory for MigrationCategory {
    type Language = JsonLanguage;
    const CATEGORY: RuleCategory = RuleCategory::Action;

    fn record_groups<V: RegistryVisitor<Self::Language> + ?Sized>(registry: &mut V) {
        registry.record_group::<MigrationGroup>();
    }
}
