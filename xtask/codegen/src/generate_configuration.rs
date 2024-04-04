use biome_analyze::{
    GroupCategory, Queryable, RegistryVisitor, Rule, RuleCategory, RuleGroup, RuleMetadata,
};
use biome_css_syntax::CssLanguage;
use biome_js_syntax::JsLanguage;
use biome_json_syntax::JsonLanguage;
use biome_string_case::Case;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use pulldown_cmark::{Event, Parser, Tag};
use quote::quote;
use std::collections::BTreeMap;
use xtask::*;
use xtask_codegen::{to_capitalized, update};

pub(crate) fn generate_rules_configuration(mode: Mode) -> Result<()> {
    let config_root = project_root().join("crates/biome_configuration/src/linter");
    let push_rules_directory = project_root().join("crates/biome_configuration/src");

    #[derive(Default)]
    struct LintRulesVisitor {
        groups: BTreeMap<&'static str, BTreeMap<&'static str, RuleMetadata>>,
    }

    impl RegistryVisitor<JsLanguage> for LintRulesVisitor {
        fn record_category<C: GroupCategory<Language = JsLanguage>>(&mut self) {
            if matches!(C::CATEGORY, RuleCategory::Lint) {
                C::record_groups(self);
            }
        }

        fn record_rule<R>(&mut self)
        where
            R: Rule + 'static,
            R::Query: Queryable<Language = JsLanguage>,
            <R::Query as Queryable>::Output: Clone,
        {
            self.groups
                .entry(<R::Group as RuleGroup>::NAME)
                .or_insert_with(BTreeMap::new)
                .insert(R::METADATA.name, R::METADATA);
        }
    }

    impl RegistryVisitor<JsonLanguage> for LintRulesVisitor {
        fn record_category<C: GroupCategory<Language = JsonLanguage>>(&mut self) {
            if matches!(C::CATEGORY, RuleCategory::Lint) {
                C::record_groups(self);
            }
        }

        fn record_rule<R>(&mut self)
        where
            R: Rule + 'static,
            R::Query: Queryable<Language = JsonLanguage>,
            <R::Query as Queryable>::Output: Clone,
        {
            self.groups
                .entry(<R::Group as RuleGroup>::NAME)
                .or_insert_with(BTreeMap::new)
                .insert(R::METADATA.name, R::METADATA);
        }
    }

    impl RegistryVisitor<CssLanguage> for LintRulesVisitor {
        fn record_category<C: GroupCategory<Language = CssLanguage>>(&mut self) {
            if matches!(C::CATEGORY, RuleCategory::Lint) {
                C::record_groups(self);
            }
        }

        fn record_rule<R>(&mut self)
        where
            R: Rule + 'static,
            R::Query: Queryable<Language = CssLanguage>,
            <R::Query as Queryable>::Output: Clone,
        {
            self.groups
                .entry(<R::Group as RuleGroup>::NAME)
                .or_insert_with(BTreeMap::new)
                .insert(R::METADATA.name, R::METADATA);
        }
    }

    let mut visitor = LintRulesVisitor::default();
    biome_js_analyze::visit_registry(&mut visitor);
    biome_json_analyze::visit_registry(&mut visitor);
    biome_css_analyze::visit_registry(&mut visitor);

    let LintRulesVisitor { groups } = visitor;

    let mut struct_groups = Vec::new();
    let mut line_groups = Vec::new();
    let mut default_for_groups = Vec::new();
    let mut group_as_default_rules = Vec::new();
    let mut group_match_code = Vec::new();
    let mut group_get_severity = Vec::new();
    let mut group_name_list = vec!["recommended", "all"];
    let mut push_rule_list = Vec::new();
    for (group, rules) in groups {
        group_name_list.push(group);
        let property_group_name = Ident::new(&Case::Snake.convert(group), Span::call_site());
        let group_struct_name = Ident::new(&to_capitalized(group), Span::call_site());
        let group_name_string_literal = Literal::string(group);

        struct_groups.push(generate_struct(group, &rules));
        push_rule_list.push(generate_push_to_analyzer_rules(group));
        line_groups.push(quote! {
            #[deserializable(rename = #group_name_string_literal)]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub #property_group_name: Option<#group_struct_name>
        });
        default_for_groups.push(quote! {
            #property_group_name: None
        });

        let (global_all, global_recommended) = if group == "nursery" {
            (
                quote! { self.is_all() && biome_flags::is_unstable() },
                quote! { self.is_recommended() && biome_flags::is_unstable() },
            )
        } else {
            (quote! { self.is_all() }, quote! { self.is_recommended() })
        };

        group_as_default_rules.push(quote! {
            if let Some(group) = self.#property_group_name.as_ref() {
                group.collect_preset_rules(
                    #global_all,
                    #global_recommended,
                    &mut enabled_rules,
                );
                enabled_rules.extend(&group.get_enabled_rules());
                disabled_rules.extend(&group.get_disabled_rules());
            } else if #global_all {
                enabled_rules.extend(#group_struct_name::all_rules_as_filters());
            } else if #global_recommended {
                enabled_rules.extend(#group_struct_name::recommended_rules_as_filters());
            }
        });

        group_get_severity.push(quote! {
            #group => self
                .#property_group_name
                .as_ref()
                .and_then(|#property_group_name| #property_group_name.get_rule_configuration(rule_name))
                .map_or_else(|| {
                    if #group_struct_name::is_recommended_rule(rule_name) {
                        Severity::Error
                    } else {
                        Severity::Warning
                    }
                }, |(level, _)| level.into())
        });
        group_match_code.push(quote! {
           #group => #group_struct_name::has_rule(rule_name).then_some((category, rule_name))
        });
    }

    let groups = quote! {
        use crate::RuleConfiguration;
        use biome_analyze::{options::RuleOptions, RuleFilter};
        use biome_console::markup;
        use biome_deserialize::{DeserializableValidator, DeserializationDiagnostic};
        use biome_deserialize_macros::{Deserializable, Merge};
        use biome_diagnostics::{Category, Severity};
        use biome_js_analyze::options::*;
        use biome_json_analyze::options::*;
        use biome_css_analyze::options::*;
        use biome_rowan::TextRange;
        use indexmap::IndexSet;
        use serde::{Deserialize, Serialize};
        #[cfg(feature = "schema")]
        use schemars::JsonSchema;

        use super::RulePlainConfiguration;

        #[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
        #[deserializable(with_validator)]
        #[cfg_attr(feature = "schema", derive(JsonSchema))]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        pub struct Rules {
            /// It enables the lint rules recommended by Biome. `true` by default.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub recommended: Option<bool>,

            /// It enables ALL rules. The rules that belong to `nursery` won't be enabled.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub all: Option<bool>,

            #( #line_groups ),*
        }

        impl DeserializableValidator for Rules {
            fn validate(
                &mut self,
                _name: &str,
                range: TextRange,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> bool {
                if self.recommended == Some(true) && self.all == Some(true) {
                    diagnostics
                        .push(DeserializationDiagnostic::new(markup!(
                            <Emphasis>"'recommended'"</Emphasis>" and "<Emphasis>"'all'"</Emphasis>" can't be both "<Emphasis>"'true'"</Emphasis>". You should choose only one of them."
                        ))
                        .with_range(range)
                        .with_note(markup!("Biome will fallback to its defaults for this section.")));
                    return false;
                }

                true
            }
        }

        impl Rules {
            /// Checks if the code coming from [biome_diagnostics::Diagnostic] corresponds to a rule.
            /// Usually the code is built like {category}/{rule_name}
            pub fn matches_diagnostic_code<'a>(
                &self,
                category: Option<&'a str>,
                rule_name: Option<&'a str>,
            ) -> Option<(&'a str, &'a str)> {
                match (category, rule_name) {
                    (Some(category), Some(rule_name)) => match category {
                        #( #group_match_code ),*,

                        _ => None
                    },
                    _ => None
                }
            }

            /// Given a category coming from [Diagnostic](biome_diagnostics::Diagnostic), this function returns
            /// the [Severity](biome_diagnostics::Severity) associated to the rule, if the configuration changed it.
            ///
            /// If not, the function returns [None].
            pub fn get_severity_from_code(&self, category: &Category) -> Option<Severity> {
                let mut split_code = category.name().split('/');

                let _lint = split_code.next();
                debug_assert_eq!(_lint, Some("lint"));

                let group = split_code.next();
                let rule_name = split_code.next();

                if let Some((group, rule_name)) = self.matches_diagnostic_code(group, rule_name) {
                    let severity = match group {
                        #( #group_get_severity ),*,

                        _ => unreachable!("this group should not exist, found {}", group),
                    };
                    Some(severity)
                } else {
                    None
                }
            }

            pub(crate) const fn is_recommended(&self) -> bool {
                // It is only considered _not_ recommended when
                // the configuration is `"recommended": false`.
                // Hence, omission of the setting or set to `true` are considered recommended.
                !matches!(self.recommended, Some(false))
            }

            pub(crate) const fn is_all(&self) -> bool {
                matches!(self.all, Some(true))
            }

            /// It returns the enabled rules by default.
            ///
            /// The enabled rules are calculated from the difference with the disabled rules.
            pub fn as_enabled_rules(&self) -> IndexSet<RuleFilter> {
                let mut enabled_rules = IndexSet::new();
                let mut disabled_rules = IndexSet::new();
                #( #group_as_default_rules )*

                enabled_rules.difference(&disabled_rules).copied().collect()
            }
        }

        #( #struct_groups )*
    };

    let push_rules = quote! {
        use crate::linter::*;
        use crate::Rules;
        use biome_analyze::{AnalyzerRules, MetadataRegistry};

        pub fn push_to_analyzer_rules(
            rules: &Rules,
            metadata: &MetadataRegistry,
            analyzer_rules: &mut AnalyzerRules,
        ) {
            #( #push_rule_list )*
        }
    };

    let configuration = groups.to_string();
    let push_rules = push_rules.to_string();

    update(
        &config_root.join("rules.rs"),
        &xtask::reformat(configuration)?,
        &mode,
    )?;

    update(
        &push_rules_directory.join("generated.rs"),
        &xtask::reformat(push_rules)?,
        &mode,
    )?;

    Ok(())
}

