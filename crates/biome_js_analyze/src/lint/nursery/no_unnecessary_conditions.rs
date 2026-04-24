use crate::services::typed::Typed;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, JsBinaryOperator,
    JsCallExpression, JsCaseClause, JsCatchClause, JsComputedMemberExpression,
    JsConditionalExpression, JsDoWhileStatement, JsForStatement, JsIfStatement,
    JsLogicalExpression, JsLogicalOperator, JsStaticMemberExpression, JsSwitchStatement,
    JsUnaryOperator, JsWhileStatement, inner_string_text,
};
use biome_js_type_info::Type;
use biome_rowan::{AstNode, TextRange, declare_node_union};
use biome_rule_options::no_unnecessary_conditions::NoUnnecessaryConditionsOptions;

declare_lint_rule! {
    /// Disallow unnecessary type-based conditions that can be statically determined as redundant.
    ///
    /// This rule detects if expressions inside conditions are statically inferrable and yield
    /// falsy or truthy values that don't change during the life cycle of the program.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts
    /// function head<T>(items: T[]) {
    ///   if (items) {  // This check is unnecessary
    ///     return items[0].toUpperCase();
    ///   }
    /// }
    /// ```
    ///
    /// ```ts
    /// function foo(arg: 'bar' | 'baz') {
    ///   if (arg) {  // This check is unnecessary
    ///   }
    /// }
    /// ```
    ///
    /// ```ts
    /// function bar(arg: string) {
    ///   return arg?.length;  // ?. is unnecessary
    /// }
    /// ```
    ///
    /// ```ts
    /// interface Foo { items: string[] }
    /// function f(x: Foo) {
    ///   return x.items || [];  // x.items is string[], always truthy
    /// }
    /// ```
    ///
    /// ```ts
    /// const a = [];
    /// if (!a) {}  // always false — array literals are always truthy
    /// ```
    ///
    /// ```ts
    /// function f(x: string) {
    ///   return x === null;  // always false — x is never null
    /// }
    /// ```
    ///
    /// ```ts
    /// function f(v: 'a' | 'b') {
    ///   switch (v) { case 'c': return 1; }  // 'c' is unreachable
    /// }
    /// ```
    ///
    /// Contrary to the source rule, this rule doesn't trigger bindings that are assigned to multiple
    /// values. In the following example, the variable `greeting` is assigned to multiple values; hence
    /// it can't be inferred to a truthy or falsy value.
    ///
    /// ```ts
    /// let greeting = false;
    ///
    /// function changeGreeting() {
    ///     greeting = "Hello World!"
    /// }
    ///
    /// if (greeting) {} // rule not triggered here
    ///
    /// ```
    ///
    ///
    /// ### Valid
    ///
    /// ```ts
    /// function head<T>(items: T[] | null) {
    ///   if (items) {  // This check is necessary
    ///     return items[0].toUpperCase();
    ///   }
    /// }
    /// ```
    ///
    /// ```ts
    /// function foo(arg: 'bar' | 'baz' | null) {
    ///   if (arg) {  // This check is necessary
    ///   }
    /// }
    /// ```
    ///
    /// ```ts
    /// function bar(arg: string | undefined) {
    ///   return arg?.length;  // ?. is necessary
    /// }
    /// ```
    ///
    /// ```ts
    /// function f(v: 'a' | 'b' | 'c') {
    ///   switch (v) {
    ///     case 'a': break;
    ///     case 'b': break;
    ///     case 'c': break;  // all cases reachable — no flag
    ///   }
    /// }
    /// ```
    ///
    pub NoUnnecessaryConditions {
        version: "2.1.4",
        name: "noUnnecessaryConditions",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("no-unnecessary-condition").inspired()],
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Types],
        issue_number: Some("6611"),
    }
}

declare_node_union! {
    pub AnyConditionLike =
        JsIfStatement
        | JsConditionalExpression
        | JsLogicalExpression
        | JsStaticMemberExpression
        | JsComputedMemberExpression
        | JsCallExpression
        | JsBinaryExpression
        | JsWhileStatement
        | JsDoWhileStatement
        | JsForStatement
        | JsCaseClause
}

