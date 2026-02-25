use crate::{JsRuleAction, services::typed::Typed};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::{Applicability, Severity};
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, JsBinaryOperator,
    JsConditionalExpression, JsDoWhileStatement, JsForStatement, JsIfStatement,
    JsLogicalExpression, JsLogicalOperator, JsParenthesizedExpression, JsSyntaxKind,
    JsWhileStatement, T,
};
use biome_js_type_info::ConditionalType;
use biome_rowan::{
    AstNode, BatchMutationExt, Direction, TextRange, TriviaPieceKind, declare_node_union,
};
use biome_rule_options::use_nullish_coalescing::UseNullishCoalescingOptions;

declare_lint_rule! {
    /// Enforce using nullish coalescing operator (`??`) instead of logical or (`||`)
    /// or ternary expressions that perform explicit nullish checks.
    ///
    /// The `??` operator only checks for `null` and `undefined`, while `||` checks
    /// for any falsy value including `0`, `''`, and `false`. This can prevent bugs
    /// where legitimate falsy values are incorrectly treated as missing.
    ///
    /// This rule triggers when:
    /// - The left operand of `||` is possibly nullish (contains `null` or `undefined`
    ///   in its type). A safe fix is only offered when the type analysis confirms the
    ///   left operand can only be truthy or nullish (not other falsy values like `0`
    ///   or `''`).
    /// - A ternary expression performs an explicit nullish check such as
    ///   `a !== null ? a : b` or `a == null ? b : a`, which can be simplified to
    ///   `a ?? b`.
    ///
    /// By default, `||` expressions in conditional test positions (if/while/for/ternary)
    /// are ignored, as the falsy-checking behavior is often intentional there. This can
    /// be disabled with the `ignoreConditionalTests` option.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=logical-or-null.ts
    /// declare const maybeString: string | null;
    /// const value = maybeString || 'default';
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=logical-or-undefined.ts
    /// declare const maybeNumber: number | undefined;
    /// const value = maybeNumber || 0;
    /// ```
    ///
    /// ```js,expect_diagnostic,file=ternary-strict-null.js
    /// const value = a !== null ? a : 'default';
    /// ```
    ///
    /// ```js,expect_diagnostic,file=ternary-strict-undefined.js
    /// const value = a === undefined ? 'default' : a;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid-nullish-coalescing.ts
    /// declare const maybeString: string | null;
    /// const value = maybeString ?? 'default';
    /// ```
    ///
    /// ```ts,file=valid-non-nullish.ts
    /// declare const definiteString: string;
    /// const value = definiteString || 'fallback';
    /// ```
    ///
    /// ```ts,file=valid-conditional-test.ts
    /// declare const cond: string | null;
    /// if (cond || 'fallback') {
    ///   console.log('in if');
    /// }
    /// ```
    ///
    pub UseNullishCoalescing {
        version: "next",
        name: "useNullishCoalescing",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("prefer-nullish-coalescing").inspired()],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Types],
        issue_number: Some("8043"),
    }
}

declare_node_union! {
    pub AnyNullishCoalescingQuery = JsLogicalExpression | JsConditionalExpression
}

