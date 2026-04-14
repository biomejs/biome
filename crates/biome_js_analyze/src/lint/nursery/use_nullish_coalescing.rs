use crate::{JsRuleAction, services::typed::Typed, utils::is_node_equal};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsAssignmentPattern, AnyJsExpression, JsAssignmentExpression, JsAssignmentOperator,
    JsBinaryOperator, JsConditionalExpression, JsDoWhileStatement, JsForStatement,
    JsIfStatement, JsLogicalExpression, JsLogicalOperator, JsParenthesizedExpression,
    JsSyntaxKind, JsWhileStatement, OperatorPrecedence, T,
};
use biome_js_type_info::{ConditionalType, TypeData};
use biome_rowan::{
    AstNode, BatchMutationExt, SyntaxResult, TextRange, declare_node_union,
    trim_leading_trivia_pieces,
};
use biome_rule_options::use_nullish_coalescing::UseNullishCoalescingOptions;

declare_lint_rule! {
    /// Enforce using the nullish coalescing operator (`??`) instead of logical or (`||`).
    ///
    /// `??` only checks for `null` and `undefined`, while `||` checks for any falsy value
    /// including `0`, `''`, and `false`. The rule reports `||`, `||=`, and ternary patterns
    /// (`x !== null ? x : y`) when type analysis shows the left operand is possibly nullish.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid-or.ts
    /// declare const maybeString: string | null;
    /// const value = maybeString || 'default';
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-or-undefined.ts
    /// declare const maybeNumber: number | undefined;
    /// const value = maybeNumber || 0;
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-or-assign.ts
    /// declare let x: string | null;
    /// x ||= 'default';
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// declare const x: string | null;
    /// const value = x !== null ? x : 'default';
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// declare const x: string | null;
    /// const value = x == null ? 'default' : x;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// declare const maybeString: string | null;
    /// const value = maybeString ?? 'default';
    /// ```
    ///
    /// ```ts
    /// declare const definiteString: string;
    /// const value = definiteString || 'fallback';
    /// ```
    ///
    /// ```ts
    /// declare const cond: string | null;
    /// if (cond || 'fallback') {
    ///   console.log('in if');
    /// }
    /// ```
    ///
    /// ```ts
    /// declare let y: string | null;
    /// y ??= 'default';
    /// ```
    ///
    /// ## Options
    ///
    /// ### ignoreConditionalTests
    ///
    /// Ignore `||` expressions inside conditional test positions (if/while/for/do-while/ternary).
    /// Default: `true`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreConditionalTests": false
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,use_options
    /// declare const cond: string | null;
    /// if (cond || 'fallback') {}
    /// ```
    ///
    /// ### ignoreTernaryTests
    ///
    /// Ignore ternary expressions that check for `null` or `undefined`. Default: `false`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreTernaryTests": true
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,use_options
    /// declare const x: string | null;
    /// const value = x !== null ? x : 'default';
    /// ```
    ///
    /// ### ignoreMixedLogicalExpressions
    ///
    /// Ignore `||` and `||=` whose connected logical tree also contains a `&&`. Default: `false`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreMixedLogicalExpressions": true
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,use_options
    /// declare const a: string | null;
    /// declare const b: string;
    /// const r = (a || 'default') && b;
    /// declare let assigned: string | null;
    /// assigned ||= b && 'fallback';
    /// ```
    ///
    pub UseNullishCoalescing {
        version: "2.4.5",
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
    pub UseNullishCoalescingQuery = JsLogicalExpression | JsAssignmentExpression | JsConditionalExpression
}

declare_node_union! {
    pub AnyJsLogicalOrLikeExpression = JsLogicalExpression | JsAssignmentExpression
}

impl AnyJsLogicalOrLikeExpression {
    fn is_logical_or_form(&self) -> bool {
        match self {
            Self::JsLogicalExpression(logical) => {
                logical.operator().ok() == Some(JsLogicalOperator::LogicalOr)
            }
            Self::JsAssignmentExpression(assignment) => {
                assignment.operator().ok() == Some(JsAssignmentOperator::LogicalOrAssign)
            }
        }
    }
}

pub enum UseNullishCoalescingState {
    LogicalOr {
        operator_range: TextRange,
        can_fix: bool,
    },
    LogicalOrAssignment {
        operator_range: TextRange,
        can_fix: bool,
    },
    Ternary {
        test_range: TextRange,
        checked_expr: AnyJsExpression,
        fallback_expr: AnyJsExpression,
        is_positive: bool,
        can_fix: bool,
    },
}

impl Rule for UseNullishCoalescing {
    type Query = Typed<UseNullishCoalescingQuery>;
    type State = UseNullishCoalescingState;
    type Signals = Option<Self::State>;
    type Options = UseNullishCoalescingOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            UseNullishCoalescingQuery::JsLogicalExpression(logical) => {
                run_logical_or(ctx, logical)
            }
            UseNullishCoalescingQuery::JsAssignmentExpression(assignment) => {
                run_logical_or_assignment(ctx, assignment)
            }
            UseNullishCoalescingQuery::JsConditionalExpression(ternary) => {
                run_ternary(ctx, ternary)
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
            UseNullishCoalescingState::LogicalOrAssignment { operator_range, .. } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *operator_range,
                    markup! {
                        "Use "<Emphasis>"??="</Emphasis>" instead of "<Emphasis>"||="</Emphasis>"."
                    },
                )
                .note(markup! {
                    "The "<Emphasis>"||="</Emphasis>" operator assigns when the left side is falsy, while "<Emphasis>"??="</Emphasis>" only assigns when it is null or undefined."
                }),
            ),
            UseNullishCoalescingState::Ternary { test_range, .. } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *test_range,
                    markup! {
                        "Prefer "<Emphasis>"??"</Emphasis>" over a ternary expression checking for nullish."
                    },
                ),
            ),
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let query = ctx.query();
        let mut mutation = ctx.root().begin();

        let message = match (state, query) {
            (
                UseNullishCoalescingState::LogicalOr { can_fix, .. },
                UseNullishCoalescingQuery::JsLogicalExpression(logical),
            ) => {
                if !can_fix {
                    return None;
                }
                let old_token = logical.operator_token().ok()?;

                let new_token = make::token(T![??])
                    .with_leading_trivia_pieces(old_token.leading_trivia().pieces())
                    .with_trailing_trivia_pieces(old_token.trailing_trivia().pieces());

                mutation.replace_token(old_token, new_token);
                markup! { "Replace "<Emphasis>"||"</Emphasis>" with "<Emphasis>"??"</Emphasis>"." }.to_owned()
            }
            (
                UseNullishCoalescingState::LogicalOrAssignment { can_fix, .. },
                UseNullishCoalescingQuery::JsAssignmentExpression(assignment),
            ) => {
                if !can_fix {
                    return None;
                }
                let old_token = assignment.operator_token().ok()?;

                let new_token = make::token(T![??=])
                    .with_leading_trivia_pieces(old_token.leading_trivia().pieces())
                    .with_trailing_trivia_pieces(old_token.trailing_trivia().pieces());

                mutation.replace_token(old_token, new_token);
                markup! { "Replace "<Emphasis>"||="</Emphasis>" with "<Emphasis>"??="</Emphasis>"." }.to_owned()
            }
            (
                UseNullishCoalescingState::Ternary {
                    checked_expr,
                    fallback_expr,
                    is_positive,
                    can_fix,
                    ..
                },
                UseNullishCoalescingQuery::JsConditionalExpression(ternary),
            ) => {
                if !can_fix {
                    return None;
                }

                // Strip trailing whitespace. Ternary layout trivia is not appropriate for `??`.
                let checked_expr = checked_expr.clone().trim_trailing_trivia()?;
                let fallback_expr = fallback_expr.clone().trim_trailing_trivia()?;

                // Transfer trivia from the ? and : tokens to the branch expressions they precede.
                let question = ternary.question_mark_token().ok()?;
                let colon = ternary.colon_token().ok()?;
                let question_trivia =
                    trim_leading_trivia_pieces(question.trailing_trivia().pieces());
                let colon_trivia =
                    trim_leading_trivia_pieces(colon.trailing_trivia().pieces());

                let (checked_expr, fallback_expr) = if *is_positive {
                    (
                        checked_expr.prepend_trivia_pieces(question_trivia)?,
                        fallback_expr.prepend_trivia_pieces(colon_trivia)?,
                    )
                } else {
                    (
                        checked_expr.prepend_trivia_pieces(colon_trivia)?,
                        fallback_expr.prepend_trivia_pieces(question_trivia)?,
                    )
                };

                let checked = maybe_parenthesize_for_nullish(checked_expr);
                let fallback = maybe_parenthesize_for_nullish(fallback_expr);

                let new_expr = make::js_logical_expression(
                    checked,
                    make::token_decorated_with_space(T![??]),
                    fallback,
                );

                // Transfer leading/trailing trivia from the original ternary
                let new_expr = AnyJsExpression::from(new_expr)
                    .prepend_trivia_pieces(
                        ternary.syntax().first_leading_trivia()?.pieces(),
                    )?
                    .append_trivia_pieces(
                        ternary.syntax().last_trailing_trivia()?.pieces(),
                    )?;

                mutation.replace_node_discard_trivia(
                    AnyJsExpression::from(ternary.clone()),
                    new_expr,
                );
                markup! { "Replace the ternary with "<Emphasis>"??"</Emphasis>"." }.to_owned()
            }
            _ => return None,
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message,
            mutation,
        ))
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

    if options.ignore_mixed_logical_expressions()
        && is_in_mixed_logical_tree(AnyJsLogicalOrLikeExpression::from(logical.clone()))
            .is_some_and(|mixed| mixed)
    {
        return None;
    }

    let left = logical.left().ok()?;
    let left_ty = ctx.type_of_expression(&left);

    if !is_possibly_nullish(&left_ty) {
        return None;
    }

    let can_fix =
        is_safe_type_for_replacement(&left_ty) && is_safe_syntax_context_for_replacement(logical);

    Some(UseNullishCoalescingState::LogicalOr {
        operator_range: logical.operator_token().ok()?.text_trimmed_range(),
        can_fix,
    })
}