pub enum IssueKind {
    /// An if condition that is always truthy
    AlwaysTruthyCondition(TextRange),
    /// An if condition that is always falsy
    AlwaysFalsyCondition(TextRange),
    /// Optional chaining used on a non-nullish type
    UnnecessaryOptionalChain(TextRange, TextRange),
    /// The operator `??` isn't needed
    UnnecessaryCoalescing(TextRange, TextRange),
    /// A binary comparison that will always have the same result
    UnnecessaryComparison(TextRange),
    /// A `case` clause whose test value can never equal the switch discriminant.
    UnreachableCase(TextRange),
}

impl Rule for NoUnnecessaryConditions {
    type Query = Typed<AnyConditionLike>;
    type State = IssueKind;
    type Signals = Option<Self::State>;
    type Options = NoUnnecessaryConditionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        match query {
            AnyConditionLike::JsIfStatement(if_stmt) => {
                let test = if_stmt.test().ok()?;
                check_condition_necessity(&test, ctx)
            }
            AnyConditionLike::JsConditionalExpression(cond_expr) => {
                let test = cond_expr.test().ok()?;
                check_condition_necessity(&test, ctx)
            }
            AnyConditionLike::JsLogicalExpression(log_expr) => {
                let operator = log_expr.operator().ok()?;
                let left = log_expr.left().ok()?;

                match operator {
                    JsLogicalOperator::LogicalAnd | JsLogicalOperator::LogicalOr => {
                        check_condition_necessity(&left, ctx)
                    }
                    JsLogicalOperator::NullishCoalescing => check_nullish_necessity(
                        &left,
                        log_expr.operator_token().ok()?.text_trimmed_range(),
                        ctx,
                    ),
                }
            }
            AnyConditionLike::JsStaticMemberExpression(member_expr) => {
                // Check if this uses optional chaining unnecessarily
                if member_expr.is_optional_chain() {
                    let object = member_expr.object().ok()?;
                    check_optional_chain_necessity(
                        &object,
                        member_expr.operator_token().ok()?.text_trimmed_range(),
                        ctx,
                    )
                } else {
                    None
                }
            }
            AnyConditionLike::JsComputedMemberExpression(member_expr) => {
                // Check if this uses optional chaining unnecessarily
                if member_expr.is_optional_chain() {
                    let object = member_expr.object().ok()?;
                    check_optional_chain_necessity(
                        &object,
                        member_expr.optional_chain_token()?.text_trimmed_range(),
                        ctx,
                    )
                } else {
                    None
                }
            }
            AnyConditionLike::JsCallExpression(call_expr) => {
                // Check if this uses optional chaining unnecessarily
                if call_expr.is_optional_chain() {
                    let callee = call_expr.callee().ok()?;
                    check_optional_chain_necessity(
                        &callee,
                        call_expr.optional_chain_token()?.text_trimmed_range(),
                        ctx,
                    )
                } else {
                    None
                }
            }
            AnyConditionLike::JsBinaryExpression(bin_expr) => {
                let operator = bin_expr.operator().ok()?;
                match operator {
                    JsBinaryOperator::Equality
                    | JsBinaryOperator::Inequality
                    | JsBinaryOperator::StrictEquality
                    | JsBinaryOperator::StrictInequality
                    | JsBinaryOperator::LessThan
                    | JsBinaryOperator::GreaterThan
                    | JsBinaryOperator::LessThanOrEqual
                    | JsBinaryOperator::GreaterThanOrEqual => {
                        let left = bin_expr.left().ok()?;
                        let right = bin_expr.right().ok()?;
                        check_comparison_necessity(&left, &right, operator, ctx)
                    }
                    _ => None,
                }
            }
            AnyConditionLike::JsWhileStatement(while_stmt) => {
                let test = while_stmt.test().ok()?;
                check_condition_necessity(&test, ctx)
            }
            AnyConditionLike::JsDoWhileStatement(do_while_stmt) => {
                let test = do_while_stmt.test().ok()?;
                check_condition_necessity(&test, ctx)
            }
            AnyConditionLike::JsForStatement(for_stmt) => {
                let test = for_stmt.test()?;
                check_condition_necessity(&test, ctx)
            }
            AnyConditionLike::JsCaseClause(case_clause) => {
                check_case_clause_reachability(case_clause, ctx)
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            IssueKind::AlwaysTruthyCondition(range) => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "This condition is always truthy based on the type."
                    },
                )
                .note(markup! {
                    "The type being checked can never be falsy, making this condition redundant."
                })
                .note(markup!{
                    "Remove the condition."
                }),
            ),
            IssueKind::AlwaysFalsyCondition(range) => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "This condition is always falsy based on the type."
                    },
                )
                .note(markup! {
                    "The type being checked can never be truthy, making this condition redundant."
                }).note(markup!{
                    "Remove the condition."
                }),
            ),
            IssueKind::UnnecessaryOptionalChain(node_range, operator_range) => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node_range,
                    markup! {
                        "Optional chaining is unnecessary for this type."
                    },
                )
                .note(markup! {
                    "The type being accessed is guaranteed to be non-nullish, making optional chaining redundant."
                }).detail(operator_range,markup!{

                    "Remove the optional chaining."
                }),
            ),
            IssueKind::UnnecessaryComparison(range) => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "This comparison will always have the same result."
                    },
                )
                .note(markup! {
                    "Based on the types being compared, this condition is redundant."
                }).note(markup!{
                    "Remove the comparison."
                }),
            ),
            IssueKind::UnnecessaryCoalescing(node_range, operator_range) => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node_range,
                    markup! {
                        "Coalescing is unnecessary for this type."
                    },
                )
                .note(markup! {
                    "The type being accessed is guaranteed to be non-nullish, making coalescing redundant."
                }).detail(operator_range,markup!{
                    "This is a nullish coalescing operator, which is unnecessary."
                })
            ),
            IssueKind::UnreachableCase(range) => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "This case is unreachable."
                    },
                )
                .note(markup! {
                    "The discriminant's type can never equal this value."
                })
                .note(markup! {
                    "Remove the case, or change the discriminant."
                }),
            ),
        }
    }
}

