#![expect(
    clippy::disallowed_methods,
    reason = "This rule compares CSS values that can span multiple tokens."
)]

use crate::fonts::{
    AnyCssFontValue, CssFontComponent, CssFontValue, find_font_family, font_components,
    is_font_family_keyword,
};
use crate::utils::{DuplicateGroup, collect_duplicates};
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{CssGenericProperty, decode_css_identifier};
use biome_diagnostics::Severity;
use biome_rowan::AstNode;
use biome_rule_options::no_duplicate_font_names::NoDuplicateFontNamesOptions;
use biome_string_case::StrLikeExtension;

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
    type State = DuplicateGroup<CssFontValue>;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateFontNamesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let Ok(property_name) = node.name() else {
            return Box::default();
        };
        let property_name = property_name.to_trimmed_text();
        let property_name = property_name.to_ascii_lowercase_cow();

        let is_font_family = property_name == "font-family";
        let is_font = property_name == "font";

        if !is_font_family && !is_font {
            return Box::default();
        }

        let Some(components) = node.value().ok().and_then(font_components) else {
            return Box::default();
        };
        let is_comma_separated_pair =
            matches!(components.as_slice(), [_, CssFontComponent::Comma, _]);
        let font_families = find_font_family(&components);

        if is_font_family
            && is_comma_separated_pair
            && let [first, second] = font_families.as_slice()
            && is_monospace_keyword(first)
            && is_monospace_keyword(second)
        {
            return Box::default();
        }

        let mut families = Vec::new();
        for css_value in font_families {
            // check the case: "Arial", Arial
            // we ignore the case of the font name is a keyword(context: https://github.com/stylelint/stylelint/issues/1284)
            // e.g "sans-serif", sans-serif
            if is_font
                && css_value.is_identifier()
                && css_value
                    .to_string()
                    .is_some_and(|value| is_font_family_keyword(&value))
            {
                continue;
            }

            let range = css_value.range();
            families.push((css_value.clone(), css_value, range));
        }

        collect_duplicates(families)
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let DuplicateGroup {
            name,
            first,
            duplicates,
        } = state;
        let name = name.to_string()?;
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            first,
            markup! {
                "The font name "<Emphasis>{name}</Emphasis>" is used more than once in the same property."
            },
        );

        for range in duplicates {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "The font name "<Emphasis>{name}</Emphasis>" is repeated here."
                },
            );
        }

        Some(diagnostic.note(markup! {
            "Duplicate font names are redundant and unnecessary. Remove them from the property."
        }))
    }
}

fn is_monospace_keyword(value: &CssFontValue) -> bool {
    let CssFontValue::SingleValue(AnyCssFontValue::CssIdentifier(identifier)) = value else {
        return false;
    };
    identifier.value_token().is_ok_and(|token| {
        decode_css_identifier(token.text_trimmed()).eq_ignore_ascii_case("monospace")
    })
}
