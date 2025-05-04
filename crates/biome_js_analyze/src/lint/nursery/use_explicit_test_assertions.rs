use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, RuleSourceKind, RuleSourceWithKind,
    context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, AnyJsStatement, JsCallExpression, JsFunctionBody};
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

        let [Some(second)] = node.arguments().ok()?.get_arguments_by_index([1]) else {
            return None;
        };
        let second_expr = second.as_any_js_expression()?;
        let try_function_body = match second_expr {
            AnyJsExpression::JsArrowFunctionExpression(function) => function.body().ok(),
            _ => None,
        };
        if let Some(function_body) = try_function_body {
            if let Some(js_function_body) = function_body.as_js_function_body() {
                if fn_body_contains_expect(js_function_body) {
                    return None;
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
fn is_test_assertion_expression(node: &JsCallExpression) -> Option<bool> {
    let member_name = node.callee().ok()?.get_callee_member_name()?;
    let member_trimmed = member_name.text_trimmed();
    match member_trimmed == "expect" || member_trimmed == "assert" {
        true => Some(true),
        _ => None,
    }
}

/// Detect if a function body has a call expression in given names list
fn fn_body_contains_expect(node: &JsFunctionBody) -> bool {
    node.statements().iter().any(|statement| match statement {
        AnyJsStatement::JsExpressionStatement(try_expression) => {
            if let Ok(expression) = try_expression.expression() {
                if let Some(call_expression) = expression.as_js_call_expression() {
                    match is_test_assertion_expression(call_expression) {
                        Some(true) => return true,
                        _ => return false,
                    }
                }
            }
            false
        }
        _ => false,
    })
}