/// Returns `true` if `expr` is a static member, computed member, or call
/// expression that uses optional chaining (`?.`). Those are handled by a
/// dedicated `AnyConditionLike` arm and must be skipped here to avoid emitting
/// the wrong diagnostic kind.
fn is_optional_chain_expr(expr: &AnyJsExpression) -> bool {
    match expr {
        AnyJsExpression::JsStaticMemberExpression(m) => m.is_optional_chain(),
        AnyJsExpression::JsComputedMemberExpression(m) => m.is_optional_chain(),
        AnyJsExpression::JsCallExpression(c) => c.is_optional_chain(),
        _ => false,
    }
}

/// Returns `true` if `expr` is the identifier `undefined`.
fn is_undefined_identifier(expr: &AnyJsExpression) -> bool {
    matches!(expr, AnyJsExpression::JsIdentifierExpression(id)
        if id.name().ok()
            .and_then(|n| n.value_token().ok())
            .is_some_and(|t| t.text_trimmed() == "undefined"))
}

fn check_condition_necessity(
    expr: &AnyJsExpression,
    ctx: &RuleContext<NoUnnecessaryConditions>,
) -> Option<IssueKind> {
    // if the expression is inside a catch, let's not flag it
    let inside_catch = expr
        .syntax()
        .ancestors()
        .skip(1)
        .any(|ancestor| JsCatchClause::can_cast(ancestor.kind()));

    if inside_catch {
        return None;
    }

    // Only detect obvious literal cases to avoid false positives
    match expr {
        AnyJsExpression::AnyJsLiteralExpression(literal_expr) => match literal_expr {
            AnyJsLiteralExpression::JsBooleanLiteralExpression(bool_expr) => {
                if let Ok(literal) = bool_expr.value_token() {
                    if literal.text_trimmed() == "true" {
                        return Some(IssueKind::AlwaysTruthyCondition(expr.range()));
                    } else if literal.text_trimmed() == "false" {
                        return Some(IssueKind::AlwaysFalsyCondition(expr.range()));
                    }
                }
            }
            AnyJsLiteralExpression::JsNumberLiteralExpression(num_expr) => {
                if let Ok(literal) = num_expr.value_token()
                    && let Ok(value) = literal.text_trimmed().parse::<f64>()
                {
                    if value != 0.0 && !value.is_nan() {
                        return Some(IssueKind::AlwaysTruthyCondition(expr.range()));
                    } else if value == 0.0 {
                        return Some(IssueKind::AlwaysFalsyCondition(expr.range()));
                    }
                }
            }
            AnyJsLiteralExpression::JsStringLiteralExpression(str_expr) => {
                if let Ok(literal) = str_expr.value_token() {
                    return if inner_string_text(&literal).is_empty() {
                        Some(IssueKind::AlwaysFalsyCondition(expr.range()))
                    } else {
                        Some(IssueKind::AlwaysTruthyCondition(expr.range()))
                    };
                }
            }
            AnyJsLiteralExpression::JsNullLiteralExpression(_) => {
                return Some(IssueKind::AlwaysFalsyCondition(expr.range()));
            }
            _ => {}
        },
        AnyJsExpression::JsObjectExpression(_) => {
            // Object literals are always truthy
            return Some(IssueKind::AlwaysTruthyCondition(expr.range()));
        }
        AnyJsExpression::JsArrayExpression(_) => {
            // Array literals are always truthy
            return Some(IssueKind::AlwaysTruthyCondition(expr.range()));
        }
        AnyJsExpression::JsIdentifierExpression(id_expr) => {
            if let Ok(name) = id_expr.name()
                && name.value_token().ok()?.text_trimmed() == "undefined"
            {
                return Some(IssueKind::AlwaysFalsyCondition(expr.range()));
            }

            // Use type inference to check if this identifier always refers to a truthy/falsy value
            let ty = ctx.type_of_expression(expr);
            let conditional = ty.conditional_semantics();
            if conditional.is_truthy() {
                return Some(IssueKind::AlwaysTruthyCondition(expr.range()));
            } else if conditional.is_falsy() {
                return Some(IssueKind::AlwaysFalsyCondition(expr.range()));
            }
        }
        AnyJsExpression::JsUnaryExpression(unary_expr) => {
            // Handle `!expr` — the truthiness of !expr is the inverse of expr's.
            let Ok(JsUnaryOperator::LogicalNot) = unary_expr.operator() else {
                return None;
            };
            let argument = unary_expr.argument().ok()?;
            return match check_condition_necessity(&argument, ctx)? {
                IssueKind::AlwaysTruthyCondition(_) => {
                    Some(IssueKind::AlwaysFalsyCondition(expr.range()))
                }
                IssueKind::AlwaysFalsyCondition(_) => {
                    Some(IssueKind::AlwaysTruthyCondition(expr.range()))
                }
                // Other kinds (UnnecessaryOptionalChain, UnnecessaryCoalescing,
                // UnnecessaryComparison) don't invert meaningfully — drop them here.
                _ => None,
            };
        }
        AnyJsExpression::JsStaticMemberExpression(_)
        | AnyJsExpression::JsComputedMemberExpression(_)
        | AnyJsExpression::JsCallExpression(_) => {
            // Skip optional chains — those are handled separately and would
            // produce the wrong diagnostic kind.
            if is_optional_chain_expr(expr) {
                return None;
            }

            let ty = ctx.type_of_expression(expr);
            let conditional = ty.conditional_semantics();
            if conditional.is_truthy() {
                return Some(IssueKind::AlwaysTruthyCondition(expr.range()));
            } else if conditional.is_falsy() {
                return Some(IssueKind::AlwaysFalsyCondition(expr.range()));
            }
        }
        _ => {}
    }

    None
}

