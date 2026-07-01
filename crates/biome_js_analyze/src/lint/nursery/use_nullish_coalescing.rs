use crate::{JsRuleAction, services::typed::Typed, utils::is_node_equal};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsAssignmentPattern, AnyJsExpression, AnyJsStatement, JsAssignmentExpression,
    JsAssignmentOperator, JsBinaryOperator, JsCallArgumentList, JsCallArguments, JsCallExpression,
    JsConditionalExpression, JsDoWhileStatement, JsExpressionStatement, JsForStatement,
    JsIfStatement, JsLogicalExpression, JsLogicalOperator, JsParenthesizedExpression, JsSyntaxKind,
    JsUnaryOperator, JsWhileStatement, OperatorPrecedence, T,
};
use biome_js_type_info::{ConditionalType, TypeData};
use biome_rowan::{
    AstNode, AstNodeList, BatchMutationExt, Direction, SyntaxResult, TextRange, declare_node_union,
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
    /// #### Invalid
    ///
    /// `||` and `||=` are still reported when the surrounding logical tree does not contain `&&`.
    ///
    /// ```ts,expect_diagnostic,use_options,file=invalid-mixed-or.ts
    /// declare const maybeString: string | null;
    /// const value = maybeString || 'default';
    /// ```
    ///
    /// ```ts,expect_diagnostic,use_options,file=invalid-mixed-or-assign.ts
    /// declare let assigned: string | null;
    /// assigned ||= 'default';
    /// ```
    ///
    /// #### Valid
    ///
    /// `||` and `||=` mixed with `&&` in the same logical tree are not reported.
    ///
    /// ```ts,use_options
    /// declare const a: string | null;
    /// declare const b: string;
    /// const r = (a || 'default') && b;
    /// ```
    ///
    /// ```ts,use_options
    /// declare const b: string;
    /// declare let assigned: string | null;
    /// assigned ||= b && 'fallback';
    /// ```
    ///
    /// ### ignoreBooleanCoercion
    ///
    /// Ignore `||` and `||=` used inside a `Boolean()` call, where coalescing on
    /// falsy values is intentional. Default: `false`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreBooleanCoercion": true
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// `||` and `||=` outside a `Boolean()` call are still reported.
    ///
    /// ```ts,expect_diagnostic,use_options,file=invalid-boolean-coercion.ts
    /// declare const maybeString: string | null;
    /// const value = maybeString || 'default';
    /// ```
    ///
    /// #### Valid
    ///
    /// `||` and `||=` inside a `Boolean()` call are not reported.
    ///
    /// ```ts,use_options,file=valid-boolean-coercion.ts
    /// declare const a: string | null;
    /// declare const b: string;
    /// const r = Boolean(a || b);
    /// ```
    ///
    /// ### ignorePrimitives
    ///
    /// Ignore `||`, `||=`, and ternary expressions when every non-nullish variant
    /// of the operand is a primitive the option opts out of. Use `true` to ignore
    /// all primitives, or an object selecting `string`, `number`, `boolean`, or
    /// `bigint`. Default: none.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignorePrimitives": { "string": true }
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// Primitive kinds that are not opted out of are still reported.
    ///
    /// ```ts,expect_diagnostic,use_options,file=invalid-primitives.ts
    /// declare const count: number | null;
    /// const value = count || 0;
    /// ```
    ///
    /// #### Valid
    ///
    /// A `string` operand is not reported when `string` is ignored.
    ///
    /// ```ts,use_options,file=valid-primitives.ts
    /// declare const name: string | null;
    /// const value = name || 'default';
    /// ```
    ///
    /// ### ignoreIfStatements
    ///
    /// By default Biome also reports an `if` statement that only assigns to a
    /// nullish variable, since it can be rewritten as `??=`. Set this to `true`
    /// to ignore those statements. Default: `false`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreIfStatements": true
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// `||` and `||=` are still reported when only `if` statements are ignored.
    ///
    /// ```ts,expect_diagnostic,use_options,file=invalid-if-statements.ts
    /// declare const maybeString: string | null;
    /// const value = maybeString || 'default';
    /// ```
    ///
    /// #### Valid
    ///
    /// An `if` statement performing a nullish assignment is not reported.
    ///
    /// ```ts,use_options,file=valid-if-statements.ts
    /// declare let a: { x: string } | null;
    /// declare function makeA(): { x: string };
    /// if (!a) {
    ///     a = makeA();
    /// }
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
    pub UseNullishCoalescingQuery = JsLogicalExpression | JsAssignmentExpression | JsConditionalExpression | JsIfStatement
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
    IfStatement {
        if_range: TextRange,
        assignment_target: AnyJsAssignmentPattern,
        assignment_value: AnyJsExpression,
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
            UseNullishCoalescingQuery::JsIfStatement(if_stmt) => run_if_statement(ctx, if_stmt),
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
            UseNullishCoalescingState::IfStatement { if_range, .. } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *if_range,
                    markup! {
                        "Use "<Emphasis>"??="</Emphasis>" instead of an "<Emphasis>"if"</Emphasis>" statement for nullish assignment."
                    },
                )
                .note(markup! {
                    "This "<Emphasis>"if"</Emphasis>" statement only assigns when the variable is nullish, which "<Emphasis>"??="</Emphasis>" expresses directly."
                }),
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
            (
                UseNullishCoalescingState::IfStatement {
                    assignment_target,
                    assignment_value,
                    can_fix,
                    ..
                },
                UseNullishCoalescingQuery::JsIfStatement(if_stmt),
            ) => {
                if !can_fix {
                    return None;
                }
                let target = assignment_target.clone().trim_trivia()?;
                let value = assignment_value.clone().trim_trivia()?;

                let new_assignment = make::js_assignment_expression(
                    target,
                    make::token_decorated_with_space(T![??=]),
                    value,
                );

                let new_stmt = make::js_expression_statement(AnyJsExpression::from(new_assignment))
                    .with_semicolon_token(make::token(T![;]))
                    .build();

                // Transfer leading/trailing trivia from the original if statement.
                let new_stmt = AnyJsStatement::from(new_stmt)
                    .prepend_trivia_pieces(if_stmt.syntax().first_leading_trivia()?.pieces())?
                    .append_trivia_pieces(if_stmt.syntax().last_trailing_trivia()?.pieces())?;

                mutation.replace_node_discard_trivia(
                    AnyJsStatement::from(if_stmt.clone()),
                    new_stmt,
                );
                markup! { "Replace the "<Emphasis>"if"</Emphasis>" statement with "<Emphasis>"??="</Emphasis>"." }.to_owned()
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

    if options.ignore_boolean_coercion()
        && is_in_boolean_call_context(&AnyJsLogicalOrLikeExpression::from(logical.clone()))
    {
        return None;
    }

    let left = logical.left().ok()?;
    let left_ty = ctx.type_of_expression(&left);

    if !is_possibly_nullish(&left_ty) {
        return None;
    }

    if options.has_any_ignore_primitives() && should_ignore_for_primitives(options, &left_ty) {
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

    if options.ignore_boolean_coercion()
        && is_in_boolean_call_context(&AnyJsLogicalOrLikeExpression::from(assignment.clone()))
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

    if options.has_any_ignore_primitives() && should_ignore_for_primitives(options, &left_ty) {
        return None;
    }

    let can_fix = is_safe_type_for_replacement(&left_ty);

    Some(UseNullishCoalescingState::LogicalOrAssignment {
        operator_range: assignment.operator_token().ok()?.text_trimmed_range(),
        can_fix,
    })
}

fn run_if_statement(
    ctx: &RuleContext<UseNullishCoalescing>,
    if_stmt: &JsIfStatement,
) -> Option<UseNullishCoalescingState> {
    let options = ctx.options();
    if options.ignore_if_statements() {
        return None;
    }

    // An `else` branch means the statement does more than a nullish fallback.
    if if_stmt.else_clause().is_some() {
        return None;
    }

    let assignment = extract_if_body_assignment(if_stmt)?;

    let left = assignment.left().ok()?;
    if !matches!(
        left.syntax().kind(),
        JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT
            | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT
            | JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT
    ) {
        return None;
    }

    // The test must check the same reference that is assigned in the body.
    let test = if_stmt.test().ok()?;
    let (subject, test_kind) = extract_if_null_check(&test)?;
    if subject.syntax().text_trimmed() != left.syntax().text_trimmed() {
        return None;
    }

    let subject_ty = ctx.type_of_expression(&subject);
    if !is_possibly_nullish(&subject_ty) {
        return None;
    }
    if options.has_any_ignore_primitives() && should_ignore_for_primitives(options, &subject_ty) {
        return None;
    }

    // The fix to `??=` is only safe when the test catches exactly `null`/`undefined`.
    // A `!x` truthiness test also catches other falsy values (`0`, `''`, `false`), so it
    // is only safe when the non-nullish part of the type cannot itself be falsy. A single
    // strict check (`x === null`) misses the other nullish value, mirroring the ternary path.
    let type_allows_fix = match test_kind {
        IfTestKind::Truthiness => is_safe_type_for_replacement(&subject_ty),
        IfTestKind::Nullish(NullishCheckKind::Loose | NullishCheckKind::Compound) => true,
        IfTestKind::Nullish(NullishCheckKind::StrictSingle(lit)) => match lit {
            NullishLiteral::Null => !type_has_undefined(&subject_ty),
            NullishLiteral::Undefined => !type_has_null(&subject_ty),
        },
    };

    // The original `if` evaluates the subject twice (test + assignment) while `??=`
    // evaluates it once, so the rewrite only preserves semantics when re-evaluating the
    // reference is side-effect-free and stable (e.g. `getObj().x` or `arr[i++]` are not).
    // Inner comments would be discarded by the rewrite, so skip the fix in that case too.
    let can_fix = type_allows_fix
        && is_stable_reference(&subject)
        && !if_statement_has_inner_comments(if_stmt);

    Some(UseNullishCoalescingState::IfStatement {
        if_range: if_stmt.syntax().text_trimmed_range(),
        assignment_target: assignment.left().ok()?,
        assignment_value: assignment.right().ok()?,
        can_fix,
    })
}

/// How an `if` condition tests its subject for nullishness.
#[derive(Clone, Copy)]
enum IfTestKind {
    /// `!x` truthiness negation. Catches every falsy value, not only nullish ones.
    Truthiness,
    /// A comparison against `null`/`undefined`, carrying the same coverage kinds
    /// as the ternary path.
    Nullish(NullishCheckKind),
}

/// Returns the single plain `=` assignment in an `if` body.
/// Handles both `if (test) { target = value; }` and `if (test) target = value;`.
/// `||=`/`??=` bodies are left to the operator logic, avoiding a double report.
fn extract_if_body_assignment(if_stmt: &JsIfStatement) -> Option<JsAssignmentExpression> {
    let consequent = if_stmt.consequent().ok()?;

    let expr_stmt = match &consequent {
        AnyJsStatement::JsBlockStatement(block) => {
            let statements = block.statements();
            if statements.len() != 1 {
                return None;
            }
            JsExpressionStatement::cast(statements.into_iter().next()?.into_syntax())?
        }
        AnyJsStatement::JsExpressionStatement(expr_stmt) => expr_stmt.clone(),
        _ => return None,
    };

    let assignment = expr_stmt
        .expression()
        .ok()?
        .as_js_assignment_expression()?
        .clone();
    (assignment.operator().ok()? == JsAssignmentOperator::Assign).then_some(assignment)
}

/// Classifies an `if` test as a truthiness negation (`!x`) or a nullish comparison,
/// returning the tested reference and the kind.
///
/// Supported tests:
/// - `!x`
/// - `x == null` / `x == undefined` (and reversed operands)
/// - `x === null` / `x === undefined` (and reversed operands)
/// - `x === null || x === undefined`
fn extract_if_null_check(test: &AnyJsExpression) -> Option<(AnyJsExpression, IfTestKind)> {
    match test {
        AnyJsExpression::JsUnaryExpression(unary) => {
            if unary.operator().ok()? != JsUnaryOperator::LogicalNot {
                return None;
            }
            let arg = unary.argument().ok()?;
            is_member_access_like_expr(&arg).then_some((arg, IfTestKind::Truthiness))
        }
        AnyJsExpression::JsBinaryExpression(_) => {
            let (subject, kind) = match test.as_js_binary_expression()?.operator().ok()? {
                JsBinaryOperator::Equality => {
                    let (subject, _) =
                        extract_nullish_comparison_operand(test, JsBinaryOperator::Equality)?;
                    (subject, NullishCheckKind::Loose)
                }
                JsBinaryOperator::StrictEquality => {
                    let (subject, lit) =
                        extract_nullish_comparison_operand(test, JsBinaryOperator::StrictEquality)?;
                    (subject, NullishCheckKind::StrictSingle(lit))
                }
                _ => return None,
            };
            is_member_access_like_expr(&subject).then_some((subject, IfTestKind::Nullish(kind)))
        }
        AnyJsExpression::JsLogicalExpression(logical) => {
            if logical.operator().ok()? != JsLogicalOperator::LogicalOr {
                return None;
            }
            let left = logical.left().ok()?;
            let right = logical.right().ok()?;
            let (left_subject, left_lit) =
                extract_nullish_comparison_operand(&left, JsBinaryOperator::StrictEquality)?;
            let (right_subject, right_lit) =
                extract_nullish_comparison_operand(&right, JsBinaryOperator::StrictEquality)?;
            if !is_member_access_like_expr(&left_subject)
                || !expressions_equivalent(&left_subject, &right_subject)
            {
                return None;
            }
            // Distinct literals (one null, one undefined) cover both nullish values.
            let kind = if left_lit != right_lit {
                NullishCheckKind::Compound
            } else {
                NullishCheckKind::StrictSingle(left_lit)
            };
            Some((left_subject, IfTestKind::Nullish(kind)))
        }
        _ => None,
    }
}

fn is_member_access_like_expr(expr: &AnyJsExpression) -> bool {
    matches!(
        expr,
        AnyJsExpression::JsIdentifierExpression(_)
            | AnyJsExpression::JsStaticMemberExpression(_)
            | AnyJsExpression::JsComputedMemberExpression(_)
    )
}

/// Whether re-evaluating `expr` is side-effect-free and yields the same reference.
/// Rewriting an `if` into `??=` collapses two evaluations of the subject into one, so
/// the rewrite only preserves semantics for stable references. Calls, updates
/// (`i++`), and other observable expressions anywhere in the reference are rejected.
fn is_stable_reference(expr: &AnyJsExpression) -> bool {
    match expr {
        AnyJsExpression::JsIdentifierExpression(_) | AnyJsExpression::JsThisExpression(_) => true,
        AnyJsExpression::JsStaticMemberExpression(member) => {
            member.object().is_ok_and(|object| is_stable_reference(&object))
        }
        AnyJsExpression::JsComputedMemberExpression(member) => {
            member.object().is_ok_and(|object| is_stable_reference(&object))
                && member.member().is_ok_and(|index| is_stable_index(&index))
        }
        AnyJsExpression::JsParenthesizedExpression(paren) => {
            paren.expression().is_ok_and(|inner| is_stable_reference(&inner))
        }
        _ => false,
    }
}

/// A computed-member index is stable when it is a literal or another stable reference,
/// so `obj['a']` and `obj[key]` are fixable but `arr[i++]` is not.
fn is_stable_index(expr: &AnyJsExpression) -> bool {
    matches!(expr, AnyJsExpression::AnyJsLiteralExpression(_)) || is_stable_reference(expr)
}

/// Whether the `if` statement carries comments that the `??=` rewrite would discard.
/// The rewrite preserves the statement's leading trivia (comments above the `if`) and
/// its trailing trivia, but drops everything in between, so those are ignored here.
fn if_statement_has_inner_comments(if_stmt: &JsIfStatement) -> bool {
    let syntax = if_stmt.syntax();
    let first = syntax.first_token();
    let last = syntax.last_token();
    syntax.descendants_tokens(Direction::Next).any(|token| {
        let is_first = first.as_ref() == Some(&token);
        let is_last = last.as_ref() == Some(&token);
        (!is_first && token.has_leading_comments()) || (!is_last && token.has_trailing_comments())
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

/// Returns `true` when every non-nullish variant of `ty` is a primitive that the
/// configured `ignorePrimitives` option suppresses. A non-union type is never
/// ignored here, since the diagnostic only fires once the operand is a nullish
/// union in the first place.
fn should_ignore_for_primitives(
    options: &UseNullishCoalescingOptions,
    ty: &biome_js_type_info::Type,
) -> bool {
    if !ty.is_union() {
        return false;
    }
    ty.flattened_union_variants()
        .filter(|variant| !variant.conditional_semantics().is_nullish())
        .all(|variant| is_ignored_primitive(options, &variant))
}

/// Maps a resolved primitive (or primitive literal) type to its matching
/// `ignorePrimitives` selector. Non-primitive types are never ignored.
fn is_ignored_primitive(
    options: &UseNullishCoalescingOptions,
    ty: &biome_js_type_info::Type,
) -> bool {
    ty.resolved_data().is_some_and(|data| match data.as_raw_data() {
        TypeData::String => options.should_ignore_primitive_string(),
        TypeData::Number => options.should_ignore_primitive_number(),
        TypeData::Boolean => options.should_ignore_primitive_boolean(),
        TypeData::BigInt => options.should_ignore_primitive_bigint(),
        TypeData::Literal(literal) => match literal.as_ref() {
            biome_js_type_info::Literal::String(_) => options.should_ignore_primitive_string(),
            biome_js_type_info::Literal::Number(_) => options.should_ignore_primitive_number(),
            biome_js_type_info::Literal::Boolean(_) => options.should_ignore_primitive_boolean(),
            biome_js_type_info::Literal::BigInt(_) => options.should_ignore_primitive_bigint(),
            _ => false,
        },
        _ => false,
    })
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

/// Returns `true` when the `||`/`||=` expression is boolean-coerced by an
/// enclosing `Boolean(...)` call, where coalescing on falsy values is the
/// intended behavior.
///
/// It skips parenthesized and logical-expression ancestors (both `||` and `&&`,
/// since the coerced value still flows into the call) so that
/// `Boolean(a || b || c)` and `Boolean(x && (a || b))` are both treated as
/// coerced, then inspects the first ancestor that is neither. That ancestor must
/// be the argument list of a `Boolean(...)` call for the expression to be
/// considered coerced.
fn is_in_boolean_call_context(node: &AnyJsLogicalOrLikeExpression) -> bool {
    node.syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| {
            !(JsParenthesizedExpression::can_cast(ancestor.kind())
                || JsLogicalExpression::can_cast(ancestor.kind()))
        })
        .and_then(JsCallArgumentList::cast)
        .and_then(|list| list.parent::<JsCallArguments>())
        .and_then(|args| args.parent::<JsCallExpression>())
        .is_some_and(|call| call.has_callee("Boolean"))
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

    let options = ctx.options();
    if options.has_any_ignore_primitives() {
        let checked_ty = ctx.type_of_expression(&checked_expr);
        if should_ignore_for_primitives(options, &checked_ty) {
            return None;
        }
    }

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
