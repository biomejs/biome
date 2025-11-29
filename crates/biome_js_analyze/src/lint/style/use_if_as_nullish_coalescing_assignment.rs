use crate::JsRuleAction;
use crate::utils::is_node_equal;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsAssignmentPattern, AnyJsExpression, AnyJsStatement, JsAssignmentExpression,
    JsAssignmentOperator, JsBinaryOperator, JsIfStatement, JsLogicalOperator,
};
use biome_rowan::{AstNode, AstNodeList};

declare_lint_rule! {
    /// Enforce using the nullish coalescing assignment operator (`??=`) instead of if statements for default value assignment.
    ///
    /// If statements that explicitly check for nullish values (`null` or `undefined`) and assign a default
    /// can be simplified using the nullish coalescing assignment operator (`??=`).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (foo == null) {
    ///   foo = makeFoo();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (foo === null || foo === undefined) {
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
    /// // if (!foo) is NOT converted because it also checks for 0, '', and false
    /// if (!foo) {
    ///   foo = makeFoo();
    /// }
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
        sources: &[RuleSource::EslintTypeScript("prefer-nullish-coalescing").same()],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

pub struct RuleState {
    assignment: JsAssignmentExpression,
}

/// Checks if an if statement matches the pattern for `??=` conversion
///
/// Returns `(checked_variable, assignment)` if the pattern matches:
/// - No else clause
/// - Single assignment statement in the consequent
/// - Assignment uses `=` operator (not `+=`, `-=`, etc.)
/// - Condition checks if the assigned variable is nullish
///
/// ## Examples
/// ```js
/// if (foo == null) { foo = bar; }    // Matches
/// if (foo === null || foo === undefined) { foo = bar; }  // Matches
/// if (!foo) { foo = bar; }           // Doesn't match (checks all falsy)
/// if (foo == null) { foo = bar; } else {}   // Doesn't match (has else)
/// if (foo == null) { foo += bar; }   // Doesn't match (wrong operator)
/// ```
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
///
/// Handles two cases:
/// 1. Block statement with single assignment: `{ foo = bar; }`
/// 2. Naked expression statement: `foo = bar`
///
/// Returns `None` if the statement is not an assignment or contains multiple statements.
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

/// Checks if a condition is a nullish check for the given assignment target
///
/// Returns the checked expression if one of these patterns matches:
/// - `variable == null` (loose equality)
/// - `variable === null || variable === undefined` (strict equality)
///
/// Note: `!variable` is intentionally NOT supported because it checks for all
/// falsy values (0, '', false, null, undefined, NaN), not just nullish values.
/// Converting `if (!foo) { foo = x; }` to `foo ??= x;` would change behavior
/// when `foo` is `0`, `''`, or `false`.
fn is_nullish_check_for(
    condition: &AnyJsExpression,
    target: &AnyJsAssignmentPattern,
) -> Option<AnyJsExpression> {
    // Pattern 1: variable == null (loose equality)
    if let Some(checked) = is_loose_null_check(condition)
        && expressions_match(&checked, target)
    {
        return Some(checked);
    }

    // Pattern 2: variable === null || variable === undefined (strict equality)
    if let Some(checked) = is_strict_nullish_check(condition)
        && expressions_match(&checked, target)
    {
        return Some(checked);
    }

    None
}

/// Checks for pattern: `variable == null`
///
/// Loose equality check matches both null and undefined due to JavaScript's
/// type coercion rules.
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

/// Checks for pattern: `variable === null || variable === undefined`
///
/// Returns the variable if both sides of the OR check the same variable
/// for strict equality against null or undefined.
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

/// Checks for pattern: `variable === null` or `variable === undefined`
///
/// Returns the checked variable if the binary expression uses strict equality
/// to compare against null or undefined.
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

/// Checks if an expression is a `null` literal
fn is_null(expr: &AnyJsExpression) -> bool {
    use biome_js_syntax::static_value::StaticValue;
    expr.as_static_value()
        .is_some_and(|v| matches!(v, StaticValue::Null(_)))
}

/// Checks if an expression is `null` or `undefined` literal
fn is_null_or_undefined(expr: &AnyJsExpression) -> bool {
    expr.as_static_value()
        .is_some_and(|v| v.is_null_or_undefined())
}

/// Checks if an expression and an assignment pattern refer to the same entity
fn expressions_match(expr: &AnyJsExpression, pattern: &AnyJsAssignmentPattern) -> bool {
    expr.syntax().text_trimmed() == pattern.syntax().text_trimmed()
}

/// Checks if two expressions are structurally equivalent
fn expressions_equivalent(a: &AnyJsExpression, b: &AnyJsExpression) -> bool {
    is_node_equal(a.syntax(), b.syntax())
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
