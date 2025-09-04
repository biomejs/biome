use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
#[expect(unused_imports)]
use biome_js_syntax::{
    AnyJsExpression, JsAssignmentExpression, JsBinaryExpression, JsCallArgumentList,
    JsCallArguments, JsCallExpression, JsConditionalExpression, JsLogicalExpression,
    JsNewExpression, JsParenthesizedExpression, JsSequenceExpression, JsSyntaxNode,
    JsUnaryExpression, JsUnaryOperator, T, is_in_boolean_context, is_negation,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use biome_rule_options::no_extra_boolean_cast::NoExtraBooleanCastOptions;

use crate::JsRuleAction;

pub enum ExtraBooleanCastType {
    /// !!x
    DoubleNegation,
    /// Boolean(x)
    BooleanCall,
}
declare_lint_rule! {
    /// Disallow unnecessary boolean casts
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (!Boolean(foo)) {
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// while (!!foo) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let x = 1;
    /// do {
    /// 1 + 1;
    /// } while (Boolean(x));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (; !!foo; ) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// new Boolean(!!x);
    /// ```
    ///
    /// ### Valid
    /// ```js
    /// Boolean(!x);
    /// !x;
    /// !!x;
    /// ```
    pub NoExtraBooleanCast {
        version: "1.0.0",
        name: "noExtraBooleanCast",
        language: "js",
        sources: &[RuleSource::Eslint("no-extra-boolean-cast").same()],
        recommended: true,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

/// Checks if the node is a `Boolean` Constructor Call
/// # Example
/// ```js
/// new Boolean(x);
/// ```
pub fn is_boolean_constructor_call(node: &JsSyntaxNode) -> Option<JsNewExpression> {
    let expr = JsCallArgumentList::cast_ref(node)?
        .parent::<JsCallArguments>()?
        .parent::<JsNewExpression>()?;

    expr.has_callee("Boolean").then_some(expr)
}

/// Check if the SyntaxNode is a `Boolean` Call Expression
/// ## Example
/// ```js
/// Boolean(x)
/// ```
fn is_boolean_call(node: &JsSyntaxNode) -> Option<bool> {
    let expr = JsCallExpression::cast_ref(node)?;
    Some(expr.has_callee("Boolean"))
}

impl Rule for NoExtraBooleanCast {
    type Query = Ast<AnyJsExpression>;
    type State = (AnyJsExpression, ExtraBooleanCastType);
    type Signals = Option<Self::State>;
    type Options = NoExtraBooleanCastOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let n = ctx.query();
        let parent = n.syntax().parent()?;

        // Check if parent `SyntaxNode` in any boolean `Type Coercion` context,
        // reference https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Boolean
        let parent_node_in_boolean_cast_context = is_in_boolean_context(n.syntax())
            .unwrap_or(false)
            || is_boolean_constructor_call(&parent).is_some()
            || is_negation(&parent).is_some()
            || is_boolean_call(&parent).unwrap_or(false);
        // Convert `!!x` -> `x` if parent `SyntaxNode` in any boolean `Type Coercion` context
        if parent_node_in_boolean_cast_context {
            if let Some(result) = is_double_negation_ignore_parenthesis(n.syntax()) {
                return Some(result);
            };

            // Convert `Boolean(x)` -> `x` if parent `SyntaxNode` in any boolean `Type Coercion` context
            // Only if `Boolean` Call Expression have one `JsAnyExpression` argument
            if let Some(expr) = JsCallExpression::cast_ref(n.syntax()) {
                if expr.has_callee("Boolean") {
                    let arguments = expr.arguments().ok()?;
                    let len = arguments.args().len();
                    if len == 1 {
                        return arguments
                            .args()
                            .into_iter()
                            .next()?
                            .ok()
                            .map(|item| item.into_syntax())
                            .and_then(AnyJsExpression::cast)
                            .map(|expr| (expr, ExtraBooleanCastType::BooleanCall));
                    }
                }
                return None;
            }

            // Convert `new Boolean(x)` -> `x` if parent `SyntaxNode` in any boolean `Type Coercion` context
            // Only if `Boolean` Call Expression have one `JsAnyExpression` argument
            return JsNewExpression::cast_ref(n.syntax()).and_then(|expr| {
                if expr.has_callee("Boolean") {
                    let arguments = expr.arguments()?;
                    let len = arguments.args().len();
                    if len == 1 {
                        return arguments
                            .args()
                            .into_iter()
                            .next()?
                            .ok()
                            .map(|item| item.into_syntax())
                            .and_then(AnyJsExpression::cast)
                            .map(|expr| (expr, ExtraBooleanCastType::BooleanCall));
                    }
                }
                None
            });
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let (_, extra_boolean_cast_type) = state;
        let (title, note) = match extra_boolean_cast_type {
            ExtraBooleanCastType::DoubleNegation => (
                "Avoid redundant double-negation.",
                "It is not necessary to use double-negation when a value will already be coerced to a boolean.",
            ),
            ExtraBooleanCastType::BooleanCall => (
                "Avoid redundant `Boolean` call",
                "It is not necessary to use `Boolean` call when a value will already be coerced to a boolean.",
            ),
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    {title}
                },
            )
            .note(note),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let (node_to_replace, extra_boolean_cast_type) = state;
        let message = match extra_boolean_cast_type {
            ExtraBooleanCastType::DoubleNegation => "Remove redundant double-negation",
            ExtraBooleanCastType::BooleanCall => "Remove redundant `Boolean` call",
        };

        // Check if the Boolean call is inside a unary negation and the argument needs parentheses
        let mut replacement = node_to_replace.clone();

        // Only wrap in parentheses if this is a Boolean call inside a logical NOT with complex expression
        if matches!(extra_boolean_cast_type, ExtraBooleanCastType::BooleanCall) {
            let is_negated_boolean_call = node
                .syntax()
                .parent()
                .and_then(JsUnaryExpression::cast)
                .and_then(|expr| expr.operator().ok())
                .is_some_and(|op| op == JsUnaryOperator::LogicalNot);

            if is_negated_boolean_call && needs_parentheses_when_negated(node_to_replace) {
                replacement =
                    AnyJsExpression::JsParenthesizedExpression(make::js_parenthesized_expression(
                        make::token(T!['(']),
                        replacement,
                        make::token(T![')']),
                    ));
            }
        }

        mutation.replace_node(node.clone(), replacement);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { {message} }.to_owned(),
            mutation,
        ))
    }
}

/// Determines if an expression needs parentheses when it becomes the operand of a unary negation.
/// This is needed to preserve operator precedence for expressions like binary expressions.
fn needs_parentheses_when_negated(expr: &AnyJsExpression) -> bool {
    match expr {
        // Binary expressions like `a + b` need parentheses in `!(a + b)` to maintain precedence
        AnyJsExpression::JsBinaryExpression(_) => true,
        // Logical expressions like `a && b` need parentheses in `!(a && b)` to maintain precedence
        AnyJsExpression::JsLogicalExpression(_) => true,
        // Conditional expressions like `a ? b : c` need parentheses
        AnyJsExpression::JsConditionalExpression(_) => true,
        // Assignment expressions need parentheses
        AnyJsExpression::JsAssignmentExpression(_) => true,
        // Sequence expressions (comma operator) need parentheses
        AnyJsExpression::JsSequenceExpression(_) => true,
        // Logical expressions that are already parenthesized don't need additional ones
        AnyJsExpression::JsParenthesizedExpression(_) => false,
        // Simple expressions like identifiers, literals, calls don't need parentheses
        _ => false,
    }
}

/// Check if the SyntaxNode is a Double Negation. Including the edge case
/// ```js
/// !(!x)
/// ```
/// Return [Rule::State] `(JsAnyExpression, ExtraBooleanCastType)` if it is a DoubleNegation Expression
///
fn is_double_negation_ignore_parenthesis(
    syntax: &biome_rowan::SyntaxNode<biome_js_syntax::JsLanguage>,
) -> Option<(AnyJsExpression, ExtraBooleanCastType)> {
    if let Some(negation_expr) = is_negation(syntax) {
        let argument = negation_expr.argument().ok()?;
        match argument {
            AnyJsExpression::JsUnaryExpression(expr)
                if expr.operator().ok()? == JsUnaryOperator::LogicalNot =>
            {
                expr.argument()
                    .ok()
                    .map(|argument| (argument, ExtraBooleanCastType::DoubleNegation))
            }
            // Check edge case `!(!xxx)`
            AnyJsExpression::JsParenthesizedExpression(expr) => {
                expr.expression().ok().and_then(|expr| {
                    is_negation(expr.syntax()).and_then(|negation| {
                        Some((
                            negation.argument().ok()?,
                            ExtraBooleanCastType::DoubleNegation,
                        ))
                    })
                })
            }
            _ => None,
        }
    } else {
        None
    }
}
