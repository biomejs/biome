use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{AnyCssGenericComponentValue, AnyCssValue, CssGenericProperty};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, Text, TextRange};
use biome_rule_options::no_duplicate_font_names::NoDuplicateFontNamesOptions;
use biome_string_case::StrLikeExtension;
use std::collections::HashSet;

use crate::utils::is_font_family_keyword;

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
        sources: &[RuleSource::Stylelint("font-family-no-duplicate-names").same()],
    }
}

pub struct RuleState {
    value: Box<str>,
    span: TextRange,
}

#[derive(Debug)]
struct FontFamily {
    text: Text,
    range: TextRange,
    is_quoted: bool,
}

impl Rule for NoDuplicateFontNames {
    type Query = Ast<CssGenericProperty>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoDuplicateFontNamesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let property_name = node.name().ok()?.to_trimmed_text();
        let property_name = property_name.to_ascii_lowercase_cow();

        let is_font_family = property_name == "font-family";
        let is_font = property_name == "font";

        if !is_font_family && !is_font {
            return None;
        }

        let value_list = node.value();

        let mut current_font_texts: Vec<Text> = Vec::new();
        let mut first_range: Option<TextRange> = None;
        let mut last_range: Option<TextRange> = None;
        let mut font_families: Vec<FontFamily> = Vec::new();

        for c in value_list {
            let is_last_value_node = c.syntax().next_sibling().is_none();
            match c {
                AnyCssGenericComponentValue::AnyCssValue(css_value) => match css_value {
                    AnyCssValue::CssIdentifier(val) => {
                        let text = val.to_trimmed_text();
                        let range = val.range();

                        // Last identifier without trailing comma should be treated as a complete font family
                        if is_last_value_node {
                            font_families.push(FontFamily {
                                text: text.clone(),
                                range,
                                is_quoted: false,
                            });
                            continue;
                        }

                        current_font_texts.push(text);
                        if first_range.is_none() {
                            first_range = Some(range);
                        }
                        last_range = Some(range);
                    }
                    AnyCssValue::CssString(val) => {
                        let text = val
                            .to_trimmed_string()
                            .trim_matches(|c| c == '\'' || c == '"')
                            .trim()
                            .to_string();
                        let range = val.range();

                        font_families.push(FontFamily {
                            text: text.into(),
                            range,
                            is_quoted: true,
                        });
                    }
                    _ => {}
                },
                AnyCssGenericComponentValue::CssGenericDelimiter(_) => {
                    if !current_font_texts.is_empty() {
                        let merged_font = current_font_texts.join(" ");
                        let merged_range = first_range?.cover(last_range?);

                        font_families.push(FontFamily {
                            text: merged_font.into(),
                            range: merged_range,
                            is_quoted: false,
                        });

                        current_font_texts.clear();
                        first_range = None;
                        last_range = None;
                    }
                }
            }
        }

        let mut family_names: HashSet<Text> = HashSet::new();
        let mut family_keywords: HashSet<(Text, bool)> = HashSet::new();

        for font_family in font_families {
            let is_keyword = is_font_family_keyword(&font_family.text);
            let is_quoted = font_family.is_quoted;

            // Keywords require special handling based on quote status:
            // - Quoted keywords ("sans-serif") are treated as actual font names
            // - Unquoted keywords (sans-serif) are treated as CSS generic font families
            // These are technically different and should not be considered duplicates.
            // See: https://github.com/stylelint/stylelint/issues/1284
            if is_keyword {
                if family_keywords.contains(&(font_family.text.clone(), is_quoted)) {
                    return Some(RuleState {
                        value: font_family.text.into(),
                        span: font_family.range,
                    });
                }
                family_keywords.insert((font_family.text, is_quoted));
                continue;
            }

            if family_names.contains(&font_family.text) {
                return Some(RuleState {
                    value: font_family.text.into(),
                    span: font_family.range,
                });
            }
            family_names.insert(font_family.text);
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
