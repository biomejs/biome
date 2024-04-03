use std::collections::HashSet;

use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssGenericComponentValue, AnyCssValue, CssGenericComponentValueList, CssGenericProperty,
};
use biome_rowan::{AstNode, SyntaxNodeCast, TextRange};

use crate::utils::{is_css_variable, is_font_family_keyword, is_font_shorthand_keyword};

declare_rule! {
    /// Disallow duplicate names within font families.
    ///
    /// This rule checks the `font` and `font-family` properties for duplicate font names.
    ///
    /// This rule ignores var(--custom-property) variable syntaxes.
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
    pub NoFontFamilyDuplicateNames {
        version: "next",
        name: "noFontFamilyDuplicateNames",
        recommended: true,
        source: RuleSource::EslintImportAccess("font-family-no-duplicate-names"),
    }
}

pub struct RuleState {
    value: String,
    span: TextRange,
}

impl Rule for NoFontFamilyDuplicateNames {
    type Query = Ast<CssGenericProperty>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let property_name = node.name().ok()?.text().to_lowercase();

        let is_font_family = property_name == "font-family";
        let is_font = property_name == "font";

        if !is_font_family && !is_font {
            return None;
        }

        let mut seen: HashSet<String> = HashSet::new();
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
                // A generic family name like `serif` or `sans-serif`
                AnyCssValue::CssIdentifier(val) => {
                    let font_name = val.text();
                    if seen.contains(&font_name) {
                        return Some(RuleState {
                            value: font_name.clone(),
                            span: val.range(),
                        });
                    }
                    seen.insert(font_name);
                }
                // A font family name. e.g "Lucida Grande", "Arial".
                AnyCssValue::CssString(val) => {
                    let normalized_font_name: String = val
                        .text()
                        .chars()
                        .filter(|&c| c != '\'' && c != '\"' && !c.is_whitespace())
                        .collect();

                    if seen.contains(&normalized_font_name) {
                        return Some(RuleState {
                            value: normalized_font_name,
                            span: val.range(),
                        });
                    }
                    seen.insert(normalized_font_name);
                }
                _ => continue,
            }
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let span = state.span;
        Some(RuleDiagnostic::new(
            rule_category!(),
            span,
            markup! {
                "Unexpected duplicate font name: "<Emphasis>{ state.value }</Emphasis>
            },
        ))
    }
}

// Get the font-families within a `font` shorthand property value.
fn find_font_family(value: CssGenericComponentValueList) -> Vec<AnyCssValue> {
    let mut font_families: Vec<AnyCssValue> = Vec::new();
    for v in value {
        let lower_case_value = v.text().to_lowercase();

        // Ignore CSS variables
        if is_css_variable(&lower_case_value) {
            continue;
        }

        // Ignore keywords for other font parts
        if is_font_shorthand_keyword(&lower_case_value)
            && !is_font_family_keyword(&lower_case_value)
        {
            continue;
        }

        // Ignore font-sizes
        if matches!(
            v,
            AnyCssGenericComponentValue::AnyCssValue(AnyCssValue::AnyCssDimension(_))
        ) {
            continue;
        }

        // Ignore anything come after a <font-size>/, because it's a line-height
        if let Some(prev_node) = v.syntax().prev_sibling() {
            if let Some(prev_prev_node) = prev_node.prev_sibling() {
                if let Some(slash) = prev_node.cast::<AnyCssGenericComponentValue>() {
                    if let Some(size) = prev_prev_node.cast::<AnyCssGenericComponentValue>() {
                        if matches!(
                            size,
                            AnyCssGenericComponentValue::AnyCssValue(AnyCssValue::AnyCssDimension(
                                _
                            ))
                        ) && matches!(slash, AnyCssGenericComponentValue::CssGenericDelimiter(_))
                        {
                            continue;
                        }
                    }
                };
            }
        }

        // Ignore number values
        if matches!(
            v,
            AnyCssGenericComponentValue::AnyCssValue(AnyCssValue::CssNumber(_))
        ) {
            continue;
        }

        match v {
            AnyCssGenericComponentValue::CssGenericDelimiter(_) => continue,
            AnyCssGenericComponentValue::AnyCssValue(css_value) => match css_value {
                AnyCssValue::CssIdentifier(_) | AnyCssValue::CssString(_) => {
                    font_families.push(css_value)
                }
                _ => continue,
            },
        }
    }
    font_families
}
