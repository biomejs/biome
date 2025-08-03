use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{CssGenericProperty, CssSyntaxKind};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_duplicate_font_names::NoDuplicateFontNamesOptions;
use biome_string_case::StrLikeExtension;
use rustc_hash::FxHashSet;

use crate::utils::{FontFamily, FontPropertyKind, collect_font_families, is_font_family_keyword};

fn normalize_font_name(font_name: &str) -> String {
    let trimmed = font_name.trim();

    if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
    {
        trimmed[1..trimmed.len() - 1].trim().to_string()
    } else {
        trimmed.to_string()
    }
}

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
        let is_font_shorthand = property_name == "font";

        if !is_font_family && !is_font_shorthand {
            return None;
        }

        let value_list = node.value();

        let kind = if is_font_shorthand {
            FontPropertyKind::Shorthand
        } else {
            FontPropertyKind::FontFamily
        };

        let font_families: Vec<FontFamily> = collect_font_families(value_list, kind);

        let mut family_names: FxHashSet<String> = FxHashSet::default();
        let mut family_keywords: FxHashSet<(String, bool)> = FxHashSet::default();

        for font_family in font_families {
            let text = if font_family.tokens.len() == 1 {
                font_family.tokens.first()?.token_text_trimmed().to_string()
            } else {
                font_family
                    .tokens
                    .iter()
                    .map(|token| token.token_text_trimmed())
                    .collect::<Vec<_>>()
                    .join(" ")
            };

            let is_keyword = is_font_family_keyword(&text);
            let is_quoted = font_family
                .tokens
                .iter()
                .any(|token| token.kind() == CssSyntaxKind::CSS_STRING_LITERAL);

            let range = if font_family.tokens.len() == 1 {
                font_family.tokens.first()?.text_trimmed_range()
            } else {
                let start = font_family.tokens.first()?.text_trimmed_range();
                let end = font_family.tokens.last()?.text_trimmed_range();
                start.cover(end)
            };

            // Keywords require special handling based on quote status:
            // - Quoted keywords ("sans-serif") are treated as actual font names
            // - Unquoted keywords (sans-serif) are treated as CSS generic font families
            // These are technically different and should not be considered duplicates.
            // See: https://github.com/stylelint/stylelint/issues/1284
            if is_keyword {
                if !family_keywords.insert((text.clone(), is_quoted)) {
                    return Some(RuleState {
                        value: text.into(),
                        span: range,
                    });
                }
                continue;
            }

            let normalized_text = normalize_font_name(&text);
            if !family_names.insert(normalized_text) {
                return Some(RuleState {
                    value: text.into(),
                    span: range,
                });
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
