use crate::fonts::{CssFontValue, find_font_family, is_font_family_keyword};
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::CssGenericProperty;
use biome_diagnostics::Severity;
use biome_rowan::AstNode;
use biome_rule_options::no_duplicate_font_names::NoDuplicateFontNamesOptions;
use biome_string_case::StrLikeExtension;
use std::collections::HashSet;

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
    /// b { font: normal 14px/32px -apple-system, BlinkMacSystemFont, sans-serif; }
    /// c { font-family: SF Mono, Liberation Mono, sans-serif; }
    /// d { font: 1em SF Mono, Liberation Mono, sans-serif; }
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
        let value_list = node.value();
        let font_families = find_font_family(value_list);

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
