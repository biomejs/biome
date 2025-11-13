use crate::JsRuleAction;
use crate::utils::is_node_equal;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, JsBinaryOperator, JsConditionalExpression, JsLogicalOperator,
    static_value::StaticValue,
};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Enforce using nullish coalescing operator (`??`) instead of ternary expressions
    /// that check for null or undefined.
    ///
    /// Ternary expressions that check if a value is null or undefined can be simplified
    /// using the nullish coalescing operator (`??`).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const value = x !== null ? x : y;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const value = x != null ? x : y;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const value = x ?? y;
    /// ```
    pub UseNullishCoalescingInTernary {
        version: "next",
        name: "useNullishCoalescingInTernary",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("prefer-nullish-coalescing").same()],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

pub struct RuleState {
    checked_expr: AnyJsExpression,
    fallback: AnyJsExpression,
}

impl Rule for UseNullishCoalescingInTernary {
    type Query = biome_analyze::Ast<JsConditionalExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let ternary = ctx.query();

        let (checked_expr, fallback) = check_ternary_pattern(ternary)?;

        Some(RuleState {
            checked_expr,
            fallback,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.test().ok()?.syntax().text_trimmed_range(),
                markup! {
                    "Use "<Emphasis>"??"</Emphasis>" instead of ternary expression with nullish check."
                },
            )
            .note(markup! {
                "Ternary expressions checking for null or undefined can be simplified using ??."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        use biome_js_factory::make;
        use biome_js_syntax::T;
        use biome_rowan::BatchMutationExt;

        let ternary = ctx.query();
        let mut mutation = ctx.root().begin();

        // Create new ?? expression
        let new_expr = make::js_logical_expression(
            state.checked_expr.clone(),
            make::token_decorated_with_space(T![??]),
            state.fallback.clone(),
        );

        // Preserve trivia from ternary
        let new_expr = AnyJsExpression::from(new_expr);
        let new_expr = if let Some(leading) = ternary.syntax().first_leading_trivia() {
            new_expr.prepend_trivia_pieces(leading.pieces())?
        } else {
            new_expr
        };
        let new_expr = if let Some(trailing) = ternary.syntax().last_trailing_trivia() {
            new_expr.append_trivia_pieces(trailing.pieces())?
        } else {
            new_expr
        };

        // Replace entire ternary with new expression
        mutation.replace_node_discard_trivia(AnyJsExpression::from(ternary.clone()), new_expr);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"??"</Emphasis>" instead of ternary." }.to_owned(),
            mutation,
        ))
    }
}

/// Checks if a ternary expression can be converted to ??
/// Returns (checked_expression, fallback_expression) if pattern matches
fn check_ternary_pattern(
    ternary: &JsConditionalExpression,
) -> Option<(AnyJsExpression, AnyJsExpression)> {
    let test = ternary.test().ok()?;
    let consequent = ternary.consequent().ok()?;
    let alternate = ternary.alternate().ok()?;

    // Check for: x !== null && x !== undefined ? x : y
    if let Some(checked) = is_positive_nullish_check(&test, &consequent) {
        return Some((checked, alternate));
    }

    // Check for: x === null || x === undefined ? y : x
    if let Some(checked) = is_negative_nullish_check(&test, &alternate) {
        return Some((checked, consequent));
    }

    None
}

/// Checks for patterns like `x !== null ? x : y`
///
/// Returns the checked expression if the pattern matches:
/// - Pattern 1: `x != null ? x : y` (loose equality check)
/// - Pattern 2: `x !== null ? x : y` (single strict check)
/// - Pattern 3: `x !== null && x !== undefined ? x : y` (double strict check)
fn is_positive_nullish_check(
    test: &AnyJsExpression,
    value: &AnyJsExpression,
) -> Option<AnyJsExpression> {
    // Pattern 1: x != null ? x : y (loose equality)
    if let Some(checked) = is_loose_not_null_check(test)
        && expressions_equivalent(&checked, value)
    {
        return Some(checked);
    }

    // Pattern 2: x !== null ? x : y (single strict check)
    if let Some(checked) = is_strict_not_nullish_check(test)
        && expressions_equivalent(&checked, value)
    {
        return Some(checked);
    }

    // Pattern 3: x !== null && x !== undefined ? x : y (double strict check)
    if let Some(logical) = test.as_js_logical_expression()
        && logical.operator().ok()? == JsLogicalOperator::LogicalAnd
    {
        let left = logical.left().ok()?;
        let right = logical.right().ok()?;

        if let (Some(checked_left), Some(checked_right)) = (
            is_strict_not_nullish_check(&left),
            is_strict_not_nullish_check(&right),
        ) && expressions_equivalent(&checked_left, &checked_right)
            && expressions_equivalent(&checked_left, value)
        {
            return Some(checked_left);
        }
    }

    None
}

