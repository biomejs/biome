use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_css_syntax::CssDeclarationOrRuleBlock;
use biome_rowan::AstNode;

declare_rule! {
    /// **[WIP] This rule hasn't been implemented yet.**
    ///
    /// This is sample rule
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
    pub NoColorInvalidHex {
        version: "next",
        name: "noColorInvalidHex",
        recommended: false,
    }
}

impl Rule for NoColorInvalidHex {
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
