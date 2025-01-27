use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::{CssUnknownBlockAtRule, CssUnknownValueAtRule};
use biome_rowan::{declare_node_union, AstNode, TextRange};

declare_lint_rule! {
    /// Disallow unknown at-rules.
    ///
    /// For details on known at-rules, see the [MDN web docs](https://developer.mozilla.org/en-US/docs/Web/CSS/At-rule).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// @uNkNoWn {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// @unknown-at-rule {
    ///   font-size: 14px;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// @charset 'UTF-8';
    /// ```
    ///
    /// ```css
    /// @media (max-width: 960px) {
    ///   body {
    ///     font-size: 13px;
    ///   }
    /// }
    /// ```
    pub NoUnknownAtRule {
        version: "next",
        name: "noUnknownAtRule",
        language: "css",
        recommended: true,
        sources: &[RuleSource::Stylelint("at-rule-no-unknown")],
    }
}

declare_node_union! {
  pub AnyUnknownAtRule = CssUnknownBlockAtRule | CssUnknownValueAtRule
}

pub struct NoUnknownAtRuleState {
    range: TextRange,
    name: String,
}

impl Rule for NoUnknownAtRule {
    type Query = Ast<AnyUnknownAtRule>;
    type State = NoUnknownAtRuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let rule = match node {
            AnyUnknownAtRule::CssUnknownBlockAtRule(rule) => rule.name().ok()?,
            AnyUnknownAtRule::CssUnknownValueAtRule(rule) => rule.name().ok()?,
        };
        Some(NoUnknownAtRuleState {
            range: rule.range(),
            name: rule.value_token().ok()?.text_trimmed().to_string(),
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let span = state.range;
        let name = &state.name;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Unexpected unknown at-rule: "<Emphasis>{ name }</Emphasis>" "
                },
            )
            .note(markup! {
                    ""<Emphasis>{ name }</Emphasis>" is not a standard CSS at-rule, which may lead to unexpected styling results or failure to interpret the styles as intended."
            })
            .note(markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/CSS/At-rule">"MDN web docs"</Hyperlink>" for a known list of at-rules."
            }).note(markup! {
            "To fix this issue, consider removing the unknown at-rule."
        })
        )
    }
}
