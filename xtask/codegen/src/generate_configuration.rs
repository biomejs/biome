use biome_analyze::{
    GroupCategory, Queryable, RegistryVisitor, Rule, RuleCategory, RuleGroup, RuleMetadata,
};
use biome_css_syntax::CssLanguage;
use biome_graphql_syntax::GraphqlLanguage;
use biome_html_syntax::HtmlLanguage;
use biome_js_syntax::JsLanguage;
use biome_json_syntax::JsonLanguage;
use biome_string_case::Case;
use proc_macro2::{Ident, Literal, Span};
use quote::{format_ident, quote};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use xtask_codegen::update;
use xtask_codegen::{generate_analyzer_rule_options, get_analyzer_rule_options_path};
use xtask_glue::*;

// ======= LINT ======
#[derive(Default)]
struct LintRulesVisitor {
    groups: BTreeMap<&'static str, BTreeMap<&'static str, RuleMetadata>>,
    /// Mapping from domain to group/rule
    /// e.g next => (<group>/<rule>, <group>/<rule>)
    domains: BTreeMap<&'static str, BTreeSet<(&'static str, &'static str)>>,
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

        for domain in R::METADATA.domains.iter() {
            self.domains
                .entry(domain.as_str())
                .or_default()
                .insert((<R::Group as RuleGroup>::NAME, R::METADATA.name));
        }
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

impl RegistryVisitor<HtmlLanguage> for LintRulesVisitor {
    fn record_category<C: GroupCategory<Language = HtmlLanguage>>(&mut self) {
        if matches!(C::CATEGORY, RuleCategory::Lint) {
            C::record_groups(self);
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = HtmlLanguage, Output: Clone>>
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

impl RegistryVisitor<HtmlLanguage> for AssistActionsVisitor {
    fn record_category<C: GroupCategory<Language = HtmlLanguage>>(&mut self) {
        if matches!(C::CATEGORY, RuleCategory::Action) {
            C::record_groups(self);
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = HtmlLanguage, Output: Clone>>
            + 'static,
    {
        self.groups
            .entry(<R::Group as RuleGroup>::NAME)
            .or_default()
            .insert(R::METADATA.name, R::METADATA);
    }
}

pub(crate) fn generate_rule_options(mode: Mode) -> Result<()> {
    let rule_options_root = get_analyzer_rule_options_path();
    let lib_root = rule_options_root.join("lib.rs");
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
    biome_html_analyze::visit_registry(&mut lint_visitor);
    biome_html_analyze::visit_registry(&mut assist_visitor);

    let mut rule_names = BTreeSet::default();
    let mut lib_exports = vec![quote! {
        mod shared;
        pub use shared::*;
    }];

    for group in lint_visitor.groups.values() {
        for rule_name in group.keys() {
            rule_names.insert(rule_name);
        }
    }

    for group in assist_visitor.groups.values() {
        for rule_name in group.keys() {
            rule_names.insert(rule_name);
        }
    }

    for rule_name in rule_names.iter() {
        let snake_rule_name = Case::Snake.convert(rule_name);
        let ident = Ident::new(&snake_rule_name, Span::call_site());
        lib_exports.push(quote! {
           pub mod #ident;
        });

        let file_name = format!("{snake_rule_name}.rs");
        let file_path = rule_options_root.join(file_name);
        if file_path.exists() {
            continue;
        }
        generate_analyzer_rule_options(rule_name, mode, false)?;
    }

    let content = quote! {
        #( #lib_exports )*
    };
    update(lib_root.as_path(), &xtask_glue::reformat(content)?, &mode)?;

    Ok(())
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
    biome_html_analyze::visit_registry(&mut lint_visitor);
    biome_html_analyze::visit_registry(&mut assist_visitor);

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

    generate_for_domains(lint_visitor.domains, &mode)?;

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
    #[derive(Debug)]
    struct RuleGroup {
        group_name: &'static str,
        rule_name: &'static str,
    }
    let mut rule_group_names: Vec<RuleGroup> = Vec::new();
    for (group_name, rules) in groups {
        let group_pascal_ident = quote::format_ident!("{}", &Case::Pascal.convert(group_name));
        let group_ident = quote::format_ident!("{}", group_name);

        let global_recommended = if group_name == "nursery" {
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
        group_idents.push(group_ident.clone());
        group_strings.push(Literal::string(group_name));
        // if kind == RuleCategory::Action {
        //     struct_groups.push(generate_group_struct(group_name, &rules, kind));
        //     // struct_groups.push(quote! {
        //     //     biome_configuration_macros::assist_group_struct!(#group_name);
        //     // });
        // } else {
        // }
        rule_group_names.extend(rules.keys().map(|rule_name| RuleGroup {
            rule_name,
            group_name,
        }));
    }
    if kind == RuleCategory::Lint {
        struct_groups.push(quote! {
            biome_configuration_macros::lint_group_structs!();
        });
    } else if kind == RuleCategory::Action {
        struct_groups.push(quote! {
            biome_configuration_macros::assist_group_structs!();
        });
    };

    rule_group_names.sort_unstable_by_key(|item| item.rule_name);
    rule_group_names.dedup_by_key(|item| item.rule_name);
    let rule_names: Vec<_> = rule_group_names.iter().map(|rg| rg.rule_name).collect();
    let rule_group_idents: Vec<_> = rule_group_names
        .iter()
        .map(|rg| format_ident!("{}", Case::Pascal.convert(rg.group_name)))
        .collect();
    let rule_idents: Vec<_> = rule_group_names
        .iter()
        .map(|rg| format_ident!("{}", Case::Pascal.convert(rg.rule_name)))
        .collect();

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
            use crate::analyzer::{GroupPlainConfiguration, RuleConfiguration, RulePlainConfiguration, RuleFixConfiguration, SeverityOrGroup, RuleGroupExt};
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
            impl std::fmt::Display for RuleGroup {
                fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                    fmt.write_str(self.as_str())
                }
            }

            #[derive(Clone, Copy, Debug, Deserializable, Eq, Hash, Merge, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
            #[cfg_attr(feature = "schema", derive(JsonSchema))]
            #[serde(rename_all = "camelCase")]
            pub enum ActionName {
                #( #rule_idents, )*
            }
            impl ActionName {
                pub const fn as_str(self) -> &'static str {
                    match self {
                        #( Self::#rule_idents => #rule_names, )*
                    }
                }
                pub const fn group(self) -> RuleGroup {
                    match self {
                        #( Self::#rule_idents => RuleGroup::#rule_group_idents, )*
                    }
                }
            }
            impl std::str::FromStr for ActionName {
                type Err = &'static str;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                        #( #rule_names => Ok(Self::#rule_idents), )*
                        _ => Err("This rule name doesn't exist.")
                    }
                }
            }
            impl std::fmt::Display for ActionName {
                fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                    fmt.write_str(self.as_str())
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
            impl std::fmt::Display for RuleGroup {
                fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                    fmt.write_str(self.as_str())
                }
            }

            #[derive(Clone, Copy, Debug, Deserializable, Eq, Hash, Merge, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
            #[cfg_attr(feature = "schema", derive(JsonSchema))]
            #[serde(rename_all = "camelCase")]
            pub enum RuleName {
                #( #rule_idents, )*
            }
            impl RuleName {
                pub const fn as_str(self) -> &'static str {
                    match self {
                        #( Self::#rule_idents => #rule_names, )*
                    }
                }
                pub const fn group(self) -> RuleGroup {
                    match self {
                        #( Self::#rule_idents => RuleGroup::#rule_group_idents, )*
                    }
                }
            }
            impl std::str::FromStr for RuleName {
                type Err = &'static str;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                        #( #rule_names => Ok(Self::#rule_idents), )*
                        _ => Err("This rule name doesn't exist.")
                    }
                }
            }
            impl std::fmt::Display for RuleName {
                fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                    fmt.write_str(self.as_str())
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
                                if let Some((_, Some(rule_options))) = rules.get_rule_configuration(rule_name)
                                    && let Some(rule_key) = metadata.find_rule(#group_strings, rule_name)
                                {
                                    analyzer_rules.push_rule(rule_key, rule_options);
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
                                if let Some((_, Some(rule_options))) = rules.get_rule_configuration(rule_name)
                                    && let Some(rule_key) = metadata.find_rule(#group_strings, rule_name)
                                {
                                    analyzer_rules.push_rule(rule_key, rule_options);
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
    update(path, &xtask_glue::reformat(configuration)?, mode)?;
    update(file_name, &xtask_glue::reformat(push_rules)?, mode)?;

    Ok(())
}

fn generate_for_domains(
    domains: BTreeMap<&'static str, BTreeSet<(&'static str, &'static str)>>,
    mode: &Mode,
) -> Result<()> {
    let destination =
        project_root().join("crates/biome_configuration/src/generated/domain_selector.rs");

    let mut as_rule_filters_arms = vec![];
    let mut match_rule_arms = vec![];
    let mut lazy_locks = vec![];
    for (domain_name, data) in domains {
        let vector = data
            .iter()
            .map(|(group, rules)| {
                quote! {
                    RuleFilter::Rule(#group, #rules)
                }
            })
            .collect::<Vec<_>>();
        let domain_filters = Ident::new(
            &format!("{}_FILTERS", domain_name.to_ascii_uppercase()),
            Span::call_site(),
        );

        lazy_locks.push(quote! {
            static #domain_filters: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
                vec![
                    #( #vector ),*
                ]
            });
        });

        let domain_as_string = Literal::string(domain_name);
        as_rule_filters_arms.push(quote! {
            #domain_as_string => #domain_filters.clone()
        });
        match_rule_arms.push(quote! {
            #domain_as_string => #domain_filters.iter().any(|filter| filter.match_rule::<R>())
        });
    }

    let stream = quote! {
        use std::sync::LazyLock;
        use crate::analyzer::DomainSelector;
        use biome_analyze::{Rule, RuleFilter};

        #( #lazy_locks )*

        impl DomainSelector {
            pub fn as_rule_filters(&self) -> Vec<RuleFilter<'static>> {
                match self.0 {
                    #( #as_rule_filters_arms ),*,
                    _ => unreachable!(
                        "DomainFilter::as_rule_filters: domain {} not found",
                        self.0
                    )
                }
            }

            pub fn match_rule<R>(&self) -> bool
                where
                    R: Rule,
            {
                match self.0 {
                    #( #match_rule_arms ),*,
                    _ => false,
                }
            }
        }
    };

    update(destination.as_path(), &xtask_glue::reformat(stream)?, mode)?;

    Ok(())
}
