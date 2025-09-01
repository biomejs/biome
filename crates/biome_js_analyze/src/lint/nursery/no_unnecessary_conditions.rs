use crate::services::typed::Typed;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, JsBinaryOperator,
    JsCallExpression, JsCatchClause, JsComputedMemberExpression, JsConditionalExpression,
    JsDoWhileStatement, JsForStatement, JsIfStatement, JsLogicalExpression, JsLogicalOperator,
    JsStaticMemberExpression, JsSwitchStatement, JsWhileStatement, inner_string_text,
};
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
    pub NoUnnecessaryConditions {
        version: "2.1.4",
        name: "noUnnecessaryConditions",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("no-unnecessary-condition").same()],
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Project],
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
        | JsSwitchStatement
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
                        check_comparison_necessity(&left, &right, operator)
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
            AnyConditionLike::JsSwitchStatement(switch_stmt) => {
                let discriminant = switch_stmt.discriminant().ok()?;
                check_condition_necessity(&discriminant, ctx)
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
            )
        }
    }
}

fn check_condition_necessity(
    expr: &AnyJsExpression,
    ctx: &RuleContext<NoUnnecessaryConditions>,
) -> Option<IssueKind> {
    // if the expression is inside a catch, let's not flag it
    let inside_catch = expr
        .syntax()
        .ancestors()
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
        _ => {}
    }

    None
}

fn check_nullish_necessity(
    expr: &AnyJsExpression,
    optional_chain_range: TextRange,
) -> Option<IssueKind> {
    // Only detect obvious literal cases that are never nullish
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

    None
}

fn check_optional_chain_necessity(
    expr: &AnyJsExpression,
    optional_chain_range: TextRange,
) -> Option<IssueKind> {
    // Only detect obvious literal cases that are never nullish
    match expr {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsBooleanLiteralExpression(_)
            | AnyJsLiteralExpression::JsNumberLiteralExpression(_)
            | AnyJsLiteralExpression::JsStringLiteralExpression(_),
        ) => {
            // These literals are never nullish, so ?. is unnecessary
            return Some(IssueKind::UnnecessaryOptionalChain(
                expr.range(),
                optional_chain_range,
            ));
        }
        AnyJsExpression::JsObjectExpression(_) | AnyJsExpression::JsArrayExpression(_) => {
            // These literals are never nullish, so ?. is unnecessary
            return Some(IssueKind::UnnecessaryOptionalChain(
                expr.range(),
                optional_chain_range,
            ));
        }
        _ => {}
    }

    None
}

fn check_comparison_necessity(
    left: &AnyJsExpression,
    right: &AnyJsExpression,
    operator: JsBinaryOperator,
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

    None
}