fn run_logical_or_assignment(
    ctx: &RuleContext<UseNullishCoalescing>,
    assignment: &JsAssignmentExpression,
) -> Option<UseNullishCoalescingState> {
    let operator = assignment.operator().ok()?;
    if operator != JsAssignmentOperator::LogicalOrAssign {
        return None;
    }

    let options = ctx.options();
    if options.ignore_mixed_logical_expressions()
        && is_in_mixed_logical_tree(AnyJsLogicalOrLikeExpression::from(assignment.clone()))
            .is_some_and(|mixed| mixed)
    {
        return None;
    }

    let left = assignment.left().ok()?;
    let left_ty = match &left {
        AnyJsAssignmentPattern::AnyJsAssignment(assign) => {
            let id = assign.as_js_identifier_assignment()?;
            let name = id.name_token().ok()?;
            ctx.type_of_named_value(assignment.range(), name.text_trimmed())
        }
        _ => return None,
    };

    if !is_possibly_nullish(&left_ty) {
        return None;
    }

    let can_fix = is_safe_type_for_replacement(&left_ty);

    Some(UseNullishCoalescingState::LogicalOrAssignment {
        operator_range: assignment.operator_token().ok()?.text_trimmed_range(),
        can_fix,
    })
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
    // Without parentheses, swapping `||` for `??` next to another logical operator changes precedence.
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

/// Returns `Some(true)` when `node` belongs to a connected tree of `||`/`||=`
/// operations that also contains a `&&`; `Some(false)` when no `&&` is present
/// in that tree. Returns `None` when `node` is not a `||`/`||=` form, so
/// callers can use `?` to bail out.
fn is_in_mixed_logical_tree(node: AnyJsLogicalOrLikeExpression) -> Option<bool> {
    node.is_logical_or_form().then_some(())?;
    let root = climb_to_logical_or_root(node);
    Some(has_logical_and_directly_above(&root) || tree_contains_logical_and(&root))
}

fn has_logical_and_directly_above(node: &AnyJsLogicalOrLikeExpression) -> bool {
    node.syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| !JsParenthesizedExpression::can_cast(ancestor.kind()))
        .and_then(JsLogicalExpression::cast)
        .and_then(|logical| logical.operator().ok())
        == Some(JsLogicalOperator::LogicalAnd)
}

