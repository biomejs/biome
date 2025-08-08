use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{AnyCssGenericComponentValue, AnyCssValue, CssGenericProperty};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_duplicate_font_names::NoDuplicateFontNamesOptions;
use biome_string_case::StrLikeExtension;
use std::collections::HashSet;

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
        sources: &[RuleSource::Stylelint("font-family-no-duplicate-names").same()],
    }
}

pub struct RuleState {
    value: Box<str>,
    span: TextRange,
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

        let mut family_names: HashSet<String> = HashSet::new();
        let value_list = node.value();
        
        // Parse font families, handling both quoted strings and unquoted multi-word identifiers
        let font_families = if is_font {
            parse_font_families_from_font_shorthand(find_font_family(value_list))
        } else {
            parse_font_families_from_font_family_property(value_list)
        };

        for (font_name, span) in font_families {
            // Normalize font name by removing quotes and whitespace for comparison
            let normalized_font_name: String = font_name
                .chars()
                .filter(|&c| c != '\'' && c != '\"' && !c.is_whitespace())
                .collect();

            if family_names.contains(&normalized_font_name) {
                // Allow mixed quoted/unquoted duplicates for font family keywords (e.g., "sans-serif" and sans-serif)
                // but still flag identical duplicates (e.g., sans-serif, sans-serif)
                if is_font_family_keyword(&normalized_font_name) && font_name != normalized_font_name {
                    // This is a quoted vs unquoted case - allow it
                    continue;
                }
                return Some(RuleState {
                    value: normalized_font_name.into(),
                    span,
                });
            }
            family_names.insert(normalized_font_name);
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

/// Parse font families from a font-family property value list
/// Handles multi-word unquoted font names by grouping consecutive identifiers
fn parse_font_families_from_font_family_property(
    value_list: biome_css_syntax::CssGenericComponentValueList,
) -> Vec<(String, TextRange)> {
    let mut font_families = Vec::new();
    let mut current_family_parts = Vec::new();
    let mut current_family_span_start: Option<TextRange> = None;
    let mut current_family_span_end: Option<TextRange> = None;

    for component in value_list {
        match component {
            AnyCssGenericComponentValue::CssGenericDelimiter(_) => {
                // Comma separator - finish current font family if any
                if !current_family_parts.is_empty() {
                    let font_name = current_family_parts.join(" ");
                    let span = current_family_span_start
                        .unwrap()
                        .cover(current_family_span_end.unwrap());
                    font_families.push((font_name, span));
                    current_family_parts.clear();
                    current_family_span_start = None;
                    current_family_span_end = None;
                }
            }
            AnyCssGenericComponentValue::AnyCssValue(css_value) => {
                match css_value {
                    AnyCssValue::CssIdentifier(val) => {
                        // Unquoted identifier - part of a multi-word font name
                        let part = val.to_trimmed_text();
                        current_family_parts.push(part.text().to_string());
                        if current_family_span_start.is_none() {
                            current_family_span_start = Some(val.range());
                        }
                        current_family_span_end = Some(val.range());
                    }
                    AnyCssValue::CssString(val) => {
                        // Quoted string - complete font family name
                        let font_name = val.to_trimmed_text();
                        font_families.push((font_name.text().to_string(), val.range()));
                    }
                    _ => {}
                }
            }
        }
    }

    // Handle the last font family if there's no trailing comma
    if !current_family_parts.is_empty() {
        let font_name = current_family_parts.join(" ");
        let span = current_family_span_start
            .unwrap()
            .cover(current_family_span_end.unwrap());
        font_families.push((font_name, span));
    }

    font_families
}

/// Parse font families from font shorthand property values
fn parse_font_families_from_font_shorthand(
    css_values: Vec<AnyCssValue>,
) -> Vec<(String, TextRange)> {
    let mut font_families = Vec::new();
    let mut current_family_parts = Vec::new();
    let mut current_family_span_start: Option<TextRange> = None;
    let mut current_family_span_end: Option<TextRange> = None;

    for css_value in css_values {
        match css_value {
            AnyCssValue::CssIdentifier(val) => {
                // Unquoted identifier - part of a multi-word font name
                let part = val.to_trimmed_text();
                current_family_parts.push(part.text().to_string());
                if current_family_span_start.is_none() {
                    current_family_span_start = Some(val.range());
                }
                current_family_span_end = Some(val.range());
            }
            AnyCssValue::CssString(val) => {
                // If we have accumulated parts, finish the current family first
                if !current_family_parts.is_empty() {
                    let font_name = current_family_parts.join(" ");
                    let span = current_family_span_start
                        .unwrap()
                        .cover(current_family_span_end.unwrap());
                    font_families.push((font_name, span));
                    current_family_parts.clear();
                    current_family_span_start = None;
                    current_family_span_end = None;
                }
                
                // Quoted string - complete font family name
                let font_name = val.to_trimmed_text();
                font_families.push((font_name.text().to_string(), val.range()));
            }
            _ => {}
        }
    }

    // Handle the last font family if there are remaining parts
    if !current_family_parts.is_empty() {
        let font_name = current_family_parts.join(" ");
        let span = current_family_span_start
            .unwrap()
            .cover(current_family_span_end.unwrap());
        font_families.push((font_name, span));
    }

    font_families
}
