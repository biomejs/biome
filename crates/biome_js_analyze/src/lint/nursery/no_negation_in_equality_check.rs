use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsBinaryExpression, JsBinaryOperator, JsUnaryOperator};
use biome_rowan::AstNode;
use biome_rule_options::no_negation_in_equality_check::NoNegationInEqualityCheckOptions;

declare_lint_rule! {
    /// Disallow negated operands on the left of an equality check.
    ///
    /// A `!` binds tighter than an equality operator, so `!foo === bar` is
    /// evaluated as `(!foo) === bar` — comparing a boolean against `bar`. This
    /// is almost always a mistake for `foo !== bar` (or, less commonly, the
    /// grouped `!(foo === bar)`).
    ///
    /// The rule flags a strict equality (`===`) or strict inequality (`!==`)
    /// expression whose left operand is a logical negation.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (!foo === bar) {
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (!foo !== bar) {
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (foo !== bar) {
    /// }
    /// ```
    ///
    /// ```js
    /// if (!(foo === bar)) {
    /// }
    /// ```
    ///
    pub NoNegationInEqualityCheck {
        version: "next",
        name: "noNegationInEqualityCheck",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("no-negation-in-equality-check").same()],
    }
}

impl Rule for NoNegationInEqualityCheck {
    type Query = Ast<JsBinaryExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoNegationInEqualityCheckOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binary = ctx.query();

        // Only strict equality / inequality checks are in scope, matching the
        // source rule.
        if !matches!(
            binary.operator().ok()?,
            JsBinaryOperator::StrictEquality | JsBinaryOperator::StrictInequality
        ) {
            return None;
        }

        // The left operand must be a logical negation (`!expr`).
        let left = binary.left().ok()?;
        let unary = left.as_js_unary_expression()?;
        if unary.operator().ok()? != JsUnaryOperator::LogicalNot {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let binary = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                binary.range(),
                markup! {
                    "This negation applies only to the left operand, not to the equality check."
                },
            )
            .note(markup! {
                ""<Emphasis>"!"</Emphasis>" binds tighter than the equality operator, so "<Emphasis>"!a === b"</Emphasis>" is evaluated as "<Emphasis>"(!a) === b"</Emphasis>" — it compares the boolean "<Emphasis>"!a"</Emphasis>" against "<Emphasis>"b"</Emphasis>", which is rarely what's intended."
            })
            .note(markup! {
                "To negate the comparison, use "<Emphasis>"a !== b"</Emphasis>". If comparing the negated value is intentional, make it explicit with parentheses: "<Emphasis>"(!a) === b"</Emphasis>"."
            }),
        )
    }
}