fn check_nullish_necessity(
    expr: &AnyJsExpression,
    optional_chain_range: TextRange,
    ctx: &RuleContext<NoUnnecessaryConditions>,
) -> Option<IssueKind> {
    // Literal fast-path (existing behavior)
    match expr {
        AnyJsExpression::AnyJsLiteralExpression(literal_expr) => {
            match literal_expr {
                AnyJsLiteralExpression::JsBooleanLiteralExpression(_)
                | AnyJsLiteralExpression::JsNumberLiteralExpression(_)
                | AnyJsLiteralExpression::JsStringLiteralExpression(_) => {
                    // These literals are never nullish, so ?? is unnecessary
                    return Some(IssueKind::UnnecessaryCoalescing(
                        expr.range(),
                        optional_chain_range,
                    ));
                }
                AnyJsLiteralExpression::JsNullLiteralExpression(value) => {
                    // The presence of `null` makes the logical expression nullish
                    return Some(IssueKind::UnnecessaryCoalescing(
                        expr.range(),
                        value.value_token().ok()?.text_trimmed_range(),
                    ));
                }

                AnyJsLiteralExpression::JsBigintLiteralExpression(_) => {}
                AnyJsLiteralExpression::JsRegexLiteralExpression(_) => {}
            }
        }
        AnyJsExpression::JsObjectExpression(_) | AnyJsExpression::JsArrayExpression(_) => {
            // These literals are never nullish, so ?? is unnecessary
            return Some(IssueKind::UnnecessaryCoalescing(
                expr.range(),
                optional_chain_range,
            ));
        }
        _ => {}
    }

    // Type-aware path: flag when the left-hand side is statically non-nullish.
    let ty = ctx.type_of_expression(expr);
    if ty.conditional_semantics().is_non_nullish() {
        return Some(IssueKind::UnnecessaryCoalescing(
            expr.range(),
            optional_chain_range,
        ));
    }

    None
}

