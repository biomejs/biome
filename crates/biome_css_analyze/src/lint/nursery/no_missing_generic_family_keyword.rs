use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssAtRule, AnyCssGenericComponentValue, CssAtRule, CssGenericProperty, CssSyntaxKind,
};
use biome_rowan::{AstNode, SyntaxNodeCast, TextRange};

use crate::utils::{
    find_font_family, is_css_variable, is_font_family_keyword, is_system_family_name_keyword,
};

declare_rule! {
    /// Disallow a missing generic family keyword within font families.
    ///
    /// The generic font family can be:
    /// - placed anywhere in the font family list
    /// - omitted if a keyword related to property inheritance or a system font is used
    ///
    /// This rule checks the font and font-family properties.
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
    /// a { font: caption; }
    /// ```
    ///
    /// ```css
    /// a { font-family: revert }
    /// ```
    ///
    pub NoMissingGenericFamilyKeyword {
        version: "next",
        name: "noMissingGenericFamilyKeyword",
        recommended: false,
    }
}

impl Rule for NoMissingGenericFamilyKeyword {
    type Query = Ast<CssGenericProperty>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let property_name = node.name().ok()?.text().to_lowercase();

        // Ignore `@font-face`. See more detail: https://drafts.csswg.org/css-fonts/#font-face-rule
        if node
            .syntax()
            .ancestors()
            .find(|n| n.kind() == CssSyntaxKind::CSS_AT_RULE)
            .and_then(|n| n.cast::<CssAtRule>())
            .and_then(|n| n.rule().ok())
            .map(|n| matches!(n, AnyCssAtRule::CssFontFaceAtRule(_)))
            .is_some()
        {
            return None;
        }

        let is_font_family = property_name == "font-family";
        let is_font = property_name == "font";

        if !is_font_family && !is_font {
            return None;
        }

        let value_list = node.value();
        let mut value_list_iter = value_list.clone().into_iter();
        let len = value_list_iter.len();
        if is_font && len == 1 && value_list_iter.any(|v| is_system_family_name_keyword(&v.text()))
        {
            return None;
        }

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

        if font_families.is_empty() {
            return None;
        } else {
            for f in font_families.iter() {
                if is_system_family_name_keyword(&f.text()) || is_font_family_keyword(&f.text()) {
                    return None;
                }
            }
        }

        let last_value = font_families.last()?;
        if is_css_variable(&last_value.text()) {
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
                    "Missing generic family keyword in declaration."
                },
            )
            .note(markup! {
                    "This note will give you more information."
            }),
        )
    }
}
