use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_css_syntax::{CssUnknownBlockAtRule, CssUnknownValueAtRule};
use biome_rowan::{declare_node_union, AstNode, TextRange};

declare_lint_rule! {
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
    pub NoUnknownAtRule {
        version: "next",
        name: "noUnknownAtRule",
        language: "css",
        recommended: false,
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
            name: rule.text().to_string(),
        })
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let span = node.range;
        let name = &node.name;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Unexpected unknown at-rule "<Emphasis>{ name }</Emphasis>" "
                },
            )
            .note(markup! {
                    ""<Emphasis>{ name }</Emphasis>" is not a standard CSS at-rule, which may lead to unexpected styling results or failure to interpret the styles as intended."
            }).note(markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/CSS/At-rule">"MDN web docs"</Hyperlink>" for more details."
            }),
        )
    }
}
