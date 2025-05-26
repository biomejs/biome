use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, RuleSourceKind, RuleSourceWithKind,
    context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, AnyJsStatement, JsCallExpression, JsStatementList};
use biome_rowan::{AstNode, AstNodeList};

declare_lint_rule! {
    /// Require each test function (`test()`, `it()`) to have an assertion (`expect()`, `assert()`, etc.).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// test('myLogic', () => {
    ///   console.log('myLogic');
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test('myLogic', () => {});
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,expect_diagnostic
    /// test('myLogic', () => {
    ///   const actual = myLogic();
    ///   expect(actual).toBe(true);
    /// });
    /// ```
    pub UseExplicitTestAssertions {
        version: "next",
        name: "useExplicitTestAssertions",
        language: "js",
        sources: &[RuleSourceWithKind::EslintJest("expect-expect").inspired()],
        recommended: false,
        domains: &[RuleDomain::Test],
    }
}

const ASSERTION_FUNCTION_NAMES: [&str; 3] = ["assert", "assertEquals", "expect"];

impl Rule for UseExplicitTestAssertions {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if !node.is_test_call_expression().ok()? {
            return None;
        }

        if let Ok(args) = node.arguments() {
            if let [Some(second)] = args.get_arguments_by_index([1]) {
                if let Some(test_body) = second.as_any_js_expression() {
                    if expression_contains_expect(test_body) {
                        return None;
                    }
                }
            }
        }
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "Missing assertion in test body."
                },
            )
            .note(markup! {
                "This could cause false positives where the test always passes but isn’t testing anything."
            }).note(markup! {
                "Add an expect() (Vitest/Jest) or assert() (node:assert) assertion to this test."
            }),
        )
    }
}

/// Is this an expect() or assert() call?
fn is_test_assertion_expression(node: &JsCallExpression) -> bool {
    if let Ok(callee) = node.callee() {
        match callee {
            AnyJsExpression::JsStaticMemberExpression(static_member) => {
                if let Ok(AnyJsExpression::JsCallExpression(call_expression)) =
                    static_member.object()
                {
                    return is_test_assertion_expression(&call_expression);
                }
                return false;
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                if let Ok(name) = identifier.name() {
                    if let Ok(value) = name.value_token() {
                        return ASSERTION_FUNCTION_NAMES.contains(&value.text_trimmed());
                    }
                }
                return false;
            }
            _ => {}
        }
    }
    false
}

/// Recursively crawl statements
fn expression_contains_expect(node: &AnyJsExpression) -> bool {
    match node {
        AnyJsExpression::JsArrowFunctionExpression(arrow_function) => {
            if let Ok(body) = arrow_function.body() {
                // Handle immediate return, e.g. test('name', () => expect(true));
                if let Some(body_return) = body.as_any_js_expression() {
                    if let Some(callee) = body_return.as_js_call_expression() {
                        return is_test_assertion_expression(callee);
                    }
                }
                // Handle arrow function block, e.g. test('name', () => { … });
                if let Some(body_block) = body.as_js_function_body() {
                    return statements_contain_expect(&body_block.statements());
                }
            }
            false
        }
        AnyJsExpression::JsCallExpression(call_expression) => {
            is_test_assertion_expression(call_expression)
        }
        AnyJsExpression::JsConditionalExpression(conditional_expression) => {
            if let Ok(left) = conditional_expression.test() {
                return expression_contains_expect(&left);
            }
            if let Ok(right) = conditional_expression.alternate() {
                return expression_contains_expect(&right);
            }
            false
        }
        AnyJsExpression::JsFunctionExpression(function) => {
            if let Ok(body) = function.body() {
                return statements_contain_expect(&body.statements());
            }
            false
        }
        _ => false,
    }
}

/// Detect if statements contain expect() or assert()
fn statements_contain_expect(statements: &JsStatementList) -> bool {
    statements.iter().any(|statement| match statement {
        AnyJsStatement::JsExpressionStatement(try_expression) => {
            if let Ok(expression) = try_expression.expression() {
                return expression_contains_expect(&expression);
            };
            false
        }
        AnyJsStatement::JsIfStatement(if_statement) => {
            if let Ok(body) = if_statement.consequent() {
                if maybe_block_contains_expect(&body) {
                    return true;
                }
            }
            if let Some(else_clause) = if_statement.else_clause() {
                if let Ok(else_alternate) = else_clause.alternate() {
                    match else_alternate {
                        AnyJsStatement::JsBlockStatement(else_block) => {
                            return statements_contain_expect(&else_block.statements());
                        }
                        AnyJsStatement::JsIfStatement(else_if_statement) => {
                            if let Ok(else_if_consequent) = else_if_statement.consequent() {
                                return maybe_block_contains_expect(&else_if_consequent);
                            }
                            if let Some(else_final_clause) = else_if_statement.else_clause() {
                                if let Ok(else_final_alternate) = else_final_clause.alternate() {
                                    return maybe_block_contains_expect(&else_final_alternate);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            false
        }
        AnyJsStatement::JsForStatement(for_statement) => {
            if let Ok(body) = for_statement.body() {
                return maybe_block_contains_expect(&body);
            }
            false
        }
        AnyJsStatement::JsForOfStatement(for_of_statement) => {
            if let Ok(body) = for_of_statement.body() {
                return maybe_block_contains_expect(&body);
            }
            false
        }
        AnyJsStatement::JsForInStatement(for_in_statement) => {
            if let Ok(body) = for_in_statement.body() {
                return maybe_block_contains_expect(&body);
            }
            false
        }
        AnyJsStatement::JsWhileStatement(while_statement) => {
            if let Ok(body) = while_statement.body() {
                return maybe_block_contains_expect(&body);
            }
            false
        }
        _ => false,
    })
}

/// Reduce code for any curly statement
fn maybe_block_contains_expect(node: &AnyJsStatement) -> bool {
    if let Some(try_block_statement) = node.as_js_block_statement() {
        return statements_contain_expect(&try_block_statement.statements());
    }
    false
}