impl Rule for UseNullishCoalescing {
    type Query = Typed<AnyNullishCoalescingQuery>;
    type State = UseNullishCoalescingState;
    type Signals = Option<Self::State>;
    type Options = UseNullishCoalescingOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        match query {
            AnyNullishCoalescingQuery::JsLogicalExpression(logical) => {
                run_logical_or(ctx, logical)
            }
            AnyNullishCoalescingQuery::JsConditionalExpression(conditional) => {
                run_ternary(ctx, conditional)
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            UseNullishCoalescingState::LogicalOr { operator_range, .. } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *operator_range,
                    markup! {
                        "Use "<Emphasis>"??"</Emphasis>" instead of "<Emphasis>"||"</Emphasis>"."
                    },
                )
                .note(markup! {
                    "The "<Emphasis>"||"</Emphasis>" operator checks for all falsy values (including 0, '', and false), while "<Emphasis>"??"</Emphasis>" only checks for null and undefined."
                }),
            ),
            UseNullishCoalescingState::Ternary { range, .. } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *range,
                    markup! {
                        "Prefer "<Emphasis>"??"</Emphasis>" instead of a ternary expression with an explicit nullish check."
                    },
                ),
            ),
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        match (state, node) {
            (
                UseNullishCoalescingState::LogicalOr { can_fix, .. },
                AnyNullishCoalescingQuery::JsLogicalExpression(logical),
            ) => {
                if !can_fix {
                    return None;
                }

                let old_token = logical.operator_token().ok()?;

                let new_token = make::token(T![??])
                    .with_leading_trivia_pieces(old_token.leading_trivia().pieces())
                    .with_trailing_trivia_pieces(old_token.trailing_trivia().pieces());

                let mut mutation = ctx.root().begin();
                mutation.replace_token(old_token, new_token);

                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Use "<Emphasis>"??"</Emphasis>" instead." }.to_owned(),
                    mutation,
                ))
            }
            (
                UseNullishCoalescingState::Ternary {
                    can_fix,
                    is_safe_fix,
                    subject,
                    fallback,
                    ..
                },
                AnyNullishCoalescingQuery::JsConditionalExpression(conditional),
            ) => {
                if !can_fix {
                    return None;
                }

                let mut mutation = ctx.root().begin();
                // Build clean subject and fallback without contextual trivia.
                let clean_subject = subject
                    .clone()
                    .with_leading_trivia_pieces([])?
                    .with_trailing_trivia_pieces([])?;
                let clean_fallback = fallback
                    .clone()
                    .with_leading_trivia_pieces([])?
                    .with_trailing_trivia_pieces([])?;
                let replacement = make::js_logical_expression(
                    clean_subject,
                    make::token(T![??])
                        .with_leading_trivia([(TriviaPieceKind::Whitespace, " ")])
                        .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                    clean_fallback,
                );
                mutation.replace_node(
                    AnyJsExpression::JsConditionalExpression(conditional.clone()),
                    AnyJsExpression::JsLogicalExpression(replacement),
                );

                // Strict single checks (=== / !==) only test one of null/undefined,
                // but ?? covers both — mark as unsafe in that case.
                let applicability = if *is_safe_fix {
                    Applicability::Always
                } else {
                    Applicability::MaybeIncorrect
                };

                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    applicability,
                    markup! { "Use "<Emphasis>"??"</Emphasis>" instead." }.to_owned(),
                    mutation,
                ))
            }
            _ => None,
        }
    }
}

pub enum UseNullishCoalescingState {
    LogicalOr {
        operator_range: TextRange,
        can_fix: bool,
    },
    Ternary {
        range: TextRange,
        /// The subject expression to use as the left side of `??`.
        subject: AnyJsExpression,
        /// The fallback expression to use as the right side of `??`.
        fallback: AnyJsExpression,
        /// Whether we can offer any fix (false when subject has call expressions).
        can_fix: bool,
        /// Whether the fix is semantically safe (true for loose/compound checks).
        is_safe_fix: bool,
    },
}

fn run_logical_or(
    ctx: &RuleContext<UseNullishCoalescing>,
    logical: &JsLogicalExpression,
) -> Option<UseNullishCoalescingState> {
    let operator = logical.operator().ok()?;
    if operator != JsLogicalOperator::LogicalOr {
        return None;
    }

    let options = ctx.options();
    if options.ignore_conditional_tests() && is_in_test_position(logical) {
        return None;
    }

    let left = logical.left().ok()?;
    let left_ty = ctx.type_of_expression(&left);

    if !is_possibly_nullish(&left_ty) {
        return None;
    }

    let can_fix = is_safe_type_for_replacement(&left_ty)
        && is_safe_syntax_context_for_replacement(logical);

    Some(UseNullishCoalescingState::LogicalOr {
        operator_range: logical.operator_token().ok()?.text_trimmed_range(),
        can_fix,
    })
}

/// Detect ternary expressions (`condition ? consequent : alternate`) where the
/// condition is a nullish check and the expression can be simplified to `??`.
fn run_ternary(
    ctx: &RuleContext<UseNullishCoalescing>,
    conditional: &JsConditionalExpression,
) -> Option<UseNullishCoalescingState> {
    let options = ctx.options();
    if options.ignore_ternary_tests() {
        return None;
    }

    let info = extract_ternary_nullish_check(conditional)?;

    // Don't offer any fix when the subject contains call expressions,
    // because `??` evaluates the subject once while the ternary evaluates it twice.
    let can_fix = !contains_call_expression(&info.subject);
    // The fix is only semantically safe when the check already covers both
    // null and undefined (loose equality or compound strict check).
    let is_safe_fix = can_fix && info.covers_both_nullish;

    Some(UseNullishCoalescingState::Ternary {
        range: conditional.syntax().text_trimmed_range(),
        subject: info.subject,
        fallback: info.fallback,
        can_fix,
        is_safe_fix,
    })
}