fn check_optional_chain_necessity(
    expr: &AnyJsExpression,
    optional_chain_range: TextRange,
    ctx: &RuleContext<NoUnnecessaryConditions>,
) -> Option<IssueKind> {
    // Literal fast-path (existing behavior)
    match expr {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsBooleanLiteralExpression(_)
            | AnyJsLiteralExpression::JsNumberLiteralExpression(_)
            | AnyJsLiteralExpression::JsStringLiteralExpression(_),
        )
        | AnyJsExpression::JsObjectExpression(_)
        | AnyJsExpression::JsArrayExpression(_) => {
            return Some(IssueKind::UnnecessaryOptionalChain(
                expr.range(),
                optional_chain_range,
            ));
        }
        _ => {}
    }

    // Type-aware path: flag when the object is statically non-nullish.
    let ty = ctx.type_of_expression(expr);
    if ty.conditional_semantics().is_non_nullish() {
        return Some(IssueKind::UnnecessaryOptionalChain(
            expr.range(),
            optional_chain_range,
        ));
    }

    None
}

fn check_comparison_necessity(
    left: &AnyJsExpression,
    right: &AnyJsExpression,
    operator: JsBinaryOperator,
    ctx: &RuleContext<NoUnnecessaryConditions>,
) -> Option<IssueKind> {
    // Only detect obvious literal comparisons to avoid false positives
    match (left, right) {
        // Boolean literal comparisons
        (
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsBooleanLiteralExpression(left_bool),
            ),
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsBooleanLiteralExpression(right_bool),
            ),
        ) => {
            if let (Ok(_left_token), Ok(_right_token)) =
                (left_bool.value_token(), right_bool.value_token())
            {
                match operator {
                    JsBinaryOperator::Equality
                    | JsBinaryOperator::StrictEquality
                    | JsBinaryOperator::Inequality
                    | JsBinaryOperator::StrictInequality => {
                        // Any comparison between two boolean literals is statically determinable
                        return Some(IssueKind::UnnecessaryComparison(TextRange::new(
                            left.range().start(),
                            right.range().end(),
                        )));
                    }
                    _ => {}
                }
            }
        }
        // Number literal comparisons
        (
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(left_num),
            ),
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(right_num),
            ),
        ) => {
            if let (Ok(left_token), Ok(right_token)) =
                (left_num.value_token(), right_num.value_token())
                && let (Ok(left_val), Ok(right_val)) = (
                    left_token.text_trimmed().parse::<u64>(),
                    right_token.text_trimmed().parse::<u64>(),
                )
            {
                match operator {
                    JsBinaryOperator::LessThan => {
                        if left_val < right_val {
                            return Some(IssueKind::UnnecessaryComparison(TextRange::new(
                                left.range().start(),
                                right.range().end(),
                            )));
                        }
                    }
                    JsBinaryOperator::GreaterThan => {
                        if left_val > right_val {
                            return Some(IssueKind::UnnecessaryComparison(TextRange::new(
                                left.range().start(),
                                right.range().end(),
                            )));
                        }
                    }
                    JsBinaryOperator::GreaterThanOrEqual => {
                        if left_val >= right_val {
                            return Some(IssueKind::UnnecessaryComparison(TextRange::new(
                                left.range().start(),
                                right.range().end(),
                            )));
                        }
                    }
                    JsBinaryOperator::LessThanOrEqual => {
                        if left_val <= right_val {
                            return Some(IssueKind::UnnecessaryComparison(TextRange::new(
                                left.range().start(),
                                right.range().end(),
                            )));
                        }
                    }
                    JsBinaryOperator::Equality
                    | JsBinaryOperator::StrictEquality
                    | JsBinaryOperator::Inequality
                    | JsBinaryOperator::StrictInequality => {
                        // Any comparison between two literal numbers is statically determinable
                        return Some(IssueKind::UnnecessaryComparison(TextRange::new(
                            left.range().start(),
                            right.range().end(),
                        )));
                    }
                    _ => {}
                }
            }
        }
        // String literal comparisons
        (
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(_),
            ),
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(_),
            ),
        ) => {
            match operator {
                JsBinaryOperator::Equality
                | JsBinaryOperator::StrictEquality
                | JsBinaryOperator::Inequality
                | JsBinaryOperator::StrictInequality => {
                    // Any comparison between two string literals is statically determinable
                    return Some(IssueKind::UnnecessaryComparison(TextRange::new(
                        left.range().start(),
                        right.range().end(),
                    )));
                }
                _ => {}
            }
        }
        _ => {}
    }

    // Type-aware null/undefined comparison.
    // Only runs when the existing literal-only branches didn't match.
    let typed_side = match (left, right) {
        (
            AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsNullLiteralExpression(_)),
            other,
        )
        | (
            other,
            AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsNullLiteralExpression(_)),
        ) => other,
        _ if is_undefined_identifier(right) => left,
        _ if is_undefined_identifier(left) => right,
        _ => return None,
    };

    let ty = ctx.type_of_expression(typed_side);
    let conditional = ty.conditional_semantics();

    // Is the non-null side known to be non-nullish? Then the comparison is unnecessary.
    if conditional.is_non_nullish() {
        let range = TextRange::new(left.range().start(), right.range().end());
        return Some(IssueKind::UnnecessaryComparison(range));
    }

    // Dual: if the typed side is known to BE nullish (e.g. the `undefined` identifier).
    // Note: this narrow pass does not detect `void 0` because it's a JsUnaryExpression,
    // not an identifier.
    if conditional.is_nullish() {
        let range = TextRange::new(left.range().start(), right.range().end());
        return Some(IssueKind::UnnecessaryComparison(range));
    }

    None
}

