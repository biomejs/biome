use std::collections::BTreeMap;

use biome_analyze::{FixKind, RuleCategory, RuleMetadata};
use biome_string_case::Case;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use quote::quote;

use crate::to_capitalized;

pub fn generate_group_struct(
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
    let mut rule_identifiers = Vec::new();

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
                        Tag::Emphasis | Tag::Strong | Tag::Paragraph => {}
                        _ => panic!("Unimplemented tag {:?}", { tag }),
                    },

                    Event::End(tag) => match tag {
                        TagEnd::Emphasis | TagEnd::Strong | TagEnd::Paragraph => {}
                        _ => panic!("Unimplemented tag {:?}", { tag }),
                    },

                    _ => {
                        panic!("Unimplemented event {:?}", { event })
                    }
                }
            }

            let kebab_rule_name = Case::Kebab.convert(rule);
            let url = if kind == RuleCategory::Action {
                format!("https://biomejs.dev/assist/actions/{}", kebab_rule_name)
            } else {
                format!("https://biomejs.dev/linter/rules/{}", kebab_rule_name)
            };

            if !docs.is_empty() {
                let docs = docs.trim_end_matches('.');
                format!("{}.\nSee <{}>", docs, url)
            } else {
                format!("See <{}>", url)
            }
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
        let rule_base_name = Ident::new(&Case::Snake.convert(rule), Span::call_site());
        let rule_name = Ident::new(
            &format!("{}Options", &to_capitalized(rule)),
            Span::call_site(),
        );
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
        let rule_option_type = quote! {
            biome_rule_options::#rule_base_name::#rule_name
        };

        let rule_option = quote! { Option<#rule_config_type<#rule_option_type>> };
        schema_lines_rules.push(quote! {
            #[doc = #summary]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub #rule_identifier: #rule_option
        });

        rule_enabled_check_line.push(quote! {
            if let Some(rule) = self.#rule_identifier.as_ref()
                && rule.is_enabled()
            {
                index_set.insert(RuleFilter::Rule(
                    Self::GROUP_NAME,
                    Self::GROUP_RULES[#rule_position],
                ));
            }
        });
        rule_disabled_check_line.push(quote! {
            if let Some(rule) = self.#rule_identifier.as_ref()
                && rule.is_disabled()
            {
                index_set.insert(RuleFilter::Rule(
                    Self::GROUP_NAME,
                    Self::GROUP_RULES[#rule_position],
                ));
            }
        });

        get_rule_configuration_line.push(quote! {
            #rule => self.#rule_identifier.as_ref().map(|conf| (conf.level(), conf.get_options()))
        });

        rule_identifiers.push(rule_identifier);
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
                /// Enables the recommended rules for this group
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
                /// Enables the recommended rules for this group
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

            impl From<GroupPlainConfiguration> for #group_pascal_ident {
                fn from(value: GroupPlainConfiguration) -> Self {
                    Self {
                        recommended: None,
                        #( #rule_identifiers: Some(value.into()), )*
                    }
                }
            }
        }
    }
}
