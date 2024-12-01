use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::stmt_ext::CssBlockLike;
use biome_diagnostics::Severity;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow CSS empty blocks.
    ///
    /// By default, it will allow empty blocks with comments inside.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// p {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// .b {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// @media print { a {} }
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
    /// ```css
    /// p { /* foo */ }
    /// ```
    ///
    /// ```css
    /// @media print { a { color: pink; } }
    /// ```
    ///
    pub NoEmptyBlock {
        version: "1.8.0",
        name: "noEmptyBlock",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("block-no-empty")],
    }
}

impl Rule for NoEmptyBlock {
    type Query = Ast<CssBlockLike>;
    type State = CssBlockLike;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        if node.is_empty_without_comments() {
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
                    "An empty block isn't allowed."
                },
            )
            .note(markup! {
                    "Consider removing the empty block or adding styles inside it."
            }),
        )
    }
}
