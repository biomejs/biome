use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssAtRule, AnyCssGenericComponentValue, AnyCssValue, CssAtRule,
    CssGenericComponentValueList, CssGenericProperty, CssSyntaxKind,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, SyntaxNodeCast, TextRange};
use biome_string_case::StrLikeExtension;

use crate::utils::{
    find_font_family, is_css_variable, is_font_family_keyword, is_system_family_name_keyword,
};

declare_lint_rule! {
    /// Disallow a missing generic family keyword within font families.
    ///
    /// The generic font family can be:
    /// - placed anywhere in the font family list
    /// - omitted if a keyword related to property inheritance or a system font is used
    ///
    /// This rule checks the font and font-family properties.
    /// The following special situations are ignored:
    /// - Property with a keyword value such as `inherit`, `initial`.
    /// - The last value being a CSS variable.
    /// - `font-family` property in an `@font-face` rule.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a { font-family: Arial; }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a { font: normal 14px/32px -apple-system, BlinkMacSystemFont; }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a { font-family: "Lucida Grande", "Arial", sans-serif; }
    /// ```
    ///
    /// ```css
    /// a { font-family: inherit; }
    /// ```
    ///
    /// ```css
    /// a { font-family: sans-serif; }
    /// ```
    ///
    /// ```css
    /// a { font-family: var(--font); }
    /// ```
    ///
    /// ```css
    /// @font-face { font-family: Gentium; }
    /// ```
    ///
    pub UseGenericFontNames {
        version: "1.8.0",
        name: "useGenericFontNames",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("font-family-no-missing-generic-family-keyword")],
    }
}

impl Rule for UseGenericFontNames {
    type Query = Ast<CssGenericProperty>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let property_name = node.name().ok()?.to_trimmed_string();
        let property_name = property_name.to_ascii_lowercase_cow();

        // Ignore `@font-face`. See more detail: https://drafts.csswg.org/css-fonts/#font-face-rule
        if is_in_font_face_at_rule(node) {
            return None;
        }

        let is_font_family = property_name == "font-family";
        let is_font = property_name == "font";

        if !is_font_family && !is_font {
            return None;
        }

        // handle shorthand font property with special value
        // e.g: { font: caption }, { font: inherit }
        let properties = node.value();
        if is_font && is_shorthand_font_property_with_keyword(&properties) {
            return None;
        }

        let font_families = if is_font {
            find_font_family(properties)
        } else {
            collect_font_family_properties(properties)
        };

        if font_families.is_empty() {
            return None;
        }

        if has_generic_font_family_property(&font_families) {
            return None;
        }

        // Ignore the last value if it's a CSS variable now.
        let last_value = font_families.last()?;
        if is_css_variable(&last_value.to_trimmed_string()) {
            return None;
        }

        Some(last_value.range())
    }

    fn diagnostic(_: &RuleContext<Self>, span: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Generic font family missing."
                },
            )
            .note(markup! {
                    "Consider adding a generic font family as a fallback."
            })
            .footer_list(
                markup! {
                    "For examples and more information, see" <Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/CSS/generic-family">" the MDN Web Docs"</Hyperlink>
                },
                ["serif", "sans-serif", "monospace", "etc."],
            ),
        )
    }
}

fn is_in_font_face_at_rule(node: &CssGenericProperty) -> bool {
    node.syntax()
        .ancestors()
        .find(|n| n.kind() == CssSyntaxKind::CSS_AT_RULE)
        .and_then(|n| n.cast::<CssAtRule>())
        .and_then(|n| n.rule().ok())
        .is_some_and(|n| matches!(n, AnyCssAtRule::CssFontFaceAtRule(_)))
}

fn is_shorthand_font_property_with_keyword(properties: &CssGenericComponentValueList) -> bool {
    properties.into_iter().len() == 1
        && properties
            .into_iter()
            .any(|p| is_system_family_name_keyword(&p.to_trimmed_string()))
}

fn has_generic_font_family_property(nodes: &[AnyCssValue]) -> bool {
    nodes
        .iter()
        .any(|n| is_font_family_keyword(&n.to_trimmed_string()))
}

fn collect_font_family_properties(properties: CssGenericComponentValueList) -> Vec<AnyCssValue> {
    properties
        .into_iter()
        .filter_map(|v| match v {
            AnyCssGenericComponentValue::AnyCssValue(value) => Some(value),
            _ => None,
        })
        .collect()
}
