use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::CssDeclarationOrRuleBlock;
use biome_rowan::AstNode;

declare_rule! {
    /// Disallow unknown units.
    ///
    /// This rule considers units defined in the CSS Specifications, up to and including Editor's Drafts, to be known.
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   width: 10pixels;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   width: calc(10px + 10pixels);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a {
    ///   width: 10px;
    /// }
    /// ```
    ///
    /// ```css
    /// a {
    ///   width: 10Px;
    /// }
    /// ```
    ///
    /// ```css
    /// a {
    ///   width: 10pX;
    /// }
    /// ```
    ///
    /// ```css
    /// a {
    ///   width: calc(10px + 10px);
    /// }
    /// ```
    ///
    pub NoUnknownUnit {
        version: "next",
        name: "noUnknownUnit",
        recommended: false,
        sources: &[RuleSource::Stylelint("unit-no-unknown")],
    }
}

impl Rule for NoUnknownUnit {
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
