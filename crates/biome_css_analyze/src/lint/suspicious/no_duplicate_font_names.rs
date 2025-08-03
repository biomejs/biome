use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{CssGenericProperty, CssSyntaxKind, CssSyntaxToken, inner_string_text};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_duplicate_font_names::NoDuplicateFontNamesOptions;
use biome_string_case::StrLikeExtension;
use rustc_hash::FxHashSet;

use crate::utils::{FontFamily, FontPropertyKind, collect_font_families, is_font_family_keyword};

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
            let inner_text = if font_family.tokens.len() == 1 {
                inner_string_text(font_family.tokens.first()?)
                    .trim()
                    .to_string()
            } else {
                font_family
                    .tokens
                    .iter()
                    .map(inner_string_text)
                    .collect::<Vec<_>>()
                    .join(" ")
            };

            let is_keyword = is_font_family_keyword(&inner_text);
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

            // Generic font family keywords require special handling based on quote status:
            // - Unquoted keywords (sans-serif) are treated as CSS generic font families
            // - Quoted keywords ("sans-serif") are treated as actual font names
            // These are technically different and should not be considered duplicates.
            // see: https://www.w3.org/TR/css-fonts-4/#family-name-value:~:text=Note%20that%20%3Cgeneric%2Dfamily%3E%20keywords%20cannot%20be%20quoted%20(otherwise%20they%20are%20interpreted%20as%20a%20%3Cfamily%2Dname%3E).
            if is_keyword {
                if !family_keywords.insert((inner_text.clone(), is_quoted)) {
                    let original_text = get_original_text(&font_family.tokens)?;
                    return Some(RuleState {
                        value: original_text.into(),
                        span: range,
                    });
                }
                continue;
            }

            if !family_names.insert(inner_text.clone()) {
                let original_text = get_original_text(&font_family.tokens)?;
                return Some(RuleState {
                    value: original_text.into(),
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

fn get_original_text(tokens: &[CssSyntaxToken]) -> Option<String> {
    if tokens.len() == 1 {
        tokens.first().map(|t| t.token_text_trimmed().to_string())
    } else {
        Some(
            tokens
                .iter()
                .map(|t| t.token_text_trimmed())
                .collect::<Vec<_>>()
                .join(" "),
        )
    }
}
