use crate::{JsRuleAction, services::typed::Typed};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, JsBinaryOperator,
    JsConditionalExpression, JsDoWhileStatement, JsForStatement, JsIfStatement,
    JsLogicalExpression, JsLogicalOperator, JsParenthesizedExpression, JsSyntaxKind,
    JsWhileStatement, T,
};
use biome_js_type_info::ConditionalType;
use biome_rowan::{AstNode, BatchMutationExt, TextRange, TriviaPieceKind, declare_node_union};
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
    /// ```ts
    /// declare const maybeString: string | null;
    /// const value = maybeString || 'default'; // should use ??
    /// ```
    ///
    /// ```ts
    /// declare const maybeNumber: number | undefined;
    /// const value = maybeNumber || 0; // should use ??
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const value = a !== null ? a : 'default'; // should use ??
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const value = a === undefined ? 'default' : a; // should use ??
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// // Already using ??
    /// declare const maybeString: string | null;
    /// const value = maybeString ?? 'default';
    /// ```
    ///
    /// ```ts
    /// // Type is not nullish - no null or undefined in union
    /// declare const definiteString: string;
    /// const value = definiteString || 'fallback';
    /// ```
    ///
    /// ```ts
    /// // In conditional test position (ignored by default)
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
        fix_kind: FixKind::Safe,
        domains: &[RuleDomain::Types],
        issue_number: Some("8043"),
    }
}

declare_node_union! {
    pub AnyNullishCoalescingQuery = JsLogicalExpression | JsConditionalExpression
}

pub enum UseNullishCoalescingState {
    LogicalOr {
        operator_range: TextRange,
        can_fix: bool,
    },
    Ternary {
        range: TextRange,
    },
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
                )
                .note(markup! {
                    "This ternary can be simplified to the nullish coalescing operator."
                }),
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
                UseNullishCoalescingState::Ternary { .. },
                AnyNullishCoalescingQuery::JsConditionalExpression(conditional),
            ) => {
                let info = extract_ternary_nullish_check(conditional)?;

                let mut mutation = ctx.root().begin();
                // Build clean subject and fallback without contextual trivia.
                let clean_subject = info
                    .subject
                    .clone()
                    .with_leading_trivia_pieces([])?
                    .with_trailing_trivia_pieces([])?;
                let clean_fallback = info
                    .fallback
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

                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Use "<Emphasis>"??"</Emphasis>" instead." }.to_owned(),
                    mutation,
                ))
            }
            _ => None,
        }
    }
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

fn run_ternary(
    ctx: &RuleContext<UseNullishCoalescing>,
    conditional: &JsConditionalExpression,
) -> Option<UseNullishCoalescingState> {
    let options = ctx.options();
    if options.ignore_ternary_tests() {
        return None;
    }

    extract_ternary_nullish_check(conditional)?;

    Some(UseNullishCoalescingState::Ternary {
        range: conditional.syntax().text_trimmed_range(),
    })
}

struct TernaryNullishInfo {
    /// The expression to use as the left side of `??` (e.g. `a` in `a ?? b`).
    subject: AnyJsExpression,
    /// The fallback expression (e.g. `b` in `a ?? b`).
    fallback: AnyJsExpression,
}

/// Extract info from a ternary's test expression if it performs a nullish check.
/// Returns `Some` only if the ternary can be simplified to `subject ?? fallback`.
fn extract_ternary_nullish_check(
    conditional: &JsConditionalExpression,
) -> Option<TernaryNullishInfo> {
    let test = conditional.test().ok()?.omit_parentheses();
    let consequent = conditional.consequent().ok()?;
    let alternate = conditional.alternate().ok()?;

    // Extract the check subject and equality direction from the test expression.
    let (check_subject, is_equality) =
        if let AnyJsExpression::JsLogicalExpression(logical) = &test {
            // Compound: `a === null || a === undefined` or `a !== null && a !== undefined`
            let info = try_extract_compound_nullish_check(logical)?;
            (info.check_subject, info.is_equality)
        } else if let AnyJsExpression::JsBinaryExpression(binary) = &test {
            // Simple: `a !== null`, `a === undefined`, `null !== a`, etc.
            let check = try_extract_simple_nullish_check(binary)?;
            (check.subject, check.is_equality)
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

/// Try to extract a nullish check from a binary expression like `a !== null` or `null === a`.
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

    // Determine which side is the nullish literal and which is the subject.
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

/// Try to extract a compound nullish check like `a === null || a === undefined`.
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

fn is_undefined_identifier(expr: &AnyJsExpression) -> bool {
    match expr {
        AnyJsExpression::JsIdentifierExpression(ident) => ident
            .name()
            .ok()
            .is_some_and(|name| name.value_token().ok().is_some_and(|t| t.text_trimmed() == "undefined")),
        _ => false,
    }
}

/// Compare two expressions for structural equivalence by comparing their trimmed text
/// after stripping parentheses.
fn expressions_are_equivalent(a: &AnyJsExpression, b: &AnyJsExpression) -> bool {
    let a = a.clone().omit_parentheses();
    let b = b.clone().omit_parentheses();
    a.syntax().text_trimmed() == b.syntax().text_trimmed()
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
