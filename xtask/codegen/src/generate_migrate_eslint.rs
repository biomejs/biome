use biome_analyze::{
    GroupCategory, Queryable, RegistryVisitor, Rule, RuleCategory, RuleGroup, RuleMetadata,
};
use biome_rowan::syntax::Language;
use biome_string_case::Case;
use quote::{format_ident, quote};
use std::collections::BTreeMap;
use xtask::*;
use xtask_codegen::update;

pub(crate) fn generate_migrate_eslint(mode: Mode) -> Result<()> {
    let mut visitor = EslintLintRulesVisitor::default();
    biome_js_analyze::visit_registry(&mut visitor);
    biome_json_analyze::visit_registry(&mut visitor);
    let mut lines = Vec::with_capacity(visitor.0.len());
    for (eslint_name, (group_name, rule_metadata)) in visitor.0 {
        let name = rule_metadata.name;
        let name_ident = format_ident!("{}", Case::Snake.convert(name));
        let group_ident = format_ident!("{group_name}");
        let is_inspuired = rule_metadata
            .source_kind
            .is_some_and(|source_kind| source_kind.is_inspired());
        let check_inspired = if is_inspuired {
            quote! {
                if !options.include_inspired {
                    results.has_inspired_rules = true;
                    return false;
                }
            }
        } else {
            quote! {}
        };
        let check_nursery = if group_name == "nursery" {
            quote! {
                if !options.include_nursery {
                    return false;
                }
            }
        } else {
            quote! {}
        };
        lines.push(quote! {
            #eslint_name => {
                #check_inspired
                #check_nursery
                let group = rules.#group_ident.get_or_insert_with(Default::default);
                let rule = group.unwrap_group_as_mut().#name_ident.get_or_insert(Default::default());
                rule.set_level(rule.level().max(rule_severity.into()));
            }
        });
    }
    let tokens = xtask::reformat(quote! {
        use super::{eslint_eslint, eslint_to_biome};
        pub(crate) fn migrate_eslint_any_rule(
            rules: &mut biome_configuration::Rules,
            eslint_name: &str,
            rule_severity: eslint_eslint::Severity,
            options: &eslint_to_biome::MigrationOptions,
            results: &mut eslint_to_biome::MigrationResults,
        ) -> bool {
            match eslint_name {
                #( #lines )*
                _ => {
                    return false;
                }
            }
            true
        }
    });
    let file_path =
        project_root().join("crates/biome_cli/src/execute/migrate/eslint_any_rule_to_biome.rs");
    update(&file_path, &tokens?, &mode)?;
    Ok(())
}

#[derive(Default)]
struct EslintLintRulesVisitor(BTreeMap<String, (&'static str, RuleMetadata)>);

impl<L: Language> RegistryVisitor<L> for EslintLintRulesVisitor {
    fn record_category<C: GroupCategory<Language = L>>(&mut self) {
        if matches!(C::CATEGORY, RuleCategory::Lint) {
            C::record_groups(self);
        }
    }

    fn record_rule<R: Rule + 'static>(&mut self)
    where
        R::Query: Queryable<Language = L>,
        <R::Query as Queryable>::Output: Clone,
    {
        for source in R::METADATA.sources {
            if source.is_eslint() || source.is_eslint_plugin() {
                self.0.insert(
                    source.to_namespaced_rule_name(),
                    (<R::Group as RuleGroup>::NAME, R::METADATA),
                );
            }
        }
    }
}
