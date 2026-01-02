use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_syntax::{CssAtRule, CssFileSource};
use biome_diagnostics::Severity;
use biome_rowan::AstNode;
use biome_rule_options::no_value_at_rule::NoValueAtRuleOptions;

declare_lint_rule! {
    /// Disallow use of `@value` rule in CSS modules.
    ///
    /// Use of CSS variables is recommended instead of `@value` rule.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic,file=example.module.css
    /// @value red: #FF0000;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css,file=example.module.css
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
        severity: Severity::Information,
    }
}

impl Rule for NoValueAtRule {
    type Query = Ast<CssAtRule>;
    type State = CssAtRule;
    type Signals = Option<Self::State>;
    type Options = NoValueAtRuleOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let file_source = ctx.source_type::<CssFileSource>();

        if node.rule().ok()?.as_css_value_at_rule().is_some() && file_source.is_css_modules() {
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
