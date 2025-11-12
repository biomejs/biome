use crate::JsRuleAction;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsAssignmentPattern, AnyJsExpression, AnyJsStatement, JsAssignmentExpression,
    JsAssignmentOperator, JsBinaryOperator, JsIfStatement, JsLogicalOperator, JsUnaryOperator,
};
use biome_rowan::{AstNode, AstNodeList};

declare_lint_rule! {
    /// Enforce using the nullish coalescing assignment operator (`??=`) instead of if statements for default value assignment.
    ///
    /// If statements that check for nullish values and assign a default can be simplified
    /// using the nullish coalescing assignment operator (`??=`).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (!foo) {
    ///   foo = makeFoo();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (foo == null) {
    ///   foo = makeFoo();
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// foo ??= makeFoo();
    /// ```
    ///
    /// ```js
    /// if (!foo) {
    ///   foo = makeFoo();
    /// } else {
    ///   doSomething();
    /// }
    /// ```
    ///
    pub UseIfAsNullishCoalescingAssignment {
        version: "next",
        name: "useIfAsNullishCoalescingAssignment",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("prefer-nullish-coalescing").inspired()],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

pub struct RuleState {
    assignment: JsAssignmentExpression,
}

/// Checks if an if statement matches the pattern for ??= conversion
/// Returns (checked_variable, assignment) if pattern matches
fn check_if_pattern(if_stmt: &JsIfStatement) -> Option<(AnyJsExpression, JsAssignmentExpression)> {
    // Must not have else clause
    if if_stmt.else_clause().is_some() {
        return None;
    }

    // Get the condition
    let condition = if_stmt.test().ok()?;

    // Get the body
    let body = if_stmt.consequent().ok()?;

    // Extract assignment from body
    let assignment = extract_assignment(&body)?;

    // Assignment must be simple = operator
    let operator = assignment.operator().ok()?;
    if !matches!(operator, JsAssignmentOperator::Assign) {
        return None;
    }

    // Check if condition is a nullish check for the assigned variable
    let assignment_target = assignment.left().ok()?;
    let checked_variable = is_nullish_check_for(&condition, &assignment_target)?;

    Some((checked_variable, assignment))
}

/// Extracts a single assignment expression from a statement
fn extract_assignment(stmt: &AnyJsStatement) -> Option<JsAssignmentExpression> {
    match stmt {
        // Handle block statement with single assignment
        AnyJsStatement::JsBlockStatement(block) => {
            let statements = block.statements();

            // Must be exactly one statement
            if statements.len() != 1 {
                return None;
            }

            // Get the single statement
            let first_stmt = statements.first()?;

            // Must be expression statement
            let expr_stmt = first_stmt.as_js_expression_statement()?;
            let expr = expr_stmt.expression().ok()?;

            // Must be assignment expression
            expr.as_js_assignment_expression().cloned()
        }
        // Handle naked expression statement (no braces)
        AnyJsStatement::JsExpressionStatement(expr_stmt) => {
            let expr = expr_stmt.expression().ok()?;
            expr.as_js_assignment_expression().cloned()
        }
        _ => None,
    }
}

/// Checks if condition is a nullish check for the given target
/// Returns the checked expression if it matches
fn is_nullish_check_for(
    condition: &AnyJsExpression,
    target: &AnyJsAssignmentPattern,
) -> Option<AnyJsExpression> {
    // Pattern 1: !variable
    if let Some(checked) = is_negation_check(condition)
        && expressions_match(&checked, target)
    {
        return Some(checked);
    }

    // Pattern 2: variable == null (loose equality)
    if let Some(checked) = is_loose_null_check(condition)
        && expressions_match(&checked, target)
    {
        return Some(checked);
    }

    // Pattern 3: variable === null || variable === undefined (strict equality)
    if let Some(checked) = is_strict_nullish_check(condition)
        && expressions_match(&checked, target)
    {
        return Some(checked);
    }

    None
}

/// Checks for pattern: !variable
fn is_negation_check(expr: &AnyJsExpression) -> Option<AnyJsExpression> {
    let unary = expr.as_js_unary_expression()?;
    let op = unary.operator().ok()?;

    if !matches!(op, JsUnaryOperator::LogicalNot) {
        return None;
    }

    unary.argument().ok()
}

/// Checks for pattern: variable == null (loose equality - matches both null and undefined)
fn is_loose_null_check(expr: &AnyJsExpression) -> Option<AnyJsExpression> {
    let binary = expr.as_js_binary_expression()?;
    let op = binary.operator().ok()?;

    if !matches!(op, JsBinaryOperator::Equality) {
        return None;
    }

    let left = binary.left().ok()?;
    let right = binary.right().ok()?;

    // Check if either side is null
    if is_null(&right) {
        return Some(left);
    }
    if is_null(&left) {
        return Some(right);
    }

    None
}

/// Checks for pattern: variable === null || variable === undefined
fn is_strict_nullish_check(expr: &AnyJsExpression) -> Option<AnyJsExpression> {
    let logical = expr.as_js_logical_expression()?;
    let op = logical.operator().ok()?;

    // Must be OR operator
    if !matches!(op, JsLogicalOperator::LogicalOr) {
        return None;
    }

    let left = logical.left().ok()?;
    let right = logical.right().ok()?;

    // Check if both sides are strict equality checks
    let left_checked = is_strict_equality_check(&left)?;
    let right_checked = is_strict_equality_check(&right)?;

    // Both sides must check the same variable
    if !expressions_equivalent(&left_checked, &right_checked) {
        return None;
    }

    Some(left_checked)
}

/// Checks for pattern: variable === null or variable === undefined
fn is_strict_equality_check(expr: &AnyJsExpression) -> Option<AnyJsExpression> {
    let binary = expr.as_js_binary_expression()?;
    let op = binary.operator().ok()?;

    if !matches!(op, JsBinaryOperator::StrictEquality) {
        return None;
    }

    let left = binary.left().ok()?;
    let right = binary.right().ok()?;

    // Check if either side is null or undefined
    if is_null_or_undefined(&right) {
        return Some(left);
    }
    if is_null_or_undefined(&left) {
        return Some(right);
    }

    None
}

/// Checks if expression is null literal
fn is_null(expr: &AnyJsExpression) -> bool {
    use biome_js_syntax::static_value::StaticValue;
    expr.as_static_value()
        .is_some_and(|v| matches!(v, StaticValue::Null(_)))
}

/// Checks if expression is null or undefined literal
fn is_null_or_undefined(expr: &AnyJsExpression) -> bool {
    expr.as_static_value()
        .is_some_and(|v| v.is_null_or_undefined())
}

/// Checks if an expression and an assignment pattern refer to the same variable
fn expressions_match(expr: &AnyJsExpression, pattern: &AnyJsAssignmentPattern) -> bool {
    // Convert pattern to expression for comparison
    // This is a simplified text-based comparison
    let expr_text = expr.syntax().text_trimmed().to_string();
    let pattern_text = pattern.syntax().text_trimmed().to_string();

    // Normalize whitespace
    let expr_normalized = expr_text.split_whitespace().collect::<Vec<_>>().join("");
    let pattern_normalized = pattern_text.split_whitespace().collect::<Vec<_>>().join("");

    expr_normalized == pattern_normalized
}

/// Check if two expressions are textually equivalent
fn expressions_equivalent(a: &AnyJsExpression, b: &AnyJsExpression) -> bool {
    let a_text = a.syntax().text_trimmed().to_string();
    let b_text = b.syntax().text_trimmed().to_string();

    // Normalize whitespace
    let a_normalized = a_text.split_whitespace().collect::<Vec<_>>().join("");
    let b_normalized = b_text.split_whitespace().collect::<Vec<_>>().join("");

    a_normalized == b_normalized
}

impl Rule for UseIfAsNullishCoalescingAssignment {
    type Query = biome_analyze::Ast<JsIfStatement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let if_stmt = ctx.query();

        // Check if the if statement matches the pattern
        let (_checked_variable, assignment) = check_if_pattern(if_stmt)?;

        Some(RuleState { assignment })
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "Use "<Emphasis>"??="</Emphasis>" instead of if statement for default value assignment."
                },
            )
            .note(markup! {
                "If statements checking for null or undefined can be simplified using the nullish coalescing assignment operator."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        use biome_js_factory::make;
        use biome_js_syntax::T;
        use biome_rowan::BatchMutationExt;

        let if_stmt = ctx.query();
        let mut mutation = ctx.root().begin();

        // Get the assignment components
        let assignment_left = state.assignment.left().ok()?;
        let assignment_right = state.assignment.right().ok()?;

        // Create new ??= assignment expression
        let new_assignment =
            make::js_assignment_expression(assignment_left, make::token(T![??=]), assignment_right);

        // Create expression statement from assignment using the builder pattern
        let new_expr_stmt = make::js_expression_statement(new_assignment.into())
            .with_semicolon_token(make::token(T![;]))
            .build();

        // Replace entire if statement with new expression statement
        mutation.replace_node(
            AnyJsStatement::from(if_stmt.clone()),
            AnyJsStatement::from(new_expr_stmt),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace if statement with "<Emphasis>"??="</Emphasis>" assignment." }
                .to_owned(),
            mutation,
        ))
    }
}
