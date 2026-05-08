use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsModuleItem, AnyJsStatement, JsCallExpression, JsExpressionStatement, JsModule,
    JsModuleItemList, JsScript, JsStatementList,
};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::use_test_hooks_in_order::UseTestHooksInOrderOptions;

use crate::frameworks::unit_tests::{LifecycleHook, describe_body, is_describe_call, is_unit_test};

declare_lint_rule! {
    /// Enforce that test lifecycle hooks are declared in the order they execute.
    ///
    /// Jest and Vitest always execute lifecycle hooks in the following order,
    /// regardless of how they are written in the file:
    ///
    /// 1. `beforeAll` (or `before` if you are using `node:test`)
    /// 2. `beforeEach`
    /// 3. `afterEach`
    /// 4. `afterAll` (or `after` if you are using `node:test`)
    ///
    /// Writing the hooks in a different order creates a discrepancy between
    /// the visual order in the source and the actual execution order, which
    /// makes test code harder to reason about.
    ///
    /// This rule flags any hook that appears after a hook that runs later in the
    /// execution order. Only consecutive groups of hooks in the same block are
    /// compared — test cases and other statements between hooks are allowed and
    /// reset the comparison baseline.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// describe('foo', () => {
    ///   beforeEach(() => {});
    ///   beforeAll(() => {});
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// describe('foo', () => {
    ///   afterEach(() => {});
    ///   afterAll(() => {});
    ///   beforeAll(() => {});
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// describe('foo', () => {
    ///   beforeAll(() => {});
    ///   beforeEach(() => {});
    ///   afterEach(() => {});
    ///   afterAll(() => {});
    /// });
    /// ```
    ///
    /// ```js
    /// // Hooks separated by test cases are treated independently.
    /// describe('foo', () => {
    ///   beforeEach(() => {});
    ///   it('a test', () => {});
    ///   afterAll(() => {});
    /// });
    /// ```
    ///
    /// See [`useTestHooksOnTop`](https://biomejs.dev/linter/rules/use-test-hooks-on-top) if you want to group all the hooks at the top of the block, before any test cases.
    ///
    pub UseTestHooksInOrder {
        version: "next",
        name: "useTestHooksInOrder",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        sources: &[
            RuleSource::EslintJest("prefer-hooks-in-order").same(),
            RuleSource::EslintVitest("prefer-hooks-in-order").same(),
            RuleSource::EslintPlaywright("prefer-hooks-in-order").same(),
        ],
        domains: &[RuleDomain::Test],
    }
}

/// A lifecycle hook that is out of order.
pub struct HookOrderViolation {
    /// The out-of-order hook.
    pub hook: JsCallExpression,
    /// The hook that precedes it but executes later, causing the violation.
    pub preceding_hook: JsCallExpression,
    /// The range of the preceding hook, for the detail span.
    pub preceding_hook_range: TextRange,
}

impl Rule for UseTestHooksInOrder {
    type Query = Ast<JsCallExpression>;
    type State = HookOrderViolation;
    type Signals = Vec<Self::State>;
    type Options = UseTestHooksInOrderOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let mut signals = Vec::new();

        if let Some(statement_list) = describe_body(call) {
            signals.extend(check_hooks_order(statement_list));
        }

        if let Some(module_items) = top_level_module_items_for_call(call) {
            let statements = || module_items.iter().filter_map(module_item_to_statement);
            if first_scope_call(statements()).is_some_and(|first| first.syntax() == call.syntax()) {
                signals.extend(check_hooks_order(statements()));
            }
        } else if let Some(statement_list) = top_level_statement_list_for_call(call) {
            let statements = || statement_list.clone().into_iter();
            if first_scope_call(statements()).is_some_and(|first| first.syntax() == call.syntax()) {
                signals.extend(check_hooks_order(statements()));
            }
        }

        signals
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let current_name =
            LifecycleHook::from_call_expression(&state.hook).map_or("hook", |h| h.as_str());
        let preceding_name = LifecycleHook::from_call_expression(&state.preceding_hook)
            .map_or("hook", |h| h.as_str());
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.hook.range(),
                markup! {
                    <Emphasis>{current_name}</Emphasis>" is out of order compared to "<Emphasis>{preceding_name}</Emphasis>"."
                },
            )
            .detail(
                state.preceding_hook_range,
                markup! {
                    <Emphasis>{preceding_name}</Emphasis>" is declared here but executes after "<Emphasis>{current_name}</Emphasis>"."
                },
            )
            .note(markup! {
                "Reorder the lifecycle hooks to appear in the order they execute: "<Emphasis>"beforeAll/before"</Emphasis>", "<Emphasis>"beforeEach"</Emphasis>", "<Emphasis>"afterEach"</Emphasis>", "<Emphasis>"afterAll/after"</Emphasis>"."
            }),
        )
    }
}

/// Scans a flat list of statements for lifecycle hooks that are declared out of
/// their canonical execution order.
///
/// Hooks are compared within contiguous groups: any non-hook statement (test
/// cases, variable declarations, describe blocks, etc.) resets the comparison
/// window.
fn check_hooks_order(stmts: impl IntoIterator<Item = AnyJsStatement>) -> Vec<HookOrderViolation> {
    let mut violations = vec![];
    // Tracks the last hook seen in the current contiguous run.
    let mut last_hook: Option<(JsCallExpression, LifecycleHook)> = None;

    for stmt in stmts {
        let Some(call) = stmt
            .as_js_expression_statement()
            .and_then(|e| e.expression().ok())
            .and_then(|e| e.as_js_call_expression().cloned())
        else {
            // A non-call statement breaks the contiguous hook run.
            last_hook = None;
            continue;
        };

        if let Some(hook) = LifecycleHook::from_call_expression(&call) {
            if let Some((ref prev_call, prev_hook)) = last_hook
                && hook < prev_hook
            {
                // The current hook executes *before* the previous one — violation.
                violations.push(HookOrderViolation {
                    hook: call.clone(),
                    preceding_hook: prev_call.clone(),
                    preceding_hook_range: prev_call.range(),
                });
            }

            last_hook = Some((call, hook));
        } else {
            // A non-hook call (test case, describe, utility call) breaks the run.
            last_hook = None;
        }
    }

    violations
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
