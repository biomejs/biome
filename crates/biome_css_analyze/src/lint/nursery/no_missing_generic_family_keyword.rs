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
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding stylelint rule (if any):
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// p {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// p {
    ///   color: red;
    /// }
    /// ```
    ///
    pub NoMissingGenericFamilyKeyword {
        version: "next",
        name: "noMissingGenericFamilyKeyword",
        recommended: false,
    }
}

pub struct RuleState {
    pub value: String,
    pub span: TextRange,
}

impl Rule for NoMissingGenericFamilyKeyword {
    type Query = Ast<CssGenericProperty>;
    type State = RuleState;
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

        Some(RuleState {
            value: last_value.text(),
            span: last_value.range(),
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let span = state.span;
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
