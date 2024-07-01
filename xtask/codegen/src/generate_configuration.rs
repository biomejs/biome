use biome_analyze::{
    FixKind, GroupCategory, Queryable, RegistryVisitor, Rule, RuleCategory, RuleGroup, RuleMetadata,
};
use biome_css_syntax::CssLanguage;
use biome_graphql_syntax::GraphqlLanguage;
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

    impl RegistryVisitor<GraphqlLanguage> for LintRulesVisitor {
        fn record_category<C: GroupCategory<Language = GraphqlLanguage>>(&mut self) {
            if matches!(C::CATEGORY, RuleCategory::Lint) {
                C::record_groups(self);
            }
        }

        fn record_rule<R>(&mut self)
        where
            R: Rule + 'static,
            R::Query: Queryable<Language = GraphqlLanguage>,
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
    biome_graphql_analyze::visit_registry(&mut visitor);

    let LintRulesVisitor { groups } = visitor;

    let mut struct_groups = Vec::with_capacity(groups.len());
    let mut group_pascal_idents = Vec::with_capacity(groups.len());
    let mut group_idents = Vec::with_capacity(groups.len());
    let mut group_strings = Vec::with_capacity(groups.len());
    let mut group_as_default_rules = Vec::with_capacity(groups.len());
    for (group, rules) in groups {
        let group_pascal_ident = quote::format_ident!("{}", &Case::Pascal.convert(group));
        let group_ident = quote::format_ident!("{}", group);

        let (global_all, global_recommended) = if group == "nursery" {
            (
                quote! { self.is_all_true() && biome_flags::is_unstable() },
                quote! { !self.is_recommended_false() && biome_flags::is_unstable() },
            )
        } else {
            (
                quote! { self.is_all_true() },
                quote! { !self.is_recommended_false() },
            )
        };
        group_as_default_rules.push(quote! {
            if let Some(group) = self.#group_ident.as_ref() {
                group.collect_preset_rules(
                    #global_all,
                    #global_recommended,
                    &mut enabled_rules,
                );
                enabled_rules.extend(&group.get_enabled_rules());
                disabled_rules.extend(&group.get_disabled_rules());
            } else if #global_all {
                enabled_rules.extend(#group_pascal_ident::all_rules_as_filters());
            } else if #global_recommended {
                enabled_rules.extend(#group_pascal_ident::recommended_rules_as_filters());
            }
        });

        group_pascal_idents.push(group_pascal_ident);
        group_idents.push(group_ident);
        group_strings.push(Literal::string(group));
        struct_groups.push(generate_struct(group, &rules));
    }

    let groups = quote! {
        use crate::{RuleConfiguration, RuleFixConfiguration};
        use biome_analyze::{options::RuleOptions, RuleFilter};
        use biome_console::markup;
        use biome_deserialize::{DeserializableValidator, DeserializationDiagnostic};
        use biome_deserialize_macros::{Deserializable, Merge};
        use biome_diagnostics::{Category, Severity};
        use biome_js_analyze::options::*;
        use biome_json_analyze::options::*;
        use biome_css_analyze::options::*;
        use biome_graphql_analyze::options::*;
        use biome_rowan::TextRange;
        use rustc_hash::FxHashSet;
        use serde::{Deserialize, Serialize};
        #[cfg(feature = "schema")]
        use schemars::JsonSchema;

        use super::RulePlainConfiguration;

        #[derive(Clone, Copy, Debug, Deserializable, Eq, Hash, Merge, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
        #[cfg_attr(feature = "schema", derive(JsonSchema))]
        #[serde(rename_all = "camelCase")]
        pub enum RuleGroup {
            #( #group_pascal_idents ),*
        }
        impl RuleGroup {
            pub const fn as_str(self) -> &'static str {
                match self {
                    #( Self::#group_pascal_idents => #group_pascal_idents::GROUP_NAME, )*
                }
            }
        }
        impl std::str::FromStr for RuleGroup {
            type Err = &'static str;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #( #group_pascal_idents::GROUP_NAME => Ok(Self::#group_pascal_idents), )*
                    _ => Err("This rule group doesn't exist.")
                }
            }
        }

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

            #(
                #[deserializable(rename = #group_strings)]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub #group_idents: Option<#group_pascal_idents>,
            )*
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
            /// Usually the code is built like {group}/{rule_name}
            pub fn has_rule(
                group: RuleGroup,
                rule_name: &str,
            ) -> Option<&'static str> {
                match group {
                    #(
                        RuleGroup::#group_pascal_idents => #group_pascal_idents::has_rule(rule_name),
                    )*
                }
            }

            /// Given a category coming from [Diagnostic](biome_diagnostics::Diagnostic), this function returns
            /// the [Severity](biome_diagnostics::Severity) associated to the rule, if the configuration changed it.
            /// If the severity is off or not set, then the function returns the default severity of the rule:
            /// [Severity::Error] for recommended rules and [Severity::Warning] for other rules.
            ///
            /// If not, the function returns [None].
            pub fn get_severity_from_code(&self, category: &Category) -> Option<Severity> {
                let mut split_code = category.name().split('/');

                let _lint = split_code.next();
                debug_assert_eq!(_lint, Some("lint"));

                let group = <RuleGroup as std::str::FromStr>::from_str(split_code.next()?).ok()?;
                let rule_name = split_code.next()?;
                let rule_name = Self::has_rule(group, rule_name)?;
                let severity = match group {
                    #(
                        RuleGroup::#group_pascal_idents => self
                            .#group_idents
                            .as_ref()
                            .and_then(|group| group.get_rule_configuration(rule_name))
                            .filter(|(level, _)| !matches!(level, RulePlainConfiguration::Off))
                            .map_or_else(|| {
                                if #group_pascal_idents::is_recommended_rule(rule_name) {
                                    Severity::Error
                                } else {
                                    Severity::Warning
                                }
                            }, |(level, _)| level.into()),
                    )*
                };
                Some(severity)
            }

            /// Ensure that `recommended` is set to `true` or implied.
            pub fn set_recommended(&mut self) {
                if self.all != Some(true) && self.recommended == Some(false) {
                    self.recommended = Some(true)
                }
                #(
                    if let Some(group) = &mut self.#group_idents {
                        group.recommended = None;
                    }
                )*
            }

            // Note: In top level, it is only considered _not_ recommended
            // when the recommended option is false
            pub(crate) const fn is_recommended_false(&self) -> bool {
                matches!(self.recommended, Some(false))
            }

            pub(crate) const fn is_all_true(&self) -> bool {
                matches!(self.all, Some(true))
            }

            /// It returns the enabled rules by default.
            ///
            /// The enabled rules are calculated from the difference with the disabled rules.
            pub fn as_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
                let mut enabled_rules = FxHashSet::default();
                let mut disabled_rules = FxHashSet::default();
                #( #group_as_default_rules )*

                enabled_rules.difference(&disabled_rules).copied().collect()
            }
        }

        #( #struct_groups )*

        #[test]
        fn test_order() {
            #(
                for items in #group_pascal_idents::GROUP_RULES.windows(2) {
                    assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
                }
            )*
        }
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
            #(
                if let Some(rules) = rules.#group_idents.as_ref() {
                    for rule_name in #group_pascal_idents::GROUP_RULES {
                        if let Some((_, Some(rule_options))) = rules.get_rule_configuration(rule_name) {
                            if let Some(rule_key) = metadata.find_rule(#group_strings, rule_name) {
                                analyzer_rules.push_rule(rule_key, rule_options);
                            }
                        }
                    }
                }
            )*
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
                        // Escape `[` and `<` to obtain valid Markdown
                        docs.push_str(text.replace('[', "\\[").replace('<', "\\<").as_ref());
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
        let rule_identifier = quote::format_ident!("{}", Case::Snake.convert(rule));
        let rule_config_type = quote::format_ident!(
            "{}",
            if metadata.fix_kind != FixKind::None {
                "RuleFixConfiguration"
            } else {
                "RuleConfiguration"
            }
        );
        let rule_name = Ident::new(&to_capitalized(rule), Span::call_site());
        if metadata.recommended {
            lines_recommended_rule_as_filter.push(quote! {
                RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[#rule_position])
            });

            lines_recommended_rule.push(quote! {
                #rule
            });
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
            pub #rule_identifier: Option<#rule_config_type<#rule_name>>
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

    let group_pascal_ident = Ident::new(&to_capitalized(group), Span::call_site());

    quote! {
        #[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
        #[deserializable(with_validator)]
        #[cfg_attr(feature = "schema", derive(JsonSchema))]
        #[serde(rename_all = "camelCase", default, deny_unknown_fields)]
        /// A list of rules that belong to this group
        pub struct #group_pascal_ident {
            /// It enables the recommended rules for this group
            #[serde(skip_serializing_if = "Option::is_none")]
            pub recommended: Option<bool>,

            /// It enables ALL rules for this group.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub all: Option<bool>,

            #( #schema_lines_rules ),*
        }

        impl DeserializableValidator for #group_pascal_ident {
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

        impl #group_pascal_ident {

            const GROUP_NAME: &'static str = #group;
            pub(crate) const GROUP_RULES: &'static [&'static str] = &[
                #( #lines_rule ),*
            ];

            const RECOMMENDED_RULES: &'static [&'static str] = &[
                #( #lines_recommended_rule ),*
            ];

            const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
                #( #lines_recommended_rule_as_filter ),*
            ];

            const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
                #( #lines_all_rule_as_filter ),*
            ];

            /// Retrieves the recommended rules
            pub(crate) fn is_recommended_true(&self) -> bool {
                // we should inject recommended rules only when they are set to "true"
                matches!(self.recommended, Some(true))
            }

            pub(crate) fn is_recommended_unset(&self) -> bool {
                self.recommended.is_none()
            }

            pub(crate) fn is_all_true(&self) -> bool {
                matches!(self.all, Some(true))
            }

            pub(crate) fn is_all_unset(&self) -> bool {
                self.all.is_none()
            }

            pub(crate) fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
               let mut index_set = FxHashSet::default();
               #( #rule_enabled_check_line )*
               index_set
            }

            pub(crate) fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
               let mut index_set = FxHashSet::default();
               #( #rule_disabled_check_line )*
               index_set
            }

            /// Checks if, given a rule name, matches one of the rules contained in this category
            pub(crate) fn has_rule(rule_name: &str) -> Option<&'static str> {
                Some(Self::GROUP_RULES[Self::GROUP_RULES.binary_search(&rule_name).ok()?])
            }

            /// Checks if, given a rule name, it is marked as recommended
            pub(crate) fn is_recommended_rule(rule_name: &str) -> bool {
                 Self::RECOMMENDED_RULES.contains(&rule_name)
            }

            pub(crate) fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
                Self::RECOMMENDED_RULES_AS_FILTERS
            }

            pub(crate) fn all_rules_as_filters() -> &'static [RuleFilter<'static>] {
                Self::ALL_RULES_AS_FILTERS
            }

            /// Select preset rules
            // Preset rules shouldn't populate disabled rules
            // because that will make specific rules cannot be enabled later.
            pub(crate) fn collect_preset_rules(
                &self,
                parent_is_all: bool,
                parent_is_recommended: bool,
                enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
            ) {
                // The order of the if-else branches MATTERS!
                if self.is_all_true() || self.is_all_unset() && parent_is_all {
                    enabled_rules.extend(Self::all_rules_as_filters());
                } else if self.is_recommended_true() || self.is_recommended_unset() && self.is_all_unset() && parent_is_recommended {
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
