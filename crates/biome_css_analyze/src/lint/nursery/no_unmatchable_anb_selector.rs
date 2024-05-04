use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::{AnyCssPseudoClassNth, CssPseudoClassNthSelector};
use biome_rowan::AstNode;

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
    pub NoUnmatchableAnbSelector {
        version: "next",
        name: "noUnmatchableAnbSelector",
        recommended: true,
        sources: &[RuleSource::Stylelint("selector-anb-no-unmatchable")],
    }
}

impl Rule for NoUnmatchableAnbSelector {
    type Query = Ast<CssPseudoClassNthSelector>;
    type State = CssPseudoClassNthSelector;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let nth = node.nth().ok()?;
        if is_unmatchable(&nth) {
            for n in nth.syntax().ancestors() {
                dbg!(n);
            }
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

fn is_unmatchable(nth: &AnyCssPseudoClassNth) -> bool {
    match nth {
        AnyCssPseudoClassNth::CssPseudoClassNthIdentifier(_) => false,
        AnyCssPseudoClassNth::CssPseudoClassNth(nth) => {
            let coefficient = nth.value();
            let constant = nth.offset();
            match (coefficient, constant) {
                (Some(coeff), Some(cons)) => coeff.text() == "0" && cons.text() == "0",
                (Some(coeff), None) => coeff.text() == "0",
                _ => false,
            }
        }
        AnyCssPseudoClassNth::CssPseudoClassNthNumber(nth) => nth.text() == "0",
    }
}