/// Walks up through `||`/`||=` parents (skipping parens) and stops at the
/// first non-`||`/`||=` ancestor. This keeps the traversal bounded to the
/// connected logical tree rather than walking the full ancestor chain.
fn climb_to_logical_or_root(
    start: AnyJsLogicalOrLikeExpression,
) -> AnyJsLogicalOrLikeExpression {
    let mut current = start;
    while let Some(parent) = parent_logical_or(&current) {
        current = parent;
    }
    current
}

fn parent_logical_or(
    current: &AnyJsLogicalOrLikeExpression,
) -> Option<AnyJsLogicalOrLikeExpression> {
    let mut parent = current.syntax().parent()?;
    while JsParenthesizedExpression::can_cast(parent.kind()) {
        parent = parent.parent()?;
    }
    let parent_logical = AnyJsLogicalOrLikeExpression::cast(parent)?;
    parent_logical.is_logical_or_form().then_some(parent_logical)
}

fn tree_contains_logical_and(node: &AnyJsLogicalOrLikeExpression) -> bool {
    match node {
        AnyJsLogicalOrLikeExpression::JsLogicalExpression(logical) => {
            match logical.operator() {
                Ok(JsLogicalOperator::LogicalAnd) => true,
                Ok(JsLogicalOperator::LogicalOr) => {
                    operand_reaches_logical_and(logical.left())
                        || operand_reaches_logical_and(logical.right())
                }
                _ => false,
            }
        }
        AnyJsLogicalOrLikeExpression::JsAssignmentExpression(assignment) => {
            // For `||=`, only the right-hand side is part of the logical tree.
            operand_reaches_logical_and(assignment.right())
        }
    }
}

