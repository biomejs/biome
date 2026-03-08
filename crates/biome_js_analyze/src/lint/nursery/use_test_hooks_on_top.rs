use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsModuleItem, AnyJsStatement, JsCallExpression, JsModule, JsScript};
use biome_rowan::{AstNode, TextRange, declare_node_union};
use biome_rule_options::use_test_hooks_on_top::UseTestHooksOnTopOptions;

use crate::frameworks::unit_tests::{
    LifecycleHook, describe_body_statements, is_describe_call, is_unit_test,
};

declare_lint_rule! {
    /// Enforce that lifecycle hooks appear before any test cases in the same block.
    ///
    /// Placing `beforeEach`, `beforeAll`, `afterEach`, and `afterAll` hooks after
    /// test cases (`it`, `test`) makes the setup and teardown harder to spot at a
    /// glance and can be a source of confusion for readers of the test suite.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// describe('foo', () => {
    ///   it('does something', () => {});
    ///   beforeEach(() => {});
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// describe('foo', () => {
    ///   beforeEach(() => {});
    ///   it('does something', () => {});
    /// });
    /// ```
    ///
    /// See also: [`useTestHooksInOrder`](https://biomejs.dev/linter/rules/use-test-hooks-in-order)
    ///
    pub UseTestHooksOnTop {
        version: "next",
        name: "useTestHooksOnTop",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        sources: &[
            RuleSource::EslintJest("prefer-hooks-on-top").same(),
            RuleSource::EslintVitest("prefer-hooks-on-top").same(),
        ],
        domains: &[RuleDomain::Test],
    }
}

declare_node_union! {
    /// A node that represents a scope containing test/hook calls:
    /// either the top-level module/script, or a `describe(...)` call.
    pub AnyTestScope = JsModule | JsScript | JsCallExpression
}

/// A lifecycle hook that is misplaced, together with the range of the first
/// test case that precedes it in the same block.
pub struct HookViolation {
    /// The misplaced hook call.
    pub hook: JsCallExpression,
    /// Range of the first test case in this scope that appears before the hook.
    pub first_test_range: TextRange,
}

impl Rule for UseTestHooksOnTop {
    type Query = Ast<AnyTestScope>;
    type State = HookViolation;
    type Signals = Vec<Self::State>;
    type Options = UseTestHooksOnTopOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let scope = ctx.query();

        // For a JsCallExpression, only act when it is a describe block.
        if let AnyTestScope::JsCallExpression(call) = scope {
            if !is_describe_call(call) {
                return vec![];
            }
            let Some(stmts) = describe_body_statements(call) else {
                return vec![];
            };
            return check_hooks_after_tests(&stmts);
        }

        // For the top-level module or script, gather the top-level statements.
        let stmts: Vec<AnyJsStatement> = match scope {
            AnyTestScope::JsModule(module) => module
                .items()
                .into_iter()
                .filter_map(|item| {
                    if let AnyJsModuleItem::AnyJsStatement(stmt) = item {
                        Some(stmt)
                    } else {
                        None
                    }
                })
                .collect(),
            AnyTestScope::JsScript(script) => script.statements().into_iter().collect(),
            AnyTestScope::JsCallExpression(_) => unreachable!(),
        };
        check_hooks_after_tests(&stmts)
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let hook_name =
            LifecycleHook::from_call_expression(&state.hook).map_or("hook", |h| h.as_str());
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.hook.range(),
                markup! {
                    <Emphasis>{hook_name}</Emphasis>" should appear before any test cases."
                },
            )
            .detail(
                state.first_test_range,
                markup! {
                    "Placing the hook after this test case makes it harder to spot the setup and teardown for these tests at a glance."
                },
            )
            .note(markup! {
                "Move the hook above all test cases in the same block for better readability."
            }),
        )
    }
}

/// Scans a flat list of statements and returns any lifecycle hook calls that
/// appear **after** at least one test case.
///
/// Only looks at the **direct children** of the current scope — does not
/// recurse. Recursion is handled by the query firing again for each nested
/// `describe` call.
fn check_hooks_after_tests(statements: &[AnyJsStatement]) -> Vec<HookViolation> {
    let mut first_test: Option<TextRange> = None;
    let mut violations: Vec<HookViolation> = vec![];

    for stmt in statements {
        let Some(call) = stmt
            .as_js_expression_statement()
            .and_then(|e| e.expression().ok())
            .and_then(|e| e.as_js_call_expression().cloned())
        else {
            continue;
        };

        if is_unit_test(&call) {
            if first_test.is_none() {
                first_test = Some(call.range());
            }
        } else if LifecycleHook::from_call_expression(&call).is_some()
            && let Some(first_test_range) = first_test
        {
            violations.push(HookViolation {
                hook: call,
                first_test_range,
            });
        }
        // describe blocks and other statements are ignored at this level.
    }

    violations
}
