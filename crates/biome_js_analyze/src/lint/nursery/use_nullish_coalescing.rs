use crate::{JsRuleAction, services::typed::Typed, utils::is_node_equal};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::{MarkupBuf, markup};
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsAssignmentPattern, AnyJsExpression, AnyJsLiteralExpression, AnyJsStatement,
    JsAssignmentExpression, JsAssignmentOperator, JsBinaryExpression, JsBinaryOperator,
    JsCallArgumentList, JsCallArguments, JsCallExpression, JsConditionalExpression,
    JsDoWhileStatement, JsExpressionStatement, JsForStatement, JsIfStatement,
    JsLogicalExpression, JsLogicalOperator, JsParenthesizedExpression, JsSyntaxKind,
    JsSyntaxNode, JsUnaryOperator, JsWhileStatement, OperatorPrecedence, T,
};
use biome_js_type_info::{ConditionalType, TypeData};
use biome_js_syntax::JsLanguage;
use biome_rowan::{
    AstNode, AstNodeList, BatchMutation, BatchMutationExt, TextRange, declare_node_union,
    trim_leading_trivia_pieces,
};
use biome_rule_options::use_nullish_coalescing::UseNullishCoalescingOptions;

declare_lint_rule! {
    /// Enforce using the nullish coalescing operator (`??`) instead of logical or (`||`).
    ///
    /// The `??` operator only checks for `null` and `undefined`, while `||` checks
    /// for any falsy value including `0`, `''`, and `false`. This can prevent bugs
    /// where legitimate falsy values are incorrectly treated as missing.
    ///
    /// For `||` expressions, this rule triggers when the left operand is possibly
    /// nullish (contains `null` or `undefined` in its type). A safe fix is only
    /// offered when type analysis confirms the left operand can only be truthy or
    /// nullish (not other falsy values like `0` or `''`).
    ///
    /// For `||=` assignment expressions, the same logic applies: `a ||= b` is
    /// flagged when `a` is possibly nullish and can be rewritten as `a ??= b`.
    ///
    /// For ternary expressions, this rule detects patterns like `x !== null ? x : y`
    /// and suggests rewriting them as `x ?? y`. This applies to strict and loose
    /// equality checks against `null` or `undefined`, including compound checks.
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
    /// ```ts
    /// declare let x: string | null;
    /// x ||= 'default'; // should use ??=
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// declare const x: string | null;
    /// const value = x !== null ? x : 'default'; // should use ??
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// declare const x: string | null;
    /// const value = x == null ? 'default' : x; // should use ??
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
    /// ```ts
    /// // Already using ??=
    /// declare let y: string | null;
    /// y ??= 'default';
    /// ```
    ///
    /// ## Options
    ///
    /// ### `ignoreConditionalTests`
    ///
    /// Ignore `||` expressions inside conditional test positions
    /// (if/while/for/do-while/ternary conditions), where falsy-checking
    /// behavior may be intentional.
    ///
    /// Default: `true`
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
    /// ### `ignoreTernaryTests`
    ///
    /// Ignore ternary expressions that check for `null` or `undefined`
    /// and could be replaced with `??`.
    ///
    /// Default: `false`
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
    /// const value = x !== null ? x : 'default'; // no diagnostic
    /// ```
    ///
    /// ### `ignoreMixedLogicalExpressions`
    ///
    /// When `true`, `||` mixed with `&&` in the same expression is ignored.
    ///
    /// Default: `false`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreMixedLogicalExpressions": true
    ///     }
    /// }
    /// ```
    /// ```ts,use_options
    /// declare const a: string | null;
    /// declare const b: string;
    /// const r = (a || 'default') && b;
    /// ```
    ///
    /// ### `ignorePrimitives`
    ///
    /// Suppress diagnostics when the non-nullish type variants are primitives.
    /// Set to `true` to ignore all primitives, or an object with `string`,
    /// `number`, `boolean`, `bigint` fields.
    ///
    /// Default: all `false`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignorePrimitives": true
    ///     }
    /// }
    /// ```
    /// ```ts,use_options
    /// declare const s: string | null;
    /// const r = s || 'default';
    /// ```
    ///
    /// ### `ignoreBooleanCoercion`
    ///
    /// When `true`, `||` inside `Boolean()` calls is ignored.
    ///
    /// Default: `false`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreBooleanCoercion": true
    ///     }
    /// }
    /// ```
    /// ```ts,use_options
    /// declare const a: string | null;
    /// const r = Boolean(a || 'default');
    /// ```
    ///
    /// ### `ignoreIfStatements`
    ///
    /// When `true`, `if` statements that could be simplified to `??=` are ignored.
    ///
    /// Default: `false`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreIfStatements": true
    ///     }
    /// }
    /// ```
    /// ```ts,use_options
    /// declare let a: { x: string } | null;
    /// declare function makeFoo(): { x: string };
    /// if (!a) {
    ///     a = makeFoo();
    /// }
    /// ```
    ///
    pub UseNullishCoalescing {
        version: "2.4.5",
        name: "useNullishCoalescing",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("prefer-nullish-coalescing").same()],
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
            UseNullishCoalescingQuery::JsIfStatement(if_stmt) => {
                run_if_statement(ctx, if_stmt)
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
            UseNullishCoalescingState::IfStatement { if_range, .. } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *if_range,
                    markup! {
                        "Prefer "<Emphasis>"??="</Emphasis>" instead of an if statement for nullish assignment."
                    },
                )
                .note(markup! {
                    "This if statement can be simplified to a nullish coalescing assignment."
                }),
            ),
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        let message = match (state, ctx.query()) {
            (UseNullishCoalescingState::LogicalOr { can_fix, .. },
             UseNullishCoalescingQuery::JsLogicalExpression(logical)) => {
                fix_logical_or(*can_fix, logical, &mut mutation)?
            }
            (UseNullishCoalescingState::LogicalOrAssignment { can_fix, .. },
             UseNullishCoalescingQuery::JsAssignmentExpression(assignment)) => {
                fix_logical_or_assignment(*can_fix, assignment, &mut mutation)?
            }
            (UseNullishCoalescingState::Ternary { checked_expr, fallback_expr, is_positive, can_fix, .. },
             UseNullishCoalescingQuery::JsConditionalExpression(ternary)) => {
                fix_ternary(*can_fix, ternary, checked_expr, fallback_expr, *is_positive, &mut mutation)?
            }
            (UseNullishCoalescingState::IfStatement { assignment_target, assignment_value, .. },
             UseNullishCoalescingQuery::JsIfStatement(if_stmt)) => {
                fix_if_statement(if_stmt, assignment_target, assignment_value, &mut mutation)?
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

fn fix_logical_or(
    can_fix: bool,
    logical: &JsLogicalExpression,
    mutation: &mut BatchMutation<JsLanguage>,
) -> Option<MarkupBuf> {
    if !can_fix {
        return None;
    }
    let old_token = logical.operator_token().ok()?;

    let new_token = make::token(T![??])
        .with_leading_trivia_pieces(old_token.leading_trivia().pieces())
        .with_trailing_trivia_pieces(old_token.trailing_trivia().pieces());

    mutation.replace_token(old_token, new_token);
    Some(markup! { "Use "<Emphasis>"??"</Emphasis>" instead." }.to_owned())
}

fn fix_logical_or_assignment(
    can_fix: bool,
    assignment: &JsAssignmentExpression,
    mutation: &mut BatchMutation<JsLanguage>,
) -> Option<MarkupBuf> {
    if !can_fix {
        return None;
    }
    let old_token = assignment.operator_token().ok()?;

    let new_token = make::token(T![??=])
        .with_leading_trivia_pieces(old_token.leading_trivia().pieces())
        .with_trailing_trivia_pieces(old_token.trailing_trivia().pieces());

    mutation.replace_token(old_token, new_token);
    Some(markup! { "Use "<Emphasis>"??="</Emphasis>" instead." }.to_owned())
}

fn fix_ternary(
    can_fix: bool,
    ternary: &JsConditionalExpression,
    checked_expr: &AnyJsExpression,
    fallback_expr: &AnyJsExpression,
    is_positive: bool,
    mutation: &mut BatchMutation<JsLanguage>,
) -> Option<MarkupBuf> {
    if !can_fix {
        return None;
    }

    // Strip trailing whitespace. Ternary layout trivia is not appropriate for `??`.
    let checked_expr = checked_expr.clone().trim_trailing_trivia()?;
    let fallback_expr = fallback_expr.clone().trim_trailing_trivia()?;

    // Transfer trivia from the ? and : tokens to the branch expressions they precede.
    let question = ternary.question_mark_token().ok()?;
    let colon = ternary.colon_token().ok()?;
    let question_trivia = trim_leading_trivia_pieces(question.trailing_trivia().pieces());
    let colon_trivia = trim_leading_trivia_pieces(colon.trailing_trivia().pieces());

    let (checked_expr, fallback_expr) = if is_positive {
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
        .prepend_trivia_pieces(ternary.syntax().first_leading_trivia()?.pieces())?
        .append_trivia_pieces(ternary.syntax().last_trailing_trivia()?.pieces())?;

    mutation.replace_node_discard_trivia(AnyJsExpression::from(ternary.clone()), new_expr);
    Some(markup! { "Use "<Emphasis>"??"</Emphasis>" instead." }.to_owned())
}

fn fix_if_statement(
    if_stmt: &JsIfStatement,
    assignment_target: &AnyJsAssignmentPattern,
    assignment_value: &AnyJsExpression,
    mutation: &mut BatchMutation<JsLanguage>,
) -> Option<MarkupBuf> {
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

    // Transfer leading trivia from the if-statement (indentation, comments)
    let new_stmt = AnyJsStatement::from(new_stmt)
        .prepend_trivia_pieces(if_stmt.syntax().first_leading_trivia()?.pieces())?
        .append_trivia_pieces(if_stmt.syntax().last_trailing_trivia()?.pieces())?;

    mutation.replace_node_discard_trivia(AnyJsStatement::from(if_stmt.clone()), new_stmt);
    Some(markup! { "Use "<Emphasis>"??="</Emphasis>" instead." }.to_owned())
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

    if should_skip_for_context_options(options, logical.syntax()) {
        return None;
    }

    let left = logical.left().ok()?;
    let left_ty = ctx.type_of_expression(&left);

    if should_skip_for_type(options, &left_ty) {
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

    if should_skip_for_context_options(options, assignment.syntax()) {
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

    if should_skip_for_type(options, &left_ty) {
        return None;
    }

    let can_fix = is_safe_type_for_replacement(&left_ty);

    Some(UseNullishCoalescingState::LogicalOrAssignment {
        operator_range: assignment.operator_token().ok()?.text_trimmed_range(),
        can_fix,
    })
}

/// Check context-based options that depend on the syntax node's position in the tree.
fn should_skip_for_context_options(
    options: &UseNullishCoalescingOptions,
    syntax: &JsSyntaxNode,
) -> bool {
    (options.ignore_mixed_logical_expressions() && is_in_mixed_logical_tree(syntax))
        || (options.ignore_boolean_coercion() && is_in_boolean_call_context(syntax))
}

/// Check type-based options: nullish possibility and primitive ignoring.
fn should_skip_for_type(
    options: &UseNullishCoalescingOptions,
    ty: &biome_js_type_info::Type,
) -> bool {
    if !is_possibly_nullish(ty) {
        return true;
    }
    options.has_any_ignore_primitives() && should_ignore_for_primitives(options, ty)
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
    // Check if the expression is wrapped in parentheses. If it is, the replacement
    // is safe even inside another logical expression because the parens preserve
    // grouping. If it's NOT parenthesized and sits inside another logical expression,
    // the replacement could change precedence, so we skip the fix.
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

/// BFS through the connected tree of `||`/`||=` nodes to find any `&&`
/// operator, matching ESLint's `isMixedLogicalExpression` behavior.
/// Works for both `JsLogicalExpression` (`||`) and `JsAssignmentExpression` (`||=`).
fn is_in_mixed_logical_tree(node: &JsSyntaxNode) -> bool {
    use std::collections::HashSet;

    let mut seen = HashSet::new();
    let mut queue: Vec<JsSyntaxNode> = Vec::new();

    // Seed queue with parent and children of the starting node
    if let Some(parent) = node.parent() {
        queue.push(parent);
    }
    seed_children(node, &mut queue);

    let mut i = 0;
    while i < queue.len() {
        let current = queue[i].clone();
        i += 1;

        if !seen.insert(current.text_trimmed_range()) {
            continue;
        }

        // Walk through parenthesized expressions in both directions
        if let Some(paren) = JsParenthesizedExpression::cast_ref(&current) {
            if let Ok(expr) = paren.expression() {
                queue.push(expr.into_syntax());
            }
            if let Some(parent) = current.parent() {
                queue.push(parent);
            }
            continue;
        }

        if let Some(logical) = JsLogicalExpression::cast_ref(&current)
            && let Ok(op) = logical.operator()
        {
            if op == JsLogicalOperator::LogicalAnd {
                return true;
            }
            if op == JsLogicalOperator::LogicalOr {
                if let Some(parent) = current.parent() {
                    queue.push(parent);
                }
                if let Ok(left) = logical.left() {
                    queue.push(left.into_syntax());
                }
                if let Ok(right) = logical.right() {
                    queue.push(right.into_syntax());
                }
            }
        }
    }
    false
}

/// Add child nodes to the BFS queue based on node type.
fn seed_children(node: &JsSyntaxNode, queue: &mut Vec<JsSyntaxNode>) {
    if let Some(logical) = JsLogicalExpression::cast_ref(node) {
        if let Ok(left) = logical.left() {
            queue.push(left.into_syntax());
        }
        if let Ok(right) = logical.right() {
            queue.push(right.into_syntax());
        }
    } else if let Some(assignment) = JsAssignmentExpression::cast_ref(node)
        && let Ok(right) = assignment.right()
    {
        queue.push(right.into_syntax());
    }
}

/// Returns `true` if the node is inside a `Boolean()` call.
/// Walks through parenthesized and logical expression parents since chained
/// `||` or `||=` inside `Boolean()` should all be suppressed.
fn is_in_boolean_call_context(node: &JsSyntaxNode) -> bool {
    let mut current = node.parent();
    while let Some(node) = current {
        if JsParenthesizedExpression::can_cast(node.kind())
            || JsLogicalExpression::can_cast(node.kind())
        {
            current = node.parent();
            continue;
        }
        if JsCallArgumentList::can_cast(node.kind()) {
            return node
                .parent()
                .and_then(JsCallArguments::cast)
                .and_then(|args| args.parent::<JsCallExpression>())
                .is_some_and(|call| call.has_callee("Boolean"));
        }
        break;
    }
    false
}

/// Returns `true` if the non-nullish type variants are all primitives that
/// should be ignored per the configured `ignorePrimitives` option.
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

fn is_ignored_primitive(
    options: &UseNullishCoalescingOptions,
    ty: &biome_js_type_info::Type,
) -> bool {
    let Some(data) = ty.resolved_data() else {
        return false;
    };
    match data.as_raw_data() {
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
    }
}

fn run_if_statement(
    ctx: &RuleContext<UseNullishCoalescing>,
    if_stmt: &JsIfStatement,
) -> Option<UseNullishCoalescingState> {
    let options = ctx.options();

    if options.ignore_if_statements() {
        return None;
    }

    // Must not have an else branch
    if if_stmt.else_clause().is_some() {
        return None;
    }

    // Extract the single assignment from the consequent
    let assignment = extract_if_body_assignment(if_stmt)?;

    // Left side must be an identifier or member expression
    let left = assignment.left().ok()?;
    if !matches!(
        left.syntax().kind(),
        JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT
            | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT
            | JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT
    ) {
        return None;
    }

    // Parse the test expression to determine the null-check pattern
    let test = if_stmt.test().ok()?;
    let subject = extract_null_check_subject(&test)?;

    // The test subject must match the assignment target (by trimmed text)
    if subject.syntax().text_trimmed() != left.syntax().text_trimmed() {
        return None;
    }

    // Verify the type is nullable (and not ignored by ignorePrimitives)
    let subject_ty = ctx.type_of_expression(&subject);
    if should_skip_for_type(options, &subject_ty) {
        return None;
    }

    let right = assignment.right().ok()?;
    let left = assignment.left().ok()?;

    Some(UseNullishCoalescingState::IfStatement {
        if_range: if_stmt.syntax().text_trimmed_range(),
        assignment_target: left,
        assignment_value: right,
    })
}

/// Extract an assignment expression from the if-statement's consequent.
/// Accepts `=`, `||=`, and `??=` operators.
/// Handles both `if (test) { target = value; }` and `if (test) target = value;`.
fn extract_if_body_assignment(if_stmt: &JsIfStatement) -> Option<JsAssignmentExpression> {
    let consequent = if_stmt.consequent().ok()?;

    let expr_stmt = match &consequent {
        AnyJsStatement::JsBlockStatement(block) => {
            let stmts = block.statements();
            if stmts.len() != 1 {
                return None;
            }
            let first = stmts.into_iter().next()?;
            JsExpressionStatement::cast(first.into_syntax())?
        }
        AnyJsStatement::JsExpressionStatement(expr_stmt) => expr_stmt.clone(),
        _ => return None,
    };

    let expr = expr_stmt.expression().ok()?;
    match expr {
        AnyJsExpression::JsAssignmentExpression(assignment) => {
            let op = assignment.operator().ok()?;
            // Accept `=`, `||=`, and `??=` (the if-statement is redundant for all three)
            if !matches!(
                op,
                JsAssignmentOperator::Assign
                    | JsAssignmentOperator::LogicalOrAssign
                    | JsAssignmentOperator::NullishCoalescingAssign
            ) {
                return None;
            }
            Some(assignment)
        }
        _ => None,
    }
}

/// Extract the subject expression from a null-check test pattern.
/// Returns the expression being tested for nullishness.
///
/// Supported patterns:
/// - `!foo` (truthiness negation)
/// - `foo == null` or `null == foo` (loose null check)
/// - `foo == undefined` or `undefined == foo` (loose undefined check)
/// - `foo === null || foo === undefined` (combined strict check)
fn extract_null_check_subject(test: &AnyJsExpression) -> Option<AnyJsExpression> {
    match test {
        // `!foo` - truthiness negation
        AnyJsExpression::JsUnaryExpression(unary) => {
            if unary.operator().ok()? != JsUnaryOperator::LogicalNot {
                return None;
            }
            let arg = unary.argument().ok()?;
            // argument must be identifier or member expression
            if is_member_access_like_expr(&arg) {
                Some(arg)
            } else {
                None
            }
        }
        // `foo == null`, `foo === null`, `null == foo`, etc.
        AnyJsExpression::JsBinaryExpression(binary) => {
            extract_null_check_from_binary(binary)
        }
        // `foo === null || foo === undefined`
        AnyJsExpression::JsLogicalExpression(logical) => {
            if logical.operator().ok()? != JsLogicalOperator::LogicalOr {
                return None;
            }
            let left = logical.left().ok()?;
            let right = logical.right().ok()?;
            let left_binary = match &left {
                AnyJsExpression::JsBinaryExpression(b) => b,
                _ => return None,
            };
            let right_binary = match &right {
                AnyJsExpression::JsBinaryExpression(b) => b,
                _ => return None,
            };
            // Both sides must use === or ==
            let left_subject = extract_strict_null_or_undefined_subject(left_binary)?;
            let right_subject = extract_strict_null_or_undefined_subject(right_binary)?;
            // Both sides must test the same expression
            if left_subject.syntax().text_trimmed() != right_subject.syntax().text_trimmed() {
                return None;
            }
            Some(left_subject)
        }
        _ => None,
    }
}

/// Extract subject from `foo == null` or `null == foo` (loose or strict).
fn extract_null_check_from_binary(binary: &JsBinaryExpression) -> Option<AnyJsExpression> {
    let op = binary.operator().ok()?;
    match op {
        JsBinaryOperator::Equality | JsBinaryOperator::StrictEquality => {}
        _ => return None,
    }
    let left = binary.left().ok()?;
    let right = binary.right().ok()?;

    if is_null_or_undefined(&right) && is_member_access_like_expr(&left) {
        Some(left)
    } else if is_null_or_undefined(&left) && is_member_access_like_expr(&right) {
        Some(right)
    } else {
        None
    }
}

/// Extract subject from a strict `===` check against null or undefined.
fn extract_strict_null_or_undefined_subject(
    binary: &JsBinaryExpression,
) -> Option<AnyJsExpression> {
    let op = binary.operator().ok()?;
    if op != JsBinaryOperator::StrictEquality {
        return None;
    }
    extract_null_check_from_binary(binary)
}

fn is_null_or_undefined(expr: &AnyJsExpression) -> bool {
    match expr {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNullLiteralExpression(_),
        ) => true,
        AnyJsExpression::JsIdentifierExpression(ident) => {
            ident.name().is_ok_and(|name| name.is_undefined())
        }
        _ => false,
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

    let options = ctx.options();
    let test = ternary.test().ok()?;
    let consequent = ternary.consequent().ok()?;
    let alternate = ternary.alternate().ok()?;

    let (checked_expr, fallback_expr, is_positive, check_kind) =
        check_ternary_nullish_pattern(&test, &consequent, &alternate)?;

    let checked_ty = ctx.type_of_expression(&checked_expr);
    if should_skip_for_type(&options, &checked_ty) {
        return None;
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
