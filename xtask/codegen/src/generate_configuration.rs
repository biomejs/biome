use biome_analyze::{
    FixKind, GroupCategory, Queryable, RegistryVisitor, Rule, RuleCategory, RuleGroup, RuleMetadata,
};
use biome_css_syntax::CssLanguage;
use biome_graphql_syntax::GraphqlLanguage;
use biome_js_syntax::JsLanguage;
use biome_json_syntax::JsonLanguage;
use biome_string_case::Case;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use quote::quote;
use std::collections::BTreeMap;
use std::path::Path;
use xtask::*;
use xtask_codegen::{to_capitalized, update};

// ======= LINT ======
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
        R: Rule<Options: Default, Query: Queryable<Language = JsLanguage, Output: Clone>> + 'static,
    {
        self.groups
            .entry(<R::Group as RuleGroup>::NAME)
            .or_default()
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
        R: Rule<Options: Default, Query: Queryable<Language = JsonLanguage, Output: Clone>>
            + 'static,
    {
        self.groups
            .entry(<R::Group as RuleGroup>::NAME)
            .or_default()
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
        R: Rule<Options: Default, Query: Queryable<Language = CssLanguage, Output: Clone>>
            + 'static,
    {
        self.groups
            .entry(<R::Group as RuleGroup>::NAME)
            .or_default()
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
        R: Rule<Options: Default, Query: Queryable<Language = GraphqlLanguage, Output: Clone>>
            + 'static,
    {
        self.groups
            .entry(<R::Group as RuleGroup>::NAME)
            .or_default()
            .insert(R::METADATA.name, R::METADATA);
    }
}

// ======= ASSIST ======
#[derive(Default)]
struct AssistActionsVisitor {
    groups: BTreeMap<&'static str, BTreeMap<&'static str, RuleMetadata>>,
}

impl RegistryVisitor<JsLanguage> for AssistActionsVisitor {
    fn record_category<C: GroupCategory<Language = JsLanguage>>(&mut self) {
        if matches!(C::CATEGORY, RuleCategory::Action) {
            C::record_groups(self);
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsLanguage, Output: Clone>> + 'static,
    {
        self.groups
            .entry(<R::Group as RuleGroup>::NAME)
            .or_default()
            .insert(R::METADATA.name, R::METADATA);
    }
}

impl RegistryVisitor<JsonLanguage> for AssistActionsVisitor {
    fn record_category<C: GroupCategory<Language = JsonLanguage>>(&mut self) {
        if matches!(C::CATEGORY, RuleCategory::Action) {
            C::record_groups(self);
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsonLanguage, Output: Clone>>
            + 'static,
    {
        self.groups
            .entry(<R::Group as RuleGroup>::NAME)
            .or_default()
            .insert(R::METADATA.name, R::METADATA);
    }
}

impl RegistryVisitor<CssLanguage> for AssistActionsVisitor {
    fn record_category<C: GroupCategory<Language = CssLanguage>>(&mut self) {
        if matches!(C::CATEGORY, RuleCategory::Action) {
            C::record_groups(self);
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = CssLanguage, Output: Clone>>
            + 'static,
    {
        self.groups
            .entry(<R::Group as RuleGroup>::NAME)
            .or_default()
            .insert(R::METADATA.name, R::METADATA);
    }
}

impl RegistryVisitor<GraphqlLanguage> for AssistActionsVisitor {
    fn record_category<C: GroupCategory<Language = GraphqlLanguage>>(&mut self) {
        if matches!(C::CATEGORY, RuleCategory::Action) {
            C::record_groups(self);
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = GraphqlLanguage, Output: Clone>>
            + 'static,
    {
        self.groups
            .entry(<R::Group as RuleGroup>::NAME)
            .or_default()
            .insert(R::METADATA.name, R::METADATA);
    }
}

pub(crate) fn generate_rules_configuration(mode: Mode) -> Result<()> {
    let linter_config_root = project_root().join("crates/biome_configuration/src/analyzer/linter");
    let assist_config_root = project_root().join("crates/biome_configuration/src/analyzer/assist");
    let push_rules_directory = project_root().join("crates/biome_configuration/src/generated");

    let mut lint_visitor = LintRulesVisitor::default();
    let mut assist_visitor = AssistActionsVisitor::default();
    biome_js_analyze::visit_registry(&mut lint_visitor);
    biome_js_analyze::visit_registry(&mut assist_visitor);
    biome_json_analyze::visit_registry(&mut lint_visitor);
    biome_json_analyze::visit_registry(&mut assist_visitor);
    biome_css_analyze::visit_registry(&mut lint_visitor);
    biome_css_analyze::visit_registry(&mut assist_visitor);
    biome_graphql_analyze::visit_registry(&mut lint_visitor);
    biome_graphql_analyze::visit_registry(&mut assist_visitor);

    // let LintRulesVisitor { groups } = lint_visitor;

    generate_for_groups(
        lint_visitor.groups,
        linter_config_root.as_path(),
        push_rules_directory.as_path(),
        &mode,
        RuleCategory::Lint,
    )?;
    generate_for_groups(
        assist_visitor.groups,
        assist_config_root.as_path(),
        push_rules_directory.as_path(),
        &mode,
        RuleCategory::Action,
    )?;

    Ok(())
}

fn generate_for_groups(
    groups: BTreeMap<&'static str, BTreeMap<&'static str, RuleMetadata>>,
    root: &Path,
    push_directory: &Path,
    mode: &Mode,
    kind: RuleCategory,
) -> Result<()> {
    let mut struct_groups = Vec::with_capacity(groups.len());
    let mut group_pascal_idents = Vec::with_capacity(groups.len());
    let mut group_idents = Vec::with_capacity(groups.len());
    let mut group_strings = Vec::with_capacity(groups.len());
    let mut group_as_default_rules = Vec::with_capacity(groups.len());
    let mut group_as_disabled_rules = Vec::with_capacity(groups.len());
    for (group, rules) in groups {
        let group_pascal_ident = quote::format_ident!("{}", &Case::Pascal.convert(group));
        let group_ident = quote::format_ident!("{}", group);

        let global_recommended = if group == "nursery" {
            quote! { !self.is_recommended_false() && biome_flags::is_unstable() }
        } else {
            quote! { !self.is_recommended_false() }
        };
        group_as_default_rules.push(quote! {
            if let Some(group) = self.#group_ident.as_ref() {
                group.collect_preset_rules(
                    #global_recommended,
                    &mut enabled_rules,
                );
                enabled_rules.extend(&group.get_enabled_rules());
                disabled_rules.extend(&group.get_disabled_rules());
            } else if #global_recommended {
                enabled_rules.extend(#group_pascal_ident::recommended_rules_as_filters());
            }
        });

        group_as_disabled_rules.push(quote! {
            if let Some(group) = self.#group_ident.as_ref() {
                disabled_rules.extend(&group.get_disabled_rules());
            }
        });

        group_pascal_idents.push(group_pascal_ident);
        group_idents.push(group_ident);
        group_strings.push(Literal::string(group));
        struct_groups.push(generate_group_struct(group, &rules, kind));
    }

    let severity_fn = if kind == RuleCategory::Action {
        quote! {
            /// Given a category coming from [Diagnostic](biome_diagnostics::Diagnostic), this function returns
            /// the [Severity](biome_diagnostics::Severity) associated to the rule, if the configuration changed it.
            /// If the severity is off or not set, then the function returns the default severity of the rule:
            /// [Severity::Error] for recommended rules and [Severity::Warning] for other rules.
            ///
            /// If not, the function returns [None].
            pub fn get_severity_from_category(&self, category: &Category) -> Option<Severity> {
                let mut split_code = category.name().split('/');

                let _lint = split_code.next();
                debug_assert_eq!(_lint, Some("assist"));

                let group = <RuleGroup as std::str::FromStr>::from_str(split_code.next()?).ok()?;
                let rule_name = split_code.next()?;
                let rule_name = Self::has_rule(group, rule_name)?;
                match group {
                    #(
                        RuleGroup::#group_pascal_idents => self
                            .#group_idents
                            .as_ref()
                            .and_then(|group| group.get_rule_configuration(rule_name))
                            .filter(|(level, _)| !matches!(level, RuleAssistPlainConfiguration::Off))
                            .map(|(level, _)| level.into())
                    )*
                }
            }

        }
    } else {
        quote! {

            /// Given a category coming from [Diagnostic](biome_diagnostics::Diagnostic), this function returns
            /// the [Severity](biome_diagnostics::Severity) associated to the rule, if the configuration changed it.
            /// If the severity is off or not set, then the function returns the default severity of the rule:
            /// [Severity::Error] for recommended rules and [Severity::Warning] for other rules.
            ///
            /// If not, the function returns [None].
            pub fn get_severity_from_category(&self, category: &Category, rule_severity: Severity) -> Option<Severity> {
                let mut split_code = category.name().split('/');

                let _lint = split_code.next();
                debug_assert_eq!(_lint, Some("lint"));

                let group = <RuleGroup as std::str::FromStr>::from_str(split_code.next()?).ok()?;
                let rule_name = split_code.next()?;
                let rule_name = Self::has_rule(group, rule_name)?;
                match group {
                    #(
                        RuleGroup::#group_pascal_idents => self
                            .#group_idents
                            .as_ref()
                            .and_then(|group| group.get_rule_configuration(rule_name))
                            .and_then(|(level, _)| match level {
                                RulePlainConfiguration::Off => None,
                                RulePlainConfiguration::On => Some(rule_severity),
                                RulePlainConfiguration::Info
                                | RulePlainConfiguration::Warn
                                | RulePlainConfiguration::Error => Some(Severity::from(level)),
                            }),
                    )*
                }
            }

        }
    };

    let use_rule_configuration = if kind == RuleCategory::Action {
        quote! {
            use crate::analyzer::{RuleAssistConfiguration, RuleAssistPlainConfiguration};
            use biome_analyze::{options::RuleOptions, RuleFilter};
        }
    } else {
        quote! {
            use crate::analyzer::{RuleConfiguration, RulePlainConfiguration, RuleFixConfiguration, SeverityOrGroup, RuleGroupExt};
            use biome_analyze::{options::RuleOptions, RuleFilter};
        }
    };

    let groups = if kind == RuleCategory::Action {
        quote! {
            #use_rule_configuration
            use biome_deserialize_macros::{Deserializable, Merge};
            use biome_diagnostics::{Category, Severity};
            use rustc_hash::FxHashSet;
            use serde::{Deserialize, Serialize};
            #[cfg(feature = "schema")]
            use schemars::JsonSchema;

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
            #[cfg_attr(feature = "schema", derive(JsonSchema))]
            #[serde(rename_all = "camelCase", deny_unknown_fields)]
            pub struct Actions {
                /// It enables the assist actions recommended by Biome. `true` by default.
                #[serde(skip_serializing_if = "Option::is_none")]
                pub recommended: Option<bool>,

                #(
                    #[deserializable(rename = #group_strings)]
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub #group_idents: Option<#group_pascal_idents>,
                )*
            }

            impl Actions {
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

                #severity_fn

                // Note: In top level, it is only considered _not_ recommended
                // when the recommended option is false
                pub(crate) const fn is_recommended_false(&self) -> bool {
                    matches!(self.recommended, Some(false))
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

                /// It returns the disabled rules by configuration
                pub fn as_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
                    let mut disabled_rules = FxHashSet::default();
                    #( #group_as_disabled_rules )*
                    disabled_rules
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
        }
    } else {
        quote! {
            #use_rule_configuration
            use biome_deserialize_macros::{Deserializable, Merge};
            use biome_diagnostics::{Category, Severity};
            use rustc_hash::FxHashSet;
            use serde::{Deserialize, Serialize};
            #[cfg(feature = "schema")]
            use schemars::JsonSchema;

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
            #[cfg_attr(feature = "schema", derive(JsonSchema))]
            #[serde(rename_all = "camelCase", deny_unknown_fields)]
            pub struct Rules {
                /// It enables the lint rules recommended by Biome. `true` by default.
                #[serde(skip_serializing_if = "Option::is_none")]
                pub recommended: Option<bool>,

                #(
                    #[deserializable(rename = #group_strings)]
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub #group_idents: Option<SeverityOrGroup<#group_pascal_idents>>,
                )*
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

                #severity_fn

                /// Ensure that `recommended` is set to `true` or implied.
                pub fn set_recommended(&mut self) {
                    if self.recommended == Some(false) {
                        self.recommended = Some(true)
                    }
                    #(
                        if let Some(group) = &mut self.#group_idents {
                            group.set_recommended(None);
                        }
                    )*
                }

                // Note: In top level, it is only considered _not_ recommended
                // when the recommended option is false
                pub(crate) const fn is_recommended_false(&self) -> bool {
                    matches!(self.recommended, Some(false))
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

                /// It returns the disabled rules by configuration
                pub fn as_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
                    let mut disabled_rules = FxHashSet::default();
                    #( #group_as_disabled_rules )*
                    disabled_rules
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
        }
    };

    let push_rules = match kind {
        RuleCategory::Lint => {
            quote! {
                use crate::analyzer::linter::*;
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
            }
        }
        RuleCategory::Action => {
            quote! {
                use crate::analyzer::assist::*;
                use biome_analyze::{AnalyzerRules, MetadataRegistry};

                pub fn push_to_analyzer_assist(
                    rules: &Actions,
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
            }
        }
        RuleCategory::Syntax | RuleCategory::Transformation => unimplemented!(),
    };

    let configuration = groups.to_string();
    let push_rules = push_rules.to_string();

    let file_name = match kind {
        RuleCategory::Lint => &push_directory.join("linter.rs"),
        RuleCategory::Action => &push_directory.join("assist.rs"),
        RuleCategory::Syntax | RuleCategory::Transformation => unimplemented!(),
    };

    let path = if kind == RuleCategory::Action {
        &root.join("actions.rs")
    } else {
        &root.join("rules.rs")
    };
    update(path, &xtask::reformat(configuration)?, mode)?;
    update(file_name, &xtask::reformat(push_rules)?, mode)?;

    Ok(())
}

fn generate_group_struct(
    group: &str,
    rules: &BTreeMap<&'static str, RuleMetadata>,
    kind: RuleCategory,
) -> TokenStream {
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
                    Event::End(TagEnd::Paragraph) => {
                        break;
                    }

                    Event::Start(tag) => match tag {
                        Tag::Strong | Tag::Paragraph => {
                            continue;
                        }

                        _ => panic!("Unimplemented tag {:?}", { tag }),
                    },

                    Event::End(tag) => match tag {
                        TagEnd::Strong | TagEnd::Paragraph => {
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
            if kind == RuleCategory::Action {
                "RuleAssistConfiguration"
            } else if metadata.fix_kind != FixKind::None {
                "RuleFixConfiguration"
            } else {
                "RuleConfiguration"
            }
        );
        let rule_name = Ident::new(&to_capitalized(rule), Span::call_site());
        if metadata.recommended && metadata.domains.is_empty() {
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
        let rule_option_type = match metadata.language {
            "css" => quote! {
                biome_css_analyze::options::#rule_name
            },
            "graphql" => quote! {
                biome_graphql_analyze::options::#rule_name
            },
            "json" => quote! {
                biome_json_analyze::options::#rule_name
            },
            "ts" | "js" | "jsx" | "tsx" => quote! {
                biome_js_analyze::options::#rule_name
            },
            _ => panic!("Language not supported"),
        };
        let rule_option = if kind == RuleCategory::Action {
            quote! { Option<#rule_config_type<#rule_option_type>> }
        } else {
            quote! {
                Option<#rule_config_type<#rule_option_type>>
            }
        };
        schema_lines_rules.push(quote! {
            #[doc = #summary]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub #rule_identifier: #rule_option
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

        if kind == RuleCategory::Action {
            get_rule_configuration_line.push(quote! {
                #rule => self.#rule_identifier.as_ref().map(|conf| (conf.level(), conf.get_options()))
            });
        } else {
            get_rule_configuration_line.push(quote! {
                #rule => self.#rule_identifier.as_ref().map(|conf| (conf.level(), conf.get_options()))
            });
        }
    }

    let group_pascal_ident = Ident::new(&to_capitalized(group), Span::call_site());

    let get_configuration_function = if kind == RuleCategory::Action {
        quote! {
            pub(crate) fn get_rule_configuration(&self, rule_name: &str) -> Option<(RuleAssistPlainConfiguration, Option<RuleOptions>)> {
                match rule_name {
                    #( #get_rule_configuration_line ),*,
                    _ => None
                }
            }
        }
    } else {
        quote! {
            fn get_rule_configuration(&self, rule_name: &str) -> Option<(RulePlainConfiguration, Option<RuleOptions>)> {
                match rule_name {
                    #( #get_rule_configuration_line ),*,
                    _ => None
                }
            }
        }
    };

    if kind == RuleCategory::Action {
        quote! {
            #[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
            #[cfg_attr(feature = "schema", derive(JsonSchema))]
            #[serde(rename_all = "camelCase", default, deny_unknown_fields)]
            /// A list of rules that belong to this group
            pub struct #group_pascal_ident {
                /// It enables the recommended rules for this group
                #[serde(skip_serializing_if = "Option::is_none")]
                pub recommended: Option<bool>,

                #( #schema_lines_rules ),*
            }

            impl #group_pascal_ident {

                const GROUP_NAME: &'static str = #group;
                pub(crate) const GROUP_RULES: &'static [&'static str] = &[
                    #( #lines_rule ),*
                ];

                const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
                    #( #lines_recommended_rule_as_filter ),*
                ];

                pub(crate) fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
                    Self::RECOMMENDED_RULES_AS_FILTERS
                }

                /// Retrieves the recommended rules
                pub(crate) fn is_recommended_true(&self) -> bool {
                    // we should inject recommended rules only when they are set to "true"
                    matches!(self.recommended, Some(true))
                }

                pub(crate) fn is_recommended_unset(&self) -> bool {
                    self.recommended.is_none()
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

                /// Select preset rules
                // Preset rules shouldn't populate disabled rules
                // because that will make specific rules cannot be enabled later.
                pub(crate) fn collect_preset_rules(
                    &self,
                    parent_is_recommended: bool,
                    enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
                ) {
                    // The order of the if-else branches MATTERS!
                    if self.is_recommended_true() || self.is_recommended_unset() && parent_is_recommended {
                        enabled_rules.extend(Self::recommended_rules_as_filters());
                    }
                }

                #get_configuration_function
            }
        }
    } else {
        quote! {
            #[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
            #[cfg_attr(feature = "schema", derive(JsonSchema))]
            #[serde(rename_all = "camelCase", default, deny_unknown_fields)]
            /// A list of rules that belong to this group
            pub struct #group_pascal_ident {
                /// It enables the recommended rules for this group
                #[serde(skip_serializing_if = "Option::is_none")]
                pub recommended: Option<bool>,

                #( #schema_lines_rules ),*
            }


            impl #group_pascal_ident {

                const GROUP_NAME: &'static str = #group;
                pub(crate) const GROUP_RULES: &'static [&'static str] = &[
                    #( #lines_rule ),*
                ];

                const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
                    #( #lines_recommended_rule_as_filter ),*
                ];

                const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
                    #( #lines_all_rule_as_filter ),*
                ];

            }

            impl RuleGroupExt for #group_pascal_ident {
                fn is_recommended_true(&self) -> bool {
                    // we should inject recommended rules only when they are set to "true"
                    matches!(self.recommended, Some(true))
                }

                fn is_recommended_unset(&self) -> bool {
                    self.recommended.is_none()
                }


                fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
                   let mut index_set = FxHashSet::default();
                   #( #rule_enabled_check_line )*
                   index_set
                }

                fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
                   let mut index_set = FxHashSet::default();
                   #( #rule_disabled_check_line )*
                   index_set
                }

                /// Checks if, given a rule name, matches one of the rules contained in this category
                fn has_rule(rule_name: &str) -> Option<&'static str> {
                    Some(Self::GROUP_RULES[Self::GROUP_RULES.binary_search(&rule_name).ok()?])
                }

                fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
                    Self::RECOMMENDED_RULES_AS_FILTERS
                }

                fn all_rules_as_filters() -> &'static [RuleFilter<'static>] {
                    Self::ALL_RULES_AS_FILTERS
                }

                /// Select preset rules
                // Preset rules shouldn't populate disabled rules
                // because that will make specific rules cannot be enabled later.
                fn collect_preset_rules(
                    &self,
                    parent_is_recommended: bool,
                    enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
                ) {
                    // The order of the if-else branches MATTERS!
                    if self.is_recommended_true() || self.is_recommended_unset() && parent_is_recommended {
                        enabled_rules.extend(Self::recommended_rules_as_filters());
                    }
                }

                fn set_recommended(&mut self, value: Option<bool>) {
                    self.recommended = value;
                }

                #get_configuration_function
            }
        }
    }
}