/// A literal value extracted from a `case <literal>:` clause.
enum CaseLiteral {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

/// Extracts the literal value from a `case X:` test expression, if the test is
/// a simple literal (string, number, boolean, or `null`). Returns `None` for
/// anything else so that the rule doesn't try to evaluate identifiers, member
/// accesses, arithmetic, etc.
fn extract_case_literal(test: &AnyJsExpression) -> Option<CaseLiteral> {
    let AnyJsExpression::AnyJsLiteralExpression(lit_expr) = test else {
        return None;
    };
    match lit_expr {
        AnyJsLiteralExpression::JsStringLiteralExpression(s) => {
            let token = s.value_token().ok()?;
            Some(CaseLiteral::String(inner_string_text(&token).text().to_string()))
        }
        AnyJsLiteralExpression::JsNumberLiteralExpression(n) => {
            // `as_number` handles hex/binary/octal literals and numeric separators.
            Some(CaseLiteral::Number(n.as_number()?))
        }
        AnyJsLiteralExpression::JsBooleanLiteralExpression(b) => {
            let token = b.value_token().ok()?;
            Some(CaseLiteral::Boolean(token.text_trimmed() == "true"))
        }
        AnyJsLiteralExpression::JsNullLiteralExpression(_) => Some(CaseLiteral::Null),
        _ => None,
    }
}

/// Returns whether the given `ty` could possibly equal the given literal value.
///
/// This is a narrow, conservative predicate — when in doubt (unknown types,
/// unresolved references, type variables, complex types), it returns `true`.
/// Used to determine case-clause reachability: if this returns `false`, the
/// `case` is statically guaranteed to never match.
fn type_could_equal_literal(ty: &Type, literal: &CaseLiteral) -> bool {
    if ty.is_union() {
        return ty
            .flattened_union_variants()
            .any(|variant| single_type_could_equal_literal(&variant, literal));
    }
    single_type_could_equal_literal(ty, literal)
}

fn single_type_could_equal_literal(ty: &Type, literal: &CaseLiteral) -> bool {
    use biome_js_type_info::TypeData;

    let raw = &**ty;

    // Unknown / any / inference failures: always treat as "possibly equal".
    if matches!(
        raw,
        TypeData::Unknown | TypeData::AnyKeyword | TypeData::UnknownKeyword
    ) {
        return true;
    }

    match literal {
        CaseLiteral::String(s) => {
            // `string` could be any string.
            if ty.is_string_or_string_literal() {
                // A plain `string` (non-literal) could equal any value.
                if matches!(raw, TypeData::String) {
                    return true;
                }
                // Otherwise it's a string literal — match on exact value.
                return ty.is_string_literal(s.as_str());
            }
            false
        }
        CaseLiteral::Number(n) => {
            if ty.is_number_or_number_literal() {
                if matches!(raw, TypeData::Number) {
                    return true;
                }
                return ty.is_number_literal(*n);
            }
            false
        }
        CaseLiteral::Boolean(b) => {
            if matches!(raw, TypeData::Boolean) {
                return true;
            }
            ty.is_boolean_literal(*b)
        }
        CaseLiteral::Null => {
            // Conservative: we also treat Undefined and VoidKeyword as possible
            // matches for a null-literal case, to avoid flagging ambiguous
            // narrow-pass cases. Strictly, `null === undefined` is false and
            // `case null:` inside `switch (x: undefined)` IS unreachable — but
            // detecting that requires distinguishing === semantics we do not yet
            // model. TODO: tighten to only match TypeData::Null when
            // disjointness becomes reliable.
            matches!(raw, TypeData::Null | TypeData::Undefined | TypeData::VoidKeyword)
        }
    }
}

/// Checks whether a `case <literal>:` clause is reachable given the type of
/// the enclosing `switch`'s discriminant. Mirrors the behavior of
/// `@typescript-eslint/no-unnecessary-condition`, which flags each case that
/// can never equal the discriminant.
fn check_case_clause_reachability(
    case_clause: &JsCaseClause,
    ctx: &RuleContext<NoUnnecessaryConditions>,
) -> Option<IssueKind> {
    let test = case_clause.test().ok()?;
    let case_literal = extract_case_literal(&test)?;

    // Find the enclosing switch statement.
    let switch_stmt = case_clause
        .syntax()
        .ancestors()
        .find_map(JsSwitchStatement::cast)?;
    let discriminant = switch_stmt.discriminant().ok()?;

    let discriminant_ty = ctx.type_of_expression(&discriminant);
    if type_could_equal_literal(&discriminant_ty, &case_literal) {
        None
    } else {
        Some(IssueKind::UnreachableCase(test.range()))
    }
}