/// Checks for patterns like `x === null ? y : x`
///
/// Returns the checked expression if the pattern matches:
/// - Pattern 1: `x == null ? y : x` (loose equality check)
/// - Pattern 2: `x === null ? y : x` (single strict check)
/// - Pattern 3: `x === null || x === undefined ? y : x` (double strict check)
fn is_negative_nullish_check(
    test: &AnyJsExpression,
    value: &AnyJsExpression,
) -> Option<AnyJsExpression> {
    // Pattern 1: x == null ? y : x (loose equality)
    if let Some(checked) = is_loose_null_check(test)
        && expressions_equivalent(&checked, value)
    {
        return Some(checked);
    }

    // Pattern 2: x === null ? y : x (single strict check)
    if let Some(checked) = is_strict_nullish_check(test)
        && expressions_equivalent(&checked, value)
    {
        return Some(checked);
    }

    // Pattern 3: x === null || x === undefined ? y : x (double strict check)
    if let Some(logical) = test.as_js_logical_expression()
        && logical.operator().ok()? == JsLogicalOperator::LogicalOr
    {
        let left = logical.left().ok()?;
        let right = logical.right().ok()?;

        if let (Some(checked_left), Some(checked_right)) = (
            is_strict_nullish_check(&left),
            is_strict_nullish_check(&right),
        ) && expressions_equivalent(&checked_left, &checked_right)
            && expressions_equivalent(&checked_left, value)
        {
            return Some(checked_left);
        }
    }

    None
}

/// Checks for `x !== null` or `x !== undefined`
///
/// Returns the checked expression if the binary expression uses strict inequality
/// to compare against null or undefined.
fn is_strict_not_nullish_check(expr: &AnyJsExpression) -> Option<AnyJsExpression> {
    let binary = expr.as_js_binary_expression()?;
    let op = binary.operator().ok()?;

    if op != JsBinaryOperator::StrictInequality {
        return None;
    }

    let left = binary.left().ok()?;
    let right = binary.right().ok()?;

    if is_null_or_undefined(&right) {
        return Some(left);
    }
    if is_null_or_undefined(&left) {
        return Some(right);
    }

    None
}

/// Checks for `x === null` or `x === undefined`
///
/// Returns the checked expression if the binary expression uses strict equality
/// to compare against null or undefined.
fn is_strict_nullish_check(expr: &AnyJsExpression) -> Option<AnyJsExpression> {
    let binary = expr.as_js_binary_expression()?;
    let op = binary.operator().ok()?;

    if op != JsBinaryOperator::StrictEquality {
        return None;
    }

    let left = binary.left().ok()?;
    let right = binary.right().ok()?;

    if is_null_or_undefined(&right) {
        return Some(left);
    }
    if is_null_or_undefined(&left) {
        return Some(right);
    }

    None
}

/// Checks for `x != null` (loose inequality - checks both null and undefined)
///
/// In JavaScript, `x != null` is equivalent to `x !== null && x !== undefined`.
fn is_loose_not_null_check(expr: &AnyJsExpression) -> Option<AnyJsExpression> {
    let binary = expr.as_js_binary_expression()?;
    let op = binary.operator().ok()?;

    if op != JsBinaryOperator::Inequality {
        return None;
    }

    let left = binary.left().ok()?;
    let right = binary.right().ok()?;

    // In JavaScript, x != null checks for both null and undefined
    if is_null(&right) {
        return Some(left);
    }
    if is_null(&left) {
        return Some(right);
    }

    None
}

/// Checks for `x == null` (loose equality - checks both null and undefined)
///
/// In JavaScript, `x == null` is equivalent to `x === null || x === undefined`.
fn is_loose_null_check(expr: &AnyJsExpression) -> Option<AnyJsExpression> {
    let binary = expr.as_js_binary_expression()?;
    let op = binary.operator().ok()?;

    if op != JsBinaryOperator::Equality {
        return None;
    }

    let left = binary.left().ok()?;
    let right = binary.right().ok()?;

    if is_null(&right) {
        return Some(left);
    }
    if is_null(&left) {
        return Some(right);
    }

    None
}

/// Checks if an expression is a `null` or `undefined` literal
fn is_null_or_undefined(expr: &AnyJsExpression) -> bool {
    expr.as_static_value()
        .is_some_and(|v| v.is_null_or_undefined())
}

/// Checks if an expression is a `null` literal
fn is_null(expr: &AnyJsExpression) -> bool {
    expr.as_static_value()
        .is_some_and(|v| matches!(v, StaticValue::Null(_)))
}

/// Checks if two expressions are equivalent
fn expressions_equivalent(a: &AnyJsExpression, b: &AnyJsExpression) -> bool {
    is_node_equal(a.syntax(), b.syntax())
}