fn generate_struct(group: &str, rules: &BTreeMap<&'static str, RuleMetadata>) -> TokenStream {
    let mut lines_recommended_rule = Vec::new();
    let mut lines_recommended_rule_as_filter = Vec::new();
    let mut lines_all_rule_as_filter = Vec::new();
    let mut lines_rule = Vec::new();
    let mut schema_lines_rules = Vec::new();
    let mut rule_enabled_check_line = Vec::new();
    let mut rule_disabled_check_line = Vec::new();
    let mut get_rule_configuration_line = Vec::new();

    let mut number_of_recommended_rules: u8 = 0;
    let number_of_rules = Literal::u8_unsuffixed(rules.len() as u8);
    for (index, (rule, metadata)) in rules.iter().enumerate() {
        let summary = {
            let mut docs = String::new();
            let parser = Parser::new(metadata.docs);
            for event in parser {
                match event {
                    Event::Text(text) => {
                        docs.push_str(text.as_ref());
                    }
                    Event::Code(text) => {
                        docs.push_str(text.as_ref());
                    }
                    Event::SoftBreak => {
                        docs.push(' ');
                    }

                    Event::Start(Tag::Paragraph) => {}
                    Event::End(Tag::Paragraph) => {
                        break;
                    }

                    Event::Start(tag) => match tag {
                        Tag::Strong | Tag::Paragraph => {
                            continue;
                        }

                        _ => panic!("Unimplemented tag {:?}", { tag }),
                    },

                    Event::End(tag) => match tag {
                        Tag::Strong | Tag::Paragraph => {
                            continue;
                        }
                        _ => panic!("Unimplemented tag {:?}", { tag }),
                    },

                    _ => {
                        panic!("Unimplemented event {:?}", { event })
                    }
                }
            }
            docs
        };

        let rule_position = Literal::u8_unsuffixed(index as u8);
        let rule_identifier = Ident::new(&Case::Snake.convert(rule), Span::call_site());
        let rule_name = Ident::new(&to_capitalized(rule), Span::call_site());
        if metadata.recommended {
            lines_recommended_rule_as_filter.push(quote! {
                RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[#rule_position])
            });

            lines_recommended_rule.push(quote! {
                #rule
            });
            number_of_recommended_rules += 1;
        }
        lines_all_rule_as_filter.push(quote! {
            RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[#rule_position])
        });
        lines_rule.push(quote! {
             #rule
        });
        schema_lines_rules.push(quote! {
            #[doc = #summary]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub #rule_identifier: Option<RuleConfiguration<#rule_name>>
        });

        rule_enabled_check_line.push(quote! {
            if let Some(rule) = self.#rule_identifier.as_ref() {
                if rule.is_enabled() {
                    index_set.insert(RuleFilter::Rule(
                        Self::GROUP_NAME,
                        Self::GROUP_RULES[#rule_position],
                    ));
                }
            }
        });
        rule_disabled_check_line.push(quote! {
            if let Some(rule) = self.#rule_identifier.as_ref() {
                if rule.is_disabled() {
                    index_set.insert(RuleFilter::Rule(
                        Self::GROUP_NAME,
                        Self::GROUP_RULES[#rule_position],
                    ));
                }
            }
        });

        get_rule_configuration_line.push(quote! {
            #rule => self.#rule_identifier.as_ref().map(|conf| (conf.level(), conf.get_options()))
        });
    }

    let group_struct_name = Ident::new(&to_capitalized(group), Span::call_site());

    let number_of_recommended_rules = Literal::u8_unsuffixed(number_of_recommended_rules);

    quote! {
        #[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
        #[deserializable(with_validator)]
        #[cfg_attr(feature = "schema", derive(JsonSchema))]
        #[serde(rename_all = "camelCase", default, deny_unknown_fields)]
        /// A list of rules that belong to this group
        pub struct #group_struct_name {
            /// It enables the recommended rules for this group
            #[serde(skip_serializing_if = "Option::is_none")]
            pub recommended: Option<bool>,

            /// It enables ALL rules for this group.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub all: Option<bool>,

            #( #schema_lines_rules ),*
        }

        impl DeserializableValidator for #group_struct_name {
            fn validate(
                &mut self,
                _name: &str,
                range: TextRange,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> bool {
                if self.recommended == Some(true) && self.all == Some(true) {
                    diagnostics
                        .push(DeserializationDiagnostic::new(markup!(
                            <Emphasis>"'recommended'"</Emphasis>" and "<Emphasis>"'all'"</Emphasis>" can't be both "<Emphasis>"'true'"</Emphasis>". You should choose only one of them."
                        ))
                        .with_range(range)
                        .with_note(markup!("Biome will fallback to its defaults for this section.")));
                    return false;
                }

                true
            }
        }

        impl #group_struct_name {

            const GROUP_NAME: &'static str = #group;
            pub(crate) const GROUP_RULES: [&'static str; #number_of_rules] = [
                #( #lines_rule ),*
            ];

            const RECOMMENDED_RULES: [&'static str; #number_of_recommended_rules] = [
                #( #lines_recommended_rule ),*
            ];

            const RECOMMENDED_RULES_AS_FILTERS: [RuleFilter<'static>; #number_of_recommended_rules] = [
                #( #lines_recommended_rule_as_filter ),*
            ];

            const ALL_RULES_AS_FILTERS: [RuleFilter<'static>; #number_of_rules] = [
                #( #lines_all_rule_as_filter ),*
            ];

            /// Retrieves the recommended rules
            pub(crate) fn is_recommended(&self) -> bool {
                // we should inject recommended rules only when they are set to "true"
                matches!(self.recommended, Some(true))
            }

            pub(crate) fn is_recommended_unset(&self) -> bool {
                self.recommended.is_none()
            }

            pub(crate) fn is_all(&self) -> bool {
                matches!(self.all, Some(true))
            }

            pub(crate) fn is_all_unset(&self) -> bool {
                self.all.is_none()
            }

            pub(crate) fn get_enabled_rules(&self) -> IndexSet<RuleFilter> {
               let mut index_set = IndexSet::new();
               #( #rule_enabled_check_line )*
               index_set
            }

            pub(crate) fn get_disabled_rules(&self) -> IndexSet<RuleFilter> {
               let mut index_set = IndexSet::new();
               #( #rule_disabled_check_line )*
               index_set
            }

            /// Checks if, given a rule name, matches one of the rules contained in this category
            pub(crate) fn has_rule(rule_name: &str) -> bool {
                Self::GROUP_RULES.contains(&rule_name)
            }

            /// Checks if, given a rule name, it is marked as recommended
            pub(crate) fn is_recommended_rule(rule_name: &str) -> bool {
                 Self::RECOMMENDED_RULES.contains(&rule_name)
            }

            pub(crate) fn recommended_rules_as_filters() -> [RuleFilter<'static>; #number_of_recommended_rules] {
                Self::RECOMMENDED_RULES_AS_FILTERS
            }

            pub(crate) fn all_rules_as_filters() -> [RuleFilter<'static>; #number_of_rules] {
                Self::ALL_RULES_AS_FILTERS
            }

            /// Select preset rules
            // Preset rules shouldn't populate disabled rules
            // because that will make specific rules cannot be enabled later.
            pub(crate) fn collect_preset_rules(
                &self,
                parent_is_all: bool,
                parent_is_recommended: bool,
                enabled_rules: &mut IndexSet<RuleFilter>,
            ) {
                if self.is_all() || self.is_all_unset() && parent_is_all {
                    enabled_rules.extend(Self::all_rules_as_filters());
                } else if self.is_recommended() || self.is_recommended_unset() && parent_is_recommended && !parent_is_all {
                    enabled_rules.extend(Self::recommended_rules_as_filters());
                }
            }

            pub(crate) fn get_rule_configuration(&self, rule_name: &str) -> Option<(RulePlainConfiguration, Option<RuleOptions>)> {
                match rule_name {
                    #( #get_rule_configuration_line ),*,
                    _ => None
                }
            }
        }
    }
}

fn generate_push_to_analyzer_rules(group: &str) -> TokenStream {
    let group_struct_name = Ident::new(&to_capitalized(group), Span::call_site());
    let group_identifier = Ident::new(group, Span::call_site());
    quote! {
       if let Some(rules) = rules.#group_identifier.as_ref() {
            for rule_name in &#group_struct_name::GROUP_RULES {
                if let Some((_, Some(rule_options))) = rules.get_rule_configuration(rule_name) {
                    if let Some(rule_key) = metadata.find_rule(#group, rule_name) {
                        analyzer_rules.push_rule(rule_key, rule_options);
                    }
                }
            }
        }
    }
}
