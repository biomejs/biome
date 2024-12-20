use std::collections::HashSet;

use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::{AnyCssGenericComponentValue, AnyCssValue, CssGenericProperty};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange};
use biome_string_case::StrLikeExtension;

use crate::utils::{find_font_family, is_font_family_keyword};

declare_lint_rule! {
    /// Disallow duplicate names within font families.
    ///
    /// This rule checks the `font` and `font-family` properties for duplicate font names.
    ///
    /// This rule ignores var(--custom-property) variable syntaxes now.
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a { font-family: "Lucida Grande", 'Arial', sans-serif, sans-serif; }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a { font-family: 'Arial', "Lucida Grande", Arial, sans-serif; }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a { FONT: italic 300 16px/30px Arial, " Arial", serif; }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a { font-family: "Lucida Grande", "Arial", sans-serif; }
    /// ```
    ///
    /// ```css
    /// b { font: normal 14px/32px -apple-system, BlinkMacSystemFont, sans-serif; }
    /// ```
    pub NoDuplicateFontNames {
        version: "1.8.0",
        name: "noDuplicateFontNames",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("font-family-no-duplicate-names")],
    }
}

pub struct RuleState {
    value: String,
    span: TextRange,
}

impl Rule for NoDuplicateFontNames {
    type Query = Ast<CssGenericProperty>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let property_name = node.name().ok()?.to_trimmed_string();
        let property_name = property_name.to_ascii_lowercase_cow();

        let is_font_family = property_name == "font-family";
        let is_font = property_name == "font";

        if !is_font_family && !is_font {
            return None;
        }

        let mut unquoted_family_names: HashSet<String> = HashSet::new();
        let mut family_names: HashSet<String> = HashSet::new();
        let value_list = node.value();
        let font_families = if is_font {
            find_font_family(value_list)
        } else {
            value_list
                .into_iter()
                .filter_map(|v| match v {
                    AnyCssGenericComponentValue::AnyCssValue(value) => Some(value),
                    _ => None,
                })
                .collect()
        };

        for css_value in font_families {
            match css_value {
                // A generic family name like `sans-serif` or unquoted font name.
                AnyCssValue::CssIdentifier(val) => {
                    let font_name = val.to_trimmed_string();

                    // check the case: "Arial", Arial
                    // we ignore the case of the font name is a keyword(context: https://github.com/stylelint/stylelint/issues/1284)
                    // e.g "sans-serif", sans-serif
                    if family_names.contains(&font_name) && !is_font_family_keyword(&font_name) {
                        return Some(RuleState {
                            value: font_name,
                            span: val.range(),
                        });
                    }

                    // check the case: sans-self, sans-self
                    if unquoted_family_names.contains(&font_name) {
                        return Some(RuleState {
                            value: font_name,
                            span: val.range(),
                        });
                    }
                    unquoted_family_names.insert(font_name);
                }
                // A font family name. e.g "Lucida Grande", "Arial".
                AnyCssValue::CssString(val) => {
                    // FIXME: avoid String allocation
                    let normalized_font_name: String = val
                        .to_trimmed_string()
                        .chars()
                        .filter(|&c| c != '\'' && c != '\"' && !c.is_whitespace())
                        .collect();

                    if family_names.contains(&normalized_font_name)
                        || unquoted_family_names.contains(&normalized_font_name)
                    {
                        return Some(RuleState {
                            value: normalized_font_name,
                            span: val.range(),
                        });
                    }
                    family_names.insert(normalized_font_name);
                }
                _ => continue,
            }
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let span = state.span;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Duplicate font names are redundant and unnecessary: "<Emphasis>{ state.value }</Emphasis>
                },
            )
            .note(markup! {
                "Remove duplicate font names within the property"
            }),
        )
    }
}
