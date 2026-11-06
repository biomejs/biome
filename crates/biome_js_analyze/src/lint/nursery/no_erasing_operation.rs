use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, JsBinaryOperator,
};
use biome_rowan::AstNode;
use biome_rule_options::no_erasing_operation::NoErasingOperationOptions;

declare_lint_rule! {
    /// Disallow operations with zero that always result in zero.
    ///
    /// Multiplying a number by `0`, combining it with `0` using the bitwise AND operator (`&`),
    /// or dividing `0` by a number gives `0` no matter what the other number is. The other
    /// number has no effect on the result, so code like `x * 0` is most likely a mistake,
    /// such as a leftover from debugging or a typo. If the result should be zero, write `0`
    /// directly.
    ///
    /// A few special values produce a different result. For example, `NaN * 0` and
    /// `Infinity * 0` are both `NaN`, and `-5 * 0` is `-0`.
    ///
    /// The rule only reports a `0` that is written as a number, such as `0` or `0.0`. It
    /// doesn't report variables that contain zero, `0n` (a `BigInt` zero), or `0 / 0`,
    /// which results in `NaN` instead of zero.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const total = price * 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const flags = options & 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const ratio = 0 / count;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const total = price * quantity;
    /// const flags = options | 0;
    /// const ratio = count / 0;
    /// const zero = 0;
    /// ```
    ///
    pub NoErasingOperation {
        version: "next",
        name: "noErasingOperation",
        language: "js",
        sources: &[
            RuleSource::Oxc("erasing-op").same(),
            RuleSource::Clippy("erasing_op").same(),
        ],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoErasingOperation {
    type Query = Ast<JsBinaryExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoErasingOperationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let left = node.left().ok()?;
        let right = node.right().ok()?;
        // Sequence expressions aren't looked through for the zero operand: in
        // `x * (foo(), 0)`, replacing the operation with `0` would drop the call to `foo()`.
        let is_erasing = match node.operator().ok()? {
            JsBinaryOperator::Times | JsBinaryOperator::BitwiseAnd => {
                is_number_zero(left.omit_type_wrappers())
                    || is_number_zero(right.omit_type_wrappers())
            }
            // `0 / 0` is `NaN`, not zero, and so is `0 / (foo(), 0)`.
            JsBinaryOperator::Divide => {
                is_number_zero(left.omit_type_wrappers())
                    && !is_number_zero(right.inner_expression())
            }
            _ => false,
        };
        is_erasing.then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This operation always results in zero."
                },
            )
            .note(match node.operator().ok()? {
                JsBinaryOperator::Times => markup! {
                    "Multiplying by zero gives zero, so the other value has no effect on the result."
                },
                JsBinaryOperator::BitwiseAnd => markup! {
                    "A bitwise AND with zero gives zero, so the other value has no effect on the result."
                },
                _ => markup! {
                    "Dividing zero by a number gives zero, so the other value has no effect on the result."
                },
            })
            .note(markup! {
                "This is likely a mistake. If the result should be zero, replace the operation with "<Emphasis>"0"</Emphasis>"."
            }),
        )
    }
}

/// Returns whether `expression` is a number literal equal to zero, such as `0`, `0.0`, or
/// `0x0`. `BigInt` literals such as `0n` don't count.
fn is_number_zero(expression: Option<AnyJsExpression>) -> bool {
    matches!(
        expression,
        Some(AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNumberLiteralExpression(literal)
        )) if literal.as_number() == Some(0.0)
    )
}
