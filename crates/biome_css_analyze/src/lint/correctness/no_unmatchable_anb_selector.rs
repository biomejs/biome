use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssPseudoClassNth, CssPseudoClassFunctionSelectorList, CssPseudoClassNthSelector,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, SyntaxNodeCast};
use biome_rule_options::no_unmatchable_anb_selector::NoUnmatchableAnbSelectorOptions;

declare_lint_rule! {
    /// Disallow unmatchable An+B selectors.
    ///
    /// Selectors that always evaluate to 0 will not match any elements.
    /// For more details about the An+B syntax, see:
    /// https://www.w3.org/TR/css-syntax-3/#anb-microsyntax
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a:nth-child(0) {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a:nth-last-child(0n) {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a:nth-of-type(0n+0) {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a:nth-last-of-type(0 of a) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a:nth-child(1) {}
    /// ```
    ///
    /// ```css
    /// a:nth-last-child(1n) {}
    /// ```
    ///
    /// ```css
    /// a:nth-of-type(1n+0) {}
    /// ```
    ///
    /// ```css
    /// a:nth-last-of-type(1 of a) {}
    /// ```
    ///
    pub NoUnmatchableAnbSelector {
        version: "1.8.0",
        name: "noUnmatchableAnbSelector",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("selector-anb-no-unmatchable").same()],
    }
}

impl Rule for NoUnmatchableAnbSelector {
    type Query = Ast<CssPseudoClassNthSelector>;
    type State = CssPseudoClassNthSelector;
    type Signals = Option<Self::State>;
    type Options = NoUnmatchableAnbSelectorOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let nth = node.nth().ok()?;
        if is_unmatchable(&nth) && !is_within_not_pseudo_class(&nth) {
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
                    "This selector will never match any elements."
                },
            )
            .note(markup! {
                    "Avoid using An+B selectors that always evaluate to 0."
            }).note(markup! {
                "For more details, see "<Hyperlink href="https://www.w3.org/TR/css-syntax-3/#anb-microsyntax">"the official spec for An+B selectors"</Hyperlink>"."
            })
        )
    }
}

fn is_unmatchable(nth: &AnyCssPseudoClassNth) -> bool {
    match nth {
        AnyCssPseudoClassNth::CssPseudoClassNthIdentifier(_) => false,
        AnyCssPseudoClassNth::CssPseudoClassNth(nth) => {
            let coefficient = nth.value().and_then(|n| n.value_token().ok());
            let constant = nth
                .offset()
                .and_then(|offset| offset.value().ok())
                .and_then(|n| n.value_token().ok());

            match (coefficient, constant) {
                (Some(a), Some(b)) => a.text_trimmed() == "0" && b.text_trimmed() == "0",
                (Some(a), None) => a.text_trimmed() == "0",
                _ => false,
            }
        }
        AnyCssPseudoClassNth::CssPseudoClassNthNumber(nth) => nth
            .value()
            .ok()
            .and_then(|n| n.value_token().ok())
            .is_some_and(|n| n.text_trimmed() == "0"),
    }
}

// Check if the nth selector is effective within a `not` pseudo class
// Example: a:not(:nth-child(0)) returns true
//          a:not(:not(:nth-child(0))) returns false
fn is_within_not_pseudo_class(node: &AnyCssPseudoClassNth) -> bool {
    let number_of_not = node
        .syntax()
        .ancestors()
        .filter_map(|n| n.cast::<CssPseudoClassFunctionSelectorList>())
        .filter_map(|n| n.name().ok())
        .filter_map(|n| n.value_token().ok())
        .filter(|n| n.text_trimmed() == "not")
        .count();
    number_of_not % 2 == 1
}