fn operand_reaches_logical_and(operand: SyntaxResult<AnyJsExpression>) -> bool {
    operand.ok().is_some_and(|expr| {
        match expr.omit_parentheses() {
            AnyJsExpression::JsLogicalExpression(logical) => {
                tree_contains_logical_and(&AnyJsLogicalOrLikeExpression::from(logical))
            }
            AnyJsExpression::JsAssignmentExpression(assignment)
                if assignment.operator().ok()
                    == Some(JsAssignmentOperator::LogicalOrAssign) =>
            {
                tree_contains_logical_and(&AnyJsLogicalOrLikeExpression::from(assignment))
            }
            _ => false,
        }
    })
}

fn is_in_test_position(logical: &JsLogicalExpression) -> bool {
    let logical_range = logical.syntax().text_trimmed_range();

    let test_contains_logical = |test: Option<AnyJsExpression>| -> bool {
        test.is_some_and(|t| t.syntax().text_trimmed_range().contains_range(logical_range))
    };

    for ancestor in logical.syntax().ancestors() {
        let is_in_test = match ancestor.kind() {
            JsSyntaxKind::JS_IF_STATEMENT => test_contains_logical(
                JsIfStatement::cast_ref(&ancestor).and_then(|s| s.test().ok()),
            ),
            JsSyntaxKind::JS_WHILE_STATEMENT => test_contains_logical(
                JsWhileStatement::cast_ref(&ancestor).and_then(|s| s.test().ok()),
            ),
            JsSyntaxKind::JS_FOR_STATEMENT => test_contains_logical(
                JsForStatement::cast_ref(&ancestor).and_then(|s| s.test()),
            ),
            JsSyntaxKind::JS_DO_WHILE_STATEMENT => test_contains_logical(
                JsDoWhileStatement::cast_ref(&ancestor).and_then(|s| s.test().ok()),
            ),
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => test_contains_logical(
                JsConditionalExpression::cast_ref(&ancestor).and_then(|s| s.test().ok()),
            ),
            _ => false,
        };
        if is_in_test {
            return true;
        }
    }
    false
}

