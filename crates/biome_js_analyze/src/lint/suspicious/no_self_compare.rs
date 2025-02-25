use crate::utils::is_node_equal;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_diagnostics::Severity;
use biome_js_syntax::JsBinaryExpression;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow comparisons where both sides are exactly the same.
    ///
    /// > Comparing a variable against itself is usually an error, either a typo or refactoring error. It is confusing to the reader and may potentially introduce a runtime error.
    ///
    /// > The only time you would compare a variable against itself is when you are testing for `NaN`.
    /// However, it is far more appropriate to use `typeof x === 'number' && Number.isNaN(x)` for that use case rather than leaving the reader of the code to determine the intent of self comparison.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (x === x) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (a.b.c() !== a.b .c()) {}
    /// ```
    ///
    pub NoSelfCompare {
        version: "1.0.0",
        name: "noSelfCompare",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-self-compare"),
            RuleSource::Clippy("eq_op"),
        ],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoSelfCompare {
    type Query = Ast<JsBinaryExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if !node.is_comparison_operator() {
            return None;
        }
        let left = node.left().ok()?;
        let right = node.right().ok()?;
        is_node_equal(left.syntax(), right.syntax()).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            "Comparing to itself is potentially pointless.",
        ))
    }
}
