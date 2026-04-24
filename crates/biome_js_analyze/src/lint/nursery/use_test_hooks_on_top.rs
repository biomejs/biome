use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, RuleSuppressions, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsModuleItem, AnyJsStatement, JsCallExpression, JsExpressionStatement, JsLanguage,
    JsModule, JsModuleItemList, JsScript, JsStatementList,
};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::use_test_hooks_on_top::UseTestHooksOnTopOptions;

use crate::frameworks::unit_tests::{LifecycleHook, describe_body, is_describe_call, is_unit_test};

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
    pub UseTestHooksOnTop {
        version: "next",
        name: "useTestHooksOnTop",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        sources: &[
            RuleSource::EslintJest("prefer-hooks-on-top").same(),
            RuleSource::EslintVitest("prefer-hooks-on-top").same(),
            RuleSource::EslintPlaywright("prefer-hooks-on-top").same(),
        ],
        domains: &[RuleDomain::Test],
    }
}

impl Rule for UseTestHooksOnTop {
    type Query = Ast<JsCallExpression>;
    type State = UseTestHooksOnTopState;
    type Signals = Vec<Self::State>;
    type Options = UseTestHooksOnTopOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let mut signals = Vec::new();

        if let Some(statement_list) = describe_body(call) {
            signals.extend(scan_statements(statement_list));
        }

        if let Some(module_items) = top_level_module_items_for_call(call) {
            let statements = || module_items.iter().filter_map(module_item_to_statement);
            if first_scope_call(statements()).is_some_and(|first| first.syntax() == call.syntax()) {
                signals.extend(scan_statements(statements()));
            }
        } else if let Some(statement_list) = top_level_statement_list_for_call(call) {
            let statements = || statement_list.clone().into_iter();
            if first_scope_call(statements()).is_some_and(|first| first.syntax() == call.syntax()) {
                signals.extend(scan_statements(statements()));
            }
        }

        signals
    }

    fn suppressed_nodes(
        _ctx: &RuleContext<Self>,
        state: &Self::State,
        suppressions: &mut RuleSuppressions<JsLanguage>,
    ) {
        suppressions.suppress_node(state.hook.syntax().clone());
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let hook_name = LifecycleHook::from_call_expression(&state.hook).map(|h| h.as_str())?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.hook.range(),
                markup! {
                    "Lifecycle hook "<Emphasis>{hook_name}</Emphasis>" appears after a test case."
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

#[derive(Clone)]
pub struct UseTestHooksOnTopState {
    hook: JsCallExpression,
    first_test_range: TextRange,
}

/// Scans one block of sibling statements and reports hooks that appear after
/// the first test case in that same block.
fn scan_statements(
    statements: impl IntoIterator<Item = AnyJsStatement>,
) -> Vec<UseTestHooksOnTopState> {
    let mut first_test_range = None;
    let mut signals = Vec::new();

    for statement in statements {
        let Some(call) = call_from_statement(&statement) else {
            continue;
        };

        if first_test_range.is_none() && is_unit_test(&call) {
            first_test_range = Some(call.range());
            continue;
        }

        if let Some(first_test_range) = first_test_range
            && LifecycleHook::from_call_expression(&call).is_some()
        {
            signals.push(UseTestHooksOnTopState {
                hook: call,
                first_test_range,
            });
        }
    }

    signals
}

fn top_level_module_items_for_call(call: &JsCallExpression) -> Option<JsModuleItemList> {
    let expr_stmt = call.parent::<JsExpressionStatement>()?;
    let module_items = expr_stmt.parent::<JsModuleItemList>()?;
    module_items.parent::<JsModule>()?;
    Some(module_items)
}

fn top_level_statement_list_for_call(call: &JsCallExpression) -> Option<JsStatementList> {
    let expr_stmt = call.parent::<JsExpressionStatement>()?;
    let statement_list = expr_stmt.parent::<JsStatementList>()?;
    statement_list.parent::<JsScript>()?;
    Some(statement_list)
}

fn module_item_to_statement(item: AnyJsModuleItem) -> Option<AnyJsStatement> {
    match item {
        AnyJsModuleItem::AnyJsStatement(statement) => Some(statement),
        _ => None,
    }
}

fn first_scope_call(
    statements: impl IntoIterator<Item = AnyJsStatement>,
) -> Option<JsCallExpression> {
    statements
        .into_iter()
        .filter_map(|statement| call_from_statement(&statement))
        .find(|call| {
            is_describe_call(call)
                || is_unit_test(call)
                || LifecycleHook::from_call_expression(call).is_some()
        })
}

fn call_from_statement(statement: &AnyJsStatement) -> Option<JsCallExpression> {
    let expr_stmt = statement.as_js_expression_statement()?;
    let expression = expr_stmt.expression().ok()?;
    expression.as_js_call_expression().cloned()
}
