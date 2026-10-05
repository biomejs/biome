#![expect(
    clippy::disallowed_methods,
    reason = "This rule compares CSS values that can span multiple tokens."
)]

use crate::fonts::{AnyCssFontValue, CssFontValue, find_font_family, font_components, is_font_family_keyword};
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssGenericPropertyValueOrExpression, CssGenericProperty, T, decode_css_identifier,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::no_duplicate_font_names::NoDuplicateFontNamesOptions;
use biome_string_case::StrLikeExtension;
use std::collections::HashSet;

declare_lint_rule! {
    /// Disallow duplicate names in font-family lists.
    ///
    /// The rule checks the `font` and `font-family` properties. It skips values supplied through
    /// `var()` because their contents are not known during analysis.
    ///
    /// The unquoted pair `font-family: monospace, monospace` is allowed. This pattern preserves an
    /// inherited font size instead of using the browser's preferred monospace size. See
    /// [MDN's explanation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-family#monospace_font_size).
    ////
    /// This rule ignores `var(--custom-property)` values.
    ///
    /// ## SCSS limitations
    ///
    /// Font values that require SCSS evaluation, including variables, interpolation, and
    /// user-defined function results, are ignored because the emitted font names are unknown.
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
    /// b { font: normal 14px/32px -apple-system, BlinkMacSystemFont, sans-serif; }
    /// c { font-family: SF Mono, Liberation Mono, sans-serif; }
    /// d { font: 1em SF Mono, Liberation Mono, sans-serif; }
    /// e { font-family: monospace, monospace; }
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

impl Rule for NoDuplicateFontNames {
    type Query = Ast<CssGenericProperty>;
    type State = (CssFontValue, CssFontValue);
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

        let mut family_names: HashSet<CssFontValue> = HashSet::new();
        let components = font_components(node.value().ok()?)?;
        let is_comma_separated_pair = value_list.len() == 3
            && value_list
                .iter()
                .nth(1)
                .and_then(|value| value.as_css_generic_delimiter()?.value().ok())
                .is_some_and(|token| token.kind() == T![,]);
        let font_families = find_font_family(&components);

        if is_font_family
            && is_comma_separated_pair
            && let [first, second] = font_families.as_slice()
            && is_monospace_keyword(first)
            && is_monospace_keyword(second)
        {
            return None;
        }

        for css_value in font_families {
            let value = css_value.to_string()?;

            // check the case: "Arial", Arial
            // we ignore the case of the font name is a keyword(context: https://github.com/stylelint/stylelint/issues/1284)
            // e.g "sans-serif", sans-serif
            if css_value.is_identifier() && is_font_family_keyword(&value) && is_font {
                continue;
            }

            if let Some(duplicate) = family_names.get(&css_value) {
                return Some((css_value.clone(), duplicate.clone()));
            } else {
                family_names.insert(css_value.clone());
            }
        }
        None
    }

    fn diagnostic(
        _: &RuleContext<Self>,
        (this, duplicate): &Self::State,
    ) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                this.range(),
                markup! {
                    "Duplicate font names are redundant and unnecessary: "<Emphasis>{ this.to_string()? }</Emphasis>
                },
            )
            .detail(duplicate.range(), markup! {
                "This is where the duplicate font name is found:"
            })
            .note(markup! {
                "Remove duplicate font names within the property."
            }),
        )
    }
}

fn is_monospace_keyword(value: &CssFontValue) -> bool {
    let CssFontValue::SingleValue(AnyCssFontValue::CssIdentifier(identifier)) = value else {
        return false;
    };
    identifier
        .value_token()
        .is_ok_and(|token| {
            decode_css_identifier(token.text_trimmed()).eq_ignore_ascii_case("monospace")
        })
}