struct TernaryNullishInfo {
    /// The expression to use as the left side of `??` (e.g. `a` in `a ?? b`).
    subject: AnyJsExpression,
    /// The fallback expression (e.g. `b` in `a ?? b`).
    fallback: AnyJsExpression,
    /// Whether the check covers both null and undefined (loose equality or compound strict).
    covers_both_nullish: bool,
}

/// Analyze the condition of a ternary expression to determine if it performs a
/// nullish check that can be simplified to `subject ?? fallback`.
///
/// Recognized patterns:
/// - **Simple check**: `a !== null`, `a === undefined`, `null == a`, etc.
///   A single binary comparison against `null` or `undefined`.
/// - **Compound check**: `a === null || a === undefined` or
///   `a !== null && a !== undefined`. Two strict comparisons joined by `||` / `&&`.
///
/// Returns `None` if the condition is not a recognized nullish check or if the
/// non-nullish branch does not match the checked subject expression.
fn extract_ternary_nullish_check(
    conditional: &JsConditionalExpression,
) -> Option<TernaryNullishInfo> {
    let condition = conditional.test().ok()?.omit_parentheses();
    let consequent = conditional.consequent().ok()?;
    let alternate = conditional.alternate().ok()?;

    // Determine the subject being checked, the equality direction, and whether
    // the check covers both null and undefined.
    let (check_subject, is_equality, covers_both_nullish) =
        if let AnyJsExpression::JsLogicalExpression(logical) = &condition {
            // Compound check: `a === null || a === undefined`
            let info = try_extract_compound_nullish_check(logical)?;
            (info.check_subject, info.is_equality, true)
        } else if let AnyJsExpression::JsBinaryExpression(binary) = &condition {
            // Simple check: `a !== null`, `a === undefined`, `null !== a`, etc.
            let check = try_extract_simple_nullish_check(binary)?;
            (check.subject, check.is_equality, check.is_loose)
        } else {
            return None;
        };

    // For equality checks (=== / ==), when the test is true the value IS null,
    // so the non-nullish branch is the alternate.
    // For inequality checks (!== / !=), when the test is true the value is NOT null,
    // so the non-nullish branch is the consequent.
    let (non_nullish_branch, fallback_branch) = if is_equality {
        (&alternate, consequent)
    } else {
        (&consequent, alternate)
    };

    // The non-nullish branch must match the expression being tested.
    if !expressions_are_equivalent(&check_subject, non_nullish_branch) {
        return None;
    }

    Some(TernaryNullishInfo {
        subject: non_nullish_branch.clone(),
        fallback: fallback_branch,
        covers_both_nullish,
    })
}

struct SimpleNullishCheck {
    /// The non-nullish side of the comparison.
    subject: AnyJsExpression,
    /// Whether this is an equality check (`===` / `==`).
    is_equality: bool,
    /// Whether this uses loose equality (`==` / `!=`).
    is_loose: bool,
}

/// Try to extract a nullish check from a single binary comparison.
///
/// A "simple nullish check" is a binary expression that compares an expression
/// against `null` or `undefined` using `===`, `!==`, `==`, or `!=`.
///
/// Examples: `a !== null`, `undefined === x`, `null != value`.
///
/// The nullish literal (`null` or `undefined`) may appear on either side of the
/// operator. We check the right operand first because the most common convention
/// is `expr === null` (subject on the left, literal on the right), but we also
/// handle the reversed form `null === expr`.
fn try_extract_simple_nullish_check(binary: &JsBinaryExpression) -> Option<SimpleNullishCheck> {
    let operator = binary.operator().ok()?;
    let (is_equality, is_loose) = match operator {
        JsBinaryOperator::StrictEquality => (true, false),
        JsBinaryOperator::StrictInequality => (false, false),
        JsBinaryOperator::Equality => (true, true),
        JsBinaryOperator::Inequality => (false, true),
        _ => return None,
    };

    let left = binary.left().ok()?.omit_parentheses();
    let right = binary.right().ok()?.omit_parentheses();

    // Check the right operand first (common case: `a !== null`), then
    // fall back to checking the left operand (reversed: `null !== a`).
    if is_null_or_undefined(&right) {
        Some(SimpleNullishCheck {
            subject: left,
            is_equality,
            is_loose,
        })
    } else if is_null_or_undefined(&left) {
        Some(SimpleNullishCheck {
            subject: right,
            is_equality,
            is_loose,
        })
    } else {
        None
    }
}

