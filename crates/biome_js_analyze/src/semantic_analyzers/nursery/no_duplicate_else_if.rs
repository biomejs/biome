use crate::semantic_services::Semantic;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsElseClause, JsIfStatement, JsLogicalOperator, TextRange};
use biome_rowan::AstNode;

declare_rule! {
    /// Disallow duplicate conditions in if-else-if chains
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-dupe-else-if
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (isA(x)) {
    ///     foo();
    /// } else if (isA(x)) {
    ///     bar();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (n === 1) {
    ///     foo();
    /// } else if (n === 2) {
    ///     bar();
    /// } else if (n === 3) {
    ///     baz();
    /// } else if (n === 2) {
    ///     quux();
    /// } else if (n === 5) {
    ///     quuux();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (a && b) {
    ///     foo();
    /// } else if (a && b && c) {
    ///     bar();
    /// }
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js,expect_diagnostic
    /// if (isA(x)) {
    ///     foo();
    /// } else if (isB(x)) {
    ///     bar();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (n === 1) {
    ///     foo();
    /// } else if (n === 2) {
    ///     bar();
    /// } else if (n === 3) {
    ///     baz();
    /// } else if (n === 4) {
    ///     quux();
    /// } else if (n === 5) {
    ///     quuux();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (a && b) {
    ///     foo();
    /// } else if (a || b && c) {
    ///     bar();
    /// }
    /// ```
    ///
    pub(crate) NoDuplicateElseIf {
        version: "next",
        name: "noDuplicateElseIf",
        recommended: false,
    }
}

impl Rule for NoDuplicateElseIf {
    type Query = Semantic<JsIfStatement>;
    type State = TextRange;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut arguments = Vec::new();

        if let Ok(test) = node.test() {
            if test.as_js_call_expression().is_some()
                || test.as_js_logical_expression().is_some()
                || test.as_js_identifier_expression().is_some()
            {
                arguments.push(test);
            }
        };

        if let Some(else_clause) = node.else_clause() {
            extract_else_clause(else_clause, &mut arguments);
        }

        let mut result = Vec::new();
        for i in 0..arguments.len() {
            for j in i + 1..arguments.len() {
                if is_condition_covered(&arguments[j], &arguments[0..i + 1]) {
                    result.push(arguments[j].range().clone());
                    break;
                }
            }
        }

        result
    }

    fn diagnostic(_: &RuleContext<Self>, reference: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                reference,
                markup! {
                    "Duplicate else-if condition"
                },
            )
            .note(markup! {
                "This branch can never execute. Its condition is a duplicate or covered by previous conditions in the if-else-if chain"
            }),
        )
    }
}

fn extract_else_clause(else_clause: JsElseClause, arguments: &mut Vec<AnyJsExpression>) {
    if let Ok(alternate) = else_clause.alternate() {
        if let Some(js_if_statement) = alternate.as_js_if_statement() {
            if let Ok(test) = js_if_statement.test() {
                if let Some(_) = test.as_js_call_expression() {
                    arguments.push(test);
                } else if let Some(logical_exp) = test.as_js_logical_expression() {
                    if logical_exp.syntax().text().contains_char('|')
                        || logical_exp.syntax().text().contains_char('&')
                    {
                        let result = split_logical_expression(
                            &AnyJsExpression::JsLogicalExpression(logical_exp.clone()),
                        );
                        for r in result {
                            arguments.push(r);
                        }
                    }
                } else if let Some(_) = test.as_js_binary_expression() {
                    arguments.push(test);
                } else if let Some(_) = test.as_js_identifier_expression() {
                    arguments.push(test);
                }

                if let Some(else_clause) = js_if_statement.else_clause() {
                    extract_else_clause(else_clause, arguments);
                }
            }
        }
    }
}

fn is_condition_covered(new_cond: &AnyJsExpression, existing_conds: &[AnyJsExpression]) -> bool {
    let new_cond_parts = split_logical_expression(new_cond);
    for existing_cond in existing_conds {
        let existing_cond_parts = split_logical_expression(existing_cond);

        if is_subset(&new_cond_parts, &existing_cond_parts) {
            return true;
        }
    }

    false
}

fn split_logical_expression(expr: &AnyJsExpression) -> Vec<AnyJsExpression> {
    let mut parts = Vec::new();

    recursively_split_expression(expr, &mut parts);
    parts
}

fn recursively_split_expression(expr: &AnyJsExpression, parts: &mut Vec<AnyJsExpression>) {
    match expr {
        AnyJsExpression::JsLogicalExpression(logical_expr) => match logical_expr.operator() {
            Ok(JsLogicalOperator::LogicalAnd) | Ok(JsLogicalOperator::LogicalOr) => {
                recursively_split_expression(&logical_expr.left().unwrap(), parts);
                recursively_split_expression(&logical_expr.right().unwrap(), parts);
            }
            _ => parts.push(expr.clone()),
        },
        AnyJsExpression::JsParenthesizedExpression(paren_expr) => {
            recursively_split_expression(&paren_expr.expression().unwrap(), parts);
        }
        _ => parts.push(expr.clone()),
    }
}

fn is_subset(subset: &[AnyJsExpression], set: &[AnyJsExpression]) -> bool {
    subset.iter().all(|sub_item| {
        set.iter()
            .any(|set_item| are_expressions_equal(sub_item, set_item))
            || (sub_item.as_js_logical_expression().is_some() && is_logical_superset(sub_item, set))
    })
}

//fn is_subset(subset: &[AnyJsExpression], set: &[AnyJsExpression]) -> bool {
//    subset.iter().all(|sub_item| {
//        if let Some(_) = sub_item.as_js_logical_expression() {
//            set.iter()
//                .any(|set_item| are_expressions_equal(sub_item, set_item))
//                || is_logical_superset(sub_item, set)
//        } else {
//            set.iter()
//                .any(|set_item| are_expressions_equal(sub_item, set_item))
//        }
//    })
//}

fn is_logical_superset(sub_item: &AnyJsExpression, set: &[AnyJsExpression]) -> bool {
    if let Some(logical_expr) = sub_item.as_js_logical_expression() {
        match logical_expr.operator() {
            Ok(JsLogicalOperator::LogicalOr) => {
                let left = logical_expr.left().unwrap();
                let right = logical_expr.right().unwrap();
                set.iter().any(|item| {
                    are_expressions_equal(&left, item) || are_expressions_equal(&right, item)
                })
            }
            Ok(JsLogicalOperator::LogicalAnd) => {
                let left = logical_expr.left().unwrap();
                let right = logical_expr.right().unwrap();
                set.iter().any(|item| are_expressions_equal(&left, item))
                    && set.iter().any(|item| are_expressions_equal(&right, item))
            }
            _ => false,
        }
    } else {
        false
    }
}

fn are_expressions_equal(expr1: &AnyJsExpression, expr2: &AnyJsExpression) -> bool {
    if let (Some(logical1), Some(logical2)) = (
        expr1.as_js_logical_expression(),
        expr2.as_js_logical_expression(),
    ) {
        if logical1.operator() != logical2.operator() {
            return false;
        }
        return are_expressions_equal(&logical1.left().unwrap(), &logical2.left().unwrap())
            && are_expressions_equal(&logical1.right().unwrap(), &logical2.right().unwrap());
    }

    expr1.syntax().text() == expr2.syntax().text()
}
