use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, JsUnaryOperator,
    OperatorPrecedence,
};
use biome_rowan::AstNode;
use biome_rule_options::no_chained_comparison::NoChainedComparisonOptions;

declare_lint_rule! {
    /// Disallow chaining comparisons, as in `a < b < c`.
    ///
    /// A comparison such as `a < b` always produces `true` or `false`. When you write
    /// `a < b < c`, JavaScript first compares `a` with `b`, and then compares that `true` or
    /// `false` with `c`. It never compares `b` with `c`. For example, `3 > 2 > 1` is `false`,
    /// because `3 > 2` is `true`, and `true > 1` is `false`.
    ///
    /// The same happens with equality checks: `a === b === c` compares `a` with `b`, and then
    /// compares the result with `c`. It doesn't check that all three values are equal.
    ///
    /// To compare each pair of values, join separate comparisons with `&&`, as in
    /// `a < b && b < c`.
    ///
    /// The rule reports two comparisons in a row that both use `<`, `>`, `<=`, or `>=`, or that
    /// both use `==`, `===`, `!=`, or `!==`. Parentheses don't change the result, so
    /// `(a === b) === c` is also reported.
    ///
    /// The rule ignores an equality check when the value on its right is also `true` or
    /// `false`: the `true` and `false` literals, another comparison, or an expression that
    /// starts with `!`. Checking whether two booleans are equal is usually intended, as in
    /// `(a === null) === (b === null)`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (a < b < c) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (a === b === c) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if ((a == b) != c) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (a < b && b < c) {}
    /// if (a === b && b === c) {}
    /// if (a < b === c) {}
    /// if ((a === null) === (b === null)) {}
    /// if ((a === b) === false) {}
    /// ```
    ///
    pub NoChainedComparison {
        version: "next",
        name: "noChainedComparison",
        language: "js",
        sources: &[RuleSource::Oxc("bad-comparison-sequence").inspired()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoChainedComparison {
    type Query = Ast<JsBinaryExpression>;
    /// The comparison on the left side of the query, whose result the query compares again.
    type State = JsBinaryExpression;
    type Signals = Option<Self::State>;
    type Options = NoChainedComparisonOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let inner = chained_comparison(node)?;

        // A longer chain such as `a < b < c < d` contains the chain `a < b < c` on its left
        // side. Only the outermost comparison is reported, so each chain gets one diagnostic.
        let outer = AnyJsExpression::JsBinaryExpression(node.clone()).outer_expression()?;
        if let Some(parent) = outer.syntax().parent().and_then(JsBinaryExpression::cast)
            && parent
                .left()
                .is_ok_and(|left| left.syntax() == outer.syntax())
            && chained_comparison(&parent).is_some()
        {
            return None;
        }

        Some(inner)
    }

    fn diagnostic(ctx: &RuleContext<Self>, inner: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let is_equality = node.operator().ok()?.precedence() == OperatorPrecedence::Equality;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This compares the result of another comparison with a value."
                },
            )
            .detail(
                inner.range(),
                markup! {
                    "This comparison runs first and produces "<Emphasis>"true"</Emphasis>" or "<Emphasis>"false"</Emphasis>"."
                },
            )
            .detail(
                node.right().ok()?.range(),
                markup! {
                    "This value is then compared with that "<Emphasis>"true"</Emphasis>" or "<Emphasis>"false"</Emphasis>", not with the values of the first comparison."
                },
            )
            .note(if is_equality {
                markup! {
                    "For example, "<Emphasis>"2 === 2 === 2"</Emphasis>" is "<Emphasis>"false"</Emphasis>", because "<Emphasis>"2 === 2"</Emphasis>" is "<Emphasis>"true"</Emphasis>", and "<Emphasis>"true === 2"</Emphasis>" is "<Emphasis>"false"</Emphasis>"."
                }
            } else {
                markup! {
                    "For example, "<Emphasis>"3 > 2 > 1"</Emphasis>" is "<Emphasis>"false"</Emphasis>", because "<Emphasis>"3 > 2"</Emphasis>" is "<Emphasis>"true"</Emphasis>", and "<Emphasis>"true > 1"</Emphasis>" is "<Emphasis>"false"</Emphasis>"."
                }
            })
            .note(if is_equality {
                markup! {
                    "To compare each pair of values, join separate comparisons with "<Emphasis>"&&"</Emphasis>", as in "<Emphasis>"a === b && b === c"</Emphasis>"."
                }
            } else {
                markup! {
                    "To compare each pair of values, join separate comparisons with "<Emphasis>"&&"</Emphasis>", as in "<Emphasis>"a < b && b < c"</Emphasis>"."
                }
            }),
        )
    }
}

/// Returns the comparison on the left side of `node` when `node` compares its result again
/// with an operator of the same kind, as in `a < b < c` or `(a === b) !== c`.
///
/// Returns `None` for an equality check whose right side is clearly `true` or `false`, since
/// checking whether two booleans are equal is usually intended.
fn chained_comparison(node: &JsBinaryExpression) -> Option<JsBinaryExpression> {
    let precedence = node.operator().ok()?.precedence();
    if !matches!(
        precedence,
        OperatorPrecedence::Relational | OperatorPrecedence::Equality
    ) {
        return None;
    }
    let AnyJsExpression::JsBinaryExpression(left) = node.left().ok()?.inner_expression()? else {
        return None;
    };
    if left.operator().ok()?.precedence() != precedence
        || (precedence == OperatorPrecedence::Equality && is_boolean(&node.right().ok()?))
    {
        return None;
    }
    Some(left)
}

/// Returns whether `expression` always evaluates to `true` or `false`, judging only by its
/// syntax.
fn is_boolean(expression: &AnyJsExpression) -> bool {
    match expression.inner_expression() {
        Some(
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsBooleanLiteralExpression(_),
            )
            | AnyJsExpression::JsInExpression(_)
            | AnyJsExpression::JsInstanceofExpression(_),
        ) => true,
        Some(AnyJsExpression::JsBinaryExpression(binary)) => binary.operator().is_ok_and(|op| {
            matches!(
                op.precedence(),
                OperatorPrecedence::Relational | OperatorPrecedence::Equality
            )
        }),
        Some(AnyJsExpression::JsUnaryExpression(unary)) => unary
            .operator()
            .is_ok_and(|op| op == JsUnaryOperator::LogicalNot),
        _ => false,
    }
}
