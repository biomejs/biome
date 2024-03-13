use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsIfStatement, JsLogicalOperator, JsSyntaxKind};
use biome_rowan::{AstNode, SyntaxNodeCast};

declare_rule! {
    /// Disallow duplicate conditions in if-else-if chains
    ///
    /// if-else-if chains are commonly used when there is a need to execute only one branch
    /// (or at most one branch) out of several possible branches, based on certain conditions.
    ///
    /// Two identical test conditions in the same chain are almost always a mistake in the code.
    /// Unless there are side effects in the expressions,
    /// a duplicate will evaluate to the same true or false value as the identical expression earlier in the chain,
    /// meaning that its branch can never execute.
    ///
    /// Please note that this rule does not compare conditions from the chain with conditions inside statements
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (a) {
    ///     foo();
    /// } else if (b) {
    ///     bar();
    /// } else if (b) {
    ///     baz();
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (a) {
    ///     foo();
    /// } else if (b) {
    ///     bar();
    /// } else if (c) {
    ///     baz();
    /// }
    /// ```
    ///
    pub NoDuplicateElseIf {
        version: "next",
        name: "noDuplicateElseIf",
        recommended: true,
        source: RuleSource::Eslint("no-dupe-else-if"),
    }
}

impl Rule for NoDuplicateElseIf {
    type Query = Ast<JsIfStatement>;
    type State = AnyJsExpression;

    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let expr = node.test().ok()?;
        let mut conditions_to_check: Vec<AnyJsExpression> = vec![];
        conditions_to_check.push(expr.clone());

        if let Some(logical_expression) = expr.as_js_logical_expression() {
            if let Ok(operator_token) = logical_expression.operator() {
                conditions_to_check.append(&mut split_by_logical_operator_wrapper(
                    operator_token,
                    &expr,
                ));
            }
        }
        let mut list_to_check: Vec<Vec<Vec<AnyJsExpression>>> = conditions_to_check
            .iter()
            .map(|c| {
                split_by_logical_operator_wrapper(JsLogicalOperator::LogicalOr, c)
                    .iter()
                    .map(|f| split_by_logical_operator_wrapper(JsLogicalOperator::LogicalAnd, f))
                    .collect()
            })
            .collect();
        let mut current = node.syntax().clone();

        while let Some(grand_parent_node) = current.grand_parent() {
            current = grand_parent_node.clone();

            let ifexpr = match grand_parent_node.kind() {
                JsSyntaxKind::JS_IF_STATEMENT => {
                    grand_parent_node.cast::<JsIfStatement>()?.test().ok()
                }
                _ => {
                    break;
                }
            };
            if let Some(expr) = ifexpr {
                let current_or_operands: Vec<Vec<AnyJsExpression>> =
                    split_by_logical_operator_wrapper(JsLogicalOperator::LogicalOr, &expr)
                        .iter()
                        .map(|f| {
                            split_by_logical_operator_wrapper(JsLogicalOperator::LogicalAnd, f)
                        })
                        .collect();

                list_to_check = list_to_check
                    .iter()
                    .map(|or_operands| {
                        or_operands
                            .iter()
                            .filter(|&or_operand| {
                                !current_or_operands.iter().any(|current_or_operand| {
                                    is_subset(current_or_operand, or_operand)
                                })
                            })
                            .cloned()
                            .collect()
                    })
                    .collect();
                if list_to_check.iter().any(|f| f.is_empty()) {
                    return node.clone().test().ok();
                }
            }
        }

        None
    }

    fn diagnostic(
        _: &RuleContext<Self>,
        expression_inside_if: &Self::State,
    ) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            expression_inside_if.range(),
            markup! {
                "This branch can never execute. Its condition is a duplicate or covered by previous conditions in the if-else-if chain."
            },
        ))
    }
}

fn flatten_parent(node: &AnyJsExpression) -> AnyJsExpression {
    let expression_inside_parent = node
        .as_js_parenthesized_expression()
        .and_then(|parent| parent.expression().ok());

    if let Some(expression) = expression_inside_parent {
        return expression;
    }
    node.clone()
}
fn split_by_logical_operator_wrapper(
    operator: JsLogicalOperator,
    node: &AnyJsExpression,
) -> Vec<AnyJsExpression> {
    let mut result: Vec<AnyJsExpression> = vec![];
    split_by_logical_operator(operator, node, &mut result);
    result
}
fn split_by_logical_operator(
    operator: JsLogicalOperator,
    node: &AnyJsExpression,
    result: &mut Vec<AnyJsExpression>,
) {
    match node {
        AnyJsExpression::JsLogicalExpression(logic_expression) => {
            if let Ok(operator_token) = logic_expression.operator() {
                if operator_token != operator {
                    result.push(node.clone());
                    return;
                }
            }
            if let (Ok(left_node), Ok(right_node)) =
                (logic_expression.left(), logic_expression.right())
            {
                split_by_logical_operator(operator, &flatten_parent(&left_node), result);
                split_by_logical_operator(operator, &flatten_parent(&right_node), result);
            } else {
                result.push(node.clone());
            }
        }
        _ => {
            result.push(node.clone());
        }
    }
}

fn equal(a: &AnyJsExpression, b: &AnyJsExpression) -> bool {
    if a.syntax().kind() != b.syntax().kind() {
        return false;
    }
    if let (Some(a_exp), Some(b_exp)) = (a.as_js_logical_expression(), b.as_js_logical_expression())
    {
        if a_exp.operator() == b_exp.operator()
            && matches!(
                a_exp.operator(),
                Ok(JsLogicalOperator::LogicalAnd | JsLogicalOperator::LogicalOr)
            )
        {
            match (a_exp.left(), a_exp.right(), b_exp.left(), b_exp.right()) {
                (Ok(left_a), Ok(right_a), Ok(left_b), Ok(right_b)) => {
                    return (equal(&left_a, &left_b) && equal(&right_a, &right_b))
                        || (equal(&left_a, &right_b) && equal(&right_a, &left_b));
                }
                _ => return false,
            }
        }
        return false;
    }
    a.text() == b.text()
}

fn is_subset(arr_a: &[AnyJsExpression], arr_b: &[AnyJsExpression]) -> bool {
    arr_a.iter().all(|a| arr_b.iter().any(|b| equal(a, b)))
}
