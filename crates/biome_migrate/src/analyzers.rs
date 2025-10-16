use crate::analyzers::all::RulesAll;
use crate::analyzers::ignore_scanner::IgnoreScanner;
use crate::analyzers::includes::Includes;
use crate::analyzers::monorepo::Monorepo;
use crate::analyzers::no_restriected_globals::NoRestrictedGlobals;
use crate::analyzers::organize_imports::OrganizeImports;
use crate::analyzers::rule_mover::RuleMover;
use crate::analyzers::schema::Schema;
use crate::analyzers::trailing_comma::TrailingComma;
use crate::analyzers::use_naming_convention_enum_member_case::UseNamingConventionEnumMemberCase;
use biome_analyze::{GroupCategory, RegistryVisitor, RuleCategory, RuleGroup};
use biome_json_syntax::JsonLanguage;

mod all;
mod ignore_scanner;
mod includes;
mod monorepo;
mod no_restriected_globals;
mod organize_imports;
mod rule_mover;
mod schema;
mod trailing_comma;
mod use_naming_convention_enum_member_case;

pub(crate) struct MigrationGroup;
pub(crate) struct MigrationCategory;

impl RuleGroup for MigrationGroup {
    type Language = JsonLanguage;
    type Category = MigrationCategory;
    const NAME: &'static str = "migrations";

    fn record_rules<V: RegistryVisitor<Self::Language> + ?Sized>(registry: &mut V) {
        // Order here is important; rules should be added from the most old, to the most recent
        // v1.5.0
        registry.record_rule::<Schema>();
        // v2.0.0
        registry.record_rule::<RulesAll>();
        registry.record_rule::<OrganizeImports>();
        registry.record_rule::<RuleMover>();
        registry.record_rule::<Includes>();
        registry.record_rule::<TrailingComma>();
        registry.record_rule::<UseNamingConventionEnumMemberCase>();
        registry.record_rule::<NoRestrictedGlobals>();
        registry.record_rule::<Monorepo>();
        registry.record_rule::<IgnoreScanner>();
    }
}

impl GroupCategory for MigrationCategory {
    type Language = JsonLanguage;
    const CATEGORY: RuleCategory = RuleCategory::Action;

    fn record_groups<V: RegistryVisitor<Self::Language> + ?Sized>(registry: &mut V) {
        registry.record_group::<MigrationGroup>();
    }
}
