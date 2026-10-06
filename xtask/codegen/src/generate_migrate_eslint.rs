use biome_analyze::{
    GroupCategory, Queryable, RegistryVisitor, Rule, RuleCategory, RuleGroup, RuleSourceKind,
    RuleSourceWithKind,
};
use biome_rowan::syntax::Language;
use biome_string_case::Case;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeMap;
use xtask_codegen::update;
use xtask_glue::*;

fn generate_multi_mapping(eslint_name: Box<str>, mapped_rules: Vec<RuleMapping>) -> TokenStream {
    let rules = mapped_rules.iter().map(|RuleMapping{source_kind, rule_name, group_name}| {
        let name_ident = format_ident!("{}", Case::Snake.convert(rule_name));
        let group_ident = format_ident!("{group_name}");
        let blocked_set = if source_kind.is_inspired() || *group_name == "nursery" {
            quote! {
                let mut blocked = false;
            }
        } else {
            quote! {}
        };
        let check_inspired = if source_kind.is_inspired() {
            quote! {
                if !options.include_inspired {
                    results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                    blocked = true;
                }
            }
        } else {
            quote! {}
        };
        let check_nursery = if *group_name == "nursery" {
            quote! {
                if !options.include_nursery {
                    results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                    blocked = true;
                }
            }
        } else {
            quote! {}
        };
        let migrate = if source_kind.is_inspired() || *group_name == "nursery" {
            quote! {
                if !blocked {
                    let group = rules.#group_ident.get_or_insert_with(Default::default);
                    let rule = group.unwrap_group_as_mut().#name_ident.get_or_insert(Default::default());
                    rule.set_level(rule.level().max(rule_severity.into()));
                    migrated = true;
                }
            }
        } else {
            quote! {
                let group = rules.#group_ident.get_or_insert_with(Default::default);
                let rule = group.unwrap_group_as_mut().#name_ident.get_or_insert(Default::default());
                rule.set_level(rule.level().max(rule_severity.into()));
                migrated = true;
            }
        };

        quote! {
            {
                #blocked_set
                #check_inspired
                #check_nursery
                #migrate
            }
        }
    });

    quote! {
        #eslint_name => {
            let mut migrated = false;
            #( #rules )*
            if !migrated {
                return false;
            }
        }
    }
}

fn generate_single_mapping(eslint_name: Box<str>, mapped_rules: Vec<RuleMapping>) -> TokenStream {
    let Some(RuleMapping {
        source_kind,
        rule_name,
        group_name,
    }) = mapped_rules.first()
    else {
        return quote! {};
    };

    let name_ident = format_ident!("{}", Case::Snake.convert(rule_name));
    let group_ident = format_ident!("{group_name}");
    let check_inspired = if source_kind.is_inspired() {
        quote! {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
        }
    } else {
        quote! {}
    };
    let check_nursery = if *group_name == "nursery" {
        quote! {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #eslint_name => {
            #check_inspired
            #check_nursery
            let group = rules.#group_ident.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().#name_ident.get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
    }
}

pub(crate) fn generate_migrate_eslint(mode: Mode) -> Result<()> {
    let mut visitor = EslintLintRulesVisitor::default();
    biome_js_analyze::visit_registry(&mut visitor);
    biome_json_analyze::visit_registry(&mut visitor);
    biome_graphql_analyze::visit_registry(&mut visitor);
    biome_css_analyze::visit_registry(&mut visitor);
    biome_html_analyze::visit_registry(&mut visitor);
    let mut lines = Vec::with_capacity(visitor.0.len());
    for (eslint_name, mapped_rules) in visitor.0 {
        if mapped_rules.is_empty() {
            continue;
        }

        lines.push(if mapped_rules.len() > 1 {
            generate_multi_mapping(eslint_name, mapped_rules)
        } else {
            generate_single_mapping(eslint_name, mapped_rules)
        });
    }
    let tokens = xtask_glue::reformat(quote! {
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
                    results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Unsupported);
                    return false;
                }
            }
            results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Migrated);
            true
        }
    });
    let file_path =
        project_root().join("crates/biome_cli/src/execute/migrate/eslint_any_rule_to_biome.rs");
    update(&file_path, &tokens?, &mode)?;
    Ok(())
}

#[derive(Default)]
struct EslintLintRulesVisitor(BTreeMap<Box<str>, Vec<RuleMapping>>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RuleMapping {
    group_name: &'static str,
    rule_name: &'static str,
    source_kind: RuleSourceKind,
}

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
        for RuleSourceWithKind { kind, source } in R::METADATA.sources {
            if !source.is_eslint() && !source.is_eslint_plugin() {
                continue;
            }

            let mapped_rules = self
                .0
                .entry(source.to_namespaced_rule_name().into_boxed_str())
                .or_default();

            let existing = mapped_rules
                .iter_mut()
                .find(|mapped_rule| mapped_rule.rule_name == R::METADATA.name);

            if let Some(existing) = existing {
                if existing.source_kind.is_inspired() && !kind.is_inspired() {
                    existing.source_kind = *kind;
                }
            } else {
                mapped_rules.push(RuleMapping {
                    source_kind: *kind,
                    group_name: <R::Group as RuleGroup>::NAME,
                    rule_name: R::METADATA.name,
                });
            }
        }
    }
}