fn run_ternary(
    ctx: &RuleContext<UseNullishCoalescing>,
    ternary: &JsConditionalExpression,
) -> Option<UseNullishCoalescingState> {
    if ctx.options().ignore_ternary_tests() {
        return None;
    }

    let test = ternary.test().ok()?;
    let consequent = ternary.consequent().ok()?;
    let alternate = ternary.alternate().ok()?;

    let (checked_expr, fallback_expr, is_positive, check_kind) =
        check_ternary_nullish_pattern(&test, &consequent, &alternate)?;

    // The fix is unsafe when the checked expression contains calls or `new`, because
    // the ternary evaluates it twice (test + branch) while `??` evaluates it once.
    let has_side_effects = contains_call_or_new_expression(&checked_expr);

    let can_fix = !has_side_effects
        && match check_kind {
            // Loose equality and compound strict checks cover both null and undefined
            NullishCheckKind::Loose | NullishCheckKind::Compound => true,
            // A single strict check only covers one nullish variant. The fix to `??`
            // is safe only if the type cannot be the opposite variant.
            NullishCheckKind::StrictSingle(lit) => {
                let ty = ctx.type_of_expression(&checked_expr);
                match lit {
                    NullishLiteral::Null => !type_has_undefined(&ty),
                    NullishLiteral::Undefined => !type_has_null(&ty),
                }
            }
        };

    Some(UseNullishCoalescingState::Ternary {
        test_range: test.syntax().text_trimmed_range(),
        checked_expr,
        fallback_expr,
        is_positive,
        can_fix,
    })
}

/// Returns `(checked, fallback, is_positive, check_kind)` if the ternary can be converted to `??`.
/// `is_positive` is true when checked = consequent (e.g. `x !== null ? x : y`).
fn check_ternary_nullish_pattern(
    test: &AnyJsExpression,
    consequent: &AnyJsExpression,
    alternate: &AnyJsExpression,
) -> Option<(AnyJsExpression, AnyJsExpression, bool, NullishCheckKind)> {
    // `x !== null ? x : y` or `x != null ? x : y`
    if let Some((_, kind)) = match_nullish_check(
        test,
        consequent,
        JsBinaryOperator::Inequality,
        JsBinaryOperator::StrictInequality,
        JsLogicalOperator::LogicalAnd,
    ) {
        return Some((consequent.clone(), alternate.clone(), true, kind));
    }

    // `x === null ? y : x` or `x == null ? y : x`
    if let Some((_, kind)) = match_nullish_check(
        test,
        alternate,
        JsBinaryOperator::Equality,
        JsBinaryOperator::StrictEquality,
        JsLogicalOperator::LogicalOr,
    ) {
        return Some((alternate.clone(), consequent.clone(), false, kind));
    }

    None
}