struct CompoundNullishCheck {
    /// The expression being tested (e.g. `a` in `a === null || a === undefined`).
    check_subject: AnyJsExpression,
    /// `true` for `=== null || === undefined`, `false` for `!== null && !== undefined`.
    is_equality: bool,
}

/// Try to extract a compound nullish check from a logical expression.
///
/// A "compound nullish check" combines two strict comparisons against `null`
/// and `undefined` with a logical operator. This covers both nullish values in
/// a single condition. Two forms are recognized:
///
/// - Equality form:  `a === null || a === undefined`  (logical OR)
/// - Inequality form: `a !== null && a !== undefined`  (logical AND)
///
/// Both sides must test the same subject, use strict equality/inequality
/// (not loose), and check different nullish values (one `null`, one `undefined`).
fn try_extract_compound_nullish_check(
    logical: &JsLogicalExpression,
) -> Option<CompoundNullishCheck> {
    let operator = logical.operator().ok()?;
    let left = logical.left().ok()?.omit_parentheses();
    let right = logical.right().ok()?.omit_parentheses();

    let left_binary = left.as_js_binary_expression()?;
    let right_binary = right.as_js_binary_expression()?;

    let left_check = try_extract_simple_nullish_check(left_binary)?;
    let right_check = try_extract_simple_nullish_check(right_binary)?;

    // Both sides must use the same equality direction and both must be strict.
    if left_check.is_equality != right_check.is_equality || left_check.is_loose
        || right_check.is_loose
    {
        return None;
    }

    // Validate the logical operator matches the equality direction:
    // `=== null || === undefined` or `!== null && !== undefined`
    match (left_check.is_equality, operator) {
        (true, JsLogicalOperator::LogicalOr) => {}
        (false, JsLogicalOperator::LogicalAnd) => {}
        _ => return None,
    }

    // Both sides must check the same subject.
    if !expressions_are_equivalent(&left_check.subject, &right_check.subject) {
        return None;
    }

    // One side must check null, the other undefined.
    let left_nullish = get_nullish_kind(left_binary)?;
    let right_nullish = get_nullish_kind(right_binary)?;
    if left_nullish == right_nullish {
        return None;
    }

    Some(CompoundNullishCheck {
        check_subject: left_check.subject,
        is_equality: left_check.is_equality,
    })
}

#[derive(PartialEq)]
enum NullishKind {
    Null,
    Undefined,
}

/// Determine whether a binary comparison checks against `null` or `undefined`.
fn get_nullish_kind(binary: &JsBinaryExpression) -> Option<NullishKind> {
    let left = binary.left().ok()?.omit_parentheses();
    let right = binary.right().ok()?.omit_parentheses();

    for expr in [&left, &right] {
        if is_null_literal(expr) {
            return Some(NullishKind::Null);
        }
        if is_undefined_identifier(expr) {
            return Some(NullishKind::Undefined);
        }
    }
    None
}

/// Check if an expression is `null` or `undefined`.
fn is_null_or_undefined(expr: &AnyJsExpression) -> bool {
    is_null_literal(expr) || is_undefined_identifier(expr)
}

fn is_null_literal(expr: &AnyJsExpression) -> bool {
    matches!(
        expr,
        AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsNullLiteralExpression(
            _
        ))
    )
}

/// Check if an expression is the `undefined` identifier.
///
/// Note: this does not account for `undefined` being shadowed by a local
/// binding (e.g. `let undefined = 42`). A scope-aware check would be needed
/// for full correctness.
fn is_undefined_identifier(expr: &AnyJsExpression) -> bool {
    match expr {
        AnyJsExpression::JsIdentifierExpression(ident) => ident
            .name()
            .ok()
            .is_some_and(|name| name.value_token().ok().is_some_and(|t| t.text_trimmed() == "undefined")),
        _ => false,
    }
}

