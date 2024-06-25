use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::CssDeclarationOrRuleBlock;
use biome_rowan::AstNode;

declare_rule! {
    /// Disallow CSS empty comments.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding ESLint rule (if any):
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// /**/
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// /* */
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// /*
    ///
    /// */
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// // comment
    /// ```
    ///
    /// ```css
    /// /*
    ///  * Multi-line Comment
    /// **/
    /// ```
    ///
    /// ```css
    /// /* comment */
    /// ```
    pub NoEmptyComment {
        version: "next",
        name: "noEmptyComment",
        language: "css",
        recommended: false,
        sources: &[RuleSource::Stylelint("comment-no-empty")],
    }
}

impl Rule for NoEmptyComment {
    type Query = Ast<CssDeclarationOrRuleBlock>;
    type State = CssDeclarationOrRuleBlock;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        if node.items().into_iter().next().is_none() {
            return Some(node.clone());
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        let span = node.range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Unexpected empty block is not allowed"
                },
            )
            .note(markup! {
                    "This note will give you more information."
            }),
        )
    }
}