/// Returns the checked expression and the kind of nullish check if the test matches
/// a nullish comparison pattern (loose, strict, or compound) using the given operator family.
fn match_nullish_check(
    test: &AnyJsExpression,
    value: &AnyJsExpression,
    loose_op: JsBinaryOperator,
    strict_op: JsBinaryOperator,
    compound_logical_op: JsLogicalOperator,
) -> Option<(AnyJsExpression, NullishCheckKind)> {
    // Loose: `x != null` or `x == null`
    if let Some((checked, _)) = extract_nullish_comparison_operand(test, loose_op)
        && expressions_equivalent(&checked, value)
    {
        return Some((checked, NullishCheckKind::Loose));
    }

    // Strict: `x !== null` or `x === undefined`
    if let Some((checked, lit)) = extract_nullish_comparison_operand(test, strict_op)
        && expressions_equivalent(&checked, value)
    {
        return Some((checked, NullishCheckKind::StrictSingle(lit)));
    }

    // Compound: `x !== null && x !== undefined` or `x === null || x === undefined`
    if let Some(logical) = test.as_js_logical_expression()
        && logical.operator().ok()? == compound_logical_op
    {
        let left = logical.left().ok()?;
        let right = logical.right().ok()?;

        if let (Some((checked_left, lit_left)), Some((checked_right, lit_right))) = (
            extract_nullish_comparison_operand(&left, strict_op),
            extract_nullish_comparison_operand(&right, strict_op),
        ) && expressions_equivalent(&checked_left, &checked_right)
            && expressions_equivalent(&checked_left, value)
        {
            // Both sides must test different literals (one null, one undefined)
            // to be a true compound check. If both test the same literal,
            // treat it as a single strict check.
            if lit_left != lit_right {
                return Some((checked_left, NullishCheckKind::Compound));
            }
            return Some((checked_left, NullishCheckKind::StrictSingle(lit_left)));
        }
    }

    None
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum NullishLiteral {
    Null,
    Undefined,
}

#[derive(Clone, Copy)]
enum NullishCheckKind {
    /// Loose equality: `!= null` or `== null` (covers both null and undefined)
    Loose,
    /// Single strict equality: `!== null` or `=== undefined` (covers only one)
    StrictSingle(NullishLiteral),
    /// Compound strict: `!== null && !== undefined` (covers both)
    Compound,
}

/// Returns the non-nullish operand of a binary comparison against `null` or `undefined`.
///
/// Checks the right operand first because the conventional form places the literal
/// on the right (`x !== null`). The left check handles the reversed form (`null !== x`).
fn extract_nullish_comparison_operand(
    expr: &AnyJsExpression,
    expected_op: JsBinaryOperator,
) -> Option<(AnyJsExpression, NullishLiteral)> {
    let binary = expr.as_js_binary_expression()?;
    if binary.operator().ok()? != expected_op {
        return None;
    }
    let left = binary.left().ok()?;
    let right = binary.right().ok()?;
    if let Some(lit) = nullish_literal_kind(&right) {
        return Some((left, lit));
    }
    if let Some(lit) = nullish_literal_kind(&left) {
        return Some((right, lit));
    }
    None
}

fn nullish_literal_kind(expr: &AnyJsExpression) -> Option<NullishLiteral> {
    use biome_js_syntax::static_value::StaticValue;
    match expr.as_static_value()? {
        StaticValue::Null(_) => Some(NullishLiteral::Null),
        StaticValue::Undefined(_) => Some(NullishLiteral::Undefined),
        _ => None,
    }
}

fn expressions_equivalent(a: &AnyJsExpression, b: &AnyJsExpression) -> bool {
    is_node_equal(a.syntax(), b.syntax())
}

/// Returns true if the expression contains a call or `new` expression.
///
/// In a ternary like `foo() !== null ? foo() : y`, the subject is evaluated twice
/// (once in the test, once in the branch). Replacing with `foo() ?? y` evaluates it
/// once, which is a semantic change if the call has side effects.
fn contains_call_or_new_expression(expr: &AnyJsExpression) -> bool {
    expr.syntax().descendants().any(|node| {
        matches!(
            node.kind(),
            JsSyntaxKind::JS_CALL_EXPRESSION | JsSyntaxKind::JS_NEW_EXPRESSION
        )
    })
}

fn type_has_null(ty: &biome_js_type_info::Type) -> bool {
    let is_null = |t: &biome_js_type_info::Type| {
        t.resolved_data()
            .is_some_and(|d| matches!(d.as_raw_data(), TypeData::Null))
    };
    if ty.is_union() {
        ty.flattened_union_variants().any(|v| is_null(&v))
    } else {
        is_null(ty)
    }
}

fn type_has_undefined(ty: &biome_js_type_info::Type) -> bool {
    let is_undef = |t: &biome_js_type_info::Type| {
        t.resolved_data()
            .is_some_and(|d| matches!(d.as_raw_data(), TypeData::Undefined | TypeData::VoidKeyword))
    };
    if ty.is_union() {
        ty.flattened_union_variants().any(|v| is_undef(&v))
    } else {
        is_undef(ty)
    }
}

/// Wraps the expression in parentheses if it would be invalid or change meaning
/// as an operand of `??`. This covers two cases:
/// - Expressions with lower precedence than `??` (conditional, assignment, yield, comma)
/// - `||` and `&&` expressions, which cannot be mixed with `??` without parentheses
fn maybe_parenthesize_for_nullish(expr: AnyJsExpression) -> AnyJsExpression {
    if expr
        .precedence()
        .is_ok_and(|p| p < OperatorPrecedence::Coalesce)
    {
        return make::parenthesized(expr).into();
    }
    if let Some(logical) = expr.as_js_logical_expression()
        && logical.operator().is_ok_and(|op| {
            matches!(
                op,
                JsLogicalOperator::LogicalOr | JsLogicalOperator::LogicalAnd
            )
        })
    {
        return make::parenthesized(expr).into();
    }
    expr
}