/// Compare two expressions for structural equivalence by comparing their tokens
/// one-by-one, ignoring trivia (whitespace, comments) attached to each token.
///
/// This avoids false results from `SyntaxNode::text_trimmed()`, which only
/// strips trivia at the edges of a node but preserves internal trivia between
/// tokens (e.g. `a . b` vs `a.b` would differ at the node level).
fn expressions_are_equivalent(a: &AnyJsExpression, b: &AnyJsExpression) -> bool {
    let a = a.clone().omit_parentheses();
    let b = b.clone().omit_parentheses();
    let mut a_tokens = a
        .syntax()
        .descendants_with_tokens(Direction::Next)
        .filter_map(|el| el.into_token());
    let mut b_tokens = b
        .syntax()
        .descendants_with_tokens(Direction::Next)
        .filter_map(|el| el.into_token());
    loop {
        match (a_tokens.next(), b_tokens.next()) {
            (Some(a_tok), Some(b_tok)) => {
                if a_tok.kind() != b_tok.kind()
                    || a_tok.text_trimmed() != b_tok.text_trimmed()
                {
                    return false;
                }
            }
            (None, None) => return true,
            _ => return false,
        }
    }
}

/// Check whether an expression contains a call expression anywhere in its subtree.
/// When the subject contains calls, the ternary evaluates them twice (test + branch)
/// while `??` would only evaluate once, changing side-effect semantics.
fn contains_call_expression(expr: &AnyJsExpression) -> bool {
    expr.syntax()
        .descendants()
        .any(|node| node.kind() == JsSyntaxKind::JS_CALL_EXPRESSION)
}

fn is_safe_type_for_replacement(ty: &biome_js_type_info::Type) -> bool {
    if ty.is_union() {
        ty.flattened_union_variants().all(|variant| {
            matches!(
                variant.conditional_semantics(),
                ConditionalType::Truthy | ConditionalType::Nullish
            )
        })
    } else {
        matches!(
            ty.conditional_semantics(),
            ConditionalType::Truthy | ConditionalType::Nullish
        )
    }
}

fn is_possibly_nullish(ty: &biome_js_type_info::Type) -> bool {
    if ty.is_union() {
        ty.flattened_union_variants()
            .any(|variant| variant.conditional_semantics().is_nullish())
    } else {
        ty.conditional_semantics().is_nullish()
    }
}

fn is_safe_syntax_context_for_replacement(logical: &JsLogicalExpression) -> bool {
    let is_parenthesized = logical
        .syntax()
        .parent()
        .is_some_and(|parent| JsParenthesizedExpression::can_cast(parent.kind()));

    if !is_parenthesized
        && logical
            .syntax()
            .parent()
            .is_some_and(|parent| JsLogicalExpression::can_cast(parent.kind()))
    {
        return false;
    }

    let left = logical.left().ok();
    let right = logical.right().ok();

    if left
        .as_ref()
        .is_some_and(is_unparenthesized_and_or_expression)
        || right
            .as_ref()
            .is_some_and(is_unparenthesized_and_or_expression)
    {
        return false;
    }

    true
}

fn is_unparenthesized_and_or_expression(expr: &AnyJsExpression) -> bool {
    match expr {
        AnyJsExpression::JsParenthesizedExpression(_) => false,
        AnyJsExpression::JsLogicalExpression(logical) => {
            logical.operator().ok().is_some_and(|op| {
                matches!(
                    op,
                    JsLogicalOperator::LogicalAnd | JsLogicalOperator::LogicalOr
                )
            })
        }
        _ => false,
    }
}

fn is_in_test_position(logical: &JsLogicalExpression) -> bool {
    let logical_range = logical.syntax().text_trimmed_range();

    let test_contains_logical = |test: Option<AnyJsExpression>| -> bool {
        test.is_some_and(|t| t.syntax().text_trimmed_range().contains_range(logical_range))
    };

    for ancestor in logical.syntax().ancestors() {
        let is_in_test = match ancestor.kind() {
            JsSyntaxKind::JS_IF_STATEMENT => {
                test_contains_logical(JsIfStatement::cast_ref(&ancestor).and_then(|s| s.test().ok()))
            }
            JsSyntaxKind::JS_WHILE_STATEMENT => {
                test_contains_logical(JsWhileStatement::cast_ref(&ancestor).and_then(|s| s.test().ok()))
            }
            JsSyntaxKind::JS_FOR_STATEMENT => {
                test_contains_logical(JsForStatement::cast_ref(&ancestor).and_then(|s| s.test()))
            }
            JsSyntaxKind::JS_DO_WHILE_STATEMENT => {
                test_contains_logical(JsDoWhileStatement::cast_ref(&ancestor).and_then(|s| s.test().ok()))
            }
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
                test_contains_logical(JsConditionalExpression::cast_ref(&ancestor).and_then(|s| s.test().ok()))
            }
            _ => false,
        };
        if is_in_test {
            return true;
        }
    }
    false
}
