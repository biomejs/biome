use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_css_syntax::CssAtRule;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow use of `@value` rule in css modules.
    ///
    /// Use of CSS variables is recommended instead of `@value` rule.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// @value red: #FF0000;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// :root {
    ///   --red: #FF0000
    /// }
    ///
    /// p {
    ///   background-color: var(--red);
    /// }
    /// ```
    ///
    pub NoValueAtRule {
        version: "1.8.0",
        name: "noValueAtRule",
        language: "css",
        recommended: false,
    }
}

impl Rule for NoValueAtRule {
    type Query = Ast<CssAtRule>;
    type State = CssAtRule;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        if node.rule().ok()?.as_css_value_at_rule().is_some() {
            return Some(node.clone());
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let span = node.range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Use of "<Emphasis>"@value"</Emphasis>" rule is disallowed"
                },
            )
            .note(markup! {
                "Using @value is not recommended, consider using CSS variables instead."
            }).note(markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/CSS/Using_CSS_custom_properties">"MDN web docs"</Hyperlink>" for more details."
            }),
        )
    }
}
